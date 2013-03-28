/**
 * High level bindings for GLFW.
 */

use core::libc::*;

use shared::event;
pub use shared::consts::*;

/**
 * A struct containing a low-level monitor handle
 */
pub struct Monitor(*::ml::GLFWmonitor);

/**
 * A struct containing a low-level window handle
 */
pub struct Window(*::ml::GLFWwindow);

pub type ErrorFun           = @fn(error: c_int, format: ~str);
pub type WindowPosFun       = @fn(window: &Window, width: int, height: int);
pub type WindowSizeFun      = @fn(window: &Window, width: int, height: int);
pub type WindowCloseFun     = @fn(window: &Window);
pub type WindowRefreshFun   = @fn(window: &Window);
pub type WindowFocusFun     = @fn(window: &Window, activated: bool);
pub type WindowIconifyFun   = @fn(window: &Window, iconified: bool);
pub type MouseButtonFun     = @fn(window: &Window, button: c_int, action: c_int);
pub type CursorPosFun       = @fn(window: &Window, x: int, y: int);
pub type CursorEnterFun     = @fn(window: &Window, entered: bool);
pub type ScrollFun          = @fn(window: &Window, x: f64, y: f64);
pub type KeyFun             = @fn(window: &Window, key: c_int, action: c_int);
pub type CharFun            = @fn(window: &Window, character: char);
pub type MonitorFun         = @fn(monitor: &Monitor, event: c_int);

pub struct VidMode {
    width      : c_int,
    height     : c_int,
    redBits    : c_int,
    blueBits   : c_int,
    greenBits  : c_int,
}

pub struct GammaRamp {
    red     : [c_ushort, ..GAMMA_RAMP_SIZE],
    green   : [c_ushort, ..GAMMA_RAMP_SIZE],
    blue    : [c_ushort, ..GAMMA_RAMP_SIZE],
}

pub type GLProc = ::ml::GLFWglproc;

/**
 * Initialises GLFW on the main platform thread. `glfw::terminate` is
 * automatically called on the success or failure of `f`
 */
pub fn spawn(f: ~fn()) {
    do task::spawn_sched(task::PlatformThread) {
        use core::unstable::finally::Finally;

        do (|| {
            if ::ml::init() == TRUE {
                f();        // do user-defined work
            } else {
                fail!(~"Failed to initialize GLFW");
            }
        }).finally {
            ::ml::terminate();    // terminate glfw on completion or failure
        }
    }
}

/**
 * Returns a struct containing the GLFW version numbers.
 */
pub struct Version {
    major: int,
    minor: int,
    rev:   int,
}

pub fn get_version() -> Version {
    let (major, minor, rev) = ::ml::get_version();
    Version {
        major: major as int,
        minor: minor as int,
        rev:   rev   as int,
    }
}

pub fn get_version_string() -> ~str {
    ::ml::get_version_string()
}

pub fn set_error_callback(cbfun: ErrorFun) {
    do event::error::set_callback(cbfun) |ext_cb| {
        ::ml::set_error_callback(ext_cb);
    }
}

pub fn get_monitors() -> ~[Monitor] {
    ::ml::get_monitors().map(|&p| Monitor(p))
}

pub fn get_primary_monitor() -> Monitor {
    Monitor(::ml::get_primary_monitor())
}

pub impl Monitor {
    fn ptr(&self) -> *::ml::GLFWmonitor { **self }
    
    fn is_null(&self) -> bool { self.ptr().is_null() }
    
    fn null() -> Monitor { Monitor(ptr::null()) }

    fn get_pos(&self) -> (int, int) {
        let (xpos, ypos) = ::ml::get_monitor_pos(self.ptr());
        (xpos as int, ypos as int)
    }

    fn get_physical_size(&self) -> (int, int) {
        let (width, height) = ::ml::get_monitor_physical_size(self.ptr());
        (width as int, height as int)
    }

    fn get_name(&self) -> ~str {
        ::ml::get_monitor_name(self.ptr())
    }

    fn get_video_modes(&self) -> ~[VidMode] {
        ::ml::get_video_modes(self.ptr())
    }

    fn get_video_mode(&self) -> VidMode {
        ::ml::get_video_mode(self.ptr())
    }

    /* Gamma ramp functions */

    pub fn set_gamma(&self, gamma: float) {
        ::ml::set_gamma(self.ptr(), gamma as c_float);
    }

    pub fn get_gamma_ramp(&self) -> GammaRamp {
        ::ml::get_gamma_ramp(self.ptr())
    }

    pub fn set_gamma_ramp(&self, ramp: &GammaRamp) {
        ::ml::set_gamma_ramp(self.ptr(), ramp);
    }
}

fn set_monitor_callback(cbfun: MonitorFun) {
    do event::monitor::set_callback(cbfun) |ext_cb| {
        ::ml::set_monitor_callback(ext_cb);
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

pub fn default_window_hints() {
    ::ml::default_window_hints();
}

pub fn window_hint(target: c_int, hint: c_int) {
    ::ml::window_hint(target, hint);
}

pub enum WindowMode {
    FullScreen(Monitor),
    Windowed,
}

pub impl Window {
    fn ptr(&self) -> *::ml::GLFWwindow { **self }

    fn is_null(&self) -> bool { self.ptr().is_null() }
    
    fn null() -> Window { Window(ptr::null()) }

    fn create(width: uint, height: uint, title: &str, mode: WindowMode) -> Option<Window> {
        Window::create_shared(width, height, title, mode, &Window::null())
    }

    fn create_shared(width: uint, height: uint, title: &str, window_mode: WindowMode, share: &Window) -> Option<Window> {
        let window = Window(
            ::ml::create_window(
                width as c_int,
                height as c_int,
                title,
                match window_mode {
                    FullScreen(m) => m.ptr(),
                    Windowed => ptr::null()
                },
                share.ptr()
            )
        );

        if !window.is_null() { Some(window) } else { None }
    }

    fn destroy(&self) {
        ::ml::destroy_window(self.ptr());
    }
    
    fn should_close(&self) -> bool {
        ::ml::window_should_close(self.ptr()) as bool
    }
    
    fn set_should_close(&self, value: bool) {
        ::ml::set_window_should_close(self.ptr(), value as c_int)
    }

    fn set_title(&self, title: &str) {
        ::ml::set_window_title(self.ptr(), title)
    }

    fn get_pos(&self) -> (int, int) {
        let (xpos, ypos) = ::ml::get_window_pos(self.ptr());
        (xpos as int, ypos as int)
    }

    fn set_pos(&self, xpos: int, ypos: int) {
        ::ml::set_window_pos(self.ptr(), xpos as c_int, ypos as c_int);
    }

    fn get_size(&self) -> (int, int) {
        let (width, height) = ::ml::get_window_size(self.ptr());
        (width as int, height as int)
    }

    fn set_size(&self, width: int, height: int) {
        ::ml::set_window_size(self.ptr(), width as c_int, height as c_int);
    }

    fn iconify(&self) {
        ::ml::iconify_window(self.ptr());
    }

    fn restore(&self) {
        ::ml::restore_window(self.ptr());
    }

    fn show(&self) {
        ::ml::show_window(self.ptr());
    }

    fn hide(&self) {
        ::ml::hide_window(self.ptr());
    }

    fn get_monitor(&self) -> WindowMode {
        let m = ::ml::get_window_monitor(self.ptr());
        
        if m.is_null() { Windowed } else { FullScreen(Monitor(m)) }
    }

    fn get_param(&self, param: c_int) -> c_int {
        ::ml::get_window_param(self.ptr(), param)
    }

    fn set_user_pointer(&self, pointer: *c_void) {
        ::ml::set_window_user_pointer(self.ptr(), pointer);
    }

    fn get_user_pointer(&self) -> *c_void {
        ::ml::get_window_user_pointer(self.ptr())
    }

    fn set_pos_callback(&self, cbfun: WindowSizeFun) {
        do event::windowpos::set_callback(cbfun) |ext_cb| {
            ::ml::set_window_pos_callback(self.ptr(), ext_cb);
        }
    }

    fn set_size_callback(&self, cbfun: WindowSizeFun) {
        do event::windowsize::set_callback(cbfun) |ext_cb| {
            ::ml::set_window_size_callback(self.ptr(), ext_cb);
        }
    }

    fn set_close_callback(&self, cbfun: WindowCloseFun) {
        do event::windowclose::set_callback(cbfun) |ext_cb| {
            ::ml::set_window_close_callback(self.ptr(), ext_cb);
        }
    }

    fn set_refresh_callback(&self, cbfun: WindowRefreshFun) {
        do event::windowrefresh::set_callback(cbfun) |ext_cb| {
            ::ml::set_window_refresh_callback(self.ptr(), ext_cb);
        }
    }

    fn set_focus_callback(&self, cbfun: WindowFocusFun) {
        do event::windowfocus::set_callback(cbfun) |ext_cb| {
            ::ml::set_window_focus_callback(self.ptr(), ext_cb);
        }
    }

    fn set_iconify_callback(&self, cbfun: WindowIconifyFun) {
        do event::windowiconify::set_callback(cbfun) |ext_cb| {
            ::ml::set_window_iconify_callback(self.ptr(), ext_cb);
        }
    }

    fn get_input_mode(&self, mode: c_int) -> int {
        ::ml::get_input_mode(self.ptr(), mode) as int
    }

    fn set_input_mode(&self, mode: c_int, value: int) {
        ::ml::set_input_mode(self.ptr(), mode, value as c_int);
    }

    fn get_key(&self, key: c_int) -> c_int {
        ::ml::get_key(self.ptr(), key)
    }

    fn get_mouse_button(&self, button: c_int) -> c_int {
        ::ml::get_mouse_button(self.ptr(), button)
    }

    fn get_cursor_pos(&self) -> (int, int) {
        let (xpos, ypos) = ::ml::get_cursor_pos(self.ptr());
        (xpos as int, ypos as int)
    }

    fn set_cursor_pos(&self, xpos: int, ypos: int) {
        ::ml::set_cursor_pos(self.ptr(), xpos as c_int, ypos as c_int);
    }

    fn set_key_callback(&self, cbfun: KeyFun) {
        do event::key::set_callback(cbfun) |ext_cb| {
            ::ml::set_key_callback(self.ptr(), ext_cb);
        }
    }

    fn set_char_callback(&self, cbfun: CharFun) {
        do event::char::set_callback(cbfun) |ext_cb| {
            ::ml::set_char_callback(self.ptr(), ext_cb);
        }
    }

    fn set_mouse_button_callback(&self, cbfun: MouseButtonFun) {
        do event::mousebutton::set_callback(cbfun) |ext_cb| {
            ::ml::set_mouse_button_callback(self.ptr(), ext_cb);
        }
    }

    fn set_cursor_pos_callback(&self, cbfun: CursorPosFun) {
        do event::cursorpos::set_callback(cbfun) |ext_cb| {
            ::ml::set_cursor_pos_callback(self.ptr(), ext_cb);
        }
    }

    fn set_cursor_enter_callback(&self, cbfun: CursorEnterFun) {
        do event::cursorenter::set_callback(cbfun) |ext_cb| {
            ::ml::set_cursor_enter_callback(self.ptr(), ext_cb);
        }
    }

    fn set_scroll_callback(&self, cbfun: ScrollFun) {
        do event::scroll::set_callback(cbfun) |ext_cb| {
            ::ml::set_scroll_callback(self.ptr(), ext_cb);
        }
    }

    fn set_clipboard_string(&self, string: &str) {
        ::ml::set_clipboard_string(self.ptr(), string);
    }

    fn get_clipboard_string(&self) -> ~str {
        ::ml::get_clipboard_string(self.ptr())
    }

    fn make_context_current(&self) {
        ::ml::make_context_current(self.ptr());
    }

    fn swap_buffers(&self) {
        ::ml::swap_buffers(self.ptr());
    }
}

pub fn poll_events() {
    ::ml::poll_events();
}

pub fn wait_events() {
    ::ml::wait_events();
}

pub mod joystick {
    use core::libc::*;
    
    pub fn get_param(joy: c_int, param: c_int) -> c_int  {
        ::ml::get_joystick_param(joy, param)
    }
    
    pub fn is_present(joy: c_int) -> bool {
        get_param(joy, ::hl::PRESENT) as bool
    }
    
    pub fn num_axes(joy: c_int) -> Option<uint> {
        let axes = get_param(joy, ::hl::AXES);
        if axes > 0 { Some(axes as uint) } else { None }
    }
    
    pub fn num_buttons(joy: c_int) -> Option<uint> {
        let buttons = get_param(joy, ::hl::BUTTONS);
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
    ::ml::get_time() as f64
}

pub fn set_time(time: f64) {
    ::ml::set_time(time as c_double);
}

pub fn get_current_context() -> Window {
    Window(::ml::get_current_context())
}

pub fn set_swap_interval(interval: int) {
    ::ml::set_swap_interval(interval as c_int);
}

pub fn extension_supported(extension: &str) -> bool {
    ::ml::extension_supported(extension) as bool
}

pub fn get_proc_address(procname: &str) -> GLProc {
    ::ml::get_proc_address(procname)
}