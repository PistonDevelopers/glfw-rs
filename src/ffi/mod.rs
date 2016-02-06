// Copyright 2013-2014 The GLFW-RS Developers. For a full listing of the authors,
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

#![allow(bad_style)] // yeah yeah, but it's ffi

use libc::{c_char, c_double, c_float, c_int};
use libc::{c_uchar, c_uint, c_ushort, c_void};

mod link;

pub const FALSE                        : c_int = 0;
pub const TRUE                         : c_int = 1;

pub const RELEASE                      : c_int = 0;
pub const PRESS                        : c_int = 1;
pub const REPEAT                       : c_int = 2;

pub const KEY_SPACE                    : c_int = 32;
pub const KEY_APOSTROPHE               : c_int = 39;
pub const KEY_COMMA                    : c_int = 44;
pub const KEY_MINUS                    : c_int = 45;
pub const KEY_PERIOD                   : c_int = 46;
pub const KEY_SLASH                    : c_int = 47;
pub const KEY_0                        : c_int = 48;
pub const KEY_1                        : c_int = 49;
pub const KEY_2                        : c_int = 50;
pub const KEY_3                        : c_int = 51;
pub const KEY_4                        : c_int = 52;
pub const KEY_5                        : c_int = 53;
pub const KEY_6                        : c_int = 54;
pub const KEY_7                        : c_int = 55;
pub const KEY_8                        : c_int = 56;
pub const KEY_9                        : c_int = 57;
pub const KEY_SEMICOLON                : c_int = 59;
pub const KEY_EQUAL                    : c_int = 61;
pub const KEY_A                        : c_int = 65;
pub const KEY_B                        : c_int = 66;
pub const KEY_C                        : c_int = 67;
pub const KEY_D                        : c_int = 68;
pub const KEY_E                        : c_int = 69;
pub const KEY_F                        : c_int = 70;
pub const KEY_G                        : c_int = 71;
pub const KEY_H                        : c_int = 72;
pub const KEY_I                        : c_int = 73;
pub const KEY_J                        : c_int = 74;
pub const KEY_K                        : c_int = 75;
pub const KEY_L                        : c_int = 76;
pub const KEY_M                        : c_int = 77;
pub const KEY_N                        : c_int = 78;
pub const KEY_O                        : c_int = 79;
pub const KEY_P                        : c_int = 80;
pub const KEY_Q                        : c_int = 81;
pub const KEY_R                        : c_int = 82;
pub const KEY_S                        : c_int = 83;
pub const KEY_T                        : c_int = 84;
pub const KEY_U                        : c_int = 85;
pub const KEY_V                        : c_int = 86;
pub const KEY_W                        : c_int = 87;
pub const KEY_X                        : c_int = 88;
pub const KEY_Y                        : c_int = 89;
pub const KEY_Z                        : c_int = 90;
pub const KEY_LEFT_BRACKET             : c_int = 91;
pub const KEY_BACKSLASH                : c_int = 92;
pub const KEY_RIGHT_BRACKET            : c_int = 93;
pub const KEY_GRAVE_ACCENT             : c_int = 96;
pub const KEY_WORLD_1                  : c_int = 161;
pub const KEY_WORLD_2                  : c_int = 162;

pub const KEY_ESCAPE                   : c_int = 256;
pub const KEY_ENTER                    : c_int = 257;
pub const KEY_TAB                      : c_int = 258;
pub const KEY_BACKSPACE                : c_int = 259;
pub const KEY_INSERT                   : c_int = 260;
pub const KEY_DELETE                   : c_int = 261;
pub const KEY_RIGHT                    : c_int = 262;
pub const KEY_LEFT                     : c_int = 263;
pub const KEY_DOWN                     : c_int = 264;
pub const KEY_UP                       : c_int = 265;
pub const KEY_PAGE_UP                  : c_int = 266;
pub const KEY_PAGE_DOWN                : c_int = 267;
pub const KEY_HOME                     : c_int = 268;
pub const KEY_END                      : c_int = 269;
pub const KEY_CAPS_LOCK                : c_int = 280;
pub const KEY_SCROLL_LOCK              : c_int = 281;
pub const KEY_NUM_LOCK                 : c_int = 282;
pub const KEY_PRINT_SCREEN             : c_int = 283;
pub const KEY_PAUSE                    : c_int = 284;
pub const KEY_F1                       : c_int = 290;
pub const KEY_F2                       : c_int = 291;
pub const KEY_F3                       : c_int = 292;
pub const KEY_F4                       : c_int = 293;
pub const KEY_F5                       : c_int = 294;
pub const KEY_F6                       : c_int = 295;
pub const KEY_F7                       : c_int = 296;
pub const KEY_F8                       : c_int = 297;
pub const KEY_F9                       : c_int = 298;
pub const KEY_F10                      : c_int = 299;
pub const KEY_F11                      : c_int = 300;
pub const KEY_F12                      : c_int = 301;
pub const KEY_F13                      : c_int = 302;
pub const KEY_F14                      : c_int = 303;
pub const KEY_F15                      : c_int = 304;
pub const KEY_F16                      : c_int = 305;
pub const KEY_F17                      : c_int = 306;
pub const KEY_F18                      : c_int = 307;
pub const KEY_F19                      : c_int = 308;
pub const KEY_F20                      : c_int = 309;
pub const KEY_F21                      : c_int = 310;
pub const KEY_F22                      : c_int = 311;
pub const KEY_F23                      : c_int = 312;
pub const KEY_F24                      : c_int = 313;
pub const KEY_F25                      : c_int = 314;
pub const KEY_KP_0                     : c_int = 320;
pub const KEY_KP_1                     : c_int = 321;
pub const KEY_KP_2                     : c_int = 322;
pub const KEY_KP_3                     : c_int = 323;
pub const KEY_KP_4                     : c_int = 324;
pub const KEY_KP_5                     : c_int = 325;
pub const KEY_KP_6                     : c_int = 326;
pub const KEY_KP_7                     : c_int = 327;
pub const KEY_KP_8                     : c_int = 328;
pub const KEY_KP_9                     : c_int = 329;
pub const KEY_KP_DECIMAL               : c_int = 330;
pub const KEY_KP_DIVIDE                : c_int = 331;
pub const KEY_KP_MULTIPLY              : c_int = 332;
pub const KEY_KP_SUBTRACT              : c_int = 333;
pub const KEY_KP_ADD                   : c_int = 334;
pub const KEY_KP_ENTER                 : c_int = 335;
pub const KEY_KP_EQUAL                 : c_int = 336;
pub const KEY_LEFT_SHIFT               : c_int = 340;
pub const KEY_LEFT_CONTROL             : c_int = 341;
pub const KEY_LEFT_ALT                 : c_int = 342;
pub const KEY_LEFT_SUPER               : c_int = 343;
pub const KEY_RIGHT_SHIFT              : c_int = 344;
pub const KEY_RIGHT_CONTROL            : c_int = 345;
pub const KEY_RIGHT_ALT                : c_int = 346;
pub const KEY_RIGHT_SUPER              : c_int = 347;
pub const KEY_MENU                     : c_int = 348;
pub const KEY_LAST                     : c_int = KEY_MENU;

pub const MOD_SHIFT                    : c_int = 0x0001;
pub const MOD_CONTROL                  : c_int = 0x0002;
pub const MOD_ALT                      : c_int = 0x0004;
pub const MOD_SUPER                    : c_int = 0x0008;

pub const JOYSTICK_1                   : c_int = 0;
pub const JOYSTICK_2                   : c_int = 1;
pub const JOYSTICK_3                   : c_int = 2;
pub const JOYSTICK_4                   : c_int = 3;
pub const JOYSTICK_5                   : c_int = 4;
pub const JOYSTICK_6                   : c_int = 5;
pub const JOYSTICK_7                   : c_int = 6;
pub const JOYSTICK_8                   : c_int = 7;
pub const JOYSTICK_9                   : c_int = 8;
pub const JOYSTICK_10                  : c_int = 9;
pub const JOYSTICK_11                  : c_int = 10;
pub const JOYSTICK_12                  : c_int = 11;
pub const JOYSTICK_13                  : c_int = 12;
pub const JOYSTICK_14                  : c_int = 13;
pub const JOYSTICK_15                  : c_int = 14;
pub const JOYSTICK_16                  : c_int = 15;
pub const JOYSTICK_LAST                : c_int = JOYSTICK_16;

pub const MOUSE_BUTTON_1               : c_int = 0;
pub const MOUSE_BUTTON_2               : c_int = 1;
pub const MOUSE_BUTTON_3               : c_int = 2;
pub const MOUSE_BUTTON_4               : c_int = 3;
pub const MOUSE_BUTTON_5               : c_int = 4;
pub const MOUSE_BUTTON_6               : c_int = 5;
pub const MOUSE_BUTTON_7               : c_int = 6;
pub const MOUSE_BUTTON_8               : c_int = 7;
pub const MOUSE_BUTTON_LEFT            : c_int = MOUSE_BUTTON_1;
pub const MOUSE_BUTTON_RIGHT           : c_int = MOUSE_BUTTON_2;
pub const MOUSE_BUTTON_MIDDLE          : c_int = MOUSE_BUTTON_3;
pub const MOUSE_BUTTON_LAST            : c_int = MOUSE_BUTTON_8;

pub const NOT_INITIALIZED              : c_int = 0x00010001;
pub const NO_CURRENT_CONTEXT           : c_int = 0x00010002;
pub const INVALID_ENUM                 : c_int = 0x00010003;
pub const INVALID_VALUE                : c_int = 0x00010004;
pub const OUT_OF_MEMORY                : c_int = 0x00010005;
pub const API_UNAVAILABLE              : c_int = 0x00010006;
pub const VERSION_UNAVAILABLE          : c_int = 0x00010007;
pub const PLATFORM_ERROR               : c_int = 0x00010008;
pub const FORMAT_UNAVAILABLE           : c_int = 0x00010009;

pub const FOCUSED                      : c_int = 0x00020001;
pub const ICONIFIED                    : c_int = 0x00020002;
pub const RESIZABLE                    : c_int = 0x00020003;
pub const VISIBLE                      : c_int = 0x00020004;
pub const DECORATED                    : c_int = 0x00020005;
pub const AUTO_ICONIFY                 : c_int = 0x00020006;
pub const FLOATING                     : c_int = 0x00020007;

pub const RED_BITS                     : c_int = 0x00021001;
pub const GREEN_BITS                   : c_int = 0x00021002;
pub const BLUE_BITS                    : c_int = 0x00021003;
pub const ALPHA_BITS                   : c_int = 0x00021004;
pub const DEPTH_BITS                   : c_int = 0x00021005;
pub const STENCIL_BITS                 : c_int = 0x00021006;
pub const ACCUM_RED_BITS               : c_int = 0x00021007;
pub const ACCUM_GREEN_BITS             : c_int = 0x00021008;
pub const ACCUM_BLUE_BITS              : c_int = 0x00021009;
pub const ACCUM_ALPHA_BITS             : c_int = 0x0002100A;
pub const AUX_BUFFERS                  : c_int = 0x0002100B;
pub const STEREO                       : c_int = 0x0002100C;
pub const SAMPLES                      : c_int = 0x0002100D;
pub const SRGB_CAPABLE                 : c_int = 0x0002100E;
pub const REFRESH_RATE                 : c_int = 0x0002100F;
pub const DOUBLEBUFFER                 : c_int = 0x00021010; // TODO: Not yet exposed

pub const CLIENT_API                   : c_int = 0x00022001;
pub const CONTEXT_VERSION_MAJOR        : c_int = 0x00022002;
pub const CONTEXT_VERSION_MINOR        : c_int = 0x00022003;
pub const CONTEXT_REVISION             : c_int = 0x00022004;
pub const CONTEXT_ROBUSTNESS           : c_int = 0x00022005;
pub const OPENGL_FORWARD_COMPAT        : c_int = 0x00022006;
pub const OPENGL_DEBUG_CONTEXT         : c_int = 0x00022007;
pub const OPENGL_PROFILE               : c_int = 0x00022008;
pub const CONTEXT_RELEASE_BEHAVIOR     : c_int = 0x00022009; // TODO: Not yet exposed

pub const OPENGL_API                   : c_int = 0x00030001;
pub const OPENGL_ES_API                : c_int = 0x00030002;

pub const NO_ROBUSTNESS                : c_int = 0x00000000;
pub const NO_RESET_NOTIFICATION        : c_int = 0x00031001;
pub const LOSE_CONTEXT_ON_RESET        : c_int = 0x00031002;

pub const OPENGL_ANY_PROFILE           : c_int = 0x00000000;
pub const OPENGL_CORE_PROFILE          : c_int = 0x00032001;
pub const OPENGL_COMPAT_PROFILE        : c_int = 0x00032002;

pub const CURSOR                       : c_int = 0x00033001;
pub const STICKY_KEYS                  : c_int = 0x00033002;
pub const STICKY_MOUSE_BUTTONS         : c_int = 0x00033003;

pub const CURSOR_NORMAL                : c_int = 0x00034001;
pub const CURSOR_HIDDEN                : c_int = 0x00034002;
pub const CURSOR_DISABLED              : c_int = 0x00034003;

pub const ANY_RELEASE_BEHAVIOR         : c_int = 0; // TODO: Not yet exposed
pub const RELEASE_BEHAVIOR_FLUSH       : c_int = 0x00035001; // TODO: Not yet exposed
pub const RELEASE_BEHAVIOR_NONE        : c_int = 0x00035002; // TODO: Not yet exposed

pub const ARROW_CURSOR                 : c_int = 0x00036001; // TODO: Not yet exposed
pub const IBEAM_CURSOR                 : c_int = 0x00036002; // TODO: Not yet exposed
pub const CROSSHAIR_CURSOR             : c_int = 0x00036003; // TODO: Not yet exposed
pub const HAND_CURSOR                  : c_int = 0x00036004; // TODO: Not yet exposed
pub const HRESIZE_CURSOR               : c_int = 0x00036005; // TODO: Not yet exposed
pub const VRESIZE_CURSOR               : c_int = 0x00036006; // TODO: Not yet exposed

pub const CONNECTED                    : c_int = 0x00040001;
pub const DISCONNECTED                 : c_int = 0x00040002;

pub const DONT_CARE                    : c_int = -1; // TODO: Not yet exposed

pub type GLFWglproc             = *const c_void;

pub type GLFWerrorfun           = extern "C" fn(c_int, *const c_char);
pub type GLFWwindowposfun       = extern "C" fn(*mut GLFWwindow, c_int, c_int);
pub type GLFWwindowsizefun      = extern "C" fn(*mut GLFWwindow, c_int, c_int);
pub type GLFWwindowclosefun     = extern "C" fn(*mut GLFWwindow);
pub type GLFWwindowrefreshfun   = extern "C" fn(*mut GLFWwindow);
pub type GLFWwindowfocusfun     = extern "C" fn(*mut GLFWwindow, c_int);
pub type GLFWwindowiconifyfun   = extern "C" fn(*mut GLFWwindow, c_int);
pub type GLFWframebuffersizefun = extern "C" fn(*mut GLFWwindow, c_int, c_int);
pub type GLFWmousebuttonfun     = extern "C" fn(*mut GLFWwindow, c_int, c_int, c_int);
pub type GLFWcursorposfun       = extern "C" fn(*mut GLFWwindow, c_double, c_double);
pub type GLFWcursorenterfun     = extern "C" fn(*mut GLFWwindow, c_int);
pub type GLFWscrollfun          = extern "C" fn(*mut GLFWwindow, c_double, c_double);
pub type GLFWkeyfun             = extern "C" fn(*mut GLFWwindow, c_int, c_int, c_int, c_int);
pub type GLFWcharfun            = extern "C" fn(*mut GLFWwindow, c_uint);
pub type GLFWcharmodsfun        = extern "C" fn(*mut GLFWwindow, c_uint, c_int); // TODO: Not yet exposed
pub type GLFWdropfun            = extern "C" fn(*mut GLFWwindow, c_int, *mut *const c_char); // TODO: Not yet exposed
pub type GLFWmonitorfun         = extern "C" fn(*mut GLFWmonitor, c_int);

#[allow(missing_copy_implementations)]
pub enum GLFWmonitor {}

#[allow(missing_copy_implementations)]
pub enum GLFWwindow {}

#[allow(missing_copy_implementations)]
pub enum GLFWcursor {}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLFWgammaramp {
    pub red:    *mut c_ushort,
    pub green:  *mut c_ushort,
    pub blue:   *mut c_ushort,
    pub size:   c_uint,
}

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct GLFWvidmode {
    pub width:       c_int,
    pub height:      c_int,
    pub redBits:     c_int,
    pub greenBits:   c_int,
    pub blueBits:    c_int,
    pub refreshRate: c_int,
}

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct GLFWimage {
    pub width: c_int,
    pub height: c_int,
    pub pixels: *mut c_uchar,
}

// C function bindings

extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();
    pub fn glfwGetVersion(major: *mut c_int, minor: *mut c_int, rev: *mut c_int);
    pub fn glfwGetVersionString() -> *const c_char;

    pub fn glfwSetErrorCallback(cbfun: Option<GLFWerrorfun>) -> Option<GLFWerrorfun>;

    pub fn glfwGetMonitors(count: *mut c_int) -> *mut *mut GLFWmonitor;
    pub fn glfwGetPrimaryMonitor() -> *mut GLFWmonitor;
    pub fn glfwGetMonitorPos(monitor: *mut GLFWmonitor, xpos: *mut c_int, ypos: *mut c_int);
    pub fn glfwGetMonitorPhysicalSize(monitor: *mut GLFWmonitor, width: *mut c_int, height: *mut c_int);
    pub fn glfwGetMonitorName(monitor: *mut GLFWmonitor) -> *const c_char;
    pub fn glfwSetMonitorCallback(cbfun: Option<GLFWmonitorfun>) -> Option<GLFWmonitorfun>;
    pub fn glfwGetVideoModes(monitor: *mut GLFWmonitor, count: *mut c_int) -> *const GLFWvidmode;
    pub fn glfwGetVideoMode(monitor: *mut GLFWmonitor) -> *const GLFWvidmode;
    pub fn glfwSetGamma(monitor: *mut GLFWmonitor, gamma: c_float);
    pub fn glfwGetGammaRamp(monitor: *mut GLFWmonitor) -> *const GLFWgammaramp;
    pub fn glfwSetGammaRamp(monitor: *mut GLFWmonitor, ramp: *const GLFWgammaramp);

    pub fn glfwDefaultWindowHints();
    pub fn glfwWindowHint(target: c_int, hint: c_int);
    pub fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;
    pub fn glfwDestroyWindow(window: *mut GLFWwindow);
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
    pub fn glfwSetWindowShouldClose(window: *mut GLFWwindow, value: c_int);
    pub fn glfwSetWindowTitle(window: *mut GLFWwindow, title: *const c_char);
    pub fn glfwGetWindowPos(window: *mut GLFWwindow, xpos: *mut c_int, ypos: *mut c_int);
    pub fn glfwSetWindowPos(window: *mut GLFWwindow, xpos: c_int, ypos: c_int);
    pub fn glfwGetWindowSize(window: *mut GLFWwindow, width: *mut c_int, height: *mut c_int);
    pub fn glfwSetWindowSize(window: *mut GLFWwindow, width: c_int, height: c_int);
    pub fn glfwGetFramebufferSize(window: *mut GLFWwindow, width: *mut c_int, height: *mut c_int);
    pub fn glfwIconifyWindow(window: *mut GLFWwindow);
    pub fn glfwRestoreWindow(window: *mut GLFWwindow);
    pub fn glfwShowWindow(window: *mut GLFWwindow);
    pub fn glfwHideWindow(window: *mut GLFWwindow);
    pub fn glfwGetWindowMonitor(window: *mut GLFWwindow) -> *mut GLFWmonitor;
    pub fn glfwGetWindowAttrib(window: *mut GLFWwindow, attrib: c_int) -> c_int;
    pub fn glfwSetWindowUserPointer(window: *mut GLFWwindow, pointer: *mut c_void);
    pub fn glfwGetWindowUserPointer(window: *mut GLFWwindow) -> *mut c_void;
    pub fn glfwSetWindowPosCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowposfun>) -> Option<GLFWwindowposfun>;
    pub fn glfwSetWindowSizeCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowsizefun>) -> Option<GLFWwindowsizefun>;
    pub fn glfwSetWindowCloseCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowclosefun>) -> Option<GLFWwindowclosefun>;
    pub fn glfwSetWindowRefreshCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowrefreshfun>) -> Option<GLFWwindowrefreshfun>;
    pub fn glfwSetWindowFocusCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowfocusfun>) -> Option<GLFWwindowfocusfun>;
    pub fn glfwSetWindowIconifyCallback(window: *mut GLFWwindow, cbfun: Option<GLFWwindowiconifyfun>) -> Option<GLFWwindowiconifyfun>;
    pub fn glfwSetFramebufferSizeCallback(window: *mut GLFWwindow, cbfun: Option<GLFWframebuffersizefun>) -> Option<GLFWframebuffersizefun>;

    pub fn glfwPollEvents();
    pub fn glfwWaitEvents();
    pub fn glfwPostEmptyEvent(); // TODO: Not yet exposed

    pub fn glfwGetInputMode(window: *mut GLFWwindow, mode: c_int) -> c_int;
    pub fn glfwSetInputMode(window: *mut GLFWwindow, mode: c_int, value: c_int);
    pub fn glfwGetKey(window: *mut GLFWwindow, key: c_int) -> c_int;
    pub fn glfwGetMouseButton(window: *mut GLFWwindow, button: c_int) -> c_int;
    pub fn glfwGetCursorPos(window: *mut GLFWwindow, xpos: *mut c_double, ypos: *mut c_double);
    pub fn glfwSetCursorPos(window: *mut GLFWwindow, xpos: c_double, ypos: c_double);
    pub fn glfwCreateCursor(image: *const GLFWimage, xhot: c_int, yhot: c_int) -> *mut GLFWcursor; // TODO: Not yet exposed
    pub fn glfwCreateStandardCursor(shape: c_int) -> *mut GLFWcursor; // TODO: Not yet exposed
    pub fn glfwDestroyCursor(cursor: *mut GLFWcursor); // TODO: Not yet exposed
    pub fn glfwSetCursor(window: *mut GLFWwindow, cursor: *mut GLFWcursor); // TODO: Not yet exposed
    pub fn glfwSetKeyCallback(window: *mut GLFWwindow, cbfun: Option<GLFWkeyfun>) -> Option<GLFWkeyfun>;
    pub fn glfwSetCharCallback(window: *mut GLFWwindow, cbfun: Option<GLFWcharfun>) -> Option<GLFWcharfun>;
    pub fn glfwSetCharModsCallback(window: *mut GLFWwindow, cbfun: Option<GLFWcharmodsfun>) -> Option<GLFWcharmodsfun>; // TODO: Not yet exposed
    pub fn glfwSetMouseButtonCallback(window: *mut GLFWwindow, cbfun: Option<GLFWmousebuttonfun>) -> Option<GLFWmousebuttonfun>;
    pub fn glfwSetCursorPosCallback(window: *mut GLFWwindow, cbfun: Option<GLFWcursorposfun>) -> Option<GLFWcursorposfun>;
    pub fn glfwSetCursorEnterCallback(window: *mut GLFWwindow, cbfun: Option<GLFWcursorenterfun>) -> Option<GLFWcursorenterfun>;
    pub fn glfwSetScrollCallback(window: *mut GLFWwindow, cbfun: Option<GLFWscrollfun>) -> Option<GLFWscrollfun>;
    pub fn glfwSetDropCallback(window: *mut GLFWwindow, cbfun: Option<GLFWdropfun>) -> Option<GLFWdropfun>; // TODO: Not yet exposed

    pub fn glfwJoystickPresent(joy: c_int) -> c_int;
    pub fn glfwGetJoystickAxes(joy: c_int, count: *mut c_int) -> *const c_float;
    pub fn glfwGetJoystickButtons(joy: c_int, count: *mut c_int) -> *const c_uchar;
    pub fn glfwGetJoystickName(joy: c_int) -> *const c_char;

    pub fn glfwSetClipboardString(window: *mut GLFWwindow, string: *const c_char);
    pub fn glfwGetClipboardString(window: *mut GLFWwindow) -> *const c_char;

    pub fn glfwGetTime() -> c_double;
    pub fn glfwSetTime(time: c_double);

    pub fn glfwMakeContextCurrent(window: *mut GLFWwindow);
    pub fn glfwGetCurrentContext() -> *mut GLFWwindow;
    pub fn glfwSwapBuffers(window: *mut GLFWwindow);
    pub fn glfwSwapInterval(interval: c_int);
    pub fn glfwExtensionSupported(extension: *const c_char) -> c_int;
    pub fn glfwGetProcAddress(procname: *const c_char) -> GLFWglproc;

    // native APIs

    #[cfg(target_os="windows")] pub fn glfwGetWin32Window(window: *mut GLFWwindow) -> *mut c_void;
    #[cfg(target_os="windows")] pub fn glfwGetWGLContext(window: *mut GLFWwindow) -> *mut c_void;

    #[cfg(target_os="macos")] pub fn glfwGetCocoaWindow(window: *mut GLFWwindow) -> *mut c_void;
    #[cfg(target_os="macos")] pub fn glfwGetNSGLContext(window: *mut GLFWwindow) -> *mut c_void;

    #[cfg(target_os="linux")] pub fn glfwGetX11Window(window: *mut GLFWwindow) -> *mut c_void;
    #[cfg(target_os="linux")] pub fn glfwGetX11Display() -> *mut c_void;
    #[cfg(target_os="linux")] pub fn glfwGetGLXContext(window: *mut GLFWwindow) -> *mut c_void;
}
