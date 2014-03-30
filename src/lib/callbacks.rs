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

use std::cast;
use std::libc::*;
use std::local_data;
use std::str;
use sync::one::{Once, ONCE_INIT};

use super::*;

// Global callbacks

local_data_key!(ERROR_SENDER: Sender<(f64, Error, ~str)>)
local_data_key!(MONITOR_CALLBACK: ~MonitorCallback:'static)

pub extern "C" fn error_callback(error: c_int, description: *c_char) {
    unsafe {
        local_data::get(ERROR_SENDER, |sender| {
            sender.as_ref().expect("ERROR_SENDER not initialized")
                  .send((ffi::glfwGetTime() as f64, cast::transmute(error), str::raw::from_c_str(description)))
        });
    }
}

pub fn init_error_receiver() -> Option<Receiver<(f64, Error, ~str)>> {
    static mut INIT: Once = ONCE_INIT;
    let mut errors = None;
    unsafe {
        INIT.doit(|| {
            let (sender, receiver) = channel();
            local_data::set(ERROR_SENDER, sender);
            ffi::glfwSetErrorCallback(Some(error_callback));
            errors = Some(receiver);
        });
    }
    errors
}

pub extern "C" fn monitor_callback(monitor: *ffi::GLFWmonitor, event: c_int) {
    local_data::get(MONITOR_CALLBACK, (|data| {
        data.as_ref().map(|&ref cb| {
            unsafe { cb.call(&Monitor { ptr: monitor }, cast::transmute(event)) }
        });
    }));
}

pub fn set_monitor_callback(callback: ~MonitorCallback:'static, f: |ffi::GLFWmonitorfun| ) {
    local_data::set(MONITOR_CALLBACK, callback);
    f(monitor_callback);
}

unsafe fn get_sender<'a>(window: &'a *ffi::GLFWwindow) -> &'a Sender<(f64, WindowEvent)> {
    cast::transmute(ffi::glfwGetWindowUserPointer(*window))
}

macro_rules! window_callback(
    (fn $name:ident () => $event:ident) => (
        pub extern "C" fn $name(window: *ffi::GLFWwindow) {
            unsafe { get_sender(&window).send((ffi::glfwGetTime() as f64, $event)); }
        }
     );
    (fn $name:ident ($($ext_arg:ident: $ext_arg_ty:ty),*) => $event:ident($($arg_conv:expr),*)) => (
        pub extern "C" fn $name(window: *ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
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
window_callback!(fn mouse_button_callback(button: c_int, action: c_int, mods: c_int)        => MouseButtonEvent(cast::transmute(button), cast::transmute(action), Modifiers { values: mods }))
window_callback!(fn cursor_pos_callback(xpos: c_double, ypos: c_double)                     => CursorPosEvent(xpos as f64, ypos as f64))
window_callback!(fn cursor_enter_callback(entered: c_int)                                   => CursorEnterEvent(entered == ffi::TRUE))
window_callback!(fn scroll_callback(xpos: c_double, ypos: c_double)                         => ScrollEvent(xpos as f64, ypos as f64))
window_callback!(fn key_callback(key: c_int, scancode: c_int, action: c_int, mods: c_int)   => KeyEvent(cast::transmute(key), scancode, cast::transmute(action), Modifiers { values: mods }))
window_callback!(fn char_callback(character: c_uint)                                        => CharEvent(::std::char::from_u32(character).unwrap()))
