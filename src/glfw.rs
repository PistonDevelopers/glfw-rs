#[link(name = "glfw",
	   vers = "0.1",
       uuid = "6199FAD3-6D03-4E29-87E7-7DC1B1B65C2C",
	   author = "Brendan Zabarauskas",
	   url = "https://github.com/bjz/glfw3-rs")];

#[comment = "Bindings and wrapper functions for glfw3."];
#[crate_type = "lib"];

// TODO: Document differences between GLFW and glfw-rs

use std::libc::*;

// re-export constants
pub use consts::*;

pub mod ll;
pub mod ml;
#[path = "support/private.rs"]
priv mod private;
#[path = "support/consts.rs"]
pub mod consts;

/// A struct that wraps a `*GLFWmonitor` handle.
#[deriving(Eq)]
pub struct Monitor {
    ptr: *ml::GLFWmonitor
}

/// A struct that wraps a `*GLFWwindow` handle.
#[deriving(Eq, IterBytes)]
pub struct Window {
    ptr: *ml::GLFWwindow
}

pub type ErrorFun = @fn(error: c_int, description: ~str);
pub type WindowPosFun = @fn(window: &Window, xpos: int, ypos: int);
pub type WindowSizeFun = @fn(window: &Window, width: int, height: int);
pub type WindowCloseFun = @fn(window: &Window);
pub type WindowRefreshFun = @fn(window: &Window);
pub type WindowFocusFun = @fn(window: &Window, focused: bool);
pub type WindowIconifyFun = @fn(window: &Window, iconified: bool);
pub type MouseButtonFun = @fn(window: &Window, button: c_int, action: c_int, mods: c_int);
pub type CursorPosFun = @fn(window: &Window, xpos: float, ypos: float);
pub type CursorEnterFun = @fn(window: &Window, entered: bool);
pub type ScrollFun = @fn(window: &Window, xpos: float, ypos: float);
pub type KeyFun = @fn(window: &Window, key: c_int, action: c_int, mods: c_int);
pub type CharFun = @fn(window: &Window, character: char);
pub type MonitorFun = @fn(monitor: &Monitor, event: c_int);

/// Describes a single video mode.
pub struct VidMode {
    width:      c_int,
    height:     c_int,
    red_bits:   c_int,
    green_bits: c_int,
    blue_bits:  c_int,
}

/// Describes the gamma ramp of a monitor.
pub struct GammaRamp {
    red:    ~[c_ushort],
    green:  ~[c_ushort],
    blue:   ~[c_ushort],
}

pub type GLProc = ml::GLFWglproc;

/// Initialises GLFW on the main platform thread. Fails if the initialisation
/// was unsuccessful.
///
/// Wrapper for `glfwInit` and `glfwTerminate`.
///
/// # Parameters
///
/// - `f`: A closure to be called after the GLFW is initialised.
pub fn spawn(f: ~fn()) {
    do task::spawn_sched(task::PlatformThread) {
        use std::unstable::finally::Finally;

        private::WindowDataMap::init();

        match ml::init() {
            ml::TRUE => {
                do f.finally {
                    ml::terminate();
                }
            }
            _ => fail!(~"Failed to initialize GLFW"),
        }
    }
}

/// Holds the version information of the underlying GLFW library
pub struct Version {
    major: uint,
    minor: uint,
    rev:   uint,
}

impl ToStr for Version {
    /// Returns a string representation of the version struct.
    ///
    /// # Returns
    ///
    /// A string in the form:
    ///
    /// ~~~
    /// ~"[major].[minor].[rev]"
    /// ~~~
    fn to_str(&self) -> ~str {
        fmt!("%?.%?.%?", self.major, self.minor, self.rev)
    }
}

/// Wrapper for `glfwGetVersion`.
pub fn get_version() -> Version {
    match ml::get_version() {
        (major, minor, rev) => Version {
            major: major as uint,
            minor: minor as uint,
            rev:   rev   as uint,
        }
    }
}

/// Wrapper for `glfwGetVersionString`.
pub fn get_version_string() -> ~str {
    ml::get_version_string()
}

/// Wrapper for `glfwSetErrorCallback`.
pub fn set_error_callback(cbfun: ErrorFun) {
    do private::set_error_fun(cbfun) |ext_cb| {
        ml::set_error_callback(ext_cb);
    }
}

impl Monitor {
    /// Wrapper for `glfwGetPrimaryMonitor`.
    pub fn get_primary() -> Option<Monitor> {
        do ml::get_primary_monitor().to_option().map |&ptr| {
            Monitor { ptr: ptr }
        }
    }

    /// Wrapper for `glfwGetMonitors`.
    pub fn get_connected() -> ~[Monitor] {
        ml::get_monitors().map(|&m| Monitor { ptr: m })
    }

    /// Wrapper for `glfwGetMonitorPos`.
    pub fn get_pos(&self) -> (int, int) {
        match ml::get_monitor_pos(self.ptr) {
            (xpos, ypos) => (xpos as int, ypos as int)
        }
    }

    /// Wrapper for `glfwGetMonitorPhysicalSize`.
    pub fn get_physical_size(&self) -> (int, int) {
        match ml::get_monitor_physical_size(self.ptr) {
            (width, height) => (width as int, height as int)
        }
    }

    /// Wrapper for `glfwGetMonitorName`.
    pub fn get_name(&self) -> ~str {
        ml::get_monitor_name(self.ptr)
    }

    /// Wrapper for `glfwSetMonitorCallback`.
    pub fn set_callback(cbfun: MonitorFun) {
        do private::set_monitor_fun(cbfun) |ext_cb| {
            ml::set_monitor_callback(ext_cb);
        }
    }

    /// Wrapper for `glfwGetVideoModes`.
    pub fn get_video_modes(&self) -> ~[VidMode] {
        unsafe { cast::transmute(ml::get_video_modes(self.ptr)) }
    }

    /// Wrapper for `glfwGetVideoMode`.
    pub fn get_video_mode(&self) -> Option<VidMode> {
        do ml::get_video_mode(self.ptr).map |&vid_mode| {
            unsafe { cast::transmute(vid_mode) }
        }
    }

    /// Wrapper for `glfwSetGamma`.
    pub fn set_gamma(&self, gamma: float) {
        ml::set_gamma(self.ptr, gamma as c_float);
    }

    /// Wrapper for `glfwGetGammaRamp`.
    pub fn get_gamma_ramp(&self) -> GammaRamp {
        unsafe {
            let llramp = ml::get_gamma_ramp(self.ptr);
            GammaRamp {
                red:    vec::from_buf(llramp.red,   llramp.size as uint),
                green:  vec::from_buf(llramp.green, llramp.size as uint),
                blue:   vec::from_buf(llramp.blue,  llramp.size as uint),
            }
        }
    }

    /// Wrapper for `glfwSetGammaRamp`.
    pub fn set_gamma_ramp(&self, ramp: &GammaRamp) {
        ml::set_gamma_ramp(
            self.ptr,
            &ml::GLFWgammaramp {
                red:    vec::raw::to_ptr(ramp.red),
                green:  vec::raw::to_ptr(ramp.green),
                blue:   vec::raw::to_ptr(ramp.blue),
                size:   ramp.red.len() as c_uint,
            }
        );
    }
}

impl ToStr for VidMode {
    /// Returns a string representation of the video mode.
    ///
    /// # Returns
    ///
    /// A string in the form:
    ///
    /// ~~~
    /// ~"[width] x [height] [total_bits] ([red_bits] [green_bits] [blue_bits])"
    /// ~~~
    fn to_str(&self) -> ~str {
        fmt!("%? x %? %? (%? %? %?)",
             self.width, self.height,
             (self.red_bits + self.green_bits + self.blue_bits),
             self.red_bits, self.green_bits, self.blue_bits)
    }
}

pub mod window_hint {
    use std::libc::c_int;
    use ml;

    /// Wrapper for `glfwDefaultWindowHints`.
    pub fn default() {
        ml::default_window_hints();
    }

    /// Wrapper for `glfwWindowHint` called with `RED_BITS`.
    pub fn red_bits(bits: uint) {
        ml::window_hint(ml::RED_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `GREEN_BITS`.
    pub fn green_bits(bits: uint) {
        ml::window_hint(ml::GREEN_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `BLUE_BITS`.
    pub fn blue_bits(bits: uint) {
        ml::window_hint(ml::BLUE_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `ALPHA_BITS`.
    pub fn alpha_bits(bits: uint) {
        ml::window_hint(ml::ALPHA_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `DEPTH_BITS`.
    pub fn depth_bits(bits: uint) {
        ml::window_hint(ml::DEPTH_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `STENCIL_BITS`.
    pub fn stencil_bits(bits: uint) {
        ml::window_hint(ml::STENCIL_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_RED_BITS`.
    pub fn accum_red_bits(bits: uint) {
        ml::window_hint(ml::ACCUM_RED_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_GREEN_BITS`.
    pub fn accum_green_bits(bits: uint) {
        ml::window_hint(ml::ACCUM_GREEN_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_BLUE_BITS`.
    pub fn accum_blue_bits(bits: uint) {
        ml::window_hint(ml::ACCUM_BLUE_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_ALPHA_BITS`.
    pub fn accum_alpha_bits(bits: uint) {
        ml::window_hint(ml::ACCUM_ALPHA_BITS, bits as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `AUX_BUFFERS`.
    pub fn aux_buffers(buffers: uint) {
        ml::window_hint(ml::AUX_BUFFERS, buffers as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `STEREO`.
    pub fn stereo(value: bool) {
        ml::window_hint(ml::STEREO, value as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `SAMPLES`.
    pub fn samples(samples: uint) {
        ml::window_hint(ml::SAMPLES, samples as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `SRGB_CAPABLE`.
    pub fn srgb_capable(value: bool) {
        ml::window_hint(ml::SRGB_CAPABLE, value as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `CLIENT_API`.
    pub fn client_api(api: c_int) {
        ml::window_hint(ml::CLIENT_API, api);
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MAJOR`.
    pub fn context_version_major(major: uint) {
        ml::window_hint(ml::CONTEXT_VERSION_MAJOR, major as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MINOR`.
    pub fn context_version_minor(minor: uint) {
        ml::window_hint(ml::CONTEXT_VERSION_MINOR, minor as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MAJOR` and
    /// `CONTEXT_VERSION_MINOR`.
    pub fn context_version(major: uint, minor: uint) {
        ml::window_hint(ml::CONTEXT_VERSION_MAJOR, major as c_int);
        ml::window_hint(ml::CONTEXT_VERSION_MINOR, minor as c_int)
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_ROBUSTNESS`.
    pub fn context_robustness(value: bool) {
        ml::window_hint(ml::CONTEXT_ROBUSTNESS, value as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_FORWARD_COMPAT`.
    pub fn opengl_forward_compat(value: bool) {
        ml::window_hint(ml::OPENGL_FORWARD_COMPAT, value as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_DEBUG_CONTEXT`.
    pub fn opengl_debug_context(value: bool) {
        ml::window_hint(ml::OPENGL_DEBUG_CONTEXT, value as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_PROFILE`.
    pub fn opengl_profile(profile: c_int) {
        ml::window_hint(ml::OPENGL_PROFILE, profile);
    }

    /// Wrapper for `glfwWindowHint` called with `RESIZABLE`.
    pub fn resizable(value: bool) {
        ml::window_hint(ml::RESIZABLE, value as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `VISIBLE`.
    pub fn visible(value: bool) {
        ml::window_hint(ml::VISIBLE, value as c_int);
    }

    /// Wrapper for `glfwWindowHint` called with `DECORATED`.
    pub fn decorated(value: bool) {
        ml::window_hint(ml::DECORATED, value as c_int);
    }
}

/// Describes the mode of a window
pub enum WindowMode {
    /// Full screen mode. Contains the monitor on which the window is displayed.
    FullScreen(Monitor),

    /// Windowed mode.
    Windowed,
}

/// Private conversion methods for `glfw::WindowMode`
impl WindowMode {
    /// Extract the window mode from a low-level monitor pointer. If the pointer
    /// is null it assumes the window is in windowed mode and returns `Windowed`,
    /// otherwise it returns the pointer wrapped in `glfw::FullScreen`.
    priv fn from_ptr(ptr: *ml::GLFWmonitor) -> WindowMode {
        if ptr.is_null() {
            Windowed
        } else {
            FullScreen(Monitor { ptr: ptr })
        }
    }

    /// Returns a pointer to a monitor if the window is fullscreen, otherwise
    /// it returns a null pointer (if it is in windowed mode).
    priv fn to_ptr(&self) -> *ml::GLFWmonitor {
        match *self {
            FullScreen(monitor) => monitor.ptr,
            Windowed => ptr::null()
        }
    }
}

macro_rules! set_window_callback(
    (
        setter:   $ml_fn:ident,
        callback: $ext_fn:ident,
        field:    $data_field:ident
    ) => ({
        self.get_local_data().$data_field = Some(cbfun);
        ml::$ml_fn(self.ptr, private::$ext_fn);
    })
)

impl Window {
    /// Wrapper for `glfwCreateWindow`.
    ///
    /// # Returns
    ///
    /// The created window wrapped in `Some`, or `None` if an error occurred.
    pub fn create(width: uint, height: uint, title: &str, mode: WindowMode) -> Option<Window> {
        do ml::create_window(
            width as c_int,
            height as c_int,
            title,
            mode.to_ptr(),
            ptr::null()
        ).to_option().map |&ptr| {
            // Initialize the local data for this window in TLS
            private::WindowDataMap::get().insert(
                unsafe { cast::transmute(ptr) },
                @mut private::WindowData::new()
            );
            Window { ptr: unsafe { cast::transmute(ptr) } }
        }
    }

    ///
    /// Returns a mutable pointer to the window's local data stored in task-
    /// local storage. Fails if no data is found.
    ///
    priv fn get_local_data(&self) -> @mut private::WindowData {
        match private::WindowDataMap::get().find_mut(&self.ptr) {
            Some(&data) => data,
            None => fail!("Could not find local data for this window."),
        }
    }

    /// Wrapper for `glfwWindowShouldClose`.
    pub fn should_close(&self) -> bool {
        ml::window_should_close(self.ptr) as bool
    }

    /// Wrapper for `glfwSetWindowShouldClose`.
    pub fn set_should_close(&self, value: bool) {
        ml::set_window_should_close(self.ptr, value as c_int)
    }

    /// Wrapper for `glfwSetWindowTitle`.
    pub fn set_title(&self, title: &str) {
        ml::set_window_title(self.ptr, title);
    }

    /// Wrapper for `glfwGetWindowPos`.
    pub fn get_pos(&self) -> (int, int) {
        match ml::get_window_pos(self.ptr) {
            (xpos, ypos) => (xpos as int, ypos as int)
        }
    }

    /// Wrapper for `glfwSetWindowPos`.
    pub fn set_pos(&self, xpos: int, ypos: int) {
        ml::set_window_pos(self.ptr, xpos as c_int, ypos as c_int);
    }

    /// Wrapper for `glfwGetWindowSize`.
    pub fn get_size(&self) -> (int, int) {
        match ml::get_window_size(self.ptr) {
            (width, height) => (width as int, height as int)
        }
    }

    /// Wrapper for `glfwSetWindowSize`.
    pub fn set_size(&self, width: int, height: int) {
        ml::set_window_size(self.ptr, width as c_int, height as c_int);
    }

    /// Wrapper for `glfwIconifyWindow`.
    pub fn iconify(&self) {
        ml::iconify_window(self.ptr);
    }

    /// Wrapper for `glfwRestoreWindow`.
    pub fn restore(&self) {
        ml::restore_window(self.ptr);
    }

    /// Wrapper for `glfwShowWindow`.
    pub fn show(&self) {
        ml::show_window(self.ptr);
    }

    /// Wrapper for `glfwHideWindow`.
    pub fn hide(&self) {
        ml::hide_window(self.ptr);
    }

    /// Wrapper for `glfwGetWindowMonitor`.
    ///
    /// # Returns
    ///
    /// The window mode; either glfw::FullScreen or glfw::Windowed
    pub fn get_window_mode(&self) -> WindowMode {
        WindowMode::from_ptr(
            ml::get_window_monitor(self.ptr)
        )
    }

    /// Wrapper for `glfwGetWindowParam` called with `FOCUSED`.
    pub fn is_focused(&self) -> bool {
        ml::get_window_param(self.ptr, FOCUSED) as bool
    }

    /// Wrapper for `glfwGetWindowParam` called with `ICONIFIED`.
    pub fn is_iconified(&self) -> bool {
        ml::get_window_param(self.ptr, ICONIFIED) as bool
    }

    /// Wrapper for `glfwGetWindowParam` called with `CLIENT_API`.
    pub fn get_client_api(&self) -> c_int {
        ml::get_window_param(self.ptr, CLIENT_API)
    }

    /// Wrapper for `glfw::ll::glfwGetWindowParam` called with
    /// `CONTEXT_VERSION_MAJOR`, `CONTEXT_VERSION_MINOR` and `CONTEXT_REVISION`.
    ///
    /// # Returns
    ///
    /// The client API version of the window's context in a version struct.
    ///
    pub fn get_context_version(&self) -> Version {
        Version {
            major:  ml::get_window_param(self.ptr, CONTEXT_VERSION_MAJOR) as uint,
            minor:  ml::get_window_param(self.ptr, CONTEXT_VERSION_MINOR) as uint,
            rev:    ml::get_window_param(self.ptr, CONTEXT_REVISION) as uint,
        }
    }

    /// Wrapper for `glfwGetWindowParam` called with `CONTEXT_ROBUSTNESS`.
    pub fn get_context_robustness(&self) -> c_int {
        ml::get_window_param(self.ptr, CONTEXT_ROBUSTNESS)
    }

    /// Wrapper for `glfwGetWindowParam` called with `OPENGL_FORWARD_COMPAT`.
    pub fn is_opengl_forward_compat(&self) -> bool {
        ml::get_window_param(self.ptr, OPENGL_FORWARD_COMPAT) as bool
    }

    /// Wrapper for `glfwGetWindowParam` called with `OPENGL_DEBUG_CONTEXT`.
    pub fn is_opengl_debug_context(&self) -> bool {
        ml::get_window_param(self.ptr, OPENGL_DEBUG_CONTEXT) as bool
    }

    /// Wrapper for `glfwGetWindowParam` called with `OPENGL_PROFILE`.
    pub fn get_opengl_profile(&self) -> c_int {
        ml::get_window_param(self.ptr, OPENGL_PROFILE)
    }

    /// Wrapper for `glfwGetWindowParam` called with `RESIZABLE`.
    pub fn is_resizable(&self) -> bool {
        ml::get_window_param(self.ptr, RESIZABLE) as bool
    }

    /// Wrapper for `glfwGetWindowParam` called with `VISIBLE`.
    pub fn is_visible(&self) -> bool {
        ml::get_window_param(self.ptr, VISIBLE) as bool
    }

    /// Wrapper for `glfwGetWindowParam` called with `DECORATED`.
    pub fn is_decorated(&self) -> bool {
        ml::get_window_param(self.ptr, DECORATED) as bool
    }

    /// Wrapper for `glfwSetWindowPosCallback`.
    pub fn set_pos_callback(&self, cbfun: WindowSizeFun) {
        set_window_callback!(setter:   set_window_pos_callback,
                             callback: window_pos_callback,
                             field:    pos_fun);
    }

    /// Wrapper for `glfwSetWindowSizeCallback`.
    pub fn set_size_callback(&self, cbfun: WindowSizeFun) {
        set_window_callback!(setter:   set_window_size_callback,
                             callback: window_size_callback,
                             field:    size_fun);
    }

    /// Wrapper for `glfwSetWindowCloseCallback`.
    pub fn set_close_callback(&self, cbfun: WindowCloseFun) {
        set_window_callback!(setter:   set_window_close_callback,
                             callback: window_close_callback,
                             field:    close_fun);
    }

    /// Wrapper for `glfwSetWindowRefreshCallback`.
    pub fn set_refresh_callback(&self, cbfun: WindowRefreshFun) {
        set_window_callback!(setter:   set_window_refresh_callback,
                             callback: window_refresh_callback,
                             field:    refresh_fun);
    }

    /// Wrapper for `glfwSetWindowFocusCallback`.
    pub fn set_focus_callback(&self, cbfun: WindowFocusFun) {
        set_window_callback!(setter:   set_window_focus_callback,
                             callback: window_focus_callback,
                             field:    focus_fun);
    }

    /// Wrapper for `glfwSetWindowIconifyCallback`.
    pub fn set_iconify_callback(&self, cbfun: WindowIconifyFun) {
        set_window_callback!(setter:   set_window_iconify_callback,
                             callback: window_iconify_callback,
                             field:    iconify_fun);
    }

    /// Wrapper for `glfwGetInputMode` called with `CURSOR`.
    pub fn get_cursor_mode(&self) -> c_int {
        ml::get_input_mode(self.ptr, CURSOR)
    }

    /// Wrapper for `glfwSetInputMode` called with `CURSOR`.
    pub fn set_cursor_mode(&self, mode: c_int) {
        ml::set_input_mode(self.ptr, CURSOR, mode);
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_KEYS`.
    pub fn has_sticky_keys(&self) -> bool {
        ml::get_input_mode(self.ptr, STICKY_KEYS) as bool
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_KEYS`.
    pub fn set_sticky_keys(&self, value: bool) {
        ml::set_input_mode(self.ptr, STICKY_KEYS, value as c_int);
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn has_sticky_mouse_buttons(&self) -> bool {
        ml::get_input_mode(self.ptr, STICKY_MOUSE_BUTTONS) as bool
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn set_sticky_mouse_buttons(&self, value: bool) {
        ml::set_input_mode(self.ptr, STICKY_MOUSE_BUTTONS, value as c_int);
    }

    /// Wrapper for `glfwGetKey`.
    pub fn get_key(&self, key: c_int) -> c_int {
        ml::get_key(self.ptr, key)
    }

    /// Wrapper for `glfwGetMouseButton`.
    pub fn get_mouse_button(&self, button: c_int) -> c_int {
        ml::get_mouse_button(self.ptr, button)
    }

    /// Wrapper for `glfwGetCursorPos`.
    pub fn get_cursor_pos(&self) -> (float, float) {
        match ml::get_cursor_pos(self.ptr) {
            (xpos, ypos) => (xpos as float, ypos as float)
        }
    }

    /// Wrapper for `glfwSetCursorPos`.
    pub fn set_cursor_pos(&self, xpos: float, ypos: float) {
        ml::set_cursor_pos(self.ptr, xpos as c_double, ypos as c_double);
    }

    /// Wrapper for `glfwSetKeyCallback`.
    pub fn set_key_callback(&self, cbfun: KeyFun) {
        set_window_callback!(setter:   set_key_callback,
                             callback: key_callback,
                             field:    key_fun);
    }

    /// Wrapper for `glfwSetCharCallback`.
    pub fn set_char_callback(&self, cbfun: CharFun) {
        set_window_callback!(setter:   set_char_callback,
                             callback: char_callback,
                             field:    char_fun);
    }

    /// Wrapper for `glfwSetMouseButtonCallback`.
    pub fn set_mouse_button_callback(&self, cbfun: MouseButtonFun) {
        set_window_callback!(setter:   set_mouse_button_callback,
                             callback: mouse_button_callback,
                             field:    mouse_button_fun);
    }

    /// Wrapper for `glfwSetCursorPosCallback`.
    pub fn set_cursor_pos_callback(&self, cbfun: CursorPosFun) {
        set_window_callback!(setter:   set_cursor_pos_callback,
                             callback: cursor_pos_callback,
                             field:    cursor_pos_fun);
    }

    /// Wrapper for `glfwSetCursorEnterCallback`.
    pub fn set_cursor_enter_callback(&self, cbfun: CursorEnterFun) {
        set_window_callback!(setter:   set_cursor_enter_callback,
                             callback: cursor_enter_callback,
                             field:    cursor_enter_fun);
    }

    /// Wrapper for `glfwSetScrollCallback`.
    pub fn set_scroll_callback(&self, cbfun: ScrollFun) {
        set_window_callback!(setter:   set_scroll_callback,
                             callback: scroll_callback,
                             field:    scroll_fun);
    }

    /// Wrapper for `glfwGetClipboardString`.
    pub fn set_clipboard_string(&self, string: &str) {
        ml::set_clipboard_string(self.ptr, string);
    }

    /// Wrapper for `glfwGetClipboardString`.
    pub fn get_clipboard_string(&self) -> ~str {
        ml::get_clipboard_string(self.ptr)
    }

    /// Wrapper for `glfwMakeContextCurrent`.
    pub fn make_context_current(&self) {
        ml::make_context_current(self.ptr);
    }

    // TODO: documentation
    pub fn is_current_context(&self) -> bool {
        self.ptr == ml::get_current_context()
    }

    /// Wrapper for `glfwSwapBuffers`.
    pub fn swap_buffers(&self) {
        ml::swap_buffers(self.ptr);
    }
}

/// Wrapper for glfwMakeContextCurrent` called with `null`.
pub fn detach_current_context() {
    ml::make_context_current(ptr::null());
}

impl Drop for Window {
    /// Closes the window and removes all associated callbacks.
    ///
    /// Wrapper for `glfwDestroyWindow`.
    ///
    /// # Implementation notes
    ///
    /// Calls `glfwDestroyWindow` on the window pointer and cleans up the
    /// callbacks stored in task-local storage
    pub fn finalize(&self) {
        ml::destroy_window(self.ptr);
        private::WindowDataMap::get().remove(&self.ptr);
    }
}

/// Wrapper for `glfwPollEvents`.
pub fn poll_events() {
    ml::poll_events();
}

/// Wrapper for `glfwWaitEvents`.
pub fn wait_events() {
    ml::wait_events();
}

pub mod joystick {
    use std::libc::*;
    use ml;

    /// Wrapper for `glfwJoystickPresent`.
    pub fn is_present(joy: c_int) -> bool {
        ml::joystick_present(joy) as bool
    }

    /// Wrapper for `glfwGetJoystickAxes`.
    pub fn get_axes(joy: c_int) -> ~[float] {
        ml::get_joystick_axes(joy).map(|&a| a as float)
    }

    /// Wrapper for `glfwGetJoystickButtons`.
    pub fn get_buttons(joy: c_int) -> ~[c_int] {
        ml::get_joystick_buttons(joy).map(|&b| b as c_int)
    }

    /// Wrapper for `glfwGetJoystickName`.
    pub fn get_name(joy: c_int) -> ~str {
        ml::get_joystick_name(joy)
    }
}

/// Wrapper for `glfwGetTime`.
pub fn get_time() -> float {
    ml::get_time() as float
}

/// Wrapper for `glfwSetTime`.
pub fn set_time(time: float) {
    ml::set_time(time as c_double);
}

/// Wrapper for `glfwSwapInterval`.
pub fn set_swap_interval(interval: int) {
    ml::set_swap_interval(interval as c_int);
}

/// Wrapper for `glfwExtensionSupported`.
pub fn extension_supported(extension: &str) -> bool {
    ml::extension_supported(extension) as bool
}

/// Wrapper for `glfwGetProcAddress`.
pub fn get_proc_address(procname: &str) -> GLProc {
    ml::get_proc_address(procname)
}
