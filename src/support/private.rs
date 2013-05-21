
//! Private functions and items to be used with the high-level library wrapper.

use core::libc::*;
use core::hashmap::*;
use core::local_data::*;

use super::*;
use ll::*;

///
/// Holds the callback functions associated with a window
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
    /// Initialize the struct with all callbacks set to `None`.
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
/// A map of window data to be stored in task-local storage.
///
pub struct WindowDataMap(HashMap<*GLFWwindow, @mut WindowData>);

pub impl WindowDataMap {
    /// Function stub used for retrieving a the map of window data from
    /// task-local storage.
    priv fn tls_key(_: @@mut WindowDataMap) {}

    /// Initializes a map of window data in task-local storage.
    fn init() {
        unsafe {
            local_data_set(
                WindowDataMap::tls_key,
                @@mut WindowDataMap(HashMap::new())
            )
        }
    }

    /// Retrieves a mutable pointer to the map of window data stored task-local
    /// storage, failing if the map could not be found.
    fn get() -> @mut WindowDataMap {
        match unsafe { local_data_get(WindowDataMap::tls_key) } {
            Some(@local_data) => local_data,
            None => fail!("Could not find a WindowDataMap in thread-local storage."),
        }
    }

    /// Clears all external callbacks and removes the window from the map.
    /// Returns `true` if the window was present in the map, otherwise `false`.
    fn remove(&mut self, window: &*GLFWwindow) -> bool {
        do self.pop(window).map |&data| {
            unsafe {
                // Clear all external callbacks
                data.pos_fun.map           (|_| glfwSetWindowPosCallback(*window, ptr::null()));
                data.size_fun.map          (|_| glfwSetWindowSizeCallback(*window, ptr::null()));
                data.close_fun.map         (|_| glfwSetWindowCloseCallback(*window, ptr::null()));
                data.refresh_fun.map       (|_| glfwSetWindowRefreshCallback(*window, ptr::null()));
                data.focus_fun.map         (|_| glfwSetWindowFocusCallback(*window, ptr::null()));
                data.iconify_fun.map       (|_| glfwSetWindowIconifyCallback(*window, ptr::null()));
                data.mouse_button_fun.map  (|_| glfwSetMouseButtonCallback(*window, ptr::null()));
                data.cursor_pos_fun.map    (|_| glfwSetCursorPosCallback(*window, ptr::null()));
                data.cursor_enter_fun.map  (|_| glfwSetCursorEnterCallback(*window, ptr::null()));
                data.scroll_fun.map        (|_| glfwSetScrollCallback(*window, ptr::null()));
                data.key_fun.map           (|_| glfwSetKeyCallback(*window, ptr::null()));
                data.char_fun.map          (|_| glfwSetCharCallback(*window, ptr::null()));
            }
        }.is_some()
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

macro_rules! window_callback(
    (fn $name:ident () => $field:ident()) => (
        pub extern "C" fn $name(window: *GLFWwindow) {
            let window_ = Window { ptr: window };
            do window_.get_local_data().$field.map |&cb| {
                cb(&window_)
            };
            unsafe { cast::forget(window_); }
        }
    );
    (fn $name:ident ($($ext_arg:ident: $ext_arg_ty:ty),*) => $field:ident($($arg_conv:expr),*)) => (
        pub extern "C" fn $name(window: *GLFWwindow $(, $ext_arg: $ext_arg_ty)*) {
            let window_ = Window { ptr: window };
            do window_.get_local_data().$field.map |&cb| {
                cb(&window_ $(, $arg_conv)*)
            };
            unsafe { cast::forget(window_); }
        }
    );
)

window_callback!(fn window_pos_callback(xpos: c_int, ypos: c_int)       => pos_fun(xpos as int, ypos as int))
window_callback!(fn window_size_callback(width: c_int, height: c_int)   => size_fun(width as int, height as int))
window_callback!(fn window_close_callback()                             => close_fun())
window_callback!(fn window_refresh_callback()                           => refresh_fun())
window_callback!(fn window_focus_callback(focused: c_int)               => focus_fun(focused as bool))
window_callback!(fn window_iconify_callback(iconified: c_int)           => iconify_fun(iconified as bool))
window_callback!(fn mouse_button_callback(button: c_int, action: c_int) => mouse_button_fun(button, action))
window_callback!(fn cursor_pos_callback(xpos: c_double, ypos: c_double) => cursor_pos_fun(xpos as float, ypos as float))
window_callback!(fn cursor_enter_callback(entered: c_int)               => cursor_enter_fun(entered as bool))
window_callback!(fn scroll_callback(xpos: c_double, ypos: c_double)     => scroll_fun(xpos as float, ypos as float))
window_callback!(fn key_callback(key: c_int, action: c_int)             => key_fun(key, action))
window_callback!(fn char_callback(character: c_uint)                    => char_fun(character as char))
