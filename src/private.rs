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
use std::hashmap::*;
use std::libc::*;
use std::local_data;
use std::ptr;
use std::str;

use super::*;

///
/// Holds the callback functions associated with a window
///
pub struct WindowData {
    pos_fun:                Option<WindowPosFun>,
    size_fun:               Option<WindowSizeFun>,
    close_fun:              Option<WindowCloseFun>,
    refresh_fun:            Option<WindowRefreshFun>,
    focus_fun:              Option<WindowFocusFun>,
    iconify_fun:            Option<WindowIconifyFun>,
    framebuffer_size_fun:   Option<FramebufferSizeFun>,
    mouse_button_fun:       Option<MouseButtonFun>,
    cursor_pos_fun:         Option<CursorPosFun>,
    cursor_enter_fun:       Option<CursorEnterFun>,
    scroll_fun:             Option<ScrollFun>,
    key_fun:                Option<KeyFun>,
    char_fun:               Option<CharFun>,
}

impl WindowData {
    /// Initialize the struct with all callbacks set to `None`.
    pub fn new() -> WindowData {
        WindowData {
            pos_fun:                None,
            size_fun:               None,
            close_fun:              None,
            refresh_fun:            None,
            focus_fun:              None,
            iconify_fun:            None,
            framebuffer_size_fun:   None,
            mouse_button_fun:       None,
            cursor_pos_fun:         None,
            cursor_enter_fun:       None,
            scroll_fun:             None,
            key_fun:                None,
            char_fun:               None,
        }
    }
}

///
/// A map of window data to be stored in task-local storage.
///
pub struct WindowDataMap {
    priv data_map: HashMap<*ffi::GLFWwindow, @mut WindowData>,
}

/// Key used for retrieving the map of window data from
/// task-local storage.
static data_map_tls_key: local_data::Key<@mut WindowDataMap> = &local_data::Key;

impl WindowDataMap {
    /// Gets the window data map from task-local storage wrapped in `Some`. If
    /// no data map is found, `None` is returned.
    fn get() -> Option<@mut WindowDataMap> {
        do local_data::get(data_map_tls_key) |opt| {
            opt.map(|&data_map| *data_map)
        }
    }

    /// Gets the window data map from task-local storage. If no data map is
    /// found, a new one is initialised.
    fn get_or_init() -> @mut WindowDataMap {
        match WindowDataMap::get() {
            Some(data_map) => data_map,
            None => {
                let data_map = @mut WindowDataMap { data_map: HashMap::new() };
                local_data::set(data_map_tls_key, data_map);
                data_map
            }
        }
    }

    /// Returns a mutable pointer to the window's local data. If no data is
    /// found, it is initialized and returned.
    pub fn find_or_insert(window: *ffi::GLFWwindow) -> @mut WindowData {
        *WindowDataMap::get_or_init().data_map.find_or_insert(window, @mut WindowData::new())
    }

    /// Clears all external callbacks and removes the window data from the map.
    /// Returns `true` if the window was present in the map, otherwise `false`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn remove(window: *ffi::GLFWwindow) -> bool {
        do WindowDataMap::get().map |&data_map| {
            do data_map.data_map.pop(&window).map |&data| {
                unsafe {
                    // Clear all external callbacks
                    data.pos_fun.map                (|_| ffi::glfwSetWindowPosCallback(window, cast::transmute(ptr::null::<()>())));
                    data.size_fun.map               (|_| ffi::glfwSetWindowSizeCallback(window, cast::transmute(ptr::null::<()>())));
                    data.close_fun.map              (|_| ffi::glfwSetWindowCloseCallback(window, cast::transmute(ptr::null::<()>())));
                    data.refresh_fun.map            (|_| ffi::glfwSetWindowRefreshCallback(window, cast::transmute(ptr::null::<()>())));
                    data.focus_fun.map              (|_| ffi::glfwSetWindowFocusCallback(window, cast::transmute(ptr::null::<()>())));
                    data.iconify_fun.map            (|_| ffi::glfwSetWindowIconifyCallback(window, cast::transmute(ptr::null::<()>())));
                    data.framebuffer_size_fun.map   (|_| ffi::glfwSetFramebufferSizeCallback(window, cast::transmute(ptr::null::<()>())));
                    data.mouse_button_fun.map       (|_| ffi::glfwSetMouseButtonCallback(window, cast::transmute(ptr::null::<()>())));
                    data.cursor_pos_fun.map         (|_| ffi::glfwSetCursorPosCallback(window, cast::transmute(ptr::null::<()>())));
                    data.cursor_enter_fun.map       (|_| ffi::glfwSetCursorEnterCallback(window, cast::transmute(ptr::null::<()>())));
                    data.scroll_fun.map             (|_| ffi::glfwSetScrollCallback(window, cast::transmute(ptr::null::<()>())));
                    data.key_fun.map                (|_| ffi::glfwSetKeyCallback(window, cast::transmute(ptr::null::<()>())));
                    data.char_fun.map               (|_| ffi::glfwSetCharCallback(window, cast::transmute(ptr::null::<()>())));
                }
            }
        }.is_some()
    }
}

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

macro_rules! window_callback(
    (fn $name:ident () => $field:ident()) => (
        pub extern "C" fn $name(window: *ffi::GLFWwindow) {
            do WindowDataMap::find_or_insert(window).$field.map |&cb| {
                let window_ = Window { ptr: window, shared: false };
                cb(&window_);
                unsafe { cast::forget(window_); }
            };
        }
    );
    (fn $name:ident ($($ext_arg:ident: $ext_arg_ty:ty),*) => $field:ident($($arg_conv:expr),*)) => (
        pub extern "C" fn $name(window: *ffi::GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
            do WindowDataMap::find_or_insert(window).$field.map |&cb| {
                let window_ = Window { ptr: window, shared: false };
                cb(&window_ $(, $arg_conv)*);
                unsafe { cast::forget(window_); }
            };
        }
    );
)

window_callback!(fn window_pos_callback(xpos: c_int, ypos: c_int)                           => pos_fun(xpos as int, ypos as int))
window_callback!(fn window_size_callback(width: c_int, height: c_int)                       => size_fun(width as int, height as int))
window_callback!(fn window_close_callback()                                                 => close_fun())
window_callback!(fn window_refresh_callback()                                               => refresh_fun())
window_callback!(fn window_focus_callback(focused: c_int)                                   => focus_fun(focused as bool))
window_callback!(fn window_iconify_callback(iconified: c_int)                               => iconify_fun(iconified as bool))
window_callback!(fn framebuffer_size_callback(width: c_int, height: c_int)                  => framebuffer_size_fun(width as int, height as int))
window_callback!(fn mouse_button_callback(button: c_int, action: c_int, mods: c_int)        => mouse_button_fun(button, action, mods))
window_callback!(fn cursor_pos_callback(xpos: c_double, ypos: c_double)                     => cursor_pos_fun(xpos as float, ypos as float))
window_callback!(fn cursor_enter_callback(entered: c_int)                                   => cursor_enter_fun(entered as bool))
window_callback!(fn scroll_callback(xpos: c_double, ypos: c_double)                         => scroll_fun(xpos as float, ypos as float))
window_callback!(fn key_callback(key: c_int, scancode: c_int, action: c_int, mods: c_int)   => key_fun(key, scancode, action, mods))
window_callback!(fn char_callback(character: c_uint)                                        => char_fun(character as char))
