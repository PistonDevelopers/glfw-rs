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

use libc::{c_char, c_double, c_int, c_uint};
use std::ffi::CStr;
use std::mem;
use std::path::PathBuf;
use std::slice;
use std::str;
use std::sync::mpsc::Sender;

use super::*;

macro_rules! callback (
    (
        type Args = ($($arg:ident: $arg_ty:ty),*);
        type Callback = $Callback:ident;
        let ext_set = $ext_set:expr;
        fn callback($($ext_arg:ident: $ext_arg_ty:ty),*) $call:expr
    ) => (
        thread_local!(static CALLBACK_KEY: RefCell<Option<Box<dyn Object<Args> + 'static>>> = RefCell::new(None));

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
            let mut boxed_cb = Some(Box::new(f) as Box<dyn Object<Args> + 'static>);
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
    use libc::{c_char, c_int};
    use std::cell::RefCell;
    use std::mem;

    callback!(
        type Args = (error: ::Error, description: String);
        type Callback = ErrorCallback;
        let ext_set = |cb| unsafe { ::ffi::glfwSetErrorCallback(cb) };
        fn callback(error: c_int, description: *const c_char) {
            (mem::transmute(error), ::string_from_c_str(description))
        }
    );
}

pub mod monitor {
    use libc::c_int;
    use std::cell::RefCell;
    use std::mem;

    callback!(
        type Args = (monitor: ::Monitor, event: ::MonitorEvent);
        type Callback = MonitorCallback;
        let ext_set = |cb| unsafe { ::ffi::glfwSetMonitorCallback(cb) };
        fn callback(monitor: *mut ::ffi::GLFWmonitor, event: c_int) {
            let monitor = ::Monitor {
                ptr: monitor
            };
            (monitor, mem::transmute(event))
        }
    );
}

pub mod joystick {
    use libc::c_int;
    use std::cell::RefCell;
    use std::mem;

    callback!(
        type Args = (joystick_id: ::JoystickId, event: ::JoystickEvent);
        type Callback = JoystickCallback;
        let ext_set = |cb| unsafe { ::ffi::glfwSetJoystickCallback(cb) };
        fn callback(joystick_id: c_int, event: c_int) {
            (mem::transmute(joystick_id), mem::transmute(event))
        }
    );
}

unsafe fn get_sender<'a>(window: &'a *mut ffi::GLFWwindow) -> &'a Sender<(f64, WindowEvent)> {
    mem::transmute(ffi::glfwGetWindowUserPointer(*window))
}

pub mod unbuffered {
    use crate::{WindowEvent, WindowId};
    use std::cell::RefCell;

    type CallbackPtr = *mut std::ffi::c_void;
    type HandlerFn = fn(
        window_id: WindowId,
        event: (f64, WindowEvent),
        callback_ptr: CallbackPtr,
    ) -> Option<(f64, WindowEvent)>;

    thread_local! {
        static HANDLER: RefCell<Option<(HandlerFn, CallbackPtr)>> = RefCell::new(None);
    }

    pub struct UnsetHandlerGuard {
        _private: (),
    }

    impl Drop for UnsetHandlerGuard {
        fn drop(&mut self) {
            HANDLER.with(|ref_cell| {
                *ref_cell.borrow_mut() = None;
            })
        }
    }

    pub unsafe fn handle(
        window_id: WindowId,
        event: (f64, WindowEvent),
    ) -> Option<(f64, WindowEvent)> {
        HANDLER.with(|ref_cell| {
            if let Some((handler, callback_ptr)) = *ref_cell.borrow() {
                handler(window_id, event, callback_ptr)
            } else {
                Some(event)
            }
        })
    }

    pub unsafe fn set_handler<F>(callback: &mut F) -> UnsetHandlerGuard
    where
        F: FnMut(WindowId, (f64, WindowEvent)) -> Option<(f64, WindowEvent)>,
    {
        fn handler<F>(
            window_id: WindowId,
            event: (f64, WindowEvent),
            callback_ptr: CallbackPtr,
        ) -> Option<(f64, WindowEvent)>
        where
            F: FnMut(WindowId, (f64, WindowEvent)) -> Option<(f64, WindowEvent)>,
        {
            unsafe {
                let callback: &mut F = &mut *(callback_ptr as *mut F);
                callback(window_id, event)
            }
        }
        HANDLER.with(|ref_cell| {
            let callback_ptr = callback as *mut F as CallbackPtr;
            *ref_cell.borrow_mut() = Some((handler::<F>, callback_ptr));
        });
        UnsetHandlerGuard { _private: () }
    }
}

// Note that this macro creates a static function pointer rather than a plain function.
// This makes it more ergonomic to embed in an Option; see set_window_callback! in lib.rs
macro_rules! window_callback (
    (fn $name:ident () => $event:ident) => (
        pub static $name: extern "C" fn(window: *mut ffi::GLFWwindow) = {
            extern "C" fn actual_callback(window: *mut ffi::GLFWwindow) {
                unsafe {
                    let event = (ffi::glfwGetTime() as f64, WindowEvent::$event);
                    if let Some(event) = unbuffered::handle(window as WindowId, event) {
                        get_sender(&window).send(event).unwrap();
                    }
                }
            }
            actual_callback
        };
     );
    (fn $name:ident ($($ext_arg:ident: $ext_arg_ty:ty),*) => $event:ident($($arg_conv:expr),*)) => (
        pub static $name: extern "C" fn(window: *mut ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) = {
            extern "C" fn actual_callback(window: *mut ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
                unsafe {
                    let event = (ffi::glfwGetTime() as f64, WindowEvent::$event($($arg_conv),*));
                    if let Some(event) = unbuffered::handle(window as WindowId, event) {
                        get_sender(&window).send(event).unwrap();
                    }
                }
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
window_callback!(fn char_mods_callback(character: c_uint, mods: c_int)                      => CharModifiers(::std::char::from_u32(character).unwrap(), Modifiers::from_bits(mods).unwrap()));
window_callback!(fn drop_callback(num_paths: c_int, paths: *mut *const c_char)              => FileDrop(slice::from_raw_parts(paths, num_paths as usize).iter().map(|path| PathBuf::from(str::from_utf8(CStr::from_ptr(*path).to_bytes()).unwrap().to_string())).collect()));
window_callback!(fn window_maximize_callback(maximized: c_int)                              => Maximize(maximized == ffi::TRUE));
window_callback!(fn window_content_scale_callback(xscale: c_float, yscale: c_float)         => ContentScale(xscale as f32, yscale as f32));
