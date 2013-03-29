/**
 * Low-level GLFW library bindings and base types.
 */

use core::libc::*;

// Include OS X Frameworks
#[nolink]
#[cfg(target_os = "macos")]
#[link_args="-framework Cocoa -framework OpenGL -framework IOKit -framework CoreFoundation"]
extern mod osx_frameworks {}

// GLFW Linking
#[link_name = "glfw"]
extern mod linkhack {}

/* Function pointer types */
// Will have to be changed once we can do external C callbacks nicely
pub type GLFWerrorfun           = *u8;  // typedef void (* GLFWerrorfun)(int,const char*);
pub type GLFWwindowposfun       = *u8;  // typedef void (* GLFWwindowposfun)(*GLFWwindow,int,int);
pub type GLFWwindowsizefun      = *u8;  // typedef void (* GLFWwindowsizefun)(*GLFWwindow,int,int);
pub type GLFWwindowclosefun     = *u8;  // typedef void (* GLFWwindowclosefun)(*GLFWwindow);
pub type GLFWwindowrefreshfun   = *u8;  // typedef void (* GLFWwindowrefreshfun)(*GLFWwindow);
pub type GLFWwindowfocusfun     = *u8;  // typedef void (* GLFWwindowfocusfun)(*GLFWwindow,int);
pub type GLFWwindowiconifyfun   = *u8;  // typedef void (* GLFWwindowiconifyfun)(*GLFWwindow,int);
pub type GLFWmousebuttonfun     = *u8;  // typedef void (* GLFWmousebuttonfun)(*GLFWwindow,int,int);
pub type GLFWcursorposfun       = *u8;  // typedef void (* GLFWcursorposfun)(*GLFWwindow,int,int);
pub type GLFWcursorenterfun     = *u8;  // typedef void (* GLFWcursorenterfun)(*GLFWwindow,int);
pub type GLFWscrollfun          = *u8;  // typedef void (* GLFWscrollfun)(*GLFWwindow,double,double);
pub type GLFWkeyfun             = *u8;  // typedef void (* GLFWkeyfun)(*GLFWwindow,int,int);
pub type GLFWcharfun            = *u8;  // typedef void (* GLFWcharfun)(*GLFWwindow,unsigned int);
pub type GLFWmonitorfun         = *u8;  // typedef void (* GLFWmonitorfun)(*GLFWmonitor,int);

/* Monitor handle type */
pub struct GLFWmonitor;

/* Window handle type */
pub struct GLFWwindow;

pub type GLFWgammaramp = ::GammaRamp;
pub type GLFWvidmode = ::VidMode;
pub type GLFWglproc = ::GLProc;

pub extern "C" {
    /* GLFW initialization, termination and version querying */
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();
    pub fn glfwGetVersion(major: *c_int, minor: *c_int, rev: *c_int);
    pub fn glfwGetVersionString() -> *c_char;

    /* Error handling */
    pub fn glfwSetErrorCallback(cbfun: GLFWerrorfun);

    /* Monitor functions */
    pub fn glfwGetMonitors(count: *c_int) -> **GLFWmonitor;
    pub fn glfwGetPrimaryMonitor() -> *GLFWmonitor;
    pub fn glfwGetMonitorPos(monitor: *GLFWmonitor, xpos: *c_int, ypos: *c_int);
    pub fn glfwGetMonitorPhysicalSize(monitor: *GLFWmonitor, width: *c_int, height: *c_int);
    pub fn glfwGetMonitorName(monitor: *GLFWmonitor) -> *c_char;
    pub fn glfwSetMonitorCallback(cbfun: GLFWmonitorfun);
    pub fn glfwGetVideoModes(monitor: *GLFWmonitor, count: *c_int) -> *GLFWvidmode;
    pub fn glfwGetVideoMode(monitor: *GLFWmonitor) -> GLFWvidmode;
    pub fn glfwSetGamma(monitor: *GLFWmonitor, gamma: c_float);
    pub fn glfwGetGammaRamp(monitor: *GLFWmonitor, ramp: *GLFWgammaramp);
    pub fn glfwSetGammaRamp(monitor: *GLFWmonitor, ramp: *GLFWgammaramp);

    /* Window handling */
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
    pub fn glfwSetWindowPosCallback(window: *GLFWwindow, cbfun: GLFWwindowposfun);
    pub fn glfwSetWindowSizeCallback(window: *GLFWwindow, cbfun: GLFWwindowsizefun);
    pub fn glfwSetWindowCloseCallback(window: *GLFWwindow, cbfun: GLFWwindowclosefun);
    pub fn glfwSetWindowRefreshCallback(window: *GLFWwindow, cbfun: GLFWwindowrefreshfun);
    pub fn glfwSetWindowFocusCallback(window: *GLFWwindow, cbfun: GLFWwindowfocusfun);
    pub fn glfwSetWindowIconifyCallback(window: *GLFWwindow, cbfun: GLFWwindowiconifyfun);

    /* Event handling */
    pub fn glfwPollEvents();
    pub fn glfwWaitEvents();

    /* Input handling */
    pub fn glfwGetInputMode(window: *GLFWwindow, mode: c_int) -> c_int;
    pub fn glfwSetInputMode(window: *GLFWwindow, mode: c_int, value: c_int);
    pub fn glfwGetKey(window: *GLFWwindow, key: c_int) -> c_int;
    pub fn glfwGetMouseButton(window: *GLFWwindow, button: c_int) -> c_int;
    pub fn glfwGetCursorPos(window: *GLFWwindow, xpos: *c_int, ypos: *c_int);
    pub fn glfwSetCursorPos(window: *GLFWwindow, xpos: c_int, ypos: c_int);
    pub fn glfwSetKeyCallback(window: *GLFWwindow, cbfun: GLFWkeyfun);
    pub fn glfwSetCharCallback(window: *GLFWwindow, cbfun: GLFWcharfun);
    pub fn glfwSetMouseButtonCallback(window: *GLFWwindow, cbfun: GLFWmousebuttonfun);
    pub fn glfwSetCursorPosCallback(window: *GLFWwindow, cbfun: GLFWcursorposfun);
    pub fn glfwSetCursorEnterCallback(window: *GLFWwindow, cbfun: GLFWcursorenterfun);
    pub fn glfwSetScrollCallback(window: *GLFWwindow, cbfun: GLFWscrollfun);

    /* Joystick input */
    pub fn glfwGetJoystickParam(joy: c_int, param: c_int) -> c_int;
    pub fn glfwGetJoystickAxes(joy: c_int, axes: *c_float, numaxes: c_int) -> c_int;
    pub fn glfwGetJoystickButtons(joy: c_int, buttons: *c_uchar, numbuttons: c_int) -> c_int;
    pub fn glfwGetJoystickName(joy: c_int) -> *c_char;

    /* Clipboard */
    pub fn glfwSetClipboardString(window: *GLFWwindow, string: *c_char);
    pub fn glfwGetClipboardString(window: *GLFWwindow) -> *c_char;

    /* Time */
    pub fn glfwGetTime() -> c_double;
    pub fn glfwSetTime(time: c_double);

    /* OpenGL support */
    pub fn glfwMakeContextCurrent(window: *GLFWwindow);
    pub fn glfwGetCurrentContext() -> *GLFWwindow;
    pub fn glfwSwapBuffers(window: *GLFWwindow);
    pub fn glfwSwapInterval(interval: c_int);
    pub fn glfwExtensionSupported(extension: *c_char) -> c_int;
    pub fn glfwGetProcAddress(procname: *c_char) -> GLFWglproc;
}
