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

//! Private functions and items to be used with the high-level library wrapper.

use std::cast;
use std::libc::*;
// use std::local_data;
// use std::str;

use super::*;

// Global callbacks

// static error_callback_tls_key: local_data::Key<~ErrorCallback> = &local_data::Key;
// static monitor_callback_tls_key: local_data::Key<~MonitorCallback> = &local_data::Key;

// pub extern "C" fn error_callback(error: c_int, description: *c_char) {
//     local_data::get(error_callback_tls_key, (|data| {
//         data.as_ref().map(|&ref cb| {
//             unsafe { cb.call(cast::transmute(error), str::raw::from_c_str(description)) }
//         });
//     }));
// }

// pub fn set_error_callback<Cb: ErrorCallback + Send>(callback: ~Cb, f: |ffi::GLFWerrorfun| ) {
//     local_data::set(error_callback_tls_key, callback as ~ErrorCallback);
//     f(error_callback);
// }


// pub extern "C" fn monitor_callback(monitor: *ffi::GLFWmonitor, event: c_int) {
//     local_data::get(monitor_callback_tls_key, (|data| {
//         data.as_ref().map(|&ref cb| {
//             unsafe { cb.call(&Monitor { ptr: monitor }, cast::transmute(event)) }
//         });
//     }));
// }

// pub fn set_monitor_callback<Cb: MonitorCallback + Send>(callback: ~Cb, f: |ffi::GLFWmonitorfun| ) {
//     local_data::set(monitor_callback_tls_key, callback as ~MonitorCallback);
//     f(monitor_callback);
// }

macro_rules! window_callback(
    (fn $name:ident() => $msg:expr) => (
         pub extern "C" fn $name(window: *ffi::GLFWwindow) {
            unsafe {
                let chan: &Chan<WindowEvent> =
                    cast::transmute(ffi::glfwGetWindowUserPointer(window));
                chan.send($msg);
            }
         }
     );
    (fn $name:ident($($ext_arg:ident: $ext_arg_ty:ty),*) => $msg:expr) => (
         pub extern "C" fn $name(window: *ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
            unsafe {
                let chan: &Chan<WindowEvent> =
                    cast::transmute(ffi::glfwGetWindowUserPointer(window));
                chan.send($msg);
            }
         }
     );
)

window_callback!(fn window_pos_callback(xpos: c_int, ypos: c_int)                           => WindowPos(xpos as i32, ypos as i32))
window_callback!(fn window_size_callback(width: c_int, height: c_int)                       => WindowSize(width as i32, height as i32))
window_callback!(fn window_close_callback()                                                 => WindowClose)
window_callback!(fn window_refresh_callback()                                               => WindowRefresh)
window_callback!(fn window_focus_callback(focused: c_int)                                   => WindowFocus(focused == ffi::TRUE))
window_callback!(fn window_iconify_callback(iconified: c_int)                               => WindowIconify(iconified == ffi::TRUE))
window_callback!(fn framebuffer_size_callback(width: c_int, height: c_int)                  => FramebufferSize(width as i32, height as i32))
window_callback!(fn mouse_button_callback(button: c_int, action: c_int, mods: c_int)        => MouseButton(cast::transmute(button), cast::transmute(action), Modifiers { values: mods }))
window_callback!(fn cursor_pos_callback(xpos: c_double, ypos: c_double)                     => CursorPos(xpos as f64, ypos as f64))
window_callback!(fn cursor_enter_callback(entered: c_int)                                   => CursorEnter(entered == ffi::TRUE))
window_callback!(fn scroll_callback(xpos: c_double, ypos: c_double)                         => Scroll(xpos as f64, ypos as f64))
window_callback!(fn key_callback(key: c_int, scancode: c_int, action: c_int, mods: c_int)   => Key(cast::transmute(key), Scancode(scancode), cast::transmute(action), Modifiers { values: mods }))
window_callback!(fn char_callback(character: c_uint)                                        => Char(::std::char::from_u32(character).unwrap()))
