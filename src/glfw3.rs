/*************************************************************************
 * GLFW3 Bindings for Rust by Brendan Zabarauskas
 *
 * For instructions and more information check out the github repository at 
 * https://github.com/bjz/glfw3-rs/
 *
 *************************************************************************/
 
 /*************************************************************************
 * GLFW - An OpenGL library
 * API version: 3.0
 * WWW:         http://www.glfw.org/
 *------------------------------------------------------------------------
 * Copyright (c) 2002-2006 Marcus Geelnard
 * Copyright (c) 2006-2010 Camilla Berglund <elmindreda@elmindreda.org>
 *
 * This software is provided 'as-is', without any express or implied
 * warranty. In no event will the authors be held liable for any damages
 * arising from the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software
 *    in a product, an acknowledgment in the product documentation would
 *    be appreciated but is not required.
 *
 * 2. Altered source versions must be plainly marked as such, and must not
 *    be misrepresented as being the original software.
 *
 * 3. This notice may not be removed or altered from any source
 *    distribution.
 *
 *************************************************************************/
 
extern mod std;
use libc::*;
use ptr::to_unsafe_ptr;

use task::local_data:: {
    local_data_get,
    local_data_set
};

// Include OS X Frameworks
#[nolink]
#[cfg(target_os = "macos")]
#[link_args="-framework Cocoa -framework OpenGL -framework IOKit -framework CoreFoundation"]
extern mod osx_frameworks {}

// GLFW Linking

#[link_name = "glfw"]
#[cfg(target_os = "macos")]
extern mod linkhack {}

#[link_name = "glfw"]
#[cfg(target_os = "linux")]
extern mod linkhack {}

// TODO: Fix GLFW linking on windows
// #[link_name = "???"]
// #[cfg(target_os = "windows")]
// extern mod linkhack{}

/*************************************************************************
 * GLFW version
 *************************************************************************/
 
pub const VERSION_MAJOR        : c_int = 3;
pub const VERSION_MINOR        : c_int = 0;
pub const VERSION_REVISION     : c_int = 0;

/*************************************************************************
 * Input handling definitions
 *************************************************************************/

/* Key and button state/action definitions */
pub const RELEASE              : c_int = 0;
pub const PRESS                : c_int = 1;

/* Keyboard raw key codes.
 * These key codes are inspired by the USB HID Usage Tables v1.12 (p. 53-60),
 * but re-arranged to map to 7-bit ASCII for printable keys (function keys are
 * put in the 256+ range).
 * The naming of the key codes follow these rules:
 *  - The US keyboard layout is used.
 *  - Names of printable alpha-numeric characters are used (e.g. "A", "R",
 *    "3", etc).
 *  - For non-alphanumeric characters, Unicode:ish names are used (e.g.
 *    "COMMA", "LEFT_SQUARE_BRACKET", etc). Note that some names do not
 *    correspond to the Unicode standard (usually for brevity).
 *  - Keys that lack a clear US mapping are named "WORLD_x".
 *  - For non-printable keys, custom names are used (e.g. "F4",
 *    "BACKSPACE", etc).
 */
 
/* Printable keys */
pub const KEY_SPACE            : c_int = 32;
pub const KEY_APOSTROPHE       : c_int = 39;  /* ' */
pub const KEY_COMMA            : c_int = 44;  /* , */
pub const KEY_MINUS            : c_int = 45;  /* - */
pub const KEY_PERIOD           : c_int = 46;  /* . */
pub const KEY_SLASH            : c_int = 47;  /* / */
pub const KEY_0                : c_int = 48;
pub const KEY_1                : c_int = 49;
pub const KEY_2                : c_int = 50;
pub const KEY_3                : c_int = 51;
pub const KEY_4                : c_int = 52;
pub const KEY_5                : c_int = 53;
pub const KEY_6                : c_int = 54;
pub const KEY_7                : c_int = 55;
pub const KEY_8                : c_int = 56;
pub const KEY_9                : c_int = 57;
pub const KEY_SEMICOLON        : c_int = 59;  /* ; */
pub const KEY_EQUAL            : c_int = 61;  /* = */
pub const KEY_A                : c_int = 65;
pub const KEY_B                : c_int = 66;
pub const KEY_C                : c_int = 67;
pub const KEY_D                : c_int = 68;
pub const KEY_E                : c_int = 69;
pub const KEY_F                : c_int = 70;
pub const KEY_G                : c_int = 71;
pub const KEY_H                : c_int = 72;
pub const KEY_I                : c_int = 73;
pub const KEY_J                : c_int = 74;
pub const KEY_K                : c_int = 75;
pub const KEY_L                : c_int = 76;
pub const KEY_M                : c_int = 77;
pub const KEY_N                : c_int = 78;
pub const KEY_O                : c_int = 79;
pub const KEY_P                : c_int = 80;
pub const KEY_Q                : c_int = 81;
pub const KEY_R                : c_int = 82;
pub const KEY_S                : c_int = 83;
pub const KEY_T                : c_int = 84;
pub const KEY_U                : c_int = 85;
pub const KEY_V                : c_int = 86;
pub const KEY_W                : c_int = 87;
pub const KEY_X                : c_int = 88;
pub const KEY_Y                : c_int = 89;
pub const KEY_Z                : c_int = 90;
pub const KEY_LEFT_BRACKET     : c_int = 91;  /* [ */
pub const KEY_BACKSLASH        : c_int = 92;  /* \ */
pub const KEY_RIGHT_BRACKET    : c_int = 93;  /* ] */
pub const KEY_GRAVE_ACCENT     : c_int = 96;  /* ` */
pub const KEY_WORLD_1          : c_int = 161; /* non-US #1 */
pub const KEY_WORLD_2          : c_int = 162; /* non-US #2 */

/* Function keys */
pub const KEY_ESCAPE           : c_int = 256;
pub const KEY_ENTER            : c_int = 257;
pub const KEY_TAB              : c_int = 258;
pub const KEY_BACKSPACE        : c_int = 259;
pub const KEY_INSERT           : c_int = 260;
pub const KEY_DELETE           : c_int = 261;
pub const KEY_RIGHT            : c_int = 262;
pub const KEY_LEFT             : c_int = 263;
pub const KEY_DOWN             : c_int = 264;
pub const KEY_UP               : c_int = 265;
pub const KEY_PAGE_UP          : c_int = 266;
pub const KEY_PAGE_DOWN        : c_int = 267;
pub const KEY_HOME             : c_int = 268;
pub const KEY_END              : c_int = 269;
pub const KEY_CAPS_LOCK        : c_int = 280;
pub const KEY_SCROLL_LOCK      : c_int = 281;
pub const KEY_NUM_LOCK         : c_int = 282;
pub const KEY_PRINT_SCREEN     : c_int = 283;
pub const KEY_PAUSE            : c_int = 284;
pub const KEY_F1               : c_int = 290;
pub const KEY_F2               : c_int = 291;
pub const KEY_F3               : c_int = 292;
pub const KEY_F4               : c_int = 293;
pub const KEY_F5               : c_int = 294;
pub const KEY_F6               : c_int = 295;
pub const KEY_F7               : c_int = 296;
pub const KEY_F8               : c_int = 297;
pub const KEY_F9               : c_int = 298;
pub const KEY_F10              : c_int = 299;
pub const KEY_F11              : c_int = 300;
pub const KEY_F12              : c_int = 301;
pub const KEY_F13              : c_int = 302;
pub const KEY_F14              : c_int = 303;
pub const KEY_F15              : c_int = 304;
pub const KEY_F16              : c_int = 305;
pub const KEY_F17              : c_int = 306;
pub const KEY_F18              : c_int = 307;
pub const KEY_F19              : c_int = 308;
pub const KEY_F20              : c_int = 309;
pub const KEY_F21              : c_int = 310;
pub const KEY_F22              : c_int = 311;
pub const KEY_F23              : c_int = 312;
pub const KEY_F24              : c_int = 313;
pub const KEY_F25              : c_int = 314;
pub const KEY_KP_0             : c_int = 320;
pub const KEY_KP_1             : c_int = 321;
pub const KEY_KP_2             : c_int = 322;
pub const KEY_KP_3             : c_int = 323;
pub const KEY_KP_4             : c_int = 324;
pub const KEY_KP_5             : c_int = 325;
pub const KEY_KP_6             : c_int = 326;
pub const KEY_KP_7             : c_int = 327;
pub const KEY_KP_8             : c_int = 328;
pub const KEY_KP_9             : c_int = 329;
pub const KEY_KP_DECIMAL       : c_int = 330;
pub const KEY_KP_DIVIDE        : c_int = 331;
pub const KEY_KP_MULTIPLY      : c_int = 332;
pub const KEY_KP_SUBTRACT      : c_int = 333;
pub const KEY_KP_ADD           : c_int = 334;
pub const KEY_KP_ENTER         : c_int = 335;
pub const KEY_KP_EQUAL         : c_int = 336;
pub const KEY_LEFT_SHIFT       : c_int = 340;
pub const KEY_LEFT_CONTROL     : c_int = 341;
pub const KEY_LEFT_ALT         : c_int = 342;
pub const KEY_LEFT_SUPER       : c_int = 343;
pub const KEY_RIGHT_SHIFT      : c_int = 344;
pub const KEY_RIGHT_CONTROL    : c_int = 345;
pub const KEY_RIGHT_ALT        : c_int = 346;
pub const KEY_RIGHT_SUPER      : c_int = 347;
pub const KEY_MENU             : c_int = 348;
pub const KEY_LAST             : c_int = KEY_MENU;

/* GLFW 2.x key name aliases (deprecated) */
pub const KEY_ESC              : c_int = KEY_ESCAPE;
pub const KEY_DEL              : c_int = KEY_DELETE;
pub const KEY_PAGEUP           : c_int = KEY_PAGE_UP;
pub const KEY_PAGEDOWN         : c_int = KEY_PAGE_DOWN;
pub const KEY_KP_NUM_LOCK      : c_int = KEY_NUM_LOCK;
pub const KEY_LCTRL            : c_int = KEY_LEFT_CONTROL;
pub const KEY_LSHIFT           : c_int = KEY_LEFT_SHIFT;
pub const KEY_LALT             : c_int = KEY_LEFT_ALT;
pub const KEY_LSUPER           : c_int = KEY_LEFT_SUPER;
pub const KEY_RCTRL            : c_int = KEY_RIGHT_CONTROL;
pub const KEY_RSHIFT           : c_int = KEY_RIGHT_SHIFT;
pub const KEY_RALT             : c_int = KEY_RIGHT_ALT;
pub const KEY_RSUPER           : c_int = KEY_RIGHT_SUPER;

/* Mouse button aliases */
pub const MOUSE_BUTTON_LEFT    : c_int = MOUSE_BUTTON_1;
pub const MOUSE_BUTTON_RIGHT   : c_int = MOUSE_BUTTON_2;
pub const MOUSE_BUTTON_MIDDLE  : c_int = MOUSE_BUTTON_3;

/* Mouse button definitions */
pub const MOUSE_BUTTON_1       : c_int = 0;
pub const MOUSE_BUTTON_2       : c_int = 1;
pub const MOUSE_BUTTON_3       : c_int = 2;
pub const MOUSE_BUTTON_4       : c_int = 3;
pub const MOUSE_BUTTON_5       : c_int = 4;
pub const MOUSE_BUTTON_6       : c_int = 5;
pub const MOUSE_BUTTON_7       : c_int = 6;
pub const MOUSE_BUTTON_8       : c_int = 7;
pub const MOUSE_BUTTON_LAST    : c_int = MOUSE_BUTTON_8;

/* Joystick identifiers */
pub const JOYSTICK_1           : c_int = 0;
pub const JOYSTICK_2           : c_int = 1;
pub const JOYSTICK_3           : c_int = 2;
pub const JOYSTICK_4           : c_int = 3;
pub const JOYSTICK_5           : c_int = 4;
pub const JOYSTICK_6           : c_int = 5;
pub const JOYSTICK_7           : c_int = 6;
pub const JOYSTICK_8           : c_int = 7;
pub const JOYSTICK_9           : c_int = 8;
pub const JOYSTICK_10          : c_int = 9;
pub const JOYSTICK_11          : c_int = 10;
pub const JOYSTICK_12          : c_int = 11;
pub const JOYSTICK_13          : c_int = 12;
pub const JOYSTICK_14          : c_int = 13;
pub const JOYSTICK_15          : c_int = 14;
pub const JOYSTICK_16          : c_int = 15;
pub const JOYSTICK_LAST        : c_int = JOYSTICK_16;

/*************************************************************************
 * Other definitions
 *************************************************************************/

/* glfwCreateWindow modes */
pub const WINDOWED                     : c_int = 0x00010001;
pub const FULLSCREEN                   : c_int = 0x00010002;

/* glfwGetWindowParam tokens */
pub const ACTIVE                       : c_int = 0x00020001;
pub const ICONIFIED                    : c_int = 0x00020002;
pub const CLOSE_REQUESTED              : c_int = 0x00020003;
pub const OPENGL_REVISION              : c_int = 0x00020004;

/* glfwWindowHint tokens */
pub const RED_BITS                     : c_int = 0x00021000;
pub const GREEN_BITS                   : c_int = 0x00021001;
pub const BLUE_BITS                    : c_int = 0x00021002;
pub const ALPHA_BITS                   : c_int = 0x00021003;
pub const DEPTH_BITS                   : c_int = 0x00021004;
pub const STENCIL_BITS                 : c_int = 0x00021005;
pub const REFRESH_RATE                 : c_int = 0x00021006;
pub const ACCUM_RED_BITS               : c_int = 0x00021007;
pub const ACCUM_GREEN_BITS             : c_int = 0x00021008;
pub const ACCUM_BLUE_BITS              : c_int = 0x00021009;
pub const ACCUM_ALPHA_BITS             : c_int = 0x0002100A;
pub const AUX_BUFFERS                  : c_int = 0x0002100B;
pub const STEREO                       : c_int = 0x0002100C;
pub const WINDOW_RESIZABLE             : c_int = 0x0002100D;
pub const FSAA_SAMPLES                 : c_int = 0x0002100E;

/* The following constants are used with both glfwGetWindowParam
 * and glfwWindowHint
 */
pub const CLIENT_API                   : c_int = 0x00022000;
pub const OPENGL_VERSION_MAJOR         : c_int = 0x00022001;
pub const OPENGL_VERSION_MINOR         : c_int = 0x00022002;
pub const OPENGL_FORWARD_COMPAT        : c_int = 0x00022003;
pub const OPENGL_DEBUG_CONTEXT         : c_int = 0x00022004;
pub const OPENGL_PROFILE               : c_int = 0x00022005;
pub const OPENGL_ROBUSTNESS            : c_int = 0x00022006;
pub const RESIZABLE                    : c_int = 0x00022007;
pub const VISIBLE                      : c_int = 0x00022008;
 
/* GLFW_CLIENT_API tokens */
pub const OPENGL_API                   : c_int = 0x00000001;
pub const OPENGL_ES_API                : c_int = 0x00000002;

/* GLFW_OPENGL_ROBUSTNESS mode tokens */
pub const OPENGL_NO_ROBUSTNESS         : c_int = 0x00000000;
pub const OPENGL_NO_RESET_NOTIFICATION : c_int = 0x00000001;
pub const OPENGL_LOSE_CONTEXT_ON_RESET : c_int = 0x00000002;

/* GLFW_OPENGL_PROFILE bit tokens */
pub const OPENGL_NO_PROFILE            : c_int = 0x00000000;
pub const OPENGL_CORE_PROFILE          : c_int = 0x00000001;
pub const OPENGL_COMPAT_PROFILE        : c_int = 0x00000002;

/* glfwGetInputMode/glfwSetInputMode tokens */
pub const CURSOR_MODE                  : c_int = 0x00030001;
pub const STICKY_KEYS                  : c_int = 0x00030002;
pub const STICKY_MOUSE_BUTTONS         : c_int = 0x00030003;
pub const SYSTEM_KEYS                  : c_int = 0x00030004;
pub const KEY_REPEAT                   : c_int = 0x00030005;

/* GLFW_CURSOR_MODE values */
pub const CURSOR_NORMAL                : c_int = 0x00040001;
pub const CURSOR_HIDDEN                : c_int = 0x00040002;
pub const CURSOR_CAPTURED              : c_int = 0x00040003;

/* glfwGetJoystickParam tokens */
pub const PRESENT                      : c_int = 0x00050001;
pub const AXES                         : c_int = 0x00050002;
pub const BUTTONS                      : c_int = 0x00050003;

/* glfwGetError/glfwErrorString tokens */
pub const NO_ERROR                     : c_int = 0;
pub const NOT_INITIALIZED              : c_int = 0x00070001;
pub const NO_CURRENT_CONTEXT           : c_int = 0x00070002;
pub const INVALID_ENUM                 : c_int = 0x00070003;
pub const INVALID_VALUE                : c_int = 0x00070004;
pub const OUT_OF_MEMORY                : c_int = 0x00070005;
pub const OPENGL_UNAVAILABLE           : c_int = 0x00070006;
pub const VERSION_UNAVAILABLE          : c_int = 0x00070007;
pub const PLATFORM_ERROR               : c_int = 0x00070008;
pub const WINDOW_NOT_ACTIVE            : c_int = 0x00070009;
pub const FORMAT_UNAVAILABLE           : c_int = 0x0007000A;

/* Gamma ramps */
pub const GAMMA_RAMP_SIZE              : c_int = 256;

/*************************************************************************
 * Typedefs
 *************************************************************************/

// /* OpenGL function pointer type */
// Will have to be changed once we can do external C callbacks nicely
pub type GLProc = *u8;              // typedef void (*GLFWglproc)(void);

// Wraps * pointer in a struct for safety 
pub struct Window {
    mut ptr: api::GLFWwindow
}

/* Function pointer types */
pub type ErrorFun           = @fn(_error: c_int, _format: ~str);
pub type WindowSizeFun      = @fn(_window: Window, _width: int, _height: int);
pub type WindowCloseFun     = @fn(_window: Window) -> bool;
pub type WindowRefreshFun   = @fn(_window: Window);
pub type WindowFocusFun     = @fn(_window: Window, _activated: bool);
pub type WindowIconifyFun   = @fn(_window: Window, _iconified: bool);
pub type MouseButtonFun     = @fn(_window: Window, _button: c_int, _action: c_int);
pub type CursorPosFun       = @fn(_window: Window, _x: int, _y: int);
pub type CursorEnterFun     = @fn(_window: Window, _entered: bool);
pub type ScrollFun          = @fn(_window: Window, _x: f64, _y: f64);
pub type KeyFun             = @fn(_window: Window, _key: c_int, _action: c_int);
pub type CharFun            = @fn(_window: Window, _character: char);

/* The video mode structure used by glfwGetVideoModes */
pub struct VidMode {
    width      : c_int,
    height     : c_int,
    redBits    : c_int,
    blueBits   : c_int,
    greenBits  : c_int
}

/* Gamma ramp */
/* See https://github.com/mozilla/rust/issues/3469
struct GammaRamp {
    red     : [c_ushort * GAMMA_RAMP_SIZE],      // unsigned short red[GLFW_GAMMA_RAMP_SIZE];
    green   : [c_ushort * GAMMA_RAMP_SIZE],      // unsigned short green[GLFW_GAMMA_RAMP_SIZE];
    blue    : [c_ushort * GAMMA_RAMP_SIZE]       // unsigned short blue[GLFW_GAMMA_RAMP_SIZE];
}
*/
pub struct GammaRamp {
    red     : [c_ushort * 256],      // unsigned short red[GLFW_GAMMA_RAMP_SIZE];
    green   : [c_ushort * 256],      // unsigned short green[GLFW_GAMMA_RAMP_SIZE];
    blue    : [c_ushort * 256]       // unsigned short blue[GLFW_GAMMA_RAMP_SIZE];
}

#[nolink]
pub mod api {
    /* Function pointer types */
    // Will have to be changed once we can do external C callbacks nicely
    pub type GLFWerrorfun           = *u8;  // typedef void (* GLFWerrorfun)(int,const char*);
    pub type GLFWwindowsizefun      = *u8;  // typedef void (* GLFWwindowsizefun)(GLFWwindow,int,int);
    pub type GLFWwindowclosefun     = *u8;  // typedef int  (* GLFWwindowclosefun)(GLFWwindow);
    pub type GLFWwindowrefreshfun   = *u8;  // typedef void (* GLFWwindowrefreshfun)(GLFWwindow);
    pub type GLFWwindowfocusfun     = *u8;  // typedef void (* GLFWwindowfocusfun)(GLFWwindow,int);
    pub type GLFWwindowiconifyfun   = *u8;  // typedef void (* GLFWwindowiconifyfun)(GLFWwindow,int);
    pub type GLFWmousebuttonfun     = *u8;  // typedef void (* GLFWmousebuttonfun)(GLFWwindow,int,int);
    pub type GLFWcursorposfun       = *u8;  // typedef void (* GLFWcursorposfun)(GLFWwindow,int,int);
    pub type GLFWcursorenterfun     = *u8;  // typedef void (* GLFWcursorenterfun)(GLFWwindow,int);
    pub type GLFWscrollfun          = *u8;  // typedef void (* GLFWscrollfun)(GLFWwindow,double,double);
    pub type GLFWkeyfun             = *u8;  // typedef void (* GLFWkeyfun)(GLFWwindow,int,int);
    pub type GLFWcharfun            = *u8;  // typedef void (* GLFWcharfun)(GLFWwindow,int);
    
    /* Window handle type */
    pub type GLFWwindow = *c_void;      // typedef void* GLFWwindow;
    
    extern {
        /* GLFW initialization, termination and version querying */
        fn glfwInit() -> c_int;
        fn glfwTerminate();
        fn glfwGetVersion(++major: *c_int, ++minor: *c_int, ++rev: *c_int);
        fn glfwGetVersionString() -> *c_char;

        /* Error handling */
        fn glfwGetError() -> c_int;
        fn glfwErrorString(++error: c_int) -> *c_char;
        fn glfwSetErrorCallback(++cbfun: GLFWerrorfun);
        
        /* Video mode functions */
        fn glfwGetVideoModes(++count: *c_int) -> *VidMode;
        fn glfwGetDesktopMode(++mode: *VidMode);
        
        /* Gamma ramp functions */
        fn glfwSetGamma(++gamma: c_float);
        fn glfwGetGammaRamp(++ramp: *GammaRamp);
        fn glfwSetGammaRamp(++ramp: *GammaRamp);
        
        /* Window handling */
        fn glfwDefaultWindowHints();
        fn glfwWindowHint(++target: c_int, ++hint: c_int);
        fn glfwCreateWindow(++width: c_int, ++height: c_int, ++mode: c_int, ++title: *c_char, ++share: GLFWwindow) -> GLFWwindow;
        fn glfwDestroyWindow(++window: GLFWwindow);
        fn glfwSetWindowTitle(++window: GLFWwindow, ++title: *c_char);
        fn glfwGetWindowSize(++window: GLFWwindow, ++width: *c_int, ++height: *c_int);
        fn glfwSetWindowSize(++window: GLFWwindow, ++width: c_int, ++height: c_int);
        fn glfwGetWindowPos(++window: GLFWwindow, ++xpos: *c_int, ++ypos: *c_int);
        fn glfwSetWindowPos(++window: GLFWwindow, ++xpos: c_int, ++ypos: c_int);
        fn glfwIconifyWindow(++window: GLFWwindow);
        fn glfwRestoreWindow(++window: GLFWwindow);
        fn glfwShowWindow(++window: GLFWwindow);
        fn glfwHideWindow(++window: GLFWwindow);
        fn glfwGetWindowParam(++window: GLFWwindow, ++param: c_int) -> c_int;
        fn glfwSetWindowUserPointer(++window: GLFWwindow, ++pointer: *c_void);
        fn glfwGetWindowUserPointer(++window: GLFWwindow) -> *c_void;
        fn glfwSetWindowSizeCallback(++window: GLFWwindow, ++cbfun: GLFWwindowsizefun);
        fn glfwSetWindowCloseCallback(++window: GLFWwindow, ++cbfun: GLFWwindowclosefun);
        fn glfwSetWindowRefreshCallback(++window: GLFWwindow, ++cbfun: GLFWwindowrefreshfun);
        fn glfwSetWindowFocusCallback(++window: GLFWwindow, ++cbfun: GLFWwindowfocusfun);
        fn glfwSetWindowIconifyCallback(++window: GLFWwindow, ++cbfun: GLFWwindowiconifyfun);
        
        /* Event handling */
        fn glfwPollEvents();
        fn glfwWaitEvents();

        /* Input handling */
        fn glfwGetInputMode(++window: GLFWwindow, ++mode: c_int) -> c_int;
        fn glfwSetInputMode(++window: GLFWwindow, ++mode: c_int, ++value: c_int);
        fn glfwGetKey(++window: GLFWwindow, ++key: c_int) -> c_int;
        fn glfwGetMouseButton(++window: GLFWwindow, ++button: c_int) -> c_int;
        fn glfwGetCursorPos(++window: GLFWwindow, ++xpos: *c_int, ++ypos: *c_int);
        fn glfwSetCursorPos(++window: GLFWwindow, ++xpos: c_int, ++ypos: c_int);
        fn glfwGetScrollOffset(++window: GLFWwindow, ++xoffset: *c_double, ++yoffset: *c_double);
        fn glfwSetKeyCallback(++window: GLFWwindow, ++cbfun: GLFWkeyfun);
        fn glfwSetCharCallback(++window: GLFWwindow, ++cbfun: GLFWcharfun);
        fn glfwSetMouseButtonCallback(++window: GLFWwindow, ++cbfun: GLFWmousebuttonfun);
        fn glfwSetCursorPosCallback(++window: GLFWwindow, ++cbfun: GLFWcursorposfun);
        fn glfwSetCursorEnterCallback(++window: GLFWwindow, ++cbfun: GLFWcursorenterfun);
        fn glfwSetScrollCallback(++window: GLFWwindow, ++cbfun: GLFWscrollfun);
        
        /* Joystick input */
        fn glfwGetJoystickParam(++joy: c_int, ++param: c_int) -> c_int;
        fn glfwGetJoystickAxes(++joy: c_int, ++axes: *c_float, ++numaxes: c_int) -> c_int;
        fn glfwGetJoystickButtons(++joy: c_int, ++buttons: *c_uchar, ++numbuttons: c_int) -> c_int;
        
        /* Clipboard */
        fn glfwSetClipboardString(++window: GLFWwindow, string: *c_char);
        fn glfwGetClipboardString(++window: GLFWwindow) -> *c_char;
        
        /* Time */
        fn glfwGetTime() -> c_double;
        fn glfwSetTime(++time: c_double);
        
        /* OpenGL support */
        fn glfwMakeContextCurrent(++window: GLFWwindow);
        fn glfwGetCurrentContext() -> GLFWwindow;
        fn glfwSwapBuffers(++window: GLFWwindow);
        fn glfwSwapInterval(++interval: c_int);
        fn glfwExtensionSupported(++extension: *c_char) -> c_int;
        fn glfwGetProcAddress(++procname: *c_char) -> GLProc;
        fn glfwCopyContext(++src: GLFWwindow, ++dst: GLFWwindow, ++mask: c_ulong);
    }
}


/* GLFW initialization, termination and version querying */

pub fn init() -> int {    
    api::glfwInit() as int
}

pub fn terminate() {
    api::glfwTerminate(); 
}

pub fn get_version() -> (int, int, int) {
    let major: c_int = 0;
    let minor: c_int = 0;
    let rev:   c_int = 0;
    unsafe {
        api::glfwGetVersion(
            to_unsafe_ptr(&major),
            to_unsafe_ptr(&minor),
            to_unsafe_ptr(&rev)
        );
    }
    return (major as int, minor as int, rev as int);
}

pub fn get_version_string() -> ~str {
    unsafe { str::raw::from_c_str(api::glfwGetVersionString()) }
}

/* Error handling */

pub fn get_error() -> c_int {
    api::glfwGetError()
}

pub fn error_string(error: c_int) -> ~str {
    unsafe { str::raw::from_c_str(api::glfwErrorString(error)) }
}

fn tls_errorfun(_v: @ErrorFun) {}

pub fn set_error_callback(cbfun: @ErrorFun) {
    unsafe { local_data_set(tls_errorfun, cbfun); }
    api::glfwSetErrorCallback(error_callback);
}

extern fn error_callback(error: c_int, format: *c_char) {
    unsafe {
        match local_data_get(tls_errorfun) {
            Some(f) => { (*f)(error, str::raw::from_c_str(format)); }
            None => {}
        }
    }
}

/* Video mode functions */

pub fn get_video_modes() -> ~[VidMode] {
    let count: c_int = 0;
    let modes: ~[VidMode];
    
    let ptr = api::glfwGetVideoModes(to_unsafe_ptr(&count));
    unsafe { modes = vec::from_buf(ptr, count as uint); }
    
    return move modes;
}

pub fn get_desktop_mode() -> VidMode {
    let mode = VidMode {
        width:     0,
        height :   0,
        redBits:   0,
        blueBits:  0,
        greenBits: 0,
    };
    unsafe { api::glfwGetDesktopMode(to_unsafe_ptr(&mode)); }
    
    return mode;
}

/* Gamma ramp functions */

pub fn set_gamma(gamma: float) {
    api::glfwSetGamma(gamma as c_float);
}

pub fn get_gamma_ramp() -> GammaRamp {
    let ramp = GammaRamp {
        red:   [0, ..256],
        green: [0, ..256],
        blue:  [0, ..256],
    };
    unsafe { api::glfwGetGammaRamp(to_unsafe_ptr(&ramp)); }
    
    return ramp;
}

pub fn set_gamma_ramp(ramp: &GammaRamp) {
    unsafe { api::glfwSetGammaRamp(to_unsafe_ptr(ramp)) }
}

/* Window handling */

pub fn default_window_hints() {
    api::glfwDefaultWindowHints();
}

pub fn window_hint(target: c_int, hint: c_int) {
    api::glfwWindowHint(target, hint);
}

pub fn create_window(width: int, height: int, mode: c_int, title: &str) -> Window {
    unsafe {
        Window {
            ptr: api::glfwCreateWindow(width as c_int,
                                       height as c_int,
                                       mode,
                                       str::as_c_str(title, |a| a),
                                       ptr::null())
        }
    }
}

pub fn create_shared_window(width: int, height: int, mode: c_int, title: &str, share: &Window) -> Window {
    unsafe {
        Window {
            ptr: api::glfwCreateWindow(width as c_int,
                                       height as c_int,
                                       mode,
                                       str::as_c_str(title, |a| a),
                                       share.ptr)
        }
    }
}

pub fn destroy_window(window: &mut Window) {
    api::glfwDestroyWindow(window.ptr);
}

pub impl Window {
    
    fn set_title(title: &str) {
        api::glfwSetWindowTitle(self.ptr, str::as_c_str(title, |a| a))
    }
    
    fn get_size() -> (int, int) {
        let width:  c_int = 0;
        let height: c_int = 0;
        unsafe {
            api::glfwGetWindowSize(
                self.ptr,
                to_unsafe_ptr(&width),
                to_unsafe_ptr(&height)
            );
        }
        
        return (width as int, height as int);
    }
    
    fn set_size(width: int, height: int) {
        api::glfwSetWindowSize(self.ptr, width as c_int, height as c_int);
    }
    
    fn get_pos() -> (int, int) {
        let xpos: c_int = 0;
        let ypos: c_int = 0;
        unsafe {
            api::glfwGetWindowPos(
                self.ptr,
                to_unsafe_ptr(&xpos),
                to_unsafe_ptr(&ypos)
            );
        }
        
        return (xpos as int, ypos as int);
    }
    
    fn set_pos(xpos: int, ypos: int) {
        api::glfwSetWindowPos(self.ptr, xpos as c_int, ypos as c_int);
    }
    
    fn iconify() {
        api::glfwIconifyWindow(self.ptr);
    }
    
    fn restore() {
        api::glfwRestoreWindow(self.ptr);
    }
    
    fn show() {
        api::glfwShowWindow(self.ptr);
    }
    
    fn hide() {
        api::glfwHideWindow(self.ptr);
    }

    fn get_param(param: c_int) -> int {
        api::glfwGetWindowParam(self.ptr, param as c_int) as int
    }
    
    fn set_user_pointer(pointer: *c_void) {
        api::glfwSetWindowUserPointer(self.ptr, pointer);
    }
    
    fn get_user_pointer() -> *c_void {
        api::glfwGetWindowUserPointer(self.ptr)
    }
    
    fn set_size_callback(cbfun: @WindowSizeFun) {
        unsafe { local_data_set(tls_windowsizefun, cbfun); }
        api::glfwSetWindowSizeCallback(self.ptr, window_size_callback);
    }

    fn set_close_callback(cbfun: @WindowCloseFun) {
        unsafe { local_data_set(tls_windowclosefun, cbfun); }
        api::glfwSetWindowCloseCallback(self.ptr, window_close_callback);
    }

    fn set_refresh_callback(cbfun: @WindowRefreshFun) {
        unsafe { local_data_set(tls_windowrefreshfun, cbfun); }
        api::glfwSetWindowRefreshCallback(self.ptr, window_refresh_callback);
    }

    fn set_focus_callback(cbfun: @WindowFocusFun) {
        unsafe { local_data_set(tls_windowfocusfun, cbfun); }
        api::glfwSetWindowFocusCallback(self.ptr, window_focus_callback);
    }

    fn set_iconify_callback(cbfun: @WindowIconifyFun) {
        unsafe { local_data_set(tls_windowiconifyfun, cbfun); }
        api::glfwSetWindowIconifyCallback(self.ptr, window_iconify_callback);
    }
    
    
    /* Input handling */
    
    fn get_input_mode(mode: c_int) -> int {
        api::glfwGetInputMode(self.ptr, mode) as int
    }
    
    fn set_input_mode(mode: c_int, value: int) {
        api::glfwSetInputMode(self.ptr, mode, value as c_int);
    }
    
    fn get_key(key: c_int) -> c_int {
        api::glfwGetKey(self.ptr, key)
    }
    
    fn get_mouse_button(button: c_int) -> c_int {
        api::glfwGetMouseButton(self.ptr, button)
    }
    
    fn get_cursor_pos() -> (int, int) {
        let xpos: c_int = 0;
        let ypos: c_int = 0;
        unsafe {
            api::glfwGetCursorPos(
                self.ptr,
                to_unsafe_ptr(&xpos),
                to_unsafe_ptr(&ypos)
            );
        }
        
        return (xpos as int, ypos as int);
    }
    
    fn set_cursor_pos(xpos: int, ypos: int) {
        api::glfwSetCursorPos(self.ptr, xpos as c_int, ypos as c_int);
    }
    
    fn get_scroll_offset() -> (f64, f64) {
        let xpos: f64 = 0f64;
        let ypos: f64 = 0f64;
        unsafe {
            api::glfwGetScrollOffset(
                self.ptr,
                to_unsafe_ptr(&xpos),
                to_unsafe_ptr(&ypos)
            );
        }
        
        return (xpos as f64, ypos as f64);
    }
    
    fn set_key_callback(cbfun: @KeyFun) {
        unsafe { local_data_set(tls_keyfun, cbfun); }
        api::glfwSetKeyCallback(self.ptr, key_callback);
    }
    
    fn set_char_callback(cbfun: @CharFun) {
        unsafe { local_data_set(tls_charfun, cbfun); }
        api::glfwSetCharCallback(self.ptr, char_callback);
    }
    
    fn set_mouse_button_callback(cbfun: @MouseButtonFun) {
        unsafe { local_data_set(tls_mousebuttonfun, cbfun); }
        api::glfwSetMouseButtonCallback(self.ptr, mouse_button_callback);
    }
    
    fn set_cursor_pos_callback(cbfun: @CursorPosFun) {
        unsafe { local_data_set(tls_cursorposfun, cbfun); }
        api::glfwSetCursorPosCallback(self.ptr, cursor_pos_callback);
    }
    
    fn set_cursor_enter_callback(cbfun: @CursorEnterFun) {
        unsafe { local_data_set(tls_cursorenterfun, cbfun); }
        api::glfwSetCursorEnterCallback(self.ptr, cursor_enter_callback);
    }
    
    fn set_scroll_callback(cbfun: @ScrollFun) {
        unsafe { local_data_set(tls_scrollfun, cbfun); }
        api::glfwSetScrollCallback(self.ptr, scroll_callback);
    }
}

fn tls_windowsizefun(_v: @WindowSizeFun) {}

extern fn window_size_callback(window: api::GLFWwindow, width: c_int, height: c_int) {
    unsafe {
        match local_data_get(tls_windowsizefun) {
            Some(f) => { (*f)(Window { ptr: window }, width as int, height as int); }
            None => {}
        }
    }
}

fn tls_windowclosefun(_v: @WindowCloseFun) {}

extern fn window_close_callback(window: api::GLFWwindow) -> c_int {
    unsafe {
        match local_data_get(tls_windowclosefun) {
            Some(f) => { (*f)(Window { ptr: window }) as c_int }
            None => { false as c_int }
        }
    }
}

// FIXME: Doesn't seem to work at the moment. See ../examples/callbacks.rs
fn tls_windowrefreshfun(_v: @WindowRefreshFun) {}

extern fn window_refresh_callback(window: api::GLFWwindow) {
    unsafe {
        match local_data_get(tls_windowrefreshfun) {
            Some(f) => { (*f)(Window { ptr: window }); }
            None => {}
        }
    }
}

fn tls_windowfocusfun(_v: @WindowFocusFun) {}

extern fn window_focus_callback(window: api::GLFWwindow, activated: c_int) {
    unsafe {
        match local_data_get(tls_windowfocusfun) {
            Some(f) => { (*f)(Window { ptr: window }, activated as bool); }
            None => {}
        }
    }
}

fn tls_windowiconifyfun(_v: @WindowIconifyFun) {}

extern fn window_iconify_callback(window: api::GLFWwindow, iconified: c_int) {
    unsafe {
        match local_data_get(tls_windowiconifyfun) {
            Some(f) => { (*f)(Window { ptr: window }, iconified as bool); }
            None => {}
        }
    }
}

/* Event handling */

pub fn poll_events() {    
    api::glfwPollEvents();
}
 
pub fn wait_events() {    
    api::glfwWaitEvents();
}

/* Input handling */

fn tls_keyfun(_v: @KeyFun) {}

extern fn key_callback(window: api::GLFWwindow, key: c_int, action: c_int) {
    unsafe {
        match local_data_get(tls_keyfun) {
            Some(f) => { (*f)(Window { ptr: window }, key, action); }
            None => {}
        }
    }
}

fn tls_charfun(_v: @CharFun) {}

extern fn char_callback(window: api::GLFWwindow, character: c_int) {
    unsafe {
        match local_data_get(tls_charfun) {
            Some(f) => { (*f)(Window { ptr: window }, character as char); }
            None => {}
        }
    }
}

fn tls_mousebuttonfun(_v: @MouseButtonFun) {}

extern fn mouse_button_callback(window: api::GLFWwindow, button: c_int, action: c_int) {
    unsafe {
        match local_data_get(tls_mousebuttonfun) {
            Some(f) => { (*f)(Window { ptr: window }, button, action); }
            None => {}
        }
    }
}

fn tls_cursorposfun(_v: @CursorPosFun) {}

extern fn cursor_pos_callback(window: api::GLFWwindow, x: c_int, y: c_int) {
    unsafe {
        match local_data_get(tls_cursorposfun) {
            Some(f) => { (*f)(Window { ptr: window }, x as int, y as int); }
            None => {}
        }
    }
}

fn tls_cursorenterfun(_v: @CursorEnterFun) {}

extern fn cursor_enter_callback(window: api::GLFWwindow, entered: c_int) {
    unsafe {
        match local_data_get(tls_cursorenterfun) {
            Some(f) => { (*f)(Window { ptr: window }, entered as bool); }
            None => {}
        }
    }
}

// FIXME: Doesn't seem to work at the moment. See ../examples/callbacks.rs
fn tls_scrollfun(_v: @ScrollFun) {}

extern fn scroll_callback(window: api::GLFWwindow, x: c_double, y: c_double) {
    unsafe {
        match local_data_get(tls_scrollfun) {
            Some(f) => { (*f)(Window { ptr: window }, x as f64, y as f64); }
            None => {}
        }
    }
}

/* Joystick input */

pub fn get_joystick_param(joy: int, param: c_int) -> int {
    api::glfwGetJoystickParam(joy as c_int, param) as int
}

/**
 * Somebody with a joystick will have to test this. I don't have one, unfortunately.
 *
 * I'm also unsure about whether I've got my pointers right. Use at your own risk - sorry!
 */
pub fn get_joystick_axes(joy: int, numaxes: int) -> Option<~[float]> {
    let axes_ptr: *c_float = ptr::null();
    let n = api::glfwGetJoystickAxes(joy as c_int, axes_ptr, numaxes as c_int) as uint;
    
    let axes: ~[float];
    unsafe { axes = vec::from_buf(axes_ptr, n).map(|a| *a as float); }   // Could be inefficient
    
    if numaxes > 0 { Some(move axes) }
    else           { None }
}

/**
 * Somebody with a joystick will have to test this. I don't have one, unfortunately.
 *
 * I'm also unsure about whether I've got my pointers right. Use at your own risk - sorry!
 */
pub fn get_joystick_buttons(joy: int, numbuttons: int) -> Option<~[char]> {
    let buttons_ptr: *c_uchar = ptr::null();
    let n = api::glfwGetJoystickButtons(joy as c_int, buttons_ptr, numbuttons as c_int) as uint;
    
    let buttons: ~[char];
    unsafe { buttons = vec::from_buf(buttons_ptr, n).map(|a| *a as char ); } // Could be inefficient
    
    if numbuttons > 0 { Some(move buttons) }
    else              { None }
}

/* Clipboard */

pub impl Window {
    fn set_clipboard_string(string: &str) {
        api::glfwSetClipboardString(self.ptr, str::as_c_str(string, |a| a));
    }
    
    fn get_clipboard_string() -> ~str {
        unsafe { str::raw::from_c_str(api::glfwGetClipboardString(self.ptr)) }
    }
}

/* Time */

pub fn get_time() -> f64 {
    api::glfwGetTime() as f64
}

pub fn set_time(time: f64) {
    api::glfwSetTime(time as c_double);
}

/* OpenGL support */

pub impl Window {
    fn make_context_current() {
        api::glfwMakeContextCurrent(self.ptr);
    }
}

pub fn get_current_context() -> Window {
    Window { ptr: api::glfwGetCurrentContext() }
}

impl Window {
    pub fn swap_buffers() {
        api::glfwSwapBuffers(self.ptr);
    }
}

pub fn swap_interval(interval: int) {
    api::glfwSwapInterval(interval as c_int);
}

pub fn extension_supported(extension: &str) -> int {
    api::glfwExtensionSupported(str::as_c_str(extension, |a| a)) as int
}

pub fn get_proc_address(procname: &str) -> GLProc {
    api::glfwGetProcAddress(str::as_c_str(procname, |a| a))
}

pub fn copy_context(src: &Window, dst: &mut Window, mask: u32) {
    api::glfwCopyContext(src.ptr, dst.ptr, mask as c_ulong);
}