#[link(name = "glfw",
	   vers = "0.1",
       uuid = "6199FAD3-6D03-4E29-87E7-7DC1B1B65C2C",
	   author = "Brendan Zabarauskas",
	   url = "https://github.com/bjz/glfw3-rs")];

#[comment = "Bindings and wrapper functions for glfw3."];
#[crate_type = "lib"];

use core::libc::*;

use support::event;
pub use support::consts::*;

/// Low-level bindings
pub mod ll;

/// Mid-level wrapper functions
pub mod ml;

pub mod support {
    pub mod consts;
    pub mod event;
    pub mod types;
}

/**
 * A struct containing a low-level monitor handle
 */
pub struct Monitor { ptr: *::ml::GLFWmonitor }

/**
 * A struct containing a low-level window handle
 */
pub struct Window { ptr: *::ml::GLFWwindow }

pub type ErrorFun           = @fn(error: c_int, format: ~str);
pub type WindowPosFun       = @fn(window: &Window, width: int, height: int);
pub type WindowSizeFun      = @fn(window: &Window, width: int, height: int);
pub type WindowCloseFun     = @fn(window: &Window);
pub type WindowRefreshFun   = @fn(window: &Window);
pub type WindowFocusFun     = @fn(window: &Window, activated: bool);
pub type WindowIconifyFun   = @fn(window: &Window, iconified: bool);
pub type MouseButtonFun     = @fn(window: &Window, button: c_int, action: c_int);
pub type CursorPosFun       = @fn(window: &Window, xpos: float, ypos: float);
pub type CursorEnterFun     = @fn(window: &Window, entered: bool);
pub type ScrollFun          = @fn(window: &Window, xpos: float, ypos: float);
pub type KeyFun             = @fn(window: &Window, key: c_int, action: c_int);
pub type CharFun            = @fn(window: &Window, character: char);
pub type MonitorFun         = @fn(monitor: &Monitor, event: c_int);

pub struct VidMode {
    width:      c_int,
    height:     c_int,
    redBits:    c_int,
    greenBits:  c_int,
    blueBits:   c_int,
}

pub struct GammaRamp {
    red:    [c_ushort, ..GAMMA_RAMP_SIZE],
    green:  [c_ushort, ..GAMMA_RAMP_SIZE],
    blue:   [c_ushort, ..GAMMA_RAMP_SIZE],
}

pub type GLProc = ::ml::GLFWglproc;

/**
 * Initialises GLFW on the main platform thread. `glfw::terminate` is
 * automatically called on the success or failure of `f`
 */
pub fn spawn(f: ~fn()) {
    do task::spawn_sched(task::PlatformThread) {
        use core::unstable::finally::Finally;
        
        match ml::init() { 
            FALSE => fail!(~"Failed to initialize GLFW"),
            _ => f.finally(ml::terminate),
        }
    }
}

pub struct Version {
    major: int,
    minor: int,
    rev:   int,
}

/**
 * Returns a struct containing the GLFW version numbers.
 */
pub fn get_version() -> Version {
    match ml::get_version() {
        (major, minor, rev) => Version {
            major: major as int,
            minor: minor as int,
            rev:   rev   as int,
        }
    }
}

pub fn get_version_string() -> ~str {
    ml::get_version_string()
}

pub fn set_error_callback(cbfun: ErrorFun) {
    do event::error::set_callback(cbfun) |ext_cb| {
        ml::set_error_callback(ext_cb);
    }
}

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
        ml::get_video_modes(self.ptr)
    }

    fn get_video_mode(&self) -> VidMode {
        ml::get_video_mode(self.ptr)
    }

    /* Gamma ramp functions */

    pub fn set_gamma(&self, gamma: float) {
        ml::set_gamma(self.ptr, gamma as c_float);
    }

    pub fn get_gamma_ramp(&self) -> GammaRamp {
        ml::get_gamma_ramp(self.ptr)
    }

    pub fn set_gamma_ramp(&self, ramp: &GammaRamp) {
        ml::set_gamma_ramp(self.ptr, ramp);
    }
}

fn set_monitor_callback(cbfun: MonitorFun) {
    do event::monitor::set_callback(cbfun) |ext_cb| {
        ml::set_monitor_callback(ext_cb);
    }
}

impl ToStr for VidMode {
    fn to_str(&self) -> ~str {
        fmt!("%? x %? %? (%? %? %?)",
             self.width, self.height,
             (self.redBits + self.blueBits + self.greenBits),
             self.redBits, self.blueBits, self.greenBits)
    }
}

/* Window handling */

macro_rules! hint(
    ($name:ident $(($arg_name:ident: $arg_ty:ty) => ($hint:expr, $arg_conv:expr))+) => (
        pub fn $name($($arg_name: $arg_ty),+) {
            $(::ml::window_hint($hint, $arg_conv);)+
        }
    )
)

pub mod window_hint {
    use core::libc::c_int;
    
    pub fn default() {
        ::ml::default_window_hints();
    }
    
    hint!(red_bits               (bits: uint)      => (::RED_BITS, bits as c_int))
    hint!(green_bits             (bits: uint)      => (::GREEN_BITS, bits as c_int))
    hint!(blue_bits              (bits: uint)      => (::BLUE_BITS, bits as c_int))
    hint!(alpha_bits             (bits: uint)      => (::ALPHA_BITS, bits as c_int))
    hint!(depth_bits             (bits: uint)      => (::DEPTH_BITS, bits as c_int))
    hint!(stencil_bits           (bits: uint)      => (::STENCIL_BITS, bits as c_int))
    hint!(accum_red_bits         (bits: uint)      => (::ACCUM_RED_BITS, bits as c_int))
    hint!(accum_green_bits       (bits: uint)      => (::ACCUM_GREEN_BITS, bits as c_int))
    hint!(accum_blue_bits        (bits: uint)      => (::ACCUM_BLUE_BITS, bits as c_int))
    hint!(accum_alpha_bits       (bits: uint)      => (::ACCUM_ALPHA_BITS, bits as c_int))
    hint!(aux_buffers            (buffers: uint)   => (::AUX_BUFFERS, buffers as c_int))
    hint!(stereo                 (value: bool)     => (::STEREO, value as c_int))
    hint!(samples                (samples: uint)   => (::SAMPLES, samples as c_int))
    hint!(srgb_capable           (value: bool)     => (::SRGB_CAPABLE, value as c_int))
    hint!(client_api             (api: c_int)      => (::CLIENT_API, api))
    hint!(context_version_major  (major: uint)     => (::CONTEXT_VERSION_MAJOR, major as c_int))
    hint!(context_version_minor  (minor: uint)     => (::CONTEXT_VERSION_MINOR, minor as c_int))
    hint!(context_version        (major: uint)     => (::CONTEXT_VERSION_MAJOR, major as c_int)
                                 (minor: uint)     => (::CONTEXT_VERSION_MINOR, minor as c_int))
    hint!(context_robustness     (value: bool)     => (::CONTEXT_ROBUSTNESS, value as c_int))
    hint!(opengl_forward_compat  (value: bool)     => (::OPENGL_FORWARD_COMPAT, value as c_int))
    hint!(opengl_debug_context   (value: bool)     => (::OPENGL_DEBUG_CONTEXT, value as c_int))
    hint!(opengl_profile         (profile: c_int)  => (::OPENGL_PROFILE, profile))
    hint!(resizable              (value: bool)     => (::RESIZABLE, value as c_int))
    hint!(visible                (value: bool)     => (::VISIBLE, value as c_int))
    hint!(undecorated            (value: bool)     => (::UNDECORATED, value as c_int))
}

pub enum WindowMode {
    FullScreen(Monitor),
    Windowed,
}

pub impl Window {
    fn create(width: uint, height: uint, title: &str, mode: WindowMode) -> Result<Window,~str> {
        Window::create_shared(width, height, title, mode, &Window { ptr: ptr::null() })
    }

    fn create_shared(width: uint, height: uint, title: &str,
                     mode: WindowMode, share: &Window) -> Result<Window,~str> {
        match ml::create_window(
            width as c_int,
            height as c_int,
            title,
            match mode {
                FullScreen(m) => m.ptr,
                Windowed      => ptr::null()
            },
            share.ptr
        ) {
            w if !w.is_null() => Ok(Window { ptr: w }),
            _                 => Err(~"Failed to open GLFW window"),
        }
    }

    fn destroy(&self) {
        ml::destroy_window(self.ptr);
    }
    
    fn should_close(&self) -> bool {
        ml::window_should_close(self.ptr) as bool
    }
    
    fn set_should_close(&self, value: bool) {
        ml::set_window_should_close(self.ptr, value as c_int)
    }

    fn set_title(&self, title: &str) {
        ml::set_window_title(self.ptr, title)
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

    fn get_monitor(&self) -> WindowMode {
        match ml::get_window_monitor(self.ptr) {
            m if m.is_null() => Windowed,
            m                => FullScreen(Monitor { ptr: m }),
        }
    }

    fn is_focused(&self)                -> bool  { ml::get_window_param(self.ptr, FOCUSED)               as bool }
    fn is_iconified(&self)              -> bool  { ml::get_window_param(self.ptr, ICONIFIED)             as bool }
    fn is_context_revision(&self)       -> bool  { ml::get_window_param(self.ptr, CONTEXT_REVISION)      as bool }
    fn get_client_api(&self)            -> c_int { ml::get_window_param(self.ptr, CLIENT_API)                    }
    fn get_context_version_major(&self) -> uint  { ml::get_window_param(self.ptr, CONTEXT_VERSION_MAJOR) as uint }
    fn get_context_version_minor(&self) -> uint  { ml::get_window_param(self.ptr, CONTEXT_VERSION_MINOR) as uint }
    fn get_context_robustness(&self)    -> c_int { ml::get_window_param(self.ptr, CONTEXT_ROBUSTNESS)            }
    fn is_opengl_forward_compat(&self)  -> bool  { ml::get_window_param(self.ptr, OPENGL_FORWARD_COMPAT) as bool }
    fn is_opengl_debug_context(&self)   -> bool  { ml::get_window_param(self.ptr, OPENGL_DEBUG_CONTEXT)  as bool }
    fn get_opengl_profile(&self)        -> c_int { ml::get_window_param(self.ptr, OPENGL_PROFILE)                }
    fn is_resizable(&self)              -> bool  { ml::get_window_param(self.ptr, RESIZABLE)             as bool }
    fn is_visible(&self)                -> bool  { ml::get_window_param(self.ptr, VISIBLE)               as bool }
    fn is_undecorated(&self)            -> bool  { ml::get_window_param(self.ptr, UNDECORATED)           as bool }

    fn set_user_pointer(&self, pointer: *c_void) {
        ml::set_window_user_pointer(self.ptr, pointer);
    }

    fn get_user_pointer(&self) -> *c_void {
        ml::get_window_user_pointer(self.ptr)
    }

    fn set_pos_callback(&self, cbfun: WindowSizeFun) {
        do event::windowpos::set_callback(cbfun) |ext_cb| {
            ml::set_window_pos_callback(self.ptr, ext_cb);
        }
    }

    fn set_size_callback(&self, cbfun: WindowSizeFun) {
        do event::windowsize::set_callback(cbfun) |ext_cb| {
            ml::set_window_size_callback(self.ptr, ext_cb);
        }
    }

    fn set_close_callback(&self, cbfun: WindowCloseFun) {
        do event::windowclose::set_callback(cbfun) |ext_cb| {
            ml::set_window_close_callback(self.ptr, ext_cb);
        }
    }

    fn set_refresh_callback(&self, cbfun: WindowRefreshFun) {
        do event::windowrefresh::set_callback(cbfun) |ext_cb| {
            ml::set_window_refresh_callback(self.ptr, ext_cb);
        }
    }

    fn set_focus_callback(&self, cbfun: WindowFocusFun) {
        do event::windowfocus::set_callback(cbfun) |ext_cb| {
            ml::set_window_focus_callback(self.ptr, ext_cb);
        }
    }

    fn set_iconify_callback(&self, cbfun: WindowIconifyFun) {
        do event::windowiconify::set_callback(cbfun) |ext_cb| {
            ml::set_window_iconify_callback(self.ptr, ext_cb);
        }
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
        do event::key::set_callback(cbfun) |ext_cb| {
            ml::set_key_callback(self.ptr, ext_cb);
        }
    }

    fn set_char_callback(&self, cbfun: CharFun) {
        do event::char::set_callback(cbfun) |ext_cb| {
            ml::set_char_callback(self.ptr, ext_cb);
        }
    }

    fn set_mouse_button_callback(&self, cbfun: MouseButtonFun) {
        do event::mousebutton::set_callback(cbfun) |ext_cb| {
            ml::set_mouse_button_callback(self.ptr, ext_cb);
        }
    }

    fn set_cursor_pos_callback(&self, cbfun: CursorPosFun) {
        do event::cursorpos::set_callback(cbfun) |ext_cb| {
            ml::set_cursor_pos_callback(self.ptr, ext_cb);
        }
    }

    fn set_cursor_enter_callback(&self, cbfun: CursorEnterFun) {
        do event::cursorenter::set_callback(cbfun) |ext_cb| {
            ml::set_cursor_enter_callback(self.ptr, ext_cb);
        }
    }

    fn set_scroll_callback(&self, cbfun: ScrollFun) {
        do event::scroll::set_callback(cbfun) |ext_cb| {
            ml::set_scroll_callback(self.ptr, ext_cb);
        }
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

    fn swap_buffers(&self) {
        ml::swap_buffers(self.ptr);
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
    
    pub fn is_present(joy: c_int) -> bool {
        ::ml::get_joystick_param(joy, ::ml::PRESENT) as bool
    }
    
    pub fn num_axes(joy: c_int) -> Option<uint> {
        let axes = ::ml::get_joystick_param(joy, ::ml::AXES);
        if axes > 0 { Some(axes as uint) } else { None }
    }
    
    pub fn num_buttons(joy: c_int) -> Option<uint> {
        let buttons = ::ml::get_joystick_param(joy, ::ml::BUTTONS);
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
        ::ml::get_joystick_name(joy)
    }
}

pub fn get_time() -> f64 {
    ml::get_time() as f64
}

pub fn set_time(time: f64) {
    ml::set_time(time as c_double);
}

pub fn get_current_context() -> Window {
    Window { ptr: ml::get_current_context() }
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