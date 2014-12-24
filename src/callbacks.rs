// Copyright 2013 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Private callback support functions.

use libc::{c_double, c_int, c_uint};
use std::mem;

use super::*;

macro_rules! callback(
    (
        type Args = ($($arg:ident: $arg_ty:ty),*);
        type Callback = $Callback:ident;
        let ext_set = $ext_set:expr;
        fn callback($($ext_arg:ident: $ext_arg_ty:ty),*) $call:expr
    ) => (
        thread_local!(static CALLBACK_KEY: RefCell<Option<Box<Object<Args> + 'static>>> = RefCell::new(None));

        type Args = ($($arg_ty),*,);

        trait Object<T> {
            fn call(&self, args: T);
        }

        impl<UserData> Object<Args> for ::Callback<fn($($arg_ty),*, &UserData), UserData> {
            fn call(&self, ($($arg),*,): Args) {
                (self.f)($($arg),*, &self.data);
            }
        }

        pub fn set<UserData: 'static>(f: ::$Callback<UserData>) {
            let mut boxed_cb = Some(box f as Box<Object<Args> + 'static>);
            CALLBACK_KEY.with(|cb| {
                *cb.borrow_mut() = boxed_cb.take();
            });
            ($ext_set)(Some(callback as extern "C" fn($($ext_arg: $ext_arg_ty),*)));
        }

        pub fn unset() {
            CALLBACK_KEY.with(|cb| {
                *cb.borrow_mut() = None;
            });
            ($ext_set)(None);
        }

        extern "C" fn callback($($ext_arg: $ext_arg_ty),*) {
            CALLBACK_KEY.with(|cb| {
                match *cb.borrow() {
                    Some(ref cb) => unsafe { cb.call($call) },
                    _ => {}
                }
            })
        }
    )
);

pub mod error {
    use libc::{c_int, c_char};
    use std::cell::RefCell;
    use std::mem;
    use std::string;

    callback!(
        type Args = (error: ::Error, description: String);
        type Callback = ErrorCallback;
        let ext_set = |cb| unsafe { ::ffi::glfwSetErrorCallback(cb) };
        fn callback(error: c_int, description: *const c_char) {
            (mem::transmute(error), string::String::from_raw_buf(
                description as *const u8))
        }
    );
}

pub mod monitor {
    use libc::{c_int};
    use std::cell::RefCell;
    use std::mem;
    use std::kinds::marker;

    callback!(
        type Args = (monitor: ::Monitor, event: ::MonitorEvent);
        type Callback = MonitorCallback;
        let ext_set = |cb| unsafe { ::ffi::glfwSetMonitorCallback(cb) };
        fn callback(monitor: *mut ::ffi::GLFWmonitor, event: c_int) {
            let monitor = ::Monitor {
                ptr: monitor,
                no_copy: marker::NoCopy,
                no_send: marker::NoSend,
                no_share: marker::NoSync,
            };
            (monitor, mem::transmute(event))
        }
    );
}

unsafe fn get_sender<'a>(window: &'a *mut ffi::GLFWwindow) -> &'a Sender<(f64, WindowEvent)> {
    mem::transmute(ffi::glfwGetWindowUserPointer(*window))
}

// Note that this macro creates a static function pointer rather than a plain function.
// This makes it more ergonomic to embed in an Option; see set_window_callback! in lib.rs
macro_rules! window_callback(
    (fn $name:ident () => $event:ident) => (
        pub static $name: (extern "C" fn(window: *mut ffi::GLFWwindow)) = {
            extern "C" fn actual_callback(window: *mut ffi::GLFWwindow) {
                unsafe { get_sender(&window).send((ffi::glfwGetTime() as f64, WindowEvent::$event));}
            }
            actual_callback
        };
     );
    (fn $name:ident ($($ext_arg:ident: $ext_arg_ty:ty),*) => $event:ident($($arg_conv:expr),*)) => (
        pub static $name: (extern "C" fn(window: *mut ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*)) = {
            extern "C" fn actual_callback(window: *mut ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
                unsafe { get_sender(&window).send((ffi::glfwGetTime() as f64, WindowEvent::$event($($arg_conv),*))); }
            }
            actual_callback
        };
     );
);

window_callback!(fn window_pos_callback(xpos: c_int, ypos: c_int)                           => Pos(xpos as i32, ypos as i32));
window_callback!(fn window_size_callback(width: c_int, height: c_int)                       => Size(width as i32, height as i32));
window_callback!(fn window_close_callback()                                                 => Close);
window_callback!(fn window_refresh_callback()                                               => Refresh);
window_callback!(fn window_focus_callback(focused: c_int)                                   => Focus(focused == ffi::TRUE));
window_callback!(fn window_iconify_callback(iconified: c_int)                               => Iconify(iconified == ffi::TRUE));
window_callback!(fn framebuffer_size_callback(width: c_int, height: c_int)                  => FramebufferSize(width as i32, height as i32));
window_callback!(fn mouse_button_callback(button: c_int, action: c_int, mods: c_int)        => MouseButton(mem::transmute(button), mem::transmute(action), Modifiers::from_bits(mods).unwrap()));
window_callback!(fn cursor_pos_callback(xpos: c_double, ypos: c_double)                     => CursorPos(xpos as f64, ypos as f64));
window_callback!(fn cursor_enter_callback(entered: c_int)                                   => CursorEnter(entered == ffi::TRUE));
window_callback!(fn scroll_callback(xpos: c_double, ypos: c_double)                         => Scroll(xpos as f64, ypos as f64));
window_callback!(fn key_callback(key: c_int, scancode: c_int, action: c_int, mods: c_int)   => Key(mem::transmute(key), scancode, mem::transmute(action), Modifiers::from_bits(mods).unwrap()));
window_callback!(fn char_callback(character: c_uint)                                        => Char(::std::char::from_u32(character).unwrap()));
