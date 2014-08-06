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
        local_data_key!(CALLBACK_KEY: Box<Object<Args> + 'static>)

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
            CALLBACK_KEY.replace(Some(box f as Box<Object<Args> + 'static>));
            ($ext_set)(Some(callback));
        }

        pub fn unset() {
            CALLBACK_KEY.replace(None);
            ($ext_set)(None);
        }

        extern "C" fn callback($($ext_arg: $ext_arg_ty),*) {
            match CALLBACK_KEY.get() {
                Some(cb) => unsafe { cb.call($call) },
                _ => {}
            }
        }
    )
)

pub mod error {
    use libc::{c_int, c_char};
    use std::mem;
    use std::str;

    callback!(
        type Args = (error: ::Error, description: String);
        type Callback = ErrorCallback;
        let ext_set = |cb| unsafe { ::ffi::glfwSetErrorCallback(cb) };
        fn callback(error: c_int, description: *const c_char) {
            (mem::transmute(error), str::raw::from_c_str(description))
        }
    )
}

pub mod monitor {
    use libc::{c_int};
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
                no_share: marker::NoShare,
            };
            (monitor, mem::transmute(event))
        }
    )
}

unsafe fn get_sender<'a>(window: &'a *mut ffi::GLFWwindow) -> &'a Sender<(f64, WindowEvent)> {
    mem::transmute(ffi::glfwGetWindowUserPointer(*window))
}

macro_rules! window_callback(
    (fn $name:ident () => $event:ident) => (
        pub extern "C" fn $name(window: *mut ffi::GLFWwindow) {
            unsafe { get_sender(&window).send((ffi::glfwGetTime() as f64, $event)); }
        }
     );
    (fn $name:ident ($($ext_arg:ident: $ext_arg_ty:ty),*) => $event:ident($($arg_conv:expr),*)) => (
        pub extern "C" fn $name(window: *mut ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
            unsafe { get_sender(&window).send((ffi::glfwGetTime() as f64, $event($($arg_conv),*))); }
        }
     );
)

window_callback!(fn window_pos_callback(xpos: c_int, ypos: c_int)                           => PosEvent(xpos as i32, ypos as i32))
window_callback!(fn window_size_callback(width: c_int, height: c_int)                       => SizeEvent(width as i32, height as i32))
window_callback!(fn window_close_callback()                                                 => CloseEvent)
window_callback!(fn window_refresh_callback()                                               => RefreshEvent)
window_callback!(fn window_focus_callback(focused: c_int)                                   => FocusEvent(focused == ffi::TRUE))
window_callback!(fn window_iconify_callback(iconified: c_int)                               => IconifyEvent(iconified == ffi::TRUE))
window_callback!(fn framebuffer_size_callback(width: c_int, height: c_int)                  => FramebufferSizeEvent(width as i32, height as i32))
window_callback!(fn mouse_button_callback(button: c_int, action: c_int, mods: c_int)        => MouseButtonEvent(mem::transmute(button), mem::transmute(action), Modifiers::from_bits(mods).unwrap()))
window_callback!(fn cursor_pos_callback(xpos: c_double, ypos: c_double)                     => CursorPosEvent(xpos as f64, ypos as f64))
window_callback!(fn cursor_enter_callback(entered: c_int)                                   => CursorEnterEvent(entered == ffi::TRUE))
window_callback!(fn scroll_callback(xpos: c_double, ypos: c_double)                         => ScrollEvent(xpos as f64, ypos as f64))
window_callback!(fn key_callback(key: c_int, scancode: c_int, action: c_int, mods: c_int)   => KeyEvent(mem::transmute(key), scancode, mem::transmute(action), Modifiers::from_bits(mods).unwrap()))
window_callback!(fn char_callback(character: c_uint)                                        => CharEvent(::std::char::from_u32(character).unwrap()))
