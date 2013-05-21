#[link(name = "glfw",
	   vers = "0.1",
       uuid = "6199FAD3-6D03-4E29-87E7-7DC1B1B65C2C",
	   author = "Brendan Zabarauskas",
	   url = "https://github.com/bjz/glfw3-rs")];

#[comment = "Bindings and wrapper functions for glfw3."];
#[crate_type = "lib"];

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
/// - `character`: The Unicode code point of the character.
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
pub fn spawn(f: ~fn()) {
    do task::spawn_sched(task::PlatformThread) {
        use core::unstable::finally::Finally;

        private::WindowDataMap::init();

        match ml::init() {
            ml::TRUE => {
                do f.finally {
                    ml::terminate();
                    private::WindowDataMap::remove();
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
    major: int,
    minor: int,
    rev:   int,
}

///
/// Returns a struct containing the version numbers of the underlying GLFW
/// library.
///
pub fn get_version() -> Version {
    match ml::get_version() {
        (major, minor, rev) => Version {
            major: major as int,
            minor: minor as int,
            rev:   rev   as int,
        }
    }
}

///
/// Returns a string describing the compile time configuration of the underlying
/// GLFW library.
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

///
/// Returns the currently connected monitors.
///
pub fn get_monitors() -> ~[Monitor] {
    ml::get_monitors().map(|&m| Monitor { ptr: m })
}

pub fn get_primary_monitor() -> Monitor {
    Monitor { ptr: ml::get_primary_monitor() }
}

pub impl Monitor {
    fn get_pos(&self) -> (int, int) {
        match ml::get_monitor_pos(self.ptr) {
            (xpos, ypos) => (xpos as int, ypos as int)
        }
    }

    fn get_physical_size(&self) -> (int, int) {
        match ml::get_monitor_physical_size(self.ptr) {
            (width, height) => (width as int, height as int)
        }
    }

    fn get_name(&self) -> ~str {
        ml::get_monitor_name(self.ptr)
    }

    fn get_video_modes(&self) -> ~[VidMode] {
        unsafe { cast::transmute(ml::get_video_modes(self.ptr)) }
    }

    fn get_video_mode(&self) -> VidMode {
        unsafe { cast::transmute(ml::get_video_mode(self.ptr)) }
    }

    pub fn set_gamma(&self, gamma: float) {
        ml::set_gamma(self.ptr, gamma as c_float);
    }

    pub fn get_gamma_ramp(&self) -> GammaRamp {
        unsafe { cast::transmute(ml::get_gamma_ramp(self.ptr)) }
    }

    pub fn set_gamma_ramp(&self, ramp: &GammaRamp) {
        ml::set_gamma_ramp(self.ptr, unsafe { cast::transmute(ramp) });
    }
}

fn set_monitor_callback(cbfun: MonitorFun) {
    do private::set_monitor_fun(cbfun) |ext_cb| {
        ml::set_monitor_callback(ext_cb);
    }
}

///
/// Returns a string representation of the video mode in the form:
///
/// ~~~
/// ~"[width] x [height] [total_bits] ([red_bits] [green_bits] [blue_bits])"
/// ~~~
///
impl ToStr for VidMode {
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
/// Describes the mode that a window is currently in
///
pub enum WindowMode {
    FullScreen(Monitor),
    Windowed,
}

priv impl WindowMode {
    fn from_monitor_ptr(ptr: *ml::GLFWmonitor) -> WindowMode {
        if ptr.is_null() {
            Windowed
        } else {
            FullScreen(Monitor { ptr: ptr })
        }
    }

    fn to_monitor_ptr(&self) -> *ml::GLFWmonitor {
        match *self {
            FullScreen(monitor) => monitor.ptr,
            Windowed => ptr::null()
        }
    }
}

pub impl Window {
    ///
    /// Creates a new window and an associated context, returning a handle
    /// wrapped in `Ok` if it succeeds. If an error has occured, `Err` is
    /// returned instead.
    ///
    fn create(width: uint, height: uint, title: &str, mode: WindowMode) -> Result<Window,~str> {
        Window::create_shared(width, height, title, mode, &Window { ptr: ptr::null() })
    }

    fn create_shared(width: uint, height: uint, title: &str,
                     mode: WindowMode, share: &Window) -> Result<Window,~str> {
        let window = Window {
            ptr: ml::create_window(
                width as c_int, height as c_int, title,
                mode.to_monitor_ptr(), share.ptr
            )
        };

        if !window.ptr.is_null() {
            // Initialize the local data for this window in TLS
            private::WindowDataMap::get().insert(
                window.ptr, @mut private::WindowData::new()
            );
            Ok(window)
        } else {
            Err(~"Failed to open GLFW window")
        }
    }

    /// Gets a mutable pointer to the window's local data
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

    fn get_window_mode(&self) -> WindowMode {
        WindowMode::from_monitor_ptr(
            ml::get_window_monitor(self.ptr)
        )
    }

    fn is_focused(&self) -> bool {
        ml::get_window_param(self.ptr, FOCUSED) as bool
    }

    fn is_iconified(&self) -> bool {
        ml::get_window_param(self.ptr, ICONIFIED) as bool
    }

    fn is_context_revision(&self) -> bool {
        ml::get_window_param(self.ptr, CONTEXT_REVISION) as bool
    }

    fn get_client_api(&self) -> c_int {
        ml::get_window_param(self.ptr, CLIENT_API)
    }

    fn get_context_version_major(&self) -> uint {
        ml::get_window_param(self.ptr, CONTEXT_VERSION_MAJOR) as uint
    }

    fn get_context_version_minor(&self) -> uint {
        ml::get_window_param(self.ptr, CONTEXT_VERSION_MINOR) as uint
    }

    fn get_context_version(&self) -> (uint,uint) { (
        ml::get_window_param(self.ptr, CONTEXT_VERSION_MAJOR) as uint,
        ml::get_window_param(self.ptr, CONTEXT_VERSION_MINOR) as uint,
    ) }

    fn get_context_robustness(&self) -> c_int {
        ml::get_window_param(self.ptr, CONTEXT_ROBUSTNESS)
    }

    fn is_opengl_forward_compat(&self) -> bool {
        ml::get_window_param(self.ptr, OPENGL_FORWARD_COMPAT) as bool
    }

    fn is_opengl_debug_context(&self) -> bool {
        ml::get_window_param(self.ptr, OPENGL_DEBUG_CONTEXT) as bool
    }

    fn get_opengl_profile(&self) -> c_int {
        ml::get_window_param(self.ptr, OPENGL_PROFILE)
    }

    fn is_resizable(&self) -> bool {
        ml::get_window_param(self.ptr, RESIZABLE) as bool
    }

    fn is_visible(&self) -> bool {
        ml::get_window_param(self.ptr, VISIBLE) as bool
    }

    fn is_decorated(&self) -> bool {
        ml::get_window_param(self.ptr, DECORATED) as bool
    }

    fn set_user_pointer(&self, pointer: *c_void) {
        ml::set_window_user_pointer(self.ptr, pointer);
    }

    fn get_user_pointer(&self) -> *c_void {
        ml::get_window_user_pointer(self.ptr)
    }

    fn set_pos_callback(&self, cbfun: WindowSizeFun) {
        self.get_local_data().pos_fun = Some(cbfun);
        ml::set_window_pos_callback(self.ptr, private::window_pos_callback);
    }

    fn set_size_callback(&self, cbfun: WindowSizeFun) {
        self.get_local_data().size_fun = Some(cbfun);
        ml::set_window_size_callback(self.ptr, private::window_size_callback);
    }

    fn set_close_callback(&self, cbfun: WindowCloseFun) {
        self.get_local_data().close_fun = Some(cbfun);
        ml::set_window_close_callback(self.ptr, private::window_close_callback);
    }

    fn set_refresh_callback(&self, cbfun: WindowRefreshFun) {
        self.get_local_data().refresh_fun = Some(cbfun);
        ml::set_window_refresh_callback(self.ptr, private::window_refresh_callback);
    }

    fn set_focus_callback(&self, cbfun: WindowFocusFun) {
        self.get_local_data().focus_fun = Some(cbfun);
        ml::set_window_focus_callback(self.ptr, private::window_focus_callback);
    }

    fn set_iconify_callback(&self, cbfun: WindowIconifyFun) {
        self.get_local_data().iconify_fun = Some(cbfun);
        ml::set_window_iconify_callback(self.ptr, private::window_iconify_callback);
    }

    fn get_cursor_mode(&self) -> c_int {
        ml::get_input_mode(self.ptr, CURSOR_MODE)
    }

    fn set_cursor_mode(&self, mode: c_int) {
        ml::set_input_mode(self.ptr, CURSOR_MODE, mode);
    }

    fn has_sticky_keys(&self) -> bool {
        ml::get_input_mode(self.ptr, STICKY_KEYS) as bool
    }

    fn set_sticky_keys(&self, value: bool) {
        ml::set_input_mode(self.ptr, STICKY_KEYS, value as c_int);
    }

    fn has_sticky_mouse_buttons(&self) -> bool {
        ml::get_input_mode(self.ptr, STICKY_MOUSE_BUTTONS) as bool
    }

    fn set_sticky_mouse_buttons(&self, value: bool) {
        ml::set_input_mode(self.ptr, STICKY_MOUSE_BUTTONS, value as c_int);
    }

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
        self.get_local_data().key_fun = Some(cbfun);
        ml::set_key_callback(self.ptr, private::key_callback);
    }

    fn set_char_callback(&self, cbfun: CharFun) {
        self.get_local_data().char_fun = Some(cbfun);
        ml::set_char_callback(self.ptr, private::char_callback);
    }

    fn set_mouse_button_callback(&self, cbfun: MouseButtonFun) {
        self.get_local_data().mouse_button_fun = Some(cbfun);
        ml::set_mouse_button_callback(self.ptr, private::mouse_button_callback);
    }

    fn set_cursor_pos_callback(&self, cbfun: CursorPosFun) {
        self.get_local_data().cursor_pos_fun = Some(cbfun);
        ml::set_cursor_pos_callback(self.ptr, private::cursor_pos_callback);
    }

    fn set_cursor_enter_callback(&self, cbfun: CursorEnterFun) {
        self.get_local_data().cursor_enter_fun = Some(cbfun);
        ml::set_cursor_enter_callback(self.ptr, private::cursor_enter_callback);
    }

    fn set_scroll_callback(&self, cbfun: ScrollFun) {
        self.get_local_data().scroll_fun = Some(cbfun);
        ml::set_scroll_callback(self.ptr, private::scroll_callback);
    }

    fn set_clipboard_string(&self, string: &str) {
        ml::set_clipboard_string(self.ptr, string);
    }

    fn get_clipboard_string(&self) -> ~str {
        ml::get_clipboard_string(self.ptr)
    }

    fn make_context_current(&self) {
        ml::make_context_current(self.ptr);
    }

    fn is_current_context(&self) -> bool {
        self.ptr == ml::get_current_context()
    }

    /// Swaps the front and back buffers of the window.
    fn swap_buffers(&self) {
        ml::swap_buffers(self.ptr);
    }
}

impl Drop for Window {
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
/// subsequently altered with `set_time`.
///
pub fn get_time() -> float {
    ml::get_time() as float
}

///
/// Sets the value of the GLFW timer.
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
