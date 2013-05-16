use core::libc::*;
use core::hashmap::*;
use core::local_data::*;

use super::*;
use ll::*;

/// Holds the local data associated with a window
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

pub struct WindowDataMap(HashMap<Window, @mut WindowData>);

pub impl WindowDataMap {
    priv fn tls_key(_: @@mut WindowDataMap) {}

    /// Initializes the local data in TLS
    fn init() {
        unsafe {
            local_data_set(
                WindowDataMap::tls_key,
                @@mut WindowDataMap(HashMap::new())
            )
        }
    }

    /// Retrieves a local data struct from TLS.
    fn get() -> @mut WindowDataMap {
        match unsafe { local_data_get(WindowDataMap::tls_key) } {
            Some(@local_data) => local_data,
            None => fail!("Could not find a WindowDataMap in thread-local storage."),
        }
    }
}

// External window callbacks

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
}

pub extern "C" fn window_size_callback(window: *GLFWwindow, width: c_int, height: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().size_fun.map |&cb| {
        cb(&window_, width as int, height as int)
    };
}

pub extern "C" fn window_close_callback(window: *GLFWwindow) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().close_fun.map |&cb| {
        cb(&window_)
    };
    WindowDataMap::get().remove(&window_);
}

pub extern "C" fn window_refresh_callback(window: *GLFWwindow) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().refresh_fun.map |&cb| {
        cb(&window_)
    };
}

pub extern "C" fn window_focus_callback(window: *GLFWwindow, activated: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().focus_fun.map |&cb| {
        cb(&window_, activated as bool)
    };
}

pub extern "C" fn window_iconify_callback(window: *GLFWwindow, iconified: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().iconify_fun.map |&cb| {
        cb(&window_, iconified as bool)
    };
}

pub extern "C" fn mouse_button_callback(window: *GLFWwindow, button: c_int, action: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().mouse_button_fun.map |&cb| {
        cb(&window_, button, action)
    };
}

pub extern "C" fn cursor_pos_callback(window: *GLFWwindow, xpos: c_double, ypos: c_double) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().cursor_pos_fun.map |&cb| {
        cb(&window_, xpos as float, ypos as float)
    };
}

pub extern "C" fn cursor_enter_callback(window: *GLFWwindow, entered: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().cursor_enter_fun.map |&cb| {
        cb(&window_, entered as bool)
    };
}

pub extern "C" fn scroll_callback(window: *GLFWwindow, xpos: c_double, ypos: c_double) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().scroll_fun.map |&cb| {
        cb(&window_, xpos as float, ypos as float)
    };
}

pub extern "C" fn key_callback(window: *GLFWwindow, key: c_int, action: c_int) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().key_fun.map |&cb| {
        cb(&window_, key, action)
    };
}

pub extern "C" fn char_callback(window: *GLFWwindow, character: c_uint) {
    let window_ = Window { ptr: window };
    do window_.get_local_data().char_fun.map |&cb| {
        cb(&window_, character as char)
    };
}
