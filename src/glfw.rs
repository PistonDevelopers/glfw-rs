#[link(name = "glfw",
	   vers = "0.1",
       uuid = "6199FAD3-6D03-4E29-87E7-7DC1B1B65C2C",
	   author = "Brendan Zabarauskas",
	   url = "https://github.com/bjz/glfw3-rs")];

#[comment = "Bindings and wrapper functions for glfw3."];
#[crate_type = "lib"];

// TODO: Document differences between GLFW and glfw-rs

use core::libc::*;

// re-export constants
pub use consts::*;

pub mod ll;
pub mod ml;
#[path = "support/private.rs"]
priv mod private;
#[path = "support/consts.rs"]
pub mod consts;

///
/// A struct containing a low-level monitor handle.
///
/// # Feilds
///
/// - `ptr`: A low-level handle to the monitor.
///
#[deriving(Eq)]
pub struct Monitor {
    ptr: *ml::GLFWmonitor
}

///
/// A struct containing a low-level window handle.
///
/// # Feilds
///
/// - `ptr`: A low-level handle to the window.
///
#[deriving(Eq, IterBytes)]
pub struct Window {
    ptr: *ml::GLFWwindow
}

///
/// The function type for error callbacks.
///
/// # Parameters
///
/// - `error`: An error code.
/// - `description`: A string describing the error.
///
pub type ErrorFun = @fn(error: c_int, description: ~str);

///
/// The function type for window position callbacks.
///
/// # Parameters
///
/// - `window`: The window that the user moved.
/// - `xpos`: The new x-coordinate.
/// - `ypos`: The new y-coordinate.
///
pub type WindowPosFun = @fn(window: &Window, xpos: int, ypos: int);

///
/// The function type for window size callbacks.
///
/// # Parameters
///
/// - `window`: The window that the user resized.
/// - `width`: The new width of the window.
/// - `height`: The new height of the window.
///
pub type WindowSizeFun = @fn(window: &Window, width: int, height: int);

///
/// The function type for window close callbacks.
///
/// # Parameters
///
/// - `window`: The window that the user attempted to close.
///
pub type WindowCloseFun = @fn(window: &Window);

///
/// The function type for window refresh callbacks.
///
/// # Parameters
///
/// - `window`: The window whose content needs to be refreshed.
///
pub type WindowRefreshFun = @fn(window: &Window);

///
/// The function type for window focus/defocus callbacks.
///
/// # Parameters
///
/// - `window`: The window that was focused or defocused.
/// - `focused`: `true` if the window was focused or `false` if the window was
///   defocused.
///
pub type WindowFocusFun = @fn(window: &Window, focused: bool);

///
/// The function type for window iconify/restore callbacks.
///
/// # Parameters
///
/// - `window`: The window that was iconified or restored.
/// - `iconified`: `true` if the window was iconified or `false` if the window
///   was defocused.
///
pub type WindowIconifyFun = @fn(window: &Window, iconified: bool);

///
/// The function type for mouse button callbacks.
///
/// # Parameters
///
/// - `window`: The window that recieved the event.
/// - `button`: The mouse button that was pressed or released.
/// - `action`: Either `PRESS` or `RELEASE`.
///
pub type MouseButtonFun = @fn(window: &Window, button: c_int, action: c_int);

///
/// The function type of cursor position callbacks.
///
/// # Parameters
///
/// - `window`: The window that recieved the event.
/// - `xpos`: The new x-coordinate of the cursor.
/// - `ypos`: The new y-coordinate of the cursor.
///
pub type CursorPosFun = @fn(window: &Window, xpos: float, ypos: float);

///
/// The function type of cursor enter/leave callbacks.
///
/// # Parameters
///
/// - `window`: The window that recieved the event.
/// - `entered`: `true` if the cursor entered the window's client area or
///   `false` if the cursor left it.
pub type CursorEnterFun = @fn(window: &Window, entered: bool);

///
/// The function type of scroll callbacks.
///
/// # Parameters
///
/// - `window`: The window that recieved the event.
/// - `xpos`: The scroll offset along the x-axis.
/// - `ypos`: The scroll offset along the y-axis.
///
pub type ScrollFun = @fn(window: &Window, xpos: float, ypos: float);

///
/// The function type of key callbacks.
///
/// # Parameters
///
/// - `window`: The window that recieved the event.
/// - `key`: The key that was pressed or released.
/// - `action`: Either `PRESS`, `RELEASE`, or `REPEAT`.
///
pub type KeyFun = @fn(window: &Window, key: c_int, action: c_int);

///
/// The function type for character callbacks.
///
/// # Parameters
///
/// - `window`: The window that recieved the event.
/// - `character`: The character.
///
pub type CharFun = @fn(window: &Window, character: char);

///
/// The function type for monitor configuration callbacks.
///
/// # Parameters
///
/// - `monitor`: The monitor that was connected or disconnected.
/// - `event`: Either `CONNECTED` or `DISCONNECTED`.
pub type MonitorFun = @fn(monitor: &Monitor, event: c_int);

///
/// Describes a single video mode.
///
pub struct VidMode {
    width:      c_int,
    height:     c_int,
    red_bits:   c_int,
    green_bits: c_int,
    blue_bits:  c_int,
}

///
/// Describes the gamma ramp of a monitor.
///
pub struct GammaRamp {
    red:    [c_ushort, ..GAMMA_RAMP_SIZE],
    green:  [c_ushort, ..GAMMA_RAMP_SIZE],
    blue:   [c_ushort, ..GAMMA_RAMP_SIZE],
}

pub type GLProc = ml::GLFWglproc;

///
/// Initialises GLFW on the main platform thread. Fails if the initialisation
/// was unsuccessful.
///
/// # Parameters
///
/// - `f`: A closure to be called after the GLFW is initialised.
///
pub fn spawn(f: ~fn()) {
    do task::spawn_sched(task::PlatformThread) {
        use core::unstable::finally::Finally;

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

///
/// Holds the version information of the underlying GLFW library
///
pub struct Version {
    major: uint,
    minor: uint,
    rev:   uint,
}

impl ToStr for Version {
    ///
    /// Returns a string representation of the version struct.
    ///
    /// # Returns
    ///
    /// A string in the form:
    ///
    /// ~~~
    /// ~"[major].[minor].[rev]"
    /// ~~~
    ///
    fn to_str(&self) -> ~str {
        fmt!("%?.%?.%?", self.major, self.minor, self.rev)
    }
}

///
/// Returns a struct containing the version numbers of the underlying GLFW
/// library.
///
pub fn get_version() -> Version {
    match ml::get_version() {
        (major, minor, rev) => Version {
            major: major as uint,
            minor: minor as uint,
            rev:   rev   as uint,
        }
    }
}

///
/// Returns a string describing the compile time configuration of the underlying
/// GLFW library.
///
/// The format of the string is as follows:
///
/// - The version of GLFW
/// - The name of the window system API
/// - The name of the context creation API
/// - Any additional options or APIs
///
/// For example, when the underlying GLFW 3.0 library was compiled with MinGW
/// using the Win32 and WGL back ends, the version string may look something
/// like this:
///
/// ~~~
/// ~"3.0.0 Win32 WGL MinGW"
/// ~~~
///
pub fn get_version_string() -> ~str {
    ml::get_version_string()
}

///
/// Sets a callback to be run when an error is encountered.
///
pub fn set_error_callback(cbfun: ErrorFun) {
    do private::set_error_fun(cbfun) |ext_cb| {
        ml::set_error_callback(ext_cb);
    }
}

pub impl Monitor {
    ///
    /// Returns the primary monitor. This is usually the monitor where elements
    /// like the Windows task bar or the OS X menu bar is located.
    ///
    /// # Returns
    ///
    /// The primary monitor wrapped in `Some`, or `None` if an error occurred.
    ///
    pub fn get_primary() -> Option<Monitor> {
        do ml::get_primary_monitor().to_option().map |&ptr| {
            Monitor { ptr: ptr }
        }
    }

    ///
    /// Returns the currently connected monitors.
    ///
    /// # Returns
    ///
    /// A list of the connected monitors.
    ///
    pub fn get_connected() -> ~[Monitor] {
        ml::get_monitors().map(|&m| Monitor { ptr: m })
    }

    ///
    /// This function returns the position, in screen coordinates, of the
    /// upper-left corner of the monitor.
    ///
    /// # Returns
    ///
    /// A tuple holding the x-coordinate and y-coordinate of the monitor
    ///
    fn get_pos(&self) -> (int, int) {
        match ml::get_monitor_pos(self.ptr) {
            (xpos, ypos) => (xpos as int, ypos as int)
        }
    }

    ///
    /// This function returns the size, in millimetres, of the display area of
    /// the monitor.
    ///
    /// # Returns
    ///
    /// A tuple holding the width and height of the monitor in mm
    ///
    /// # Note
    ///
    /// Some operating systems do not provide accurate information, either
    /// because the monitor's EDID data is incorrect, or because the driver does
    /// not report it accurately.
    ///
    fn get_physical_size(&self) -> (int, int) {
        match ml::get_monitor_physical_size(self.ptr) {
            (width, height) => (width as int, height as int)
        }
    }

    ///
    /// Returns a human-readable name of the monitor.
    ///
    /// # Returns
    ///
    /// The name of the monitor. The string is empty if an error occurred.
    ///
    fn get_name(&self) -> ~str {
        ml::get_monitor_name(self.ptr)
    }

    ///
    /// This function returns an vector of all video modes supported by the
    /// specified monitor. The returned vector is sorted in ascending order,
    /// first by color bit depth (the sum of all channel depths) and then by
    /// resolution area (the product of width and height).
    ///
    /// # Returns
    ///
    /// An vector of the available modes. The vector is empty if an error
    /// occurred.
    ///
    fn get_video_modes(&self) -> ~[VidMode] {
        unsafe { cast::transmute(ml::get_video_modes(self.ptr)) }
    }

    ///
    /// Returns the current video mode of the specified monitor. If you are using
    /// a full screen window, the return value will depend on whether it is focused.
    ///
    /// # Returns
    ///
    /// The current mode of the monitor wrapped in `Some`, or `None` if an error
    /// occurred.
    ///
    fn get_video_mode(&self) -> Option<VidMode> {
        do ml::get_video_mode(self.ptr).map |&vid_mode| {
            unsafe { cast::transmute(vid_mode) }
        }
    }

    ///
    /// Generates a gamma ramp from the specified exponent and then calls
    /// `Window::set_gamma_ramp` with it.
    ///
    /// # Parameters
    ///
    /// - `gamma`: The desired exponent.
    ///
    pub fn set_gamma(&self, gamma: float) {
        ml::set_gamma(self.ptr, gamma as c_float);
    }

    ///
    /// Retrieves the current gamma ramp of the specified monitor.
    ///
    pub fn get_gamma_ramp(&self) -> GammaRamp {
        unsafe { cast::transmute(ml::get_gamma_ramp(self.ptr)) }
    }

    ///
    /// Sets the current gamma ramp of the monitor.
    ///
    /// # Parameters
    ///
    /// - `ramp`: The gamma ramp to use.
    ///
    pub fn set_gamma_ramp(&self, ramp: &GammaRamp) {
        ml::set_gamma_ramp(self.ptr, unsafe { cast::transmute(ramp) });
    }

    ///
    /// Sets the monitor configuration callback. This is called when a monitor
    /// is connected to or disconnected from the system.
    ///
    /// # Parameters
    ///
    /// - `cbfun`: The new callback.
    ///
    fn set_callback(cbfun: MonitorFun) {
        do private::set_monitor_fun(cbfun) |ext_cb| {
            ml::set_monitor_callback(ext_cb);
        }
    }
}

impl ToStr for VidMode {
    ///
    /// Returns a string representation of the video mode.
    ///
    /// # Returns
    ///
    /// A string in the form:
    ///
    /// ~~~
    /// ~"[width] x [height] [total_bits] ([red_bits] [green_bits] [blue_bits])"
    /// ~~~
    ///
    fn to_str(&self) -> ~str {
        fmt!("%? x %? %? (%? %? %?)",
             self.width, self.height,
             (self.red_bits + self.green_bits + self.blue_bits),
             self.red_bits, self.green_bits, self.blue_bits)
    }
}

macro_rules! window_hints(
    ($(fn $name:ident $(($arg_name:ident: $arg_ty:ty => $hint:expr, $arg_conv:expr))+)+) => (
        pub mod window_hint {
            use core::libc::c_int;
            use ml;

            ///
            /// Resets all window hints to their default values.
            ///
            pub fn default() {
                ml::default_window_hints();
            }

            $(pub fn $name($($arg_name: $arg_ty),+) {
                $(ml::window_hint($hint, $arg_conv);)+
            })+
        }
    )
)

window_hints!(
    fn red_bits               (bits: uint       => ml::RED_BITS, bits as c_int)
    fn green_bits             (bits: uint       => ml::GREEN_BITS, bits as c_int)
    fn blue_bits              (bits: uint       => ml::BLUE_BITS, bits as c_int)
    fn alpha_bits             (bits: uint       => ml::ALPHA_BITS, bits as c_int)
    fn depth_bits             (bits: uint       => ml::DEPTH_BITS, bits as c_int)
    fn stencil_bits           (bits: uint       => ml::STENCIL_BITS, bits as c_int)
    fn accum_red_bits         (bits: uint       => ml::ACCUM_RED_BITS, bits as c_int)
    fn accum_green_bits       (bits: uint       => ml::ACCUM_GREEN_BITS, bits as c_int)
    fn accum_blue_bits        (bits: uint       => ml::ACCUM_BLUE_BITS, bits as c_int)
    fn accum_alpha_bits       (bits: uint       => ml::ACCUM_ALPHA_BITS, bits as c_int)
    fn aux_buffers            (buffers: uint    => ml::AUX_BUFFERS, buffers as c_int)
    fn stereo                 (value: bool      => ml::STEREO, value as c_int)
    fn samples                (samples: uint    => ml::SAMPLES, samples as c_int)
    fn srgb_capable           (value: bool      => ml::SRGB_CAPABLE, value as c_int)
    fn client_api             (api: c_int       => ml::CLIENT_API, api)
    fn context_version_major  (major: uint      => ml::CONTEXT_VERSION_MAJOR, major as c_int)
    fn context_version_minor  (minor: uint      => ml::CONTEXT_VERSION_MINOR, minor as c_int)
    fn context_version        (major: uint      => ml::CONTEXT_VERSION_MAJOR, major as c_int)
                              (minor: uint      => ml::CONTEXT_VERSION_MINOR, minor as c_int)
    fn context_robustness     (value: bool      => ml::CONTEXT_ROBUSTNESS, value as c_int)
    fn opengl_forward_compat  (value: bool      => ml::OPENGL_FORWARD_COMPAT, value as c_int)
    fn opengl_debug_context   (value: bool      => ml::OPENGL_DEBUG_CONTEXT, value as c_int)
    fn opengl_profile         (profile: c_int   => ml::OPENGL_PROFILE, profile)
    fn resizable              (value: bool      => ml::RESIZABLE, value as c_int)
    fn visible                (value: bool      => ml::VISIBLE, value as c_int)
    fn decorated              (value: bool      => ml::DECORATED, value as c_int)
)

///
/// Describes the mode of a window
///
pub enum WindowMode {
    ///
    /// Full screen mode. Contains the monitor on which the window is displayed.
    ///
    FullScreen(Monitor),
    ///
    /// Windowed mode.
    ///
    Windowed,
}

///
/// Private conversion methods for `glfw::WindowMode`
///
priv impl WindowMode {
    ///
    /// Extract the window mode from a low-level monitor pointer. If the pointer
    /// is null it assumes the window is in windowed mode and returns `Windowed`,
    /// otherwise it returns the pointer wrapped in `glfw::FullScreen`.
    ///
    fn from_ptr(ptr: *ml::GLFWmonitor) -> WindowMode {
        if ptr.is_null() {
            Windowed
        } else {
            FullScreen(Monitor { ptr: ptr })
        }
    }

    ///
    /// Returns a pointer to a monitor if the window is fullscreen, otherwise
    /// it returns a null pointer (if it is in windowed mode).
    ///
    fn to_ptr(&self) -> *ml::GLFWmonitor {
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

pub impl Window {
    ///
    /// Creates a new window and an associated context.
    ///
    /// # Parameters
    ///
    /// - `width`: The desired window width, in screen coordinates.
    /// - `height`: The desired window height, in screen coordinates.
    /// - `title`: The initial window title.
    /// - `mode`: The mode of the window, either `glfw::Windowed` or
    ///   `glfw::FullScreen`
    ///
    /// # Returns
    ///
    /// The handle of the created window, wrapped in `Some`, or `None` if an
    /// error occurred.
    ///
    fn create(width: uint, height: uint, title: &str, mode: WindowMode) -> Option<Window> {
        Window::create_shared(width, height, title, mode, &Window { ptr: ptr::null() })
    }

    fn create_shared(width: uint, height: uint, title: &str,
                     mode: WindowMode, share: &Window) -> Option<Window> {
        do ml::create_window(
            width as c_int,
            height as c_int,
            title,
            mode.to_ptr(),
            share.ptr
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

    fn should_close(&self) -> bool {
        ml::window_should_close(self.ptr) as bool
    }

    fn set_should_close(&self, value: bool) {
        ml::set_window_should_close(self.ptr, value as c_int)
    }

    ///
    /// Sets the window title.
    ///
    /// # Parameters
    ///
    /// - `title`: The new window title.
    ///
    fn set_title(&self, title: &str) {
        ml::set_window_title(self.ptr, title);
    }

    fn get_pos(&self) -> (int, int) {
        match ml::get_window_pos(self.ptr) {
            (xpos, ypos) => (xpos as int, ypos as int)
        }
    }

    fn set_pos(&self, xpos: int, ypos: int) {
        ml::set_window_pos(self.ptr, xpos as c_int, ypos as c_int);
    }

    fn get_size(&self) -> (int, int) {
        match ml::get_window_size(self.ptr) {
            (width, height) => (width as int, height as int)
        }
    }

    fn set_size(&self, width: int, height: int) {
        ml::set_window_size(self.ptr, width as c_int, height as c_int);
    }

    fn iconify(&self) {
        ml::iconify_window(self.ptr);
    }

    fn restore(&self) {
        ml::restore_window(self.ptr);
    }

    fn show(&self) {
        ml::show_window(self.ptr);
    }

    fn hide(&self) {
        ml::hide_window(self.ptr);
    }

    /// Returns the window mode; either glfw::FullScreen or glfw::Windowed
    fn get_window_mode(&self) -> WindowMode {
        WindowMode::from_ptr(
            ml::get_window_monitor(self.ptr)
        )
    }

    ///
    /// Returns a boolean indicating whether the window is in focus.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::FOCUSED`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn is_focused(&self) -> bool {
        ml::get_window_param(self.ptr, FOCUSED) as bool
    }

    ///
    /// Returns a boolean indicating whether the window is iconified.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::ICONIFIED`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn is_iconified(&self) -> bool {
        ml::get_window_param(self.ptr, ICONIFIED) as bool
    }

    ///
    /// Returns the client API provided by the window's context.
    ///
    /// # Returns
    ///
    /// Either `glfw::OPENGL_API` or `glfw::OPENGL_ES_API`.
    ///
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::CLIENT_API`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn get_client_api(&self) -> c_int {
        ml::get_window_param(self.ptr, CLIENT_API)
    }

    ///
    /// Returns the client API version of the window's context in a version
    /// struct.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` three times
    /// with the following constants in turn: `glfw::CONTEXT_VERSION_MAJOR`,
    /// `glfw::CONTEXT_VERSION_MINOR` and `glfw::CONTEXT_REVISION`.
    ///
    fn get_context_version(&self) -> Version {
        Version {
            major:  ml::get_window_param(self.ptr, CONTEXT_VERSION_MAJOR) as uint,
            minor:  ml::get_window_param(self.ptr, CONTEXT_VERSION_MINOR) as uint,
            rev:    ml::get_window_param(self.ptr, CONTEXT_REVISION) as uint,
        }
    }

    ///
    /// Returns the robustness strategy used by the window's context.
    ///
    /// # Returns
    ///
    /// `glfw::LOSE_CONTEXT_ON_RESET` or `glfw::NO_RESET_NOTIFICATION` if the
    /// context supports robustness, or `glfw::NO_ROBUSTNESS` otherwise.
    ///
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::CONTEXT_ROBUSTNESS`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn get_context_robustness(&self) -> c_int {
        ml::get_window_param(self.ptr, CONTEXT_ROBUSTNESS)
    }

    ///
    /// Returns a boolean indicating whether the window's context is forward
    /// compatible.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::OPENGL_FORWARD_COMPAT`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn is_opengl_forward_compat(&self) -> bool {
        ml::get_window_param(self.ptr, OPENGL_FORWARD_COMPAT) as bool
    }

    ///
    /// Returns a boolean indicating whether the window's context is an OpenGL
    /// debug context.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::OPENGL_DEBUG_CONTEXT`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn is_opengl_debug_context(&self) -> bool {
        ml::get_window_param(self.ptr, OPENGL_DEBUG_CONTEXT) as bool
    }

    ///
    /// Returns the OpenGL profile used by the context.
    ///
    /// # Returns
    ///
    /// `glfw::OPENGL_CORE_PROFILE` or `glfw::OPENGL_COMPAT_PROFILE` if the
    /// context uses a known profile, or `glfw::OPENGL_NO_PROFILE` if the OpenGL
    /// profile is unknown or the context is for another client API.
    ///
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::OPENGL_PROFILE`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn get_opengl_profile(&self) -> c_int {
        ml::get_window_param(self.ptr, OPENGL_PROFILE)
    }

    ///
    /// Returns a boolean indicating whether the window is resizable.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::RESIZABLE`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn is_resizable(&self) -> bool {
        ml::get_window_param(self.ptr, RESIZABLE) as bool
    }

    ///
    /// Returns a boolean indicating whether the window is visible.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::VISIBLE`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn is_visible(&self) -> bool {
        ml::get_window_param(self.ptr, VISIBLE) as bool
    }

    ///
    /// Returns a boolean indicating whether the window is decorated.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetWindowParam` with the constant
    /// `glfw::DECORATED`. The function was divided into multiple methods to
    /// remove the need for casting between types.
    ///
    fn is_decorated(&self) -> bool {
        ml::get_window_param(self.ptr, DECORATED) as bool
    }

    ///
    /// Sets the user-defined pointer of the window. The current value is
    /// retained until the window's destructor is called. The initial value
    /// is null.
    ///
    /// # Parameters
    ///
    /// - `pointer`: The new pointer.
    ///
    fn set_user_pointer(&self, pointer: *c_void) {
        ml::set_window_user_pointer(self.ptr, pointer);
    }

    ///
    /// Returns the current value of the user-defined pointer of the
    /// specified window.
    ///
    /// # Returns
    ///
    /// The current pointer.
    ///
    fn get_user_pointer(&self) -> *c_void {
        ml::get_window_user_pointer(self.ptr)
    }

    fn set_pos_callback(&self, cbfun: WindowSizeFun) {
        set_window_callback!(setter:   set_window_pos_callback,
                             callback: window_pos_callback,
                             field:    pos_fun);
    }

    fn set_size_callback(&self, cbfun: WindowSizeFun) {
        set_window_callback!(setter:   set_window_size_callback,
                             callback: window_size_callback,
                             field:    size_fun);
    }

    fn set_close_callback(&self, cbfun: WindowCloseFun) {
        set_window_callback!(setter:   set_window_close_callback,
                             callback: window_close_callback,
                             field:    close_fun);
    }

    fn set_refresh_callback(&self, cbfun: WindowRefreshFun) {
        set_window_callback!(setter:   set_window_refresh_callback,
                             callback: window_refresh_callback,
                             field:    refresh_fun);
    }

    fn set_focus_callback(&self, cbfun: WindowFocusFun) {
        set_window_callback!(setter:   set_window_focus_callback,
                             callback: window_focus_callback,
                             field:    focus_fun);
    }

    fn set_iconify_callback(&self, cbfun: WindowIconifyFun) {
        set_window_callback!(setter:   set_window_iconify_callback,
                             callback: window_iconify_callback,
                             field:    iconify_fun);
    }

    ///
    /// Gets the current cursor mode of the window.
    ///
    /// # Returns
    ///
    /// One of the following constants: `glfw::CURSOR_NORMAL`,
    /// `glfw::CURSOR_HIDDEN` or `glfw::CURSOR_CAPTURED`.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetInputMode` with the constant
    /// `glfw::CURSOR_MODE`. The function has been divided up into separate
    /// methods to remove the need for casting between types.
    ///
    fn get_cursor_mode(&self) -> c_int {
        ml::get_input_mode(self.ptr, CURSOR_MODE)
    }

    ///
    /// Sets the cursor mode of the window.
    ///
    /// # Parameters
    ///
    /// - `mode`: The parameter can be one of the following constants:
    ///     - `glfw::CURSOR_NORMAL`: Makes the cursor visible and behave
    ///       normally.
    ///     - `glfw::CURSOR_HIDDEN`: Makes the cursor invisible when it is over
    ///       the client area of the window.
    ///     - `glfw::CURSOR_CAPTURED`: Makes the cursor invisible and unable to
    ///       leave the window but unconstrained in terms of position. This is
    ///       useful for first-person free-look cameras.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwSetInputMode` with the constant
    /// `glfw::CURSOR_MODE. The function has been divided up into separate
    /// methods to remove the need for casting between types.
    ///
    fn set_cursor_mode(&self, mode: c_int) {
        ml::set_input_mode(self.ptr, CURSOR_MODE, mode);
    }

    ///
    /// Returns the current sticky keys setting of the window.
    ///
    /// # Returns
    ///
    /// `true` if sticky keys is enabled or `false` if it is disabled.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetInputMode` with the constant
    /// `glfw::STICKY_KEYS`. The function has been divided up into separate
    /// methods to remove the need for casting between types.
    ///
    fn has_sticky_keys(&self) -> bool {
        ml::get_input_mode(self.ptr, STICKY_KEYS) as bool
    }

    ///
    /// Enables or disables sticky keys. If sticky keys are enabled, a key press
    /// will ensure that `glfw::Window::get_key` returns `glfw::Press` the next
    /// time it is called, even if the key had been released before hand.
    ///
    /// # Parameters
    ///
    /// - `value`: `true` to enable sticky keys, `false` to disable.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwSetInputMode` with the constant
    /// `glfw::STICKY_KEYS. The function has been divided up into separate
    /// methods to remove the need for casting between types.
    ///
    fn set_sticky_keys(&self, value: bool) {
        ml::set_input_mode(self.ptr, STICKY_KEYS, value as c_int);
    }

    ///
    /// Returns the current sticky mouse buttons setting of the window.
    ///
    /// # Returns
    ///
    /// `true` if sticky mouse buttons is enabled or `false` if it is disabled.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwGetInputMode` with the constant
    /// `glfw::STICKY_MOUSE_BUTTONS`. The function has been divided up into
    /// separate methods to remove the need for casting between types.
    ///
    fn has_sticky_mouse_buttons(&self) -> bool {
        ml::get_input_mode(self.ptr, STICKY_MOUSE_BUTTONS) as bool
    }

    ///
    /// Enables or disables sticky mouse buttons. If sticky keys mouse buttons
    /// enabled, a key press will ensure that `glfw::Window::get_mouse_button`
    /// returns `glfw::Press` the next time it is called, even if the key had
    /// been released before hand.
    ///
    /// # Parameters
    ///
    /// - `value`: `true` to enable sticky keys, `false` to disable.
    ///
    /// # Implementation Notes
    ///
    /// This method calls `glfw::ll::glfwSetInputMode` with the constant
    /// `glfw::STICKY_MOUSE_BUTTONS`. The function has been divided up into
    /// separate methods to remove the need for casting between types.
    ///
    fn set_sticky_mouse_buttons(&self, value: bool) {
        ml::set_input_mode(self.ptr, STICKY_MOUSE_BUTTONS, value as c_int);
    }

    ///
    /// Returns the last state reported for the specified key to the window.
    /// The returned state is one of `glfw::PRESS` or `glfw::RELEASE`. The
    /// higher-level state `glfw::REPEAT` is only reported to the key callback.
    ///
    /// If the the sticky keys input mode was enabled using
    /// `glfw::Window::set_sticky_keys`, this function returns `glfw::PRESS` the
    /// first time you call this function after a key has been pressed, even if
    /// the key has already been released.
    ///
    /// The key functions deal with physical keys, with key constants named
    /// after their use on the standard US keyboard layout. If you want to
    /// input text, use the Unicode character callback instead.
    ///
    /// # Parameters
    ///
    /// - `key`: The key to check.
    ///
    /// # Returns
    ///
    /// The state of the specified key, either `glfw::PRESS` or `glfw::RELEASE`.
    ///
    fn get_key(&self, key: c_int) -> c_int {
        ml::get_key(self.ptr, key)
    }

    fn get_mouse_button(&self, button: c_int) -> c_int {
        ml::get_mouse_button(self.ptr, button)
    }

    fn get_cursor_pos(&self) -> (float, float) {
        match ml::get_cursor_pos(self.ptr) {
            (xpos, ypos) => (xpos as float, ypos as float)
        }
    }

    fn set_cursor_pos(&self, xpos: float, ypos: float) {
        ml::set_cursor_pos(self.ptr, xpos as c_double, ypos as c_double);
    }

    fn set_key_callback(&self, cbfun: KeyFun) {
        set_window_callback!(setter:   set_key_callback,
                             callback: key_callback,
                             field:    key_fun);
    }

    fn set_char_callback(&self, cbfun: CharFun) {
        set_window_callback!(setter:   set_char_callback,
                             callback: char_callback,
                             field:    char_fun);
    }

    fn set_mouse_button_callback(&self, cbfun: MouseButtonFun) {
        set_window_callback!(setter:   set_mouse_button_callback,
                             callback: mouse_button_callback,
                             field:    mouse_button_fun);
    }

    fn set_cursor_pos_callback(&self, cbfun: CursorPosFun) {
        set_window_callback!(setter:   set_cursor_pos_callback,
                             callback: cursor_pos_callback,
                             field:    cursor_pos_fun);
    }

    fn set_cursor_enter_callback(&self, cbfun: CursorEnterFun) {
        set_window_callback!(setter:   set_cursor_enter_callback,
                             callback: cursor_enter_callback,
                             field:    cursor_enter_fun);
    }

    fn set_scroll_callback(&self, cbfun: ScrollFun) {
        set_window_callback!(setter:   set_scroll_callback,
                             callback: scroll_callback,
                             field:    scroll_fun);
    }

    ///
    /// Sets the system clipboard to the specified string.
    ///
    /// # Parameters
    ///
    /// - `string`: The string that the clipboard will be set to.
    ///
    fn set_clipboard_string(&self, string: &str) {
        ml::set_clipboard_string(self.ptr, string);
    }

    ///
    /// Returns the contents of the system clipboard if it contains
    /// or is convertible to a string.
    ///
    /// # Returns
    ///
    /// The clipboard contents.
    ///
    fn get_clipboard_string(&self) -> ~str {
        ml::get_clipboard_string(self.ptr)
    }

    fn make_context_current(&self) {
        ml::make_context_current(self.ptr);
    }

    fn is_current_context(&self) -> bool {
        self.ptr == ml::get_current_context()
    }

    ///
    /// Swaps the front and back buffers of the window.
    ///
    fn swap_buffers(&self) {
        ml::swap_buffers(self.ptr);
    }
}

pub fn detach_current_context() {
    ml::make_context_current(ptr::null());
}

impl Drop for Window {
    ///
    /// Closes the window and removes all associated callbacks.
    ///
    /// # Implementation notes
    ///
    /// Calls `glfw::ll::glfwDestroyWindow` on the window pointer and cleans up
    /// the callbacks stored in task-local storage
    ///
    fn finalize(&self) {
        ml::destroy_window(self.ptr);
        private::WindowDataMap::get().remove(&self.ptr);
    }
}

pub fn poll_events() {
    ml::poll_events();
}

pub fn wait_events() {
    ml::wait_events();
}

pub mod joystick {
    use core::libc::*;
    use ml;

    pub fn is_present(joy: c_int) -> bool {
        ml::get_joystick_param(joy, ml::PRESENT) as bool
    }

    pub fn num_axes(joy: c_int) -> Option<uint> {
        let axes = ml::get_joystick_param(joy, ml::AXES);
        if axes > 0 { Some(axes as uint) } else { None }
    }

    pub fn num_buttons(joy: c_int) -> Option<uint> {
        let buttons = ml::get_joystick_param(joy, ml::BUTTONS);
        if buttons > 0 { Some(buttons as uint) } else { None }
    }

    pub fn get_axes(joy: c_int) -> Result<~[float],()> {
        do num_axes(joy).map_default(Err(())) |&num| {
            unsafe {
                let mut axes = ~[];
                vec::grow(&mut axes, num, &0.0);
                vec::raw::set_len(&mut axes, num);

                if ::ll::glfwGetJoystickAxes(joy, &axes[0], num as c_int) > 0 {
                    Ok(axes.map(|&a| a as float))
                } else {
                    Err(())
                }
            }
        }
    }

    pub fn get_buttons(joy: c_int) -> Result<~[int],()> {
        do num_axes(joy).map_default(Err(())) |&num| {
            unsafe {
                let mut buttons = ~[];
                vec::grow(&mut buttons, num, &0);
                vec::raw::set_len(&mut buttons, num);

                if ::ll::glfwGetJoystickButtons(joy, &buttons[0], num as c_int) > 0 {
                    Ok(buttons.map(|&a| a as int))
                } else {
                    Err(())
                }
            }
        }
    }

    pub fn get_name(joy: c_int) -> ~str {
        ml::get_joystick_name(joy)
    }
}

///
/// Returns the time elapsed since GLFW was initialized, unless it was
/// subsequently altered with `glfw::set_time`.
///
/// # Note
///
/// The resolution of the timer is system dependent, but is usually on the
/// order of a few micro- or nanoseconds. It uses the highest-resolution
/// monotonic time source on each supported platform.
///
pub fn get_time() -> float {
    ml::get_time() as float
}

///
/// Sets the value of the GLFW timer.
///
/// # Note
///
/// The resolution of the timer is system dependent, but is usually on the
/// order of a few micro- or nanoseconds. It uses the highest-resolution
/// monotonic time source on each supported platform.
///
pub fn set_time(time: float) {
    ml::set_time(time as c_double);
}

pub fn set_swap_interval(interval: int) {
    ml::set_swap_interval(interval as c_int);
}

pub fn extension_supported(extension: &str) -> bool {
    ml::extension_supported(extension) as bool
}

pub fn get_proc_address(procname: &str) -> GLProc {
    ml::get_proc_address(procname)
}
