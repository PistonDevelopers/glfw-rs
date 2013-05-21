
//! Private functions and items used with the high-level library wrapper

use core::libc::*;
use core::hashmap::*;
use core::local_data::*;

use super::*;
use ll::*;

///
/// Holds data associated with a window for storage in TLS
///
pub struct WindowData {
    pos_fun:             Option<WindowPosFun>,
    size_fun:            Option<WindowSizeFun>,
    close_fun:           Option<WindowCloseFun>,
    refresh_fun:         Option<WindowRefreshFun>,
    focus_fun:           Option<WindowFocusFun>,
    iconify_fun:         Option<WindowIconifyFun>,
    mouse_button_fun:    Option<MouseButtonFun>,
    cursor_pos_fun:      Option<CursorPosFun>,
    cursor_enter_fun:    Option<CursorEnterFun>,
    scroll_fun:          Option<ScrollFun>,
    key_fun:             Option<KeyFun>,
    char_fun:            Option<CharFun>,
}

pub impl WindowData {
    /// Initialize an empty struct
    fn new() -> WindowData {
        WindowData {
            pos_fun:             None,
            size_fun:            None,
            close_fun:           None,
            refresh_fun:         None,
            focus_fun:           None,
            iconify_fun:         None,
            mouse_button_fun:    None,
            cursor_pos_fun:      None,
            cursor_enter_fun:    None,
            scroll_fun:          None,
            key_fun:             None,
            char_fun:            None,
        }
    }
}

///
/// A map of window data to be stored in TLS
///
pub struct WindowDataMap(HashMap<*GLFWwindow, @mut WindowData>);

pub impl WindowDataMap {
    /// Function stub used for retrieving a the map of window data from TLS.
    priv fn tls_key(_: @@mut WindowDataMap) {}

    /// Initializes a map of window data in TLS.
    fn init() {
        unsafe {
            local_data_set(
                WindowDataMap::tls_key,
                @@mut WindowDataMap(HashMap::new())
            )
        }
    }

    /// Retrieves a mutable pointer to the map of window data stored TLS,
    /// failing if the map could not be found.
    fn get() -> @mut WindowDataMap {
        match unsafe { local_data_get(WindowDataMap::tls_key) } {
            Some(@local_data) => local_data,
            None => fail!("Could not find a WindowDataMap in thread-local storage."),
        }
    }

    /// Removes the map of window data from TLS if it exists.
    fn remove() {
        unsafe {
            local_data_modify(WindowDataMap::tls_key, |_| None);
        }
    }
}

// Global callbacks

fn error_fun_tls_key(_: @ErrorFun) {}

pub extern "C" fn error_callback(error: c_int, description: *c_char) {
    unsafe {
        do local_data_get(error_fun_tls_key).map |&@cb| {
            cb(error, str::raw::from_c_str(description))
        };
    }
}

pub fn set_error_fun(cbfun: ErrorFun, f: &fn(GLFWerrorfun) ) {
    unsafe {
        local_data_set(error_fun_tls_key, @cbfun);
        f(error_callback);
    }
}

fn monitor_fun_tls_key(_: @MonitorFun) {}

pub extern "C" fn monitor_callback(monitor: *GLFWmonitor, event: c_int) {
    unsafe {
        do local_data_get(monitor_fun_tls_key).map |&@cb| {
            cb(&Monitor { ptr: monitor }, event)
        };
    }
}

pub fn set_monitor_fun(cbfun: MonitorFun, f: &fn(GLFWmonitorfun) ) {
    unsafe {
        local_data_set(monitor_fun_tls_key, @cbfun);
        f(monitor_callback);
    }
}


// External window callbacks

pub extern "C" fn window_pos_callback(window: *GLFWwindow, xpos: c_int, ypos: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().pos_fun.map |&cb| {
        cb(&window_, xpos as int, ypos as int)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn window_size_callback(window: *GLFWwindow, width: c_int, height: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().size_fun.map |&cb| {
        cb(&window_, width as int, height as int)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn window_close_callback(window: *GLFWwindow) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().close_fun.map |&cb| {
        cb(&window_)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn window_refresh_callback(window: *GLFWwindow) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().refresh_fun.map |&cb| {
        cb(&window_)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn window_focus_callback(window: *GLFWwindow, focused: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().focus_fun.map |&cb| {
        cb(&window_, focused as bool)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn window_iconify_callback(window: *GLFWwindow, iconified: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().iconify_fun.map |&cb| {
        cb(&window_, iconified as bool)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn mouse_button_callback(window: *GLFWwindow, button: c_int, action: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().mouse_button_fun.map |&cb| {
        cb(&window_, button, action)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn cursor_pos_callback(window: *GLFWwindow, xpos: c_double, ypos: c_double) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().cursor_pos_fun.map |&cb| {
        cb(&window_, xpos as float, ypos as float)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn cursor_enter_callback(window: *GLFWwindow, entered: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().cursor_enter_fun.map |&cb| {
        cb(&window_, entered as bool)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn scroll_callback(window: *GLFWwindow, xpos: c_double, ypos: c_double) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().scroll_fun.map |&cb| {
        cb(&window_, xpos as float, ypos as float)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn key_callback(window: *GLFWwindow, key: c_int, action: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().key_fun.map |&cb| {
        cb(&window_, key, action)
    };
    unsafe { cast::forget(window_); }
}

pub extern "C" fn char_callback(window: *GLFWwindow, character: c_uint) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().char_fun.map |&cb| {
        cb(&window_, character as char)
    };
    unsafe { cast::forget(window_); }
}
