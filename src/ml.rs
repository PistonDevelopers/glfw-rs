/**
 * Mid-level wrapper functions that provide rust-style funtion names. Includes public exports of
 * core types and constants.
 */

use core::libc::*;

pub use support::consts::*;
pub use support::types::*;

pub fn init() -> c_int {
    unsafe { ::ll::glfwInit() }
}

pub fn terminate() {
    unsafe { ::ll::glfwTerminate(); }
}

pub fn get_version() -> (c_int, c_int, c_int) {
    unsafe {
        let mut major = 0,
                minor = 0,
                rev   = 0;
        ::ll::glfwGetVersion(&major, &minor, &rev);
        (major, minor, rev)
    }
}

pub fn get_version_string() -> ~str {
    unsafe { str::raw::from_c_str(::ll::glfwGetVersionString()) }
}

pub fn set_error_callback(cbfun: GLFWerrorfun) {
    unsafe { ::ll::glfwSetErrorCallback(cbfun); }
}

pub fn get_monitors() -> ~[*GLFWmonitor] {
    unsafe {
        let mut count = 0;
        let ptr = ::ll::glfwGetMonitors(&count);
        vec::from_buf(ptr, count as uint)
    }
}

pub fn get_primary_monitor() -> *GLFWmonitor {
    unsafe { ::ll::glfwGetPrimaryMonitor() }
}

pub fn get_monitor_pos(monitor: *GLFWmonitor) -> (c_int, c_int) {
    unsafe {
        let mut xpos = 0, ypos = 0;
        ::ll::glfwGetMonitorPos(monitor, &xpos, &ypos);
        (xpos, ypos)
    }
}

pub fn get_monitor_physical_size(monitor: *GLFWmonitor) -> (c_int, c_int) {
    unsafe {
        let mut width = 0, height = 0;
        ::ll::glfwGetMonitorPhysicalSize(monitor, &width, &height);
        (width, height)
    }
}

pub fn get_monitor_name(monitor: *GLFWmonitor) -> ~str {
    unsafe { str::raw::from_c_str(::ll::glfwGetMonitorName(monitor)) }
}

pub fn get_video_modes(monitor: *GLFWmonitor) -> ~[GLFWvidmode] {
    unsafe {
        let mut count = 0;
        let ptr = ::ll::glfwGetVideoModes(monitor, &count);
        vec::from_buf(ptr, count as uint)
    }
}

pub fn get_video_mode(monitor: *GLFWmonitor) -> GLFWvidmode {
    unsafe { ::ll::glfwGetVideoMode(monitor) }
}

pub fn set_gamma(monitor: *GLFWmonitor, gamma: c_float) {
    unsafe { ::ll::glfwSetGamma(monitor, gamma); }
}

pub fn get_gamma_ramp(monitor: *GLFWmonitor) -> GLFWgammaramp {
    let mut ramp = ::GammaRamp {
        red:   [0, ..GAMMA_RAMP_SIZE],
        green: [0, ..GAMMA_RAMP_SIZE],
        blue:  [0, ..GAMMA_RAMP_SIZE],
    };
    unsafe { ::ll::glfwGetGammaRamp(monitor, &ramp); }
    return ramp;
}

pub fn set_gamma_ramp(monitor: *GLFWmonitor, ramp: &GLFWgammaramp) {
    unsafe { ::ll::glfwSetGammaRamp(monitor, ramp); }
}

pub fn set_monitor_callback(cbfun: GLFWmonitorfun) {
    unsafe { ::ll::glfwSetMonitorCallback(cbfun); }
}

pub fn default_window_hints() {
    unsafe { ::ll::glfwDefaultWindowHints(); }
}

pub fn window_hint(target: c_int, hint: c_int) {
    unsafe { ::ll::glfwWindowHint(target, hint); }
}

pub fn create_window(width: c_int, height: c_int, title: &str, monitor: *GLFWmonitor, share: *GLFWwindow) -> *GLFWwindow {
    unsafe { ::ll::glfwCreateWindow(width, height, str::as_c_str(title, |a| a), monitor, share) }
}

pub fn destroy_window(window: *GLFWwindow) {
    unsafe { ::ll::glfwDestroyWindow(window); }
}

pub fn window_should_close(window: *GLFWwindow) -> c_int {
    unsafe { ::ll::glfwWindowShouldClose(window) }
}

pub fn set_window_should_close(window: *GLFWwindow, value: c_int) {
    unsafe { ::ll::glfwSetWindowShouldClose(window, value) }
}

pub fn set_window_title(window: *GLFWwindow, title: &str) {
    unsafe { ::ll::glfwSetWindowTitle(window, str::as_c_str(title, |a| a)) }
}

pub fn get_window_pos(window: *GLFWwindow) -> (c_int, c_int) {
    unsafe {
        let mut xpos = 0, ypos = 0;
        ::ll::glfwGetWindowPos(window, &xpos, &ypos);
        (xpos, ypos)
    }
}

pub fn set_window_pos(window: *GLFWwindow, xpos: c_int, ypos: c_int) {
    unsafe { ::ll::glfwSetWindowPos(window, xpos, ypos); }
}

pub fn get_window_size(window: *GLFWwindow) -> (c_int, c_int) {
    unsafe {
        let mut width = 0, height = 0;
        ::ll::glfwGetWindowSize(window, &width, &height);
        (width, height)
    }
}

pub fn set_window_size(window: *GLFWwindow, width: c_int, height: c_int) {
    unsafe { ::ll::glfwSetWindowSize(window, width, height); }
}

pub fn iconify_window(window: *GLFWwindow) {
    unsafe { ::ll::glfwIconifyWindow(window); }
}

pub fn restore_window(window: *GLFWwindow) {
    unsafe { ::ll::glfwRestoreWindow(window); }
}

pub fn show_window(window: *GLFWwindow) {
    unsafe { ::ll::glfwShowWindow(window); }
}

pub fn hide_window(window: *GLFWwindow) {
    unsafe { ::ll::glfwHideWindow(window); }
}

pub fn get_window_monitor(window: *GLFWwindow) -> *GLFWmonitor {
    unsafe { ::ll::glfwGetWindowMonitor(window) }
}

pub fn get_window_param(window: *GLFWwindow, param: c_int) -> c_int {
    unsafe { ::ll::glfwGetWindowParam(window, param) }
}

pub fn set_window_user_pointer(window: *GLFWwindow, pointer: *c_void) {
    unsafe { ::ll::glfwSetWindowUserPointer(window, pointer); }
}

pub fn get_window_user_pointer(window: *GLFWwindow) -> *c_void {
    unsafe { ::ll::glfwGetWindowUserPointer(window) }
}

pub fn set_window_pos_callback(window: *GLFWwindow, cbfun: GLFWwindowposfun) {
    unsafe { ::ll::glfwSetWindowPosCallback(window, cbfun); }
}

pub fn set_window_size_callback(window: *GLFWwindow, cbfun: GLFWwindowsizefun) {
    unsafe { ::ll::glfwSetWindowSizeCallback(window, cbfun); }
}

pub fn set_window_close_callback(window: *GLFWwindow, cbfun: GLFWwindowclosefun) {
    unsafe { ::ll::glfwSetWindowCloseCallback(window, cbfun); }
}

pub fn set_window_refresh_callback(window: *GLFWwindow, cbfun: GLFWwindowrefreshfun) {
    unsafe { ::ll::glfwSetWindowRefreshCallback(window, cbfun); }
}

pub fn set_window_focus_callback(window: *GLFWwindow, cbfun: GLFWwindowfocusfun) {
    unsafe { ::ll::glfwSetWindowFocusCallback(window, cbfun); }
}

pub fn set_window_iconify_callback(window: *GLFWwindow, cbfun: GLFWwindowiconifyfun) {
    unsafe { ::ll::glfwSetWindowIconifyCallback(window, cbfun); }
}

pub fn get_input_mode(window: *GLFWwindow, mode: c_int) -> c_int {
    unsafe { ::ll::glfwGetInputMode(window, mode) }
}

pub fn set_input_mode(window: *GLFWwindow, mode: c_int, value: c_int) {
    unsafe { ::ll::glfwSetInputMode(window, mode, value); }
}

pub fn get_key(window: *GLFWwindow, key: c_int) -> c_int {
    unsafe { ::ll::glfwGetKey(window, key) }
}

pub fn get_mouse_button(window: *GLFWwindow, button: c_int) -> c_int {
    unsafe { ::ll::glfwGetMouseButton(window, button) }
}

pub fn get_cursor_pos(window: *GLFWwindow) -> (c_double, c_double) {
    unsafe {
        let mut xpos = 0.0, ypos = 0.0;
        ::ll::glfwGetCursorPos(window, &xpos, &ypos);
        (xpos, ypos)
    }
}

pub fn set_cursor_pos(window: *GLFWwindow, xpos: c_double, ypos: c_double) {
    unsafe { ::ll::glfwSetCursorPos(window, xpos, ypos); }
}

pub fn set_key_callback(window: *GLFWwindow, cbfun: GLFWkeyfun) {
    unsafe { ::ll::glfwSetKeyCallback(window, cbfun); }
}

pub fn set_char_callback(window: *GLFWwindow, cbfun: GLFWcharfun) {
    unsafe { ::ll::glfwSetCharCallback(window, cbfun); }
}

pub fn set_mouse_button_callback(window: *GLFWwindow, cbfun: GLFWmousebuttonfun) {
    unsafe { ::ll::glfwSetMouseButtonCallback(window, cbfun); }
}

pub fn set_cursor_pos_callback(window: *GLFWwindow, cbfun: GLFWcursorposfun) {
    unsafe { ::ll::glfwSetCursorPosCallback(window, cbfun); }
}

pub fn set_cursor_enter_callback(window: *GLFWwindow, cbfun: GLFWcursorenterfun) {
    unsafe { ::ll::glfwSetCursorEnterCallback(window, cbfun); }
}

pub fn set_scroll_callback(window: *GLFWwindow, cbfun: GLFWscrollfun) {
    unsafe { ::ll::glfwSetScrollCallback(window, cbfun); }
}

pub fn set_clipboard_string(window: *GLFWwindow, string: &str) {
    unsafe { ::ll::glfwSetClipboardString(window, str::as_c_str(string, |a| a)); }
}

pub fn get_clipboard_string(window: *GLFWwindow) -> ~str {
    unsafe { str::raw::from_c_str(::ll::glfwGetClipboardString(window)) }
}

pub fn make_context_current(window: *GLFWwindow) {
    unsafe { ::ll::glfwMakeContextCurrent(window); }
}

pub fn swap_buffers(window: *GLFWwindow) {
    unsafe { ::ll::glfwSwapBuffers(window); }
}

pub fn poll_events() {
    unsafe { ::ll::glfwPollEvents(); }
}

pub fn wait_events() {
    unsafe { ::ll::glfwWaitEvents(); }
}

pub fn get_joystick_param(joy: c_int, param: c_int) -> c_int {
    unsafe { ::ll::glfwGetJoystickParam(joy, param) }
}

// TODO
// pub fn get_joystick_axes(joy: c_int, axes: *c_float, numaxes: c_int) -> ~[c_int] {
//     unsafe { ::ll::glfwGetJoystickAxes(joy, ...) }
// }

// TODO
// pub fn get_joystick_buttons(joy: c_int, buttons: *c_uchar, numbuttons: c_int) -> ~[c_int] {
//     unsafe { ::ll::glfwGetJoystickButtons(joy, ...) }
// }

pub fn get_joystick_name(joy: c_int) -> ~str {
    unsafe { str::raw::from_c_str(::ll::glfwGetJoystickName(joy)) }
}

pub fn get_time() -> c_double {
    unsafe { ::ll::glfwGetTime() }
}

pub fn set_time(time: c_double) {
    unsafe { ::ll::glfwSetTime(time); }
}

pub fn get_current_context() -> *GLFWwindow {
    unsafe { ::ll::glfwGetCurrentContext() }
}

pub fn set_swap_interval(interval: c_int) {
    unsafe { ::ll::glfwSwapInterval(interval); }
}

pub fn extension_supported(extension: &str) -> c_int {
    unsafe { ::ll::glfwExtensionSupported(str::as_c_str(extension, |a| a)) }
}

pub fn get_proc_address(procname: &str) -> GLFWglproc {
    unsafe { ::ll::glfwGetProcAddress(str::as_c_str(procname, |a| a)) }
}