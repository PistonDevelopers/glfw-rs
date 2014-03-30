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

//! Low-level function bindings and constants pertaining to the underlying
//! GLFW library.

use std::libc::*;

pub static FALSE                        : c_int = 0;
pub static TRUE                         : c_int = 1;

pub static RELEASE                      : c_int = 0;
pub static PRESS                        : c_int = 1;
pub static REPEAT                       : c_int = 2;

pub static KEY_SPACE                    : c_int = 32;
pub static KEY_APOSTROPHE               : c_int = 39;
pub static KEY_COMMA                    : c_int = 44;
pub static KEY_MINUS                    : c_int = 45;
pub static KEY_PERIOD                   : c_int = 46;
pub static KEY_SLASH                    : c_int = 47;
pub static KEY_0                        : c_int = 48;
pub static KEY_1                        : c_int = 49;
pub static KEY_2                        : c_int = 50;
pub static KEY_3                        : c_int = 51;
pub static KEY_4                        : c_int = 52;
pub static KEY_5                        : c_int = 53;
pub static KEY_6                        : c_int = 54;
pub static KEY_7                        : c_int = 55;
pub static KEY_8                        : c_int = 56;
pub static KEY_9                        : c_int = 57;
pub static KEY_SEMICOLON                : c_int = 59;
pub static KEY_EQUAL                    : c_int = 61;
pub static KEY_A                        : c_int = 65;
pub static KEY_B                        : c_int = 66;
pub static KEY_C                        : c_int = 67;
pub static KEY_D                        : c_int = 68;
pub static KEY_E                        : c_int = 69;
pub static KEY_F                        : c_int = 70;
pub static KEY_G                        : c_int = 71;
pub static KEY_H                        : c_int = 72;
pub static KEY_I                        : c_int = 73;
pub static KEY_J                        : c_int = 74;
pub static KEY_K                        : c_int = 75;
pub static KEY_L                        : c_int = 76;
pub static KEY_M                        : c_int = 77;
pub static KEY_N                        : c_int = 78;
pub static KEY_O                        : c_int = 79;
pub static KEY_P                        : c_int = 80;
pub static KEY_Q                        : c_int = 81;
pub static KEY_R                        : c_int = 82;
pub static KEY_S                        : c_int = 83;
pub static KEY_T                        : c_int = 84;
pub static KEY_U                        : c_int = 85;
pub static KEY_V                        : c_int = 86;
pub static KEY_W                        : c_int = 87;
pub static KEY_X                        : c_int = 88;
pub static KEY_Y                        : c_int = 89;
pub static KEY_Z                        : c_int = 90;
pub static KEY_LEFT_BRACKET             : c_int = 91;
pub static KEY_BACKSLASH                : c_int = 92;
pub static KEY_RIGHT_BRACKET            : c_int = 93;
pub static KEY_GRAVE_ACCENT             : c_int = 96;
pub static KEY_WORLD_1                  : c_int = 161;
pub static KEY_WORLD_2                  : c_int = 162;

pub static KEY_ESCAPE                   : c_int = 256;
pub static KEY_ENTER                    : c_int = 257;
pub static KEY_TAB                      : c_int = 258;
pub static KEY_BACKSPACE                : c_int = 259;
pub static KEY_INSERT                   : c_int = 260;
pub static KEY_DELETE                   : c_int = 261;
pub static KEY_RIGHT                    : c_int = 262;
pub static KEY_LEFT                     : c_int = 263;
pub static KEY_DOWN                     : c_int = 264;
pub static KEY_UP                       : c_int = 265;
pub static KEY_PAGE_UP                  : c_int = 266;
pub static KEY_PAGE_DOWN                : c_int = 267;
pub static KEY_HOME                     : c_int = 268;
pub static KEY_END                      : c_int = 269;
pub static KEY_CAPS_LOCK                : c_int = 280;
pub static KEY_SCROLL_LOCK              : c_int = 281;
pub static KEY_NUM_LOCK                 : c_int = 282;
pub static KEY_PRINT_SCREEN             : c_int = 283;
pub static KEY_PAUSE                    : c_int = 284;
pub static KEY_F1                       : c_int = 290;
pub static KEY_F2                       : c_int = 291;
pub static KEY_F3                       : c_int = 292;
pub static KEY_F4                       : c_int = 293;
pub static KEY_F5                       : c_int = 294;
pub static KEY_F6                       : c_int = 295;
pub static KEY_F7                       : c_int = 296;
pub static KEY_F8                       : c_int = 297;
pub static KEY_F9                       : c_int = 298;
pub static KEY_F10                      : c_int = 299;
pub static KEY_F11                      : c_int = 300;
pub static KEY_F12                      : c_int = 301;
pub static KEY_F13                      : c_int = 302;
pub static KEY_F14                      : c_int = 303;
pub static KEY_F15                      : c_int = 304;
pub static KEY_F16                      : c_int = 305;
pub static KEY_F17                      : c_int = 306;
pub static KEY_F18                      : c_int = 307;
pub static KEY_F19                      : c_int = 308;
pub static KEY_F20                      : c_int = 309;
pub static KEY_F21                      : c_int = 310;
pub static KEY_F22                      : c_int = 311;
pub static KEY_F23                      : c_int = 312;
pub static KEY_F24                      : c_int = 313;
pub static KEY_F25                      : c_int = 314;
pub static KEY_KP_0                     : c_int = 320;
pub static KEY_KP_1                     : c_int = 321;
pub static KEY_KP_2                     : c_int = 322;
pub static KEY_KP_3                     : c_int = 323;
pub static KEY_KP_4                     : c_int = 324;
pub static KEY_KP_5                     : c_int = 325;
pub static KEY_KP_6                     : c_int = 326;
pub static KEY_KP_7                     : c_int = 327;
pub static KEY_KP_8                     : c_int = 328;
pub static KEY_KP_9                     : c_int = 329;
pub static KEY_KP_DECIMAL               : c_int = 330;
pub static KEY_KP_DIVIDE                : c_int = 331;
pub static KEY_KP_MULTIPLY              : c_int = 332;
pub static KEY_KP_SUBTRACT              : c_int = 333;
pub static KEY_KP_ADD                   : c_int = 334;
pub static KEY_KP_ENTER                 : c_int = 335;
pub static KEY_KP_EQUAL                 : c_int = 336;
pub static KEY_LEFT_SHIFT               : c_int = 340;
pub static KEY_LEFT_CONTROL             : c_int = 341;
pub static KEY_LEFT_ALT                 : c_int = 342;
pub static KEY_LEFT_SUPER               : c_int = 343;
pub static KEY_RIGHT_SHIFT              : c_int = 344;
pub static KEY_RIGHT_CONTROL            : c_int = 345;
pub static KEY_RIGHT_ALT                : c_int = 346;
pub static KEY_RIGHT_SUPER              : c_int = 347;
pub static KEY_MENU                     : c_int = 348;
pub static KEY_LAST                     : c_int = KEY_MENU;

pub static MOD_SHIFT                    : c_int = 0x0001;
pub static MOD_CONTROL                  : c_int = 0x0002;
pub static MOD_ALT                      : c_int = 0x0004;
pub static MOD_SUPER                    : c_int = 0x0008;

pub static JOYSTICK_1                   : c_int = 0;
pub static JOYSTICK_2                   : c_int = 1;
pub static JOYSTICK_3                   : c_int = 2;
pub static JOYSTICK_4                   : c_int = 3;
pub static JOYSTICK_5                   : c_int = 4;
pub static JOYSTICK_6                   : c_int = 5;
pub static JOYSTICK_7                   : c_int = 6;
pub static JOYSTICK_8                   : c_int = 7;
pub static JOYSTICK_9                   : c_int = 8;
pub static JOYSTICK_10                  : c_int = 9;
pub static JOYSTICK_11                  : c_int = 10;
pub static JOYSTICK_12                  : c_int = 11;
pub static JOYSTICK_13                  : c_int = 12;
pub static JOYSTICK_14                  : c_int = 13;
pub static JOYSTICK_15                  : c_int = 14;
pub static JOYSTICK_16                  : c_int = 15;
pub static JOYSTICK_LAST                : c_int = JOYSTICK_16;

pub static MOUSE_BUTTON_1               : c_int = 0;
pub static MOUSE_BUTTON_2               : c_int = 1;
pub static MOUSE_BUTTON_3               : c_int = 2;
pub static MOUSE_BUTTON_4               : c_int = 3;
pub static MOUSE_BUTTON_5               : c_int = 4;
pub static MOUSE_BUTTON_6               : c_int = 5;
pub static MOUSE_BUTTON_7               : c_int = 6;
pub static MOUSE_BUTTON_8               : c_int = 7;
pub static MOUSE_BUTTON_LEFT            : c_int = MOUSE_BUTTON_1;
pub static MOUSE_BUTTON_RIGHT           : c_int = MOUSE_BUTTON_2;
pub static MOUSE_BUTTON_MIDDLE          : c_int = MOUSE_BUTTON_3;
pub static MOUSE_BUTTON_LAST            : c_int = MOUSE_BUTTON_8;

pub static NOT_INITIALIZED              : c_int = 0x00010001;
pub static NO_CURRENT_CONTEXT           : c_int = 0x00010002;
pub static INVALID_ENUM                 : c_int = 0x00010003;
pub static INVALID_VALUE                : c_int = 0x00010004;
pub static OUT_OF_MEMORY                : c_int = 0x00010005;
pub static API_UNAVAILABLE              : c_int = 0x00010006;
pub static VERSION_UNAVAILABLE          : c_int = 0x00010007;
pub static PLATFORM_ERROR               : c_int = 0x00010008;
pub static FORMAT_UNAVAILABLE           : c_int = 0x00010009;

pub static FOCUSED                      : c_int = 0x00020001;
pub static ICONIFIED                    : c_int = 0x00020002;
pub static RESIZABLE                    : c_int = 0x00020003;
pub static VISIBLE                      : c_int = 0x00020004;
pub static DECORATED                    : c_int = 0x00020005;

pub static RED_BITS                     : c_int = 0x00021001;
pub static GREEN_BITS                   : c_int = 0x00021002;
pub static BLUE_BITS                    : c_int = 0x00021003;
pub static ALPHA_BITS                   : c_int = 0x00021004;
pub static DEPTH_BITS                   : c_int = 0x00021005;
pub static STENCIL_BITS                 : c_int = 0x00021006;
pub static ACCUM_RED_BITS               : c_int = 0x00021007;
pub static ACCUM_GREEN_BITS             : c_int = 0x00021008;
pub static ACCUM_BLUE_BITS              : c_int = 0x00021009;
pub static ACCUM_ALPHA_BITS             : c_int = 0x0002100A;
pub static AUX_BUFFERS                  : c_int = 0x0002100B;
pub static STEREO                       : c_int = 0x0002100C;
pub static SAMPLES                      : c_int = 0x0002100D;
pub static SRGB_CAPABLE                 : c_int = 0x0002100E;
pub static REFRESH_RATE                 : c_int = 0x0002100F;

pub static CLIENT_API                   : c_int = 0x00022001;
pub static CONTEXT_VERSION_MAJOR        : c_int = 0x00022002;
pub static CONTEXT_VERSION_MINOR        : c_int = 0x00022003;
pub static CONTEXT_REVISION             : c_int = 0x00022004;
pub static CONTEXT_ROBUSTNESS           : c_int = 0x00022005;
pub static OPENGL_FORWARD_COMPAT        : c_int = 0x00022006;
pub static OPENGL_DEBUG_CONTEXT         : c_int = 0x00022007;
pub static OPENGL_PROFILE               : c_int = 0x00022008;

pub static OPENGL_API                   : c_int = 0x00030001;
pub static OPENGL_ES_API                : c_int = 0x00030002;

pub static NO_ROBUSTNESS                : c_int = 0x00000000;
pub static NO_RESET_NOTIFICATION        : c_int = 0x00031001;
pub static LOSE_CONTEXT_ON_RESET        : c_int = 0x00031002;

pub static OPENGL_ANY_PROFILE           : c_int = 0x00000000;
pub static OPENGL_CORE_PROFILE          : c_int = 0x00032001;
pub static OPENGL_COMPAT_PROFILE        : c_int = 0x00032002;

pub static CURSOR                       : c_int = 0x00033001;
pub static STICKY_KEYS                  : c_int = 0x00033002;
pub static STICKY_MOUSE_BUTTONS         : c_int = 0x00033003;

pub static CURSOR_NORMAL                : c_int = 0x00034001;
pub static CURSOR_HIDDEN                : c_int = 0x00034002;
pub static CURSOR_DISABLED              : c_int = 0x00034003;

pub static CONNECTED                    : c_int = 0x00040001;
pub static DISCONNECTED                 : c_int = 0x00040002;

pub type GLFWglproc             = extern "system" fn();

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
pub type GLFWkeyfun             = extern "C" fn(*GLFWwindow,c_int,c_int,c_int,c_int);
pub type GLFWcharfun            = extern "C" fn(*GLFWwindow,c_uint);
pub type GLFWmonitorfun         = extern "C" fn(*GLFWmonitor,c_int);

pub enum GLFWmonitor {}

pub enum GLFWwindow {}

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

// C function bindings

extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();
    pub fn glfwGetVersion(major: *mut c_int, minor: *mut c_int, rev: *mut c_int);
    pub fn glfwGetVersionString() -> *c_char;

    pub fn glfwSetErrorCallback(cbfun: Option<GLFWerrorfun>) -> Option<GLFWerrorfun>;

    pub fn glfwGetMonitors(count: *mut c_int) -> **GLFWmonitor;
    pub fn glfwGetPrimaryMonitor() -> *GLFWmonitor;
    pub fn glfwGetMonitorPos(monitor: *GLFWmonitor, xpos: *mut c_int, ypos: *mut c_int);
    pub fn glfwGetMonitorPhysicalSize(monitor: *GLFWmonitor, width: *mut c_int, height: *mut c_int);
    pub fn glfwGetMonitorName(monitor: *GLFWmonitor) -> *c_char;
    pub fn glfwSetMonitorCallback(cbfun: Option<GLFWmonitorfun>) -> Option<GLFWmonitorfun>;
    pub fn glfwGetVideoModes(monitor: *GLFWmonitor, count: *mut c_int) -> *GLFWvidmode;
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
    pub fn glfwGetWindowPos(window: *GLFWwindow, xpos: *mut c_int, ypos: *mut c_int);
    pub fn glfwSetWindowPos(window: *GLFWwindow, xpos: c_int, ypos: c_int);
    pub fn glfwGetWindowSize(window: *GLFWwindow, width: *mut c_int, height: *mut c_int);
    pub fn glfwSetWindowSize(window: *GLFWwindow, width: c_int, height: c_int);
    pub fn glfwGetFramebufferSize(window: *GLFWwindow, width: *mut c_int, height: *mut c_int);
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
    pub fn glfwGetCursorPos(window: *GLFWwindow, xpos: *mut c_double, ypos: *mut c_double);
    pub fn glfwSetCursorPos(window: *GLFWwindow, xpos: c_double, ypos: c_double);
    pub fn glfwSetKeyCallback(window: *GLFWwindow, cbfun: Option<GLFWkeyfun>) -> Option<GLFWkeyfun>;
    pub fn glfwSetCharCallback(window: *GLFWwindow, cbfun: Option<GLFWcharfun>) -> Option<GLFWcharfun>;
    pub fn glfwSetMouseButtonCallback(window: *GLFWwindow, cbfun: Option<GLFWmousebuttonfun>) -> Option<GLFWmousebuttonfun>;
    pub fn glfwSetCursorPosCallback(window: *GLFWwindow, cbfun: Option<GLFWcursorposfun>) -> Option<GLFWcursorposfun>;
    pub fn glfwSetCursorEnterCallback(window: *GLFWwindow, cbfun: Option<GLFWcursorenterfun>) -> Option<GLFWcursorenterfun>;
    pub fn glfwSetScrollCallback(window: *GLFWwindow, cbfun: Option<GLFWscrollfun>) -> Option<GLFWscrollfun>;

    pub fn glfwJoystickPresent(joy: c_int) -> c_int;
    pub fn glfwGetJoystickAxes(joy: c_int, count: *mut c_int) -> *c_float;
    pub fn glfwGetJoystickButtons(joy: c_int, count: *mut c_int) -> *c_uchar;
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
