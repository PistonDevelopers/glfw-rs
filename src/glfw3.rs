/*************************************************************************
 * GLFW3 Bindings for rust (based off alegalle's rust_glfw)
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

// This typedef is not actually specified in GLFW, but is added for clarity
pub type Enum = c_int;

/*************************************************************************
 * GLFW version
 *************************************************************************/
 
pub const VERSION_MAJOR        : Enum = 3;
pub const VERSION_MINOR        : Enum = 0;
pub const VERSION_REVISION     : Enum = 0;

/*************************************************************************
 * Input handling definitions
 *************************************************************************/

/* Key and button state/action definitions */
pub const RELEASE              : Enum = 0;
pub const PRESS                : Enum = 1;

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
pub const KEY_SPACE            : Enum = 32;
pub const KEY_APOSTROPHE       : Enum = 39;  /* ' */
pub const KEY_COMMA            : Enum = 44;  /* , */
pub const KEY_MINUS            : Enum = 45;  /* - */
pub const KEY_PERIOD           : Enum = 46;  /* . */
pub const KEY_SLASH            : Enum = 47;  /* / */
pub const KEY_0                : Enum = 48;
pub const KEY_1                : Enum = 49;
pub const KEY_2                : Enum = 50;
pub const KEY_3                : Enum = 51;
pub const KEY_4                : Enum = 52;
pub const KEY_5                : Enum = 53;
pub const KEY_6                : Enum = 54;
pub const KEY_7                : Enum = 55;
pub const KEY_8                : Enum = 56;
pub const KEY_9                : Enum = 57;
pub const KEY_SEMICOLON        : Enum = 59;  /* ; */
pub const KEY_EQUAL            : Enum = 61;  /* = */
pub const KEY_A                : Enum = 65;
pub const KEY_B                : Enum = 66;
pub const KEY_C                : Enum = 67;
pub const KEY_D                : Enum = 68;
pub const KEY_E                : Enum = 69;
pub const KEY_F                : Enum = 70;
pub const KEY_G                : Enum = 71;
pub const KEY_H                : Enum = 72;
pub const KEY_I                : Enum = 73;
pub const KEY_J                : Enum = 74;
pub const KEY_K                : Enum = 75;
pub const KEY_L                : Enum = 76;
pub const KEY_M                : Enum = 77;
pub const KEY_N                : Enum = 78;
pub const KEY_O                : Enum = 79;
pub const KEY_P                : Enum = 80;
pub const KEY_Q                : Enum = 81;
pub const KEY_R                : Enum = 82;
pub const KEY_S                : Enum = 83;
pub const KEY_T                : Enum = 84;
pub const KEY_U                : Enum = 85;
pub const KEY_V                : Enum = 86;
pub const KEY_W                : Enum = 87;
pub const KEY_X                : Enum = 88;
pub const KEY_Y                : Enum = 89;
pub const KEY_Z                : Enum = 90;
pub const KEY_LEFT_BRACKET     : Enum = 91;  /* [ */
pub const KEY_BACKSLASH        : Enum = 92;  /* \ */
pub const KEY_RIGHT_BRACKET    : Enum = 93;  /* ] */
pub const KEY_GRAVE_ACCENT     : Enum = 96;  /* ` */
pub const KEY_WORLD_1          : Enum = 161; /* non-US #1 */
pub const KEY_WORLD_2          : Enum = 162; /* non-US #2 */

/* Function keys */
pub const KEY_ESCAPE           : Enum = 256;
pub const KEY_ENTER            : Enum = 257;
pub const KEY_TAB              : Enum = 258;
pub const KEY_BACKSPACE        : Enum = 259;
pub const KEY_INSERT           : Enum = 260;
pub const KEY_DELETE           : Enum = 261;
pub const KEY_RIGHT            : Enum = 262;
pub const KEY_LEFT             : Enum = 263;
pub const KEY_DOWN             : Enum = 264;
pub const KEY_UP               : Enum = 265;
pub const KEY_PAGE_UP          : Enum = 266;
pub const KEY_PAGE_DOWN        : Enum = 267;
pub const KEY_HOME             : Enum = 268;
pub const KEY_END              : Enum = 269;
pub const KEY_CAPS_LOCK        : Enum = 280;
pub const KEY_SCROLL_LOCK      : Enum = 281;
pub const KEY_NUM_LOCK         : Enum = 282;
pub const KEY_PRINT_SCREEN     : Enum = 283;
pub const KEY_PAUSE            : Enum = 284;
pub const KEY_F1               : Enum = 290;
pub const KEY_F2               : Enum = 291;
pub const KEY_F3               : Enum = 292;
pub const KEY_F4               : Enum = 293;
pub const KEY_F5               : Enum = 294;
pub const KEY_F6               : Enum = 295;
pub const KEY_F7               : Enum = 296;
pub const KEY_F8               : Enum = 297;
pub const KEY_F9               : Enum = 298;
pub const KEY_F10              : Enum = 299;
pub const KEY_F11              : Enum = 300;
pub const KEY_F12              : Enum = 301;
pub const KEY_F13              : Enum = 302;
pub const KEY_F14              : Enum = 303;
pub const KEY_F15              : Enum = 304;
pub const KEY_F16              : Enum = 305;
pub const KEY_F17              : Enum = 306;
pub const KEY_F18              : Enum = 307;
pub const KEY_F19              : Enum = 308;
pub const KEY_F20              : Enum = 309;
pub const KEY_F21              : Enum = 310;
pub const KEY_F22              : Enum = 311;
pub const KEY_F23              : Enum = 312;
pub const KEY_F24              : Enum = 313;
pub const KEY_F25              : Enum = 314;
pub const KEY_KP_0             : Enum = 320;
pub const KEY_KP_1             : Enum = 321;
pub const KEY_KP_2             : Enum = 322;
pub const KEY_KP_3             : Enum = 323;
pub const KEY_KP_4             : Enum = 324;
pub const KEY_KP_5             : Enum = 325;
pub const KEY_KP_6             : Enum = 326;
pub const KEY_KP_7             : Enum = 327;
pub const KEY_KP_8             : Enum = 328;
pub const KEY_KP_9             : Enum = 329;
pub const KEY_KP_DECIMAL       : Enum = 330;
pub const KEY_KP_DIVIDE        : Enum = 331;
pub const KEY_KP_MULTIPLY      : Enum = 332;
pub const KEY_KP_SUBTRACT      : Enum = 333;
pub const KEY_KP_ADD           : Enum = 334;
pub const KEY_KP_ENTER         : Enum = 335;
pub const KEY_KP_EQUAL         : Enum = 336;
pub const KEY_LEFT_SHIFT       : Enum = 340;
pub const KEY_LEFT_CONTROL     : Enum = 341;
pub const KEY_LEFT_ALT         : Enum = 342;
pub const KEY_LEFT_SUPER       : Enum = 343;
pub const KEY_RIGHT_SHIFT      : Enum = 344;
pub const KEY_RIGHT_CONTROL    : Enum = 345;
pub const KEY_RIGHT_ALT        : Enum = 346;
pub const KEY_RIGHT_SUPER      : Enum = 347;
pub const KEY_MENU             : Enum = 348;
pub const KEY_LAST             : Enum = KEY_MENU;

/* GLFW 2.x key name aliases (deprecated) */
pub const KEY_ESC              : Enum = KEY_ESCAPE;
pub const KEY_DEL              : Enum = KEY_DELETE;
pub const KEY_PAGEUP           : Enum = KEY_PAGE_UP;
pub const KEY_PAGEDOWN         : Enum = KEY_PAGE_DOWN;
pub const KEY_KP_NUM_LOCK      : Enum = KEY_NUM_LOCK;
pub const KEY_LCTRL            : Enum = KEY_LEFT_CONTROL;
pub const KEY_LSHIFT           : Enum = KEY_LEFT_SHIFT;
pub const KEY_LALT             : Enum = KEY_LEFT_ALT;
pub const KEY_LSUPER           : Enum = KEY_LEFT_SUPER;
pub const KEY_RCTRL            : Enum = KEY_RIGHT_CONTROL;
pub const KEY_RSHIFT           : Enum = KEY_RIGHT_SHIFT;
pub const KEY_RALT             : Enum = KEY_RIGHT_ALT;
pub const KEY_RSUPER           : Enum = KEY_RIGHT_SUPER;

/* Mouse button aliases */
pub const MOUSE_BUTTON_LEFT    : Enum = MOUSE_BUTTON_1;
pub const MOUSE_BUTTON_RIGHT   : Enum = MOUSE_BUTTON_2;
pub const MOUSE_BUTTON_MIDDLE  : Enum = MOUSE_BUTTON_3;

/* Mouse button definitions */
pub const MOUSE_BUTTON_1       : Enum = 0;
pub const MOUSE_BUTTON_2       : Enum = 1;
pub const MOUSE_BUTTON_3       : Enum = 2;
pub const MOUSE_BUTTON_4       : Enum = 3;
pub const MOUSE_BUTTON_5       : Enum = 4;
pub const MOUSE_BUTTON_6       : Enum = 5;
pub const MOUSE_BUTTON_7       : Enum = 6;
pub const MOUSE_BUTTON_8       : Enum = 7;
pub const MOUSE_BUTTON_LAST    : Enum = MOUSE_BUTTON_8;

/* Joystick identifiers */
pub const JOYSTICK_1           : Enum = 0;
pub const JOYSTICK_2           : Enum = 1;
pub const JOYSTICK_3           : Enum = 2;
pub const JOYSTICK_4           : Enum = 3;
pub const JOYSTICK_5           : Enum = 4;
pub const JOYSTICK_6           : Enum = 5;
pub const JOYSTICK_7           : Enum = 6;
pub const JOYSTICK_8           : Enum = 7;
pub const JOYSTICK_9           : Enum = 8;
pub const JOYSTICK_10          : Enum = 9;
pub const JOYSTICK_11          : Enum = 10;
pub const JOYSTICK_12          : Enum = 11;
pub const JOYSTICK_13          : Enum = 12;
pub const JOYSTICK_14          : Enum = 13;
pub const JOYSTICK_15          : Enum = 14;
pub const JOYSTICK_16          : Enum = 15;
pub const JOYSTICK_LAST        : Enum = JOYSTICK_16;

/*************************************************************************
 * Other definitions
 *************************************************************************/

/* glfwCreateWindow modes */
pub const WINDOWED                     : Enum = 0x00010001;
pub const FULLSCREEN                   : Enum = 0x00010002;

/* glfwGetWindowParam tokens */
pub const ACTIVE                       : Enum = 0x00020001;
pub const ICONIFIED                    : Enum = 0x00020002;
pub const CLOSE_REQUESTED              : Enum = 0x00020003;
pub const OPENGL_REVISION              : Enum = 0x00020004;

/* glfwWindowHint tokens */
pub const RED_BITS                     : Enum = 0x00021000;
pub const GREEN_BITS                   : Enum = 0x00021001;
pub const BLUE_BITS                    : Enum = 0x00021002;
pub const ALPHA_BITS                   : Enum = 0x00021003;
pub const DEPTH_BITS                   : Enum = 0x00021004;
pub const STENCIL_BITS                 : Enum = 0x00021005;
pub const REFRESH_RATE                 : Enum = 0x00021006;
pub const ACCUM_RED_BITS               : Enum = 0x00021007;
pub const ACCUM_GREEN_BITS             : Enum = 0x00021008;
pub const ACCUM_BLUE_BITS              : Enum = 0x00021009;
pub const ACCUM_ALPHA_BITS             : Enum = 0x0002100A;
pub const AUX_BUFFERS                  : Enum = 0x0002100B;
pub const STEREO                       : Enum = 0x0002100C;
pub const WINDOW_RESIZABLE             : Enum = 0x0002100D;
pub const FSAA_SAMPLES                 : Enum = 0x0002100E;

/* The following constants are used with both glfwGetWindowParam
 * and glfwWindowHint
 */
pub const CLIENT_API                   : Enum = 0x00022000;
pub const OPENGL_VERSION_MAJOR         : Enum = 0x00022001;
pub const OPENGL_VERSION_MINOR         : Enum = 0x00022002;
pub const OPENGL_FORWARD_COMPAT        : Enum = 0x00022003;
pub const OPENGL_DEBUG_CONTEXT         : Enum = 0x00022004;
pub const OPENGL_PROFILE               : Enum = 0x00022005;
pub const OPENGL_ROBUSTNESS            : Enum = 0x00022006;
pub const RESIZABLE                    : Enum = 0x00022007;
pub const VISIBLE                      : Enum = 0x00022008;
 
/* GLFW_CLIENT_API tokens */
pub const OPENGL_API                   : Enum = 0x00000001;
pub const OPENGL_ES_API                : Enum = 0x00000002;

/* GLFW_OPENGL_ROBUSTNESS mode tokens */
pub const OPENGL_NO_ROBUSTNESS         : Enum = 0x00000000;
pub const OPENGL_NO_RESET_NOTIFICATION : Enum = 0x00000001;
pub const OPENGL_LOSE_CONTEXT_ON_RESET : Enum = 0x00000002;

/* GLFW_OPENGL_PROFILE bit tokens */
pub const OPENGL_NO_PROFILE            : Enum = 0x00000000;
pub const OPENGL_CORE_PROFILE          : Enum = 0x00000001;
pub const OPENGL_COMPAT_PROFILE        : Enum = 0x00000002;

/* glfwGetInputMode/glfwSetInputMode tokens */
pub const CURSOR_MODE                  : Enum = 0x00030001;
pub const STICKY_KEYS                  : Enum = 0x00030002;
pub const STICKY_MOUSE_BUTTONS         : Enum = 0x00030003;
pub const SYSTEM_KEYS                  : Enum = 0x00030004;
pub const KEY_REPEAT                   : Enum = 0x00030005;

/* GLFW_CURSOR_MODE values */
pub const CURSOR_NORMAL                : Enum = 0x00040001;
pub const CURSOR_HIDDEN                : Enum = 0x00040002;
pub const CURSOR_CAPTURED              : Enum = 0x00040003;

/* glfwGetJoystickParam tokens */
pub const PRESENT                      : Enum = 0x00050001;
pub const AXES                         : Enum = 0x00050002;
pub const BUTTONS                      : Enum = 0x00050003;

/* glfwGetError/glfwErrorString tokens */
pub const NO_ERROR                     : Enum = 0;
pub const NOT_INITIALIZED              : Enum = 0x00070001;
pub const NO_CURRENT_CONTEXT           : Enum = 0x00070002;
pub const INVALID_ENUM                 : Enum = 0x00070003;
pub const INVALID_VALUE                : Enum = 0x00070004;
pub const OUT_OF_MEMORY                : Enum = 0x00070005;
pub const OPENGL_UNAVAILABLE           : Enum = 0x00070006;
pub const VERSION_UNAVAILABLE          : Enum = 0x00070007;
pub const PLATFORM_ERROR               : Enum = 0x00070008;
pub const WINDOW_NOT_ACTIVE            : Enum = 0x00070009;
pub const FORMAT_UNAVAILABLE           : Enum = 0x0007000A;

/* Gamma ramps */
pub const GAMMA_RAMP_SIZE              : Enum = 256;

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
pub type ErrorFun           = @fn(error: Enum, format: ~str);
pub type WindowSizeFun      = @fn(window: Window, width: int, height: int);
pub type WindowCloseFun     = @fn(window: Window) -> bool;
pub type WindowRefreshFun   = @fn(window: Window);
pub type WindowFocusFun     = @fn(window: Window, activated: bool);
pub type WindowIconifyFun   = @fn(window: Window, iconified: bool);
pub type MouseButtonFun     = @fn(window: Window, button: Enum, action: Enum);
pub type CursorPosFun       = @fn(window: Window, x: int, y: int);
pub type CursorEnterFun     = @fn(window: Window, entered: bool);
pub type ScrollFun          = @fn(window: Window, x: f64, y: f64);
pub type KeyFun             = @fn(window: Window, key: Enum, action: Enum);
pub type CharFun            = @fn(window: Window, character: char);

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
        fn glfwInit() -> c_int;                                                             // GLFWAPI int  glfwInit(void);
        fn glfwTerminate();                                                                 // GLFWAPI void glfwTerminate(void);
        fn glfwGetVersion(major: &mut c_int, minor: &mut c_int, rev: &mut c_int);           // GLFWAPI void glfwGetVersion(int* major, int* minor, int* rev);
        fn glfwGetVersionString() -> *c_char;                                               // GLFWAPI const char* glfwGetVersionString(void);

        /* Error handling */
        fn glfwGetError() -> Enum;                                                          // GLFWAPI int glfwGetError(void);
        fn glfwErrorString(error: Enum) -> *c_char;                                         // GLFWAPI const char* glfwErrorString(int error);
        fn glfwSetErrorCallback(cbfun: GLFWerrorfun);                                       // GLFWAPI void glfwSetErrorCallback(GLFWerrorfun cbfun);
        
        /* Video mode functions */
        fn glfwGetVideoModes(count: &mut c_int) -> *VidMode;                                // GLFWAPI GLFWvidmode* glfwGetVideoModes(int* count);
        fn glfwGetDesktopMode(mode: &mut VidMode);                                          // GLFWAPI void glfwGetDesktopMode(GLFWvidmode* mode);
        
        /* Gamma ramp functions */
        fn glfwSetGamma(gamma: c_float);                                                    // GLFWAPI void glfwSetGamma(float gamma);
        fn glfwGetGammaRamp(ramp: &mut GammaRamp);                                          // GLFWAPI void glfwGetGammaRamp(GLFWgammaramp* ramp);
        fn glfwSetGammaRamp(ramp: &mut GammaRamp);                                          // GLFWAPI void glfwSetGammaRamp(const GLFWgammaramp* ramp);
        
        /* Window handling */
        fn glfwWindowHint(target: Enum, hint: c_int);                                       // GLFWAPI void glfwWindowHint(int target, int hint);
        fn glfwCreateWindow(width: c_int, height: c_int, mode: Enum, title: *c_char, share: GLFWwindow) -> GLFWwindow; // GLFWAPI GLFWwindow glfwCreateWindow(int width, int height, int mode, const char* title, GLFWwindow share);
        fn glfwDestroyWindow(window: GLFWwindow);                                           // GLFWAPI void glfwDestroyWindow(GLFWwindow window);
        fn glfwSetWindowTitle(window: GLFWwindow, title: *c_char);                          // GLFWAPI void glfwSetWindowTitle(GLFWwindow window, const char* title);
        fn glfwGetWindowSize(window: GLFWwindow, width: &mut c_int, height: &mut c_int);    // GLFWAPI void glfwGetWindowSize(GLFWwindow window, int* width, int* height);
        fn glfwSetWindowSize(window: GLFWwindow, width: c_int, height: c_int);              // GLFWAPI void glfwSetWindowSize(GLFWwindow window, int width, int height);
        fn glfwGetWindowPos(window: GLFWwindow, xpos: &mut c_int, ypos: &mut c_int);        // GLFWAPI void glfwGetWindowPos(GLFWwindow window, int* xpos, int* ypos);
        fn glfwSetWindowPos(window: GLFWwindow, xpos: c_int, ypos: c_int);                  // GLFWAPI void glfwSetWindowPos(GLFWwindow window, int xpos, int ypos);
        fn glfwIconifyWindow(window: GLFWwindow);                                           // GLFWAPI void glfwIconifyWindow(GLFWwindow window);
        fn glfwRestoreWindow(window: GLFWwindow);                                           // GLFWAPI void glfwRestoreWindow(GLFWwindow window);
        fn glfwGetWindowParam(window: GLFWwindow, param: Enum) -> c_int;                    // GLFWAPI int  glfwGetWindowParam(GLFWwindow window, int param);
        fn glfwSetWindowUserPointer(window: GLFWwindow, pointer: *c_void);                  // GLFWAPI void glfwSetWindowUserPointer(GLFWwindow window, void* pointer);
        fn glfwGetWindowUserPointer(window: GLFWwindow) -> *c_void;                         // GLFWAPI void* glfwGetWindowUserPointer(GLFWwindow window);
        fn glfwSetWindowSizeCallback(cbfun: GLFWwindowsizefun);                             // GLFWAPI void glfwSetWindowSizeCallback(GLFWwindowsizefun cbfun);
        fn glfwSetWindowCloseCallback(cbfun: GLFWwindowclosefun);                           // GLFWAPI void glfwSetWindowCloseCallback(GLFWwindowclosefun cbfun);
        fn glfwSetWindowRefreshCallback(cbfun: GLFWwindowrefreshfun);                       // GLFWAPI void glfwSetWindowRefreshCallback(GLFWwindowrefreshfun cbfun);
        fn glfwSetWindowFocusCallback(cbfun: GLFWwindowfocusfun);                           // GLFWAPI void glfwSetWindowFocusCallback(GLFWwindowfocusfun cbfun);
        fn glfwSetWindowIconifyCallback(cbfun: GLFWwindowiconifyfun);                       // GLFWAPI void glfwSetWindowIconifyCallback(GLFWwindowiconifyfun cbfun);

        /* Event handling */
        fn glfwPollEvents();                                                                // GLFWAPI void glfwPollEvents(void);
        fn glfwWaitEvents();                                                                // GLFWAPI void glfwWaitEvents(void);

        /* Input handling */
        fn glfwGetInputMode(window: GLFWwindow, mode: Enum) -> c_int;                       // GLFWAPI int  glfwGetInputMode(GLFWwindow window, int mode);
        fn glfwSetInputMode(window: GLFWwindow, mode: Enum, value: c_int);                  // GLFWAPI void glfwSetInputMode(GLFWwindow window, int mode, int value);
        fn glfwGetKey(window: GLFWwindow, key: Enum) -> Enum;                               // GLFWAPI int  glfwGetKey(GLFWwindow window, int key);
        fn glfwGetMouseButton(window: GLFWwindow, button: Enum) -> Enum;                    // GLFWAPI int  glfwGetMouseButton(GLFWwindow window, int button);
        fn glfwGetCursorPos(window: GLFWwindow, xpos: &mut c_int, ypos: &mut c_int);        // GLFWAPI void glfwGetCursorPos(GLFWwindow window, int* xpos, int* ypos);
        fn glfwSetCursorPos(window: GLFWwindow, xpos: c_int, ypos: c_int);                  // GLFWAPI void glfwSetCursorPos(GLFWwindow window, int xpos, int ypos);
        fn glfwGetScrollOffset(window: GLFWwindow, xoffset: &mut c_double, yoffset: &mut c_double); // GLFWAPI void glfwGetScrollOffset(GLFWwindow window, double* xoffset, double* yoffset);
        fn glfwSetKeyCallback(cbfun: GLFWkeyfun);                                           // GLFWAPI void glfwSetKeyCallback(GLFWkeyfun cbfun);
        fn glfwSetCharCallback(cbfun: GLFWcharfun);                                         // GLFWAPI void glfwSetCharCallback(GLFWcharfun cbfun);
        fn glfwSetMouseButtonCallback(cbfun: GLFWmousebuttonfun);                           // GLFWAPI void glfwSetMouseButtonCallback(GLFWmousebuttonfun cbfun);
        fn glfwSetCursorPosCallback(cbfun: GLFWcursorposfun);                               // GLFWAPI void glfwSetCursorPosCallback(GLFWcursorposfun cbfun);
        fn glfwSetCursorEnterCallback(cbfun: GLFWcursorenterfun);                           // GLFWAPI void glfwSetCursorEnterCallback(GLFWcursorenterfun cbfun);
        fn glfwSetScrollCallback(cbfun: GLFWscrollfun);                                     // GLFWAPI void glfwSetScrollCallback(GLFWscrollfun cbfun);
        
        /* Joystick input */
        fn glfwGetJoystickParam(joy: c_int, param: Enum) -> c_int;                          // GLFWAPI int glfwGetJoystickParam(int joy, int param);
        fn glfwGetJoystickAxes(joy: c_int, axes: *c_float, numaxes: c_int) -> c_int;        // GLFWAPI int glfwGetJoystickAxes(int joy, float* axes, int numaxes);
        fn glfwGetJoystickButtons(joy: c_int, buttons: *c_uchar, numbuttons: c_int) -> c_int; // GLFWAPI int glfwGetJoystickButtons(int joy, unsigned char* buttons, int numbuttons);
        
        /* Clipboard */
        fn glfwSetClipboardString(window: GLFWwindow, string: *c_char);                     // GLFWAPI void glfwSetClipboardString(GLFWwindow window, const char* string);
        fn glfwGetClipboardString(window: GLFWwindow) -> *c_char;                           // GLFWAPI const char* glfwGetClipboardString(GLFWwindow window);
        
        /* Time */
        fn glfwGetTime() -> c_double;                                                       // GLFWAPI double glfwGetTime(void);
        fn glfwSetTime(time: c_double);                                                     // GLFWAPI void   glfwSetTime(double time);
        
        /* OpenGL support */
        fn glfwMakeContextCurrent(window: GLFWwindow);                                      // GLFWAPI void glfwMakeContextCurrent(GLFWwindow window);
        fn glfwGetCurrentContext() -> GLFWwindow;                                           // GLFWAPI GLFWwindow glfwGetCurrentContext(void);
        fn glfwSwapBuffers(window: GLFWwindow);                                             // GLFWAPI void  glfwSwapBuffers(GLFWwindow window);
        fn glfwSwapInterval(interval: c_int);                                               // GLFWAPI void  glfwSwapInterval(int interval);
        fn glfwExtensionSupported(extension: *c_char) -> c_int;                             // GLFWAPI int   glfwExtensionSupported(const char* extension);
        fn glfwGetProcAddress(procname: *c_char) -> GLProc;                                 // GLFWAPI GLFWglproc glfwGetProcAddress(const char* procname);
        fn glfwCopyContext(src: GLFWwindow, dst: GLFWwindow, mask: c_ulong);                // GLFWAPI void  glfwCopyContext(GLFWwindow src, GLFWwindow dst, unsigned long mask);
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
    let mut major = 0, minor = 0, rev = 0;
    unsafe { api::glfwGetVersion(&mut major, &mut minor, &mut rev); }
    return (major as int, minor as int, rev as int);
}

pub fn get_version_string() -> ~str {
    unsafe { str::raw::from_c_str(api::glfwGetVersionString()) }
}

/* Error handling */

pub fn get_error() -> Enum {
    api::glfwGetError()
}

pub fn error_string(error: Enum) -> ~str {
    unsafe { str::raw::from_c_str(api::glfwErrorString(error)) }
}

fn tls_errorfun(_v: @ErrorFun) {}

pub fn set_error_callback(cbfun: @ErrorFun) {
    unsafe { local_data_set(tls_errorfun, cbfun); }
    api::glfwSetErrorCallback(error_callback);
}

extern fn error_callback(error: Enum, format: *c_char) {
    unsafe {
        match local_data_get(tls_errorfun) {
            Some(f) => { (*f)(error, str::raw::from_c_str(format)); }
            None => {}
        }
    }
}

/* Video mode functions */

pub fn get_video_modes() -> ~[VidMode] {
    let mut count: c_int = 0;
    let mode_ptr = api::glfwGetVideoModes(&mut count);
    let modes: ~[VidMode];
    unsafe { modes = vec::from_buf(mode_ptr, count as uint); }
    
    return move modes;
}

pub fn get_desktop_mode() -> VidMode {
    let mut mode = VidMode { width: 0, height : 0, redBits: 0, blueBits: 0, greenBits: 0 }; // initialisation is necessary
    unsafe { api::glfwGetDesktopMode(&mut mode); }
    return mode;
}

/* Gamma ramp functions */

pub fn set_gamma(gamma: float) {
    api::glfwSetGamma(gamma as c_float);
}

pub fn get_gamma_ramp() -> GammaRamp {
    let mut ramp = GammaRamp { red: [0, ..256], green: [0, ..256], blue: [0, ..256] }; // initialisation is necessary
    unsafe { api::glfwGetGammaRamp(&mut ramp); }
    return ramp;
}

pub fn set_gamma_ramp(ramp: &mut GammaRamp) {
    unsafe { api::glfwSetGammaRamp(ramp) }
}

/* Window handling */

pub fn window_hint(target: Enum, hint: int) {
    api::glfwWindowHint(target, hint as c_int);
}

pub fn create_window(width: int, height: int, mode: Enum, title: &str) -> Window {
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

pub fn create_shared_window(width: int, height: int, mode: Enum, title: &str, share: &mut Window) -> Window {
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
        let mut width = 0, height = 0;
        unsafe { api::glfwGetWindowSize(self.ptr, &mut width, &mut height)}
        return (width as int, height as int);
    }
    
    fn set_size(width: int, height: int) {
        api::glfwSetWindowSize(self.ptr, width as c_int, height as c_int);
    }
    
    fn get_pos() -> (int, int) {
        let mut xpos = 0, ypos = 0;
        unsafe { api::glfwGetWindowPos(self.ptr, &mut xpos, &mut ypos); }
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

    fn get_param(param: Enum) -> int {
        api::glfwGetWindowParam(self.ptr, param as c_int) as int
    }
    
    fn set_user_pointer(pointer: *c_void) {
        api::glfwSetWindowUserPointer(self.ptr, pointer);
    }
    
    fn get_user_pointer() -> *c_void {
        api::glfwGetWindowUserPointer(self.ptr)
    }
}

fn tls_windowsizefun(_v: @WindowSizeFun) {}

pub fn set_window_size_callback(cbfun: @WindowSizeFun) {
    unsafe { local_data_set(tls_windowsizefun, cbfun); }
    api::glfwSetWindowSizeCallback(window_size_callback);
}

extern fn window_size_callback(window: api::GLFWwindow, width: c_int, height: c_int) {
    unsafe {
        match local_data_get(tls_windowsizefun) {
            Some(f) => { (*f)(Window { ptr: window }, width as int, height as int); }
            None => {}
        }
    }
}

fn tls_windowclosefun(_v: @WindowCloseFun) {}

pub fn set_window_close_callback(cbfun: @WindowCloseFun) {
    unsafe { local_data_set(tls_windowclosefun, cbfun); }
    api::glfwSetWindowCloseCallback(window_close_callback);
}

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

pub fn set_window_refresh_callback(cbfun: @WindowRefreshFun) {
    unsafe { local_data_set(tls_windowrefreshfun, cbfun); }
    api::glfwSetWindowRefreshCallback(window_refresh_callback);
}

extern fn window_refresh_callback(window: api::GLFWwindow) {
    unsafe {
        match local_data_get(tls_windowrefreshfun) {
            Some(f) => { (*f)(Window { ptr: window }); }
            None => {}
        }
    }
}

fn tls_windowfocusfun(_v: @WindowFocusFun) {}

pub fn set_window_focus_callback(cbfun: @WindowFocusFun) {
    unsafe { local_data_set(tls_windowfocusfun, cbfun); }
    api::glfwSetWindowFocusCallback(window_focus_callback);
}

extern fn window_focus_callback(window: api::GLFWwindow, activated: c_int) {
    unsafe {
        match local_data_get(tls_windowfocusfun) {
            Some(f) => { (*f)(Window { ptr: window }, activated as bool); }
            None => {}
        }
    }
}

fn tls_windowiconifyfun(_v: @WindowIconifyFun) {}

pub fn set_window_iconify_callback(cbfun: @WindowIconifyFun) {
    unsafe { local_data_set(tls_windowiconifyfun, cbfun); }
    api::glfwSetWindowIconifyCallback(window_iconify_callback);
}

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

pub impl Window {
    fn get_input_mode(mode: Enum) -> int {
        api::glfwGetInputMode(self.ptr, mode) as int
    }
    
    fn set_input_mode(mode: Enum, value: int) {
        api::glfwSetInputMode(self.ptr, mode, value as c_int);
    }
    
    fn get_key(key: Enum) -> Enum {
        api::glfwGetKey(self.ptr, key)
    }
    
    fn get_mouse_button(button: Enum) -> Enum {
        api::glfwGetMouseButton(self.ptr, button)
    }
    
    fn get_cursor_pos() -> (int, int) {
        let mut xpos = 0, ypos = 0;
        unsafe { api::glfwGetCursorPos(self.ptr, &mut xpos, &mut ypos); }
        return (xpos as int, ypos as int);
    }
    
    fn set_cursor_pos(xpos: int, ypos: int) {
        api::glfwSetCursorPos(self.ptr, xpos as c_int, ypos as c_int);
    }
    
    fn get_scroll_offset() -> (f64, f64) {
        let mut xpos = 0f64, ypos = 0f64;
        unsafe { api::glfwGetScrollOffset(self.ptr, &mut xpos, &mut ypos); }
        return (xpos as f64, ypos as f64);
    }
}

fn tls_keyfun(_v: @KeyFun) {}

pub fn set_key_callback(cbfun: @KeyFun) {
    unsafe { local_data_set(tls_keyfun, cbfun); }
    api::glfwSetKeyCallback(key_callback);
}

extern fn key_callback(window: api::GLFWwindow, key: Enum, action: Enum) {
    unsafe {
        match local_data_get(tls_keyfun) {
            Some(f) => { (*f)(Window { ptr: window }, key, action); }
            None => {}
        }
    }
}

fn tls_charfun(_v: @CharFun) {}

pub fn set_char_callback(cbfun: @CharFun) {
    unsafe { local_data_set(tls_charfun, cbfun); }
    api::glfwSetCharCallback(char_callback);
}

extern fn char_callback(window: api::GLFWwindow, character: c_int) {
    unsafe {
        match local_data_get(tls_charfun) {
            Some(f) => { (*f)(Window { ptr: window }, character as char); }
            None => {}
        }
    }
}

fn tls_mousebuttonfun(_v: @MouseButtonFun) {}

pub fn set_mouse_button_callback(cbfun: @MouseButtonFun) {
    unsafe { local_data_set(tls_mousebuttonfun, cbfun); }
    api::glfwSetMouseButtonCallback(mouse_button_callback);
}

extern fn mouse_button_callback(window: api::GLFWwindow, button: Enum, action: Enum) {
    unsafe {
        match local_data_get(tls_mousebuttonfun) {
            Some(f) => { (*f)(Window { ptr: window }, button, action); }
            None => {}
        }
    }
}

fn tls_cursorposfun(_v: @CursorPosFun) {}

pub fn set_cursor_pos_callback(cbfun: @CursorPosFun) {
    unsafe { local_data_set(tls_cursorposfun, cbfun); }
    api::glfwSetCursorPosCallback(cursor_pos_callback);
}

extern fn cursor_pos_callback(window: api::GLFWwindow, x: c_int, y: c_int) {
    unsafe {
        match local_data_get(tls_cursorposfun) {
            Some(f) => { (*f)(Window { ptr: window }, x as int, y as int); }
            None => {}
        }
    }
}

fn tls_cursorenterfun(_v: @CursorEnterFun) {}

pub fn set_cursor_enter_callback(cbfun: @CursorEnterFun) {
    unsafe { local_data_set(tls_cursorenterfun, cbfun); }
    api::glfwSetCursorEnterCallback(cursor_enter_callback);
}

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

pub fn set_scroll_callback(cbfun: @ScrollFun) {
    unsafe { local_data_set(tls_scrollfun, cbfun); }
    api::glfwSetScrollCallback(scroll_callback);
}

extern fn scroll_callback(window: api::GLFWwindow, x: c_double, y: c_double) {
    unsafe {
        match local_data_get(tls_scrollfun) {
            Some(f) => { (*f)(Window { ptr: window }, x as f64, y as f64); }
            None => {}
        }
    }
}

/* Joystick input */

pub fn get_joystick_param(joy: int, param: Enum) -> int {
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