/**
 * Low-level glfw bindings. Includes public exports of core types and constants.
 */

use core::libc::*;

pub use support::consts::*;
pub use support::types::*;

// Include OS X Frameworks
#[nolink]
#[cfg(target_os = "macos")]
#[link_args="-lglfw -framework Cocoa -framework OpenGL -framework IOKit -framework CoreFoundation"]
pub extern { }

#[nolink]
#[cfg(target_os = "linux")]
#[link_args="-lglfw"]
pub extern { }

pub extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();
    pub fn glfwGetVersion(major: *c_int, minor: *c_int, rev: *c_int);
    pub fn glfwGetVersionString() -> *c_char;

    pub fn glfwSetErrorCallback(cbfun: GLFWerrorfun) -> GLFWerrorfun;

    pub fn glfwGetMonitors(count: *c_int) -> **GLFWmonitor;
    pub fn glfwGetPrimaryMonitor() -> *GLFWmonitor;
    pub fn glfwGetMonitorPos(monitor: *GLFWmonitor, xpos: *c_int, ypos: *c_int);
    pub fn glfwGetMonitorPhysicalSize(monitor: *GLFWmonitor, width: *c_int, height: *c_int);
    pub fn glfwGetMonitorName(monitor: *GLFWmonitor) -> *c_char;
    pub fn glfwSetMonitorCallback(cbfun: GLFWmonitorfun) -> GLFWmonitorfun;
    pub fn glfwGetVideoModes(monitor: *GLFWmonitor, count: *c_int) -> *GLFWvidmode;
    pub fn glfwGetVideoMode(monitor: *GLFWmonitor) -> GLFWvidmode;
    pub fn glfwSetGamma(monitor: *GLFWmonitor, gamma: c_float);
    pub fn glfwGetGammaRamp(monitor: *GLFWmonitor, ramp: *GLFWgammaramp);
    pub fn glfwSetGammaRamp(monitor: *GLFWmonitor, ramp: *GLFWgammaramp);

    pub fn glfwDefaultWindowHints();
    pub fn glfwWindowHint(target: c_int, hint: c_int);
    pub fn glfwCreateWindow(width: c_int, height: c_int, title: *c_char, monitor: *GLFWmonitor, share: *GLFWwindow) -> *GLFWwindow;
    pub fn glfwDestroyWindow(window: *GLFWwindow);
    pub fn glfwWindowShouldClose(window: *GLFWwindow) -> c_int;
    pub fn glfwSetWindowShouldClose(window: *GLFWwindow, value: c_int);
    pub fn glfwSetWindowTitle(window: *GLFWwindow, title: *c_char);
    pub fn glfwGetWindowPos(window: *GLFWwindow, xpos: *c_int, ypos: *c_int);
    pub fn glfwSetWindowPos(window: *GLFWwindow, xpos: c_int, ypos: c_int);
    pub fn glfwGetWindowSize(window: *GLFWwindow, width: *c_int, height: *c_int);
    pub fn glfwSetWindowSize(window: *GLFWwindow, width: c_int, height: c_int);
    pub fn glfwIconifyWindow(window: *GLFWwindow);
    pub fn glfwRestoreWindow(window: *GLFWwindow);
    pub fn glfwShowWindow(window: *GLFWwindow);
    pub fn glfwHideWindow(window: *GLFWwindow);
    pub fn glfwGetWindowMonitor(window: *GLFWwindow) -> *GLFWmonitor;
    pub fn glfwGetWindowParam(window: *GLFWwindow, param: c_int) -> c_int;
    pub fn glfwSetWindowUserPointer(window: *GLFWwindow, pointer: *c_void);
    pub fn glfwGetWindowUserPointer(window: *GLFWwindow) -> *c_void;
    pub fn glfwSetWindowPosCallback(window: *GLFWwindow, cbfun: GLFWwindowposfun) -> GLFWwindowposfun;
    pub fn glfwSetWindowSizeCallback(window: *GLFWwindow, cbfun: GLFWwindowsizefun) -> GLFWwindowsizefun;
    pub fn glfwSetWindowCloseCallback(window: *GLFWwindow, cbfun: GLFWwindowclosefun) -> GLFWwindowclosefun;
    pub fn glfwSetWindowRefreshCallback(window: *GLFWwindow, cbfun: GLFWwindowrefreshfun) -> GLFWwindowrefreshfun;
    pub fn glfwSetWindowFocusCallback(window: *GLFWwindow, cbfun: GLFWwindowfocusfun) -> GLFWwindowfocusfun;
    pub fn glfwSetWindowIconifyCallback(window: *GLFWwindow, cbfun: GLFWwindowiconifyfun) -> GLFWwindowiconifyfun;

    pub fn glfwPollEvents();
    pub fn glfwWaitEvents();

    pub fn glfwGetInputMode(window: *GLFWwindow, mode: c_int) -> c_int;
    pub fn glfwSetInputMode(window: *GLFWwindow, mode: c_int, value: c_int);
    pub fn glfwGetKey(window: *GLFWwindow, key: c_int) -> c_int;
    pub fn glfwGetMouseButton(window: *GLFWwindow, button: c_int) -> c_int;
    pub fn glfwGetCursorPos(window: *GLFWwindow, xpos: *c_double, ypos: *c_double);
    pub fn glfwSetCursorPos(window: *GLFWwindow, xpos: c_double, ypos: c_double);
    pub fn glfwSetKeyCallback(window: *GLFWwindow, cbfun: GLFWkeyfun) -> GLFWkeyfun;
    pub fn glfwSetCharCallback(window: *GLFWwindow, cbfun: GLFWcharfun) -> GLFWcharfun;
    pub fn glfwSetMouseButtonCallback(window: *GLFWwindow, cbfun: GLFWmousebuttonfun) -> GLFWmousebuttonfun;
    pub fn glfwSetCursorPosCallback(window: *GLFWwindow, cbfun: GLFWcursorposfun) -> GLFWcursorposfun;
    pub fn glfwSetCursorEnterCallback(window: *GLFWwindow, cbfun: GLFWcursorenterfun) -> GLFWcursorenterfun;
    pub fn glfwSetScrollCallback(window: *GLFWwindow, cbfun: GLFWscrollfun) -> GLFWscrollfun;

    pub fn glfwGetJoystickParam(joy: c_int, param: c_int) -> c_int;
    pub fn glfwGetJoystickAxes(joy: c_int, axes: *c_float, numaxes: c_int) -> c_int;
    pub fn glfwGetJoystickButtons(joy: c_int, buttons: *c_uchar, numbuttons: c_int) -> c_int;
    pub fn glfwGetJoystickName(joy: c_int) -> *c_char;

    pub fn glfwSetClipboardString(window: *GLFWwindow, string: *c_char);
    pub fn glfwGetClipboardString(window: *GLFWwindow) -> *c_char;

    pub fn glfwGetTime() -> c_double;
    pub fn glfwSetTime(time: c_double);

    pub fn glfwMakeContextCurrent(window: *GLFWwindow);
    pub fn glfwGetCurrentContext() -> *GLFWwindow;
    pub fn glfwSwapBuffers(window: *GLFWwindow);
    pub fn glfwSwapInterval(interval: c_int);
    pub fn glfwExtensionSupported(extension: *c_char) -> c_int;
    pub fn glfwGetProcAddress(procname: *c_char) -> GLFWglproc;
}
