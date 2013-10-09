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
use std::local_data;
use std::str;

use super::*;

// Global callbacks

static error_fun_tls_key: local_data::Key<ErrorFun> = &local_data::Key;

pub extern "C" fn error_callback(error: c_int, description: *c_char) {
    do local_data::get(error_fun_tls_key) |data| {
        do data.as_ref().map |&ref cb| {
            unsafe { (**cb)(cast::transmute(error as int), str::raw::from_c_str(description)) }
        };
    }
}

pub fn set_error_fun(cbfun: ErrorFun, f: &fn(ffi::GLFWerrorfun) ) {
    local_data::set(error_fun_tls_key, cbfun);
    f(error_callback);
}

static monitor_fun_tls_key: local_data::Key<MonitorFun> = &local_data::Key;

pub extern "C" fn monitor_callback(monitor: *ffi::GLFWmonitor, event: c_int) {
    do local_data::get(monitor_fun_tls_key) |data| {
        do data.as_ref().map |&ref cb| {
            unsafe { (**cb)(&Monitor { ptr: monitor }, cast::transmute(event as int)) }
        };
    }
}

pub fn set_monitor_fun(cbfun: MonitorFun, f: &fn(ffi::GLFWmonitorfun) ) {
    local_data::set(monitor_fun_tls_key, cbfun);
    f(monitor_callback);
}

// External window callbacks
#[fixed_stack_segment]
unsafe fn get_fns(window: *ffi::GLFWwindow) -> &WindowFns {
    cast::transmute(ffi::glfwGetWindowUserPointer(window))
}

macro_rules! window_callback(
    (fn $name:ident () => $field:ident()) => (
         pub extern "C" fn $name(window: *ffi::GLFWwindow) {
            unsafe {
                let window = Window { ptr: window, is_shared: false };
                window.get_fns().$field.as_ref().map(|cb| (*cb)(&window));
                cast::forget(window);
            }
         }
     );
    (fn $name:ident ($($ext_arg:ident: $ext_arg_ty:ty),*) => $field:ident($($arg_conv:expr),*)) => (
         pub extern "C" fn $name(window: *ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
            unsafe {
                let window = Window { ptr: window, is_shared: false };
                window.get_fns().$field.as_ref().map(|cb| (*cb)(&window $(, $arg_conv)*));
                cast::forget(window);
            }
         }
     );
)

window_callback!(fn window_pos_callback(xpos: c_int, ypos: c_int)                           => pos_fun(xpos as int, ypos as int))
window_callback!(fn window_size_callback(width: c_int, height: c_int)                       => size_fun(width as int, height as int))
window_callback!(fn window_close_callback()                                                 => close_fun())
window_callback!(fn window_refresh_callback()                                               => refresh_fun())
window_callback!(fn window_focus_callback(focused: c_int)                                   => focus_fun(focused == ffi::TRUE))
window_callback!(fn window_iconify_callback(iconified: c_int)                               => iconify_fun(iconified == ffi::TRUE))
window_callback!(fn framebuffer_size_callback(width: c_int, height: c_int)                  => framebuffer_size_fun(width as int, height as int))
window_callback!(fn mouse_button_callback(button: c_int, action: c_int, mods: c_int)        => mouse_button_fun(cast::transmute(button as int), cast::transmute(action as int), Modifiers { values: mods }))
window_callback!(fn cursor_pos_callback(xpos: c_double, ypos: c_double)                     => cursor_pos_fun(xpos as f64, ypos as f64))
window_callback!(fn cursor_enter_callback(entered: c_int)                                   => cursor_enter_fun(entered == ffi::TRUE))
window_callback!(fn scroll_callback(xpos: c_double, ypos: c_double)                         => scroll_fun(xpos as f64, ypos as f64))
window_callback!(fn key_callback(key: c_int, scancode: c_int, action: c_int, mods: c_int)   => key_fun(cast::transmute(key as int), scancode, cast::transmute(action as int), Modifiers { values: mods }))
window_callback!(fn char_callback(character: c_uint)                                        => char_fun(::std::char::from_u32(character).unwrap()))
