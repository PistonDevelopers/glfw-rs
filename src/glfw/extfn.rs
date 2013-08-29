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
    unsafe {
        do local_data::get(error_fun_tls_key) |data| {
            do data.map |& &cb| {
                cb(error, str::raw::from_c_str(description))
            };
        }
    }
}

pub fn set_error_fun(cbfun: ErrorFun, f: &fn(ffi::GLFWerrorfun) ) {
    local_data::set(error_fun_tls_key, cbfun);
    f(error_callback);
}

static monitor_fun_tls_key: local_data::Key<MonitorFun> = &local_data::Key;

pub extern "C" fn monitor_callback(monitor: *ffi::GLFWmonitor, event: c_int) {
    do local_data::get(monitor_fun_tls_key) |data| {
        do data.map |& &cb| {
            cb(&Monitor { ptr: monitor }, event)
        };
    }
}

pub fn set_monitor_fun(cbfun: MonitorFun, f: &fn(ffi::GLFWmonitorfun) ) {
    local_data::set(monitor_fun_tls_key, cbfun);
    f(monitor_callback);
}


// External window callbacks

unsafe fn chan_from_ptr(ptr: *c_void) -> &Chan<WindowEvent> { cast::transmute(ptr) }

macro_rules! window_callback(
    (fn $name:ident () => $event:ident) => (
        pub extern "C" fn $name(window: *ffi::GLFWwindow) {
            let chan = unsafe { chan_from_ptr(ffi::glfwGetWindowUserPointer(window)) };
            chan.send($event);
        }
    );
    (fn $name:ident ($($ext_arg:ident: $ext_arg_ty:ty),*) => $event:ident($($arg_conv:expr),*)) => (
        pub extern "C" fn $name(window: *ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
            let chan = unsafe { chan_from_ptr(ffi::glfwGetWindowUserPointer(window)) };
            chan.send($event( $( $arg_conv),* ));
        }
    );
    (fn $name:ident ($($ext_arg:ident: $ext_arg_ty:ty),*) => $event:ident { $($fname:ident : $arg_conv:expr),* }) => (
        pub extern "C" fn $name(window: *ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
            let chan = unsafe { chan_from_ptr(ffi::glfwGetWindowUserPointer(window)) };
            chan.send($event{ $( $fname : $arg_conv),* });
        }
    );
)

window_callback!(fn window_pos_callback(xpos: c_int, ypos: c_int)                           => Pos { xpos: xpos as int, ypos: ypos as int })
window_callback!(fn window_size_callback(width: c_int, height: c_int)                       => Size { width: width as int, height: height as int })
window_callback!(fn window_close_callback()                                                 => Close)
window_callback!(fn window_refresh_callback()                                               => Refresh)
window_callback!(fn window_focus_callback(focused: c_int)                                   => Focus(focused as bool))
window_callback!(fn window_iconify_callback(iconified: c_int)                               => Iconify(iconified as bool))
window_callback!(fn framebuffer_size_callback(width: c_int, height: c_int)                  => FrameBufferSize { width: width as int, height: height as int })
window_callback!(fn mouse_button_callback(button: c_int, action: c_int, mods: c_int)        => MouseButton{ button: button, action: action, mods: mods })
window_callback!(fn cursor_pos_callback(xpos: c_double, ypos: c_double)                     => CursorPos { xpos: xpos as float, ypos: ypos as float })
window_callback!(fn cursor_enter_callback(entered: c_int)                                   => CursorEnter(entered as bool))
window_callback!(fn scroll_callback(xpos: c_double, ypos: c_double)                         => Scroll { xpos: xpos as float, ypos: ypos as float })
window_callback!(fn key_callback(key: c_int, scancode: c_int, action: c_int, mods: c_int)   => Key { key: key, scancode: scancode, action: action, mods: mods })
window_callback!(fn char_callback(character: c_uint)                                        => Char(character as char))
