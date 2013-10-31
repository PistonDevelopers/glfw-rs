// Copyright 2013 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Low-level glfw bindings. Includes public exports of core types and constants.

use std::libc::*;

// re-export constants
pub use consts::*;

pub type GLFWglproc             = extern "C" fn();

pub type GLFWerrorfun           = extern "C" fn(c_int,*c_char);
pub type GLFWwindowposfun       = extern "C" fn(*GLFWwindow,c_int,c_int);
pub type GLFWwindowsizefun      = extern "C" fn(*GLFWwindow,c_int,c_int);
pub type GLFWwindowclosefun     = extern "C" fn(*GLFWwindow);
pub type GLFWwindowrefreshfun   = extern "C" fn(*GLFWwindow);
pub type GLFWwindowfocusfun     = extern "C" fn(*GLFWwindow,c_int);
pub type GLFWwindowiconifyfun   = extern "C" fn(*GLFWwindow,c_int);
pub type GLFWframebuffersizefun = extern "C" fn(*GLFWwindow,c_int,c_int);
pub type GLFWmousebuttonfun     = extern "C" fn(*GLFWwindow,c_int,c_int,c_int);
pub type GLFWcursorposfun       = extern "C" fn(*GLFWwindow,c_double,c_double);
pub type GLFWcursorenterfun     = extern "C" fn(*GLFWwindow,c_int);
pub type GLFWscrollfun          = extern "C" fn(*GLFWwindow,c_double,c_double);
pub type GLFWzoomfun            = extern "C" fn(*GLFWwindow,c_double);
pub type GLFWkeyfun             = extern "C" fn(*GLFWwindow,c_int,c_int,c_int,c_int);
pub type GLFWcharfun            = extern "C" fn(*GLFWwindow,c_uint);
pub type GLFWmonitorfun         = extern "C" fn(*GLFWmonitor,c_int);

pub struct GLFWmonitor;

pub struct GLFWwindow;

pub struct GLFWgammaramp {
    red:    *c_ushort,
    green:  *c_ushort,
    blue:   *c_ushort,
    size:   c_uint,
}

pub struct GLFWvidmode {
    width:       c_int,
    height:      c_int,
    redBits:     c_int,
    greenBits:   c_int,
    blueBits:    c_int,
    refreshRate: c_int,
}

// Linking

#[nolink]
#[cfg(target_os = "macos")]
#[link_args="-lglfw -framework Cocoa -framework OpenGL -framework IOKit -framework CoreFoundation -framework QuartzCore"]
extern { }

#[nolink]
#[cfg(target_os = "linux")]
#[link_args="-lglfw -lX11 -lXrandr -lXi -lXxf86vm"]
extern { }

// C function bindings

extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();
    pub fn glfwGetVersion(major: *c_int, minor: *c_int, rev: *c_int);
    pub fn glfwGetVersionString() -> *c_char;

    pub fn glfwSetErrorCallback(cbfun: Option<GLFWerrorfun>) -> Option<GLFWerrorfun>;

    pub fn glfwGetMonitors(count: *c_int) -> **GLFWmonitor;
    pub fn glfwGetPrimaryMonitor() -> *GLFWmonitor;
    pub fn glfwGetMonitorPos(monitor: *GLFWmonitor, xpos: *c_int, ypos: *c_int);
    pub fn glfwGetMonitorPhysicalSize(monitor: *GLFWmonitor, width: *c_int, height: *c_int);
    pub fn glfwGetMonitorName(monitor: *GLFWmonitor) -> *c_char;
    pub fn glfwSetMonitorCallback(cbfun: Option<GLFWmonitorfun>) -> Option<GLFWmonitorfun>;
    pub fn glfwGetVideoModes(monitor: *GLFWmonitor, count: *c_int) -> *GLFWvidmode;
    pub fn glfwGetVideoMode(monitor: *GLFWmonitor) -> *GLFWvidmode;
    pub fn glfwSetGamma(monitor: *GLFWmonitor, gamma: c_float);
    pub fn glfwGetGammaRamp(monitor: *GLFWmonitor) -> *GLFWgammaramp;
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
    pub fn glfwGetFramebufferSize(window: *GLFWwindow, width: *c_int, height: *c_int);
    pub fn glfwIconifyWindow(window: *GLFWwindow);
    pub fn glfwRestoreWindow(window: *GLFWwindow);
    pub fn glfwShowWindow(window: *GLFWwindow);
    pub fn glfwHideWindow(window: *GLFWwindow);
    pub fn glfwGetWindowMonitor(window: *GLFWwindow) -> *GLFWmonitor;
    pub fn glfwGetWindowAttrib(window: *GLFWwindow, attrib: c_int) -> c_int;
    pub fn glfwSetWindowUserPointer(window: *GLFWwindow, pointer: *c_void);
    pub fn glfwGetWindowUserPointer(window: *GLFWwindow) -> *c_void;
    pub fn glfwSetWindowPosCallback(window: *GLFWwindow, cbfun: Option<GLFWwindowposfun>) -> Option<GLFWwindowposfun>;
    pub fn glfwSetWindowSizeCallback(window: *GLFWwindow, cbfun: Option<GLFWwindowsizefun>) -> Option<GLFWwindowsizefun>;
    pub fn glfwSetWindowCloseCallback(window: *GLFWwindow, cbfun: Option<GLFWwindowclosefun>) -> Option<GLFWwindowclosefun>;
    pub fn glfwSetWindowRefreshCallback(window: *GLFWwindow, cbfun: Option<GLFWwindowrefreshfun>) -> Option<GLFWwindowrefreshfun>;
    pub fn glfwSetWindowFocusCallback(window: *GLFWwindow, cbfun: Option<GLFWwindowfocusfun>) -> Option<GLFWwindowfocusfun>;
    pub fn glfwSetWindowIconifyCallback(window: *GLFWwindow, cbfun: Option<GLFWwindowiconifyfun>) -> Option<GLFWwindowiconifyfun>;
    pub fn glfwSetFramebufferSizeCallback(window: *GLFWwindow, cbfun: Option<GLFWframebuffersizefun>) -> Option<GLFWframebuffersizefun>;

    pub fn glfwPollEvents();
    pub fn glfwWaitEvents();

    pub fn glfwGetInputMode(window: *GLFWwindow, mode: c_int) -> c_int;
    pub fn glfwSetInputMode(window: *GLFWwindow, mode: c_int, value: c_int);
    pub fn glfwGetKey(window: *GLFWwindow, key: c_int) -> c_int;
    pub fn glfwGetMouseButton(window: *GLFWwindow, button: c_int) -> c_int;
    pub fn glfwGetCursorPos(window: *GLFWwindow, xpos: *c_double, ypos: *c_double);
    pub fn glfwSetCursorPos(window: *GLFWwindow, xpos: c_double, ypos: c_double);
    pub fn glfwSetKeyCallback(window: *GLFWwindow, cbfun: Option<GLFWkeyfun>) -> Option<GLFWkeyfun>;
    pub fn glfwSetCharCallback(window: *GLFWwindow, cbfun: Option<GLFWcharfun>) -> Option<GLFWcharfun>;
    pub fn glfwSetMouseButtonCallback(window: *GLFWwindow, cbfun: Option<GLFWmousebuttonfun>) -> Option<GLFWmousebuttonfun>;
    pub fn glfwSetCursorPosCallback(window: *GLFWwindow, cbfun: Option<GLFWcursorposfun>) -> Option<GLFWcursorposfun>;
    pub fn glfwSetCursorEnterCallback(window: *GLFWwindow, cbfun: Option<GLFWcursorenterfun>) -> Option<GLFWcursorenterfun>;
    pub fn glfwSetScrollCallback(window: *GLFWwindow, cbfun: Option<GLFWscrollfun>) -> Option<GLFWscrollfun>;
    pub fn glfwSetZoomCallback(window: *GLFWwindow, cbfun: Option<GLFWzoomfun>) -> Option<GLFWzoomfun>;

    pub fn glfwJoystickPresent(joy: c_int) -> c_int;
    pub fn glfwGetJoystickAxes(joy: c_int, count: *c_int) -> *c_float;
    pub fn glfwGetJoystickButtons(joy: c_int, count: *c_int) -> *c_uchar;
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
    pub fn glfwGetProcAddress(procname: *c_char) -> Option<GLFWglproc>;

    // native APIs

    #[cfg(target_os="win32")] pub fn glfwGetWin32Window(window: *GLFWwindow) -> *c_void;
    #[cfg(target_os="win32")] pub fn glfwGetWGLContext(window: *GLFWwindow) -> *c_void;

    #[cfg(target_os="macos")] pub fn glfwGetCocoaWindow(window: *GLFWwindow) -> *c_void;
    #[cfg(target_os="macos")] pub fn glfwGetNSGLContext(window: *GLFWwindow) -> *c_void;

    #[cfg(target_os="linux")] pub fn glfwGetX11Window(window: *GLFWwindow) -> *c_void;
    #[cfg(target_os="linux")] pub fn glfwGetX11Display() -> *c_void;
    #[cfg(target_os="linux")] pub fn glfwGetGLXContext(window: *GLFWwindow) -> *c_void;
}
