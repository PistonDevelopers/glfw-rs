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
 
pub const VERSION_MAJOR        : int = 3;
pub const VERSION_MINOR        : int = 0;
pub const VERSION_REVISION     : int = 0;

/*************************************************************************
 * Input handling definitions
 *************************************************************************/

/* Key and button state/action definitions */
pub const RELEASE              : int = 0;
pub const PRESS                : int = 1;

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
pub const KEY_SPACE            : int = 32;
pub const KEY_APOSTROPHE       : int = 39;  /* ' */
pub const KEY_COMMA            : int = 44;  /* , */
pub const KEY_MINUS            : int = 45;  /* - */
pub const KEY_PERIOD           : int = 46;  /* . */
pub const KEY_SLASH            : int = 47;  /* / */
pub const KEY_0                : int = 48;
pub const KEY_1                : int = 49;
pub const KEY_2                : int = 50;
pub const KEY_3                : int = 51;
pub const KEY_4                : int = 52;
pub const KEY_5                : int = 53;
pub const KEY_6                : int = 54;
pub const KEY_7                : int = 55;
pub const KEY_8                : int = 56;
pub const KEY_9                : int = 57;
pub const KEY_SEMICOLON        : int = 59;  /* ; */
pub const KEY_EQUAL            : int = 61;  /* = */
pub const KEY_A                : int = 65;
pub const KEY_B                : int = 66;
pub const KEY_C                : int = 67;
pub const KEY_D                : int = 68;
pub const KEY_E                : int = 69;
pub const KEY_F                : int = 70;
pub const KEY_G                : int = 71;
pub const KEY_H                : int = 72;
pub const KEY_I                : int = 73;
pub const KEY_J                : int = 74;
pub const KEY_K                : int = 75;
pub const KEY_L                : int = 76;
pub const KEY_M                : int = 77;
pub const KEY_N                : int = 78;
pub const KEY_O                : int = 79;
pub const KEY_P                : int = 80;
pub const KEY_Q                : int = 81;
pub const KEY_R                : int = 82;
pub const KEY_S                : int = 83;
pub const KEY_T                : int = 84;
pub const KEY_U                : int = 85;
pub const KEY_V                : int = 86;
pub const KEY_W                : int = 87;
pub const KEY_X                : int = 88;
pub const KEY_Y                : int = 89;
pub const KEY_Z                : int = 90;
pub const KEY_LEFT_BRACKET     : int = 91;  /* [ */
pub const KEY_BACKSLASH        : int = 92;  /* \ */
pub const KEY_RIGHT_BRACKET    : int = 93;  /* ] */
pub const KEY_GRAVE_ACCENT     : int = 96;  /* ` */
pub const KEY_WORLD_1          : int = 161; /* non-US #1 */
pub const KEY_WORLD_2          : int = 162; /* non-US #2 */

/* Function keys */
pub const KEY_ESCAPE           : int = 256;
pub const KEY_ENTER            : int = 257;
pub const KEY_TAB              : int = 258;
pub const KEY_BACKSPACE        : int = 259;
pub const KEY_INSERT           : int = 260;
pub const KEY_DELETE           : int = 261;
pub const KEY_RIGHT            : int = 262;
pub const KEY_LEFT             : int = 263;
pub const KEY_DOWN             : int = 264;
pub const KEY_UP               : int = 265;
pub const KEY_PAGE_UP          : int = 266;
pub const KEY_PAGE_DOWN        : int = 267;
pub const KEY_HOME             : int = 268;
pub const KEY_END              : int = 269;
pub const KEY_CAPS_LOCK        : int = 280;
pub const KEY_SCROLL_LOCK      : int = 281;
pub const KEY_NUM_LOCK         : int = 282;
pub const KEY_PRINT_SCREEN     : int = 283;
pub const KEY_PAUSE            : int = 284;
pub const KEY_F1               : int = 290;
pub const KEY_F2               : int = 291;
pub const KEY_F3               : int = 292;
pub const KEY_F4               : int = 293;
pub const KEY_F5               : int = 294;
pub const KEY_F6               : int = 295;
pub const KEY_F7               : int = 296;
pub const KEY_F8               : int = 297;
pub const KEY_F9               : int = 298;
pub const KEY_F10              : int = 299;
pub const KEY_F11              : int = 300;
pub const KEY_F12              : int = 301;
pub const KEY_F13              : int = 302;
pub const KEY_F14              : int = 303;
pub const KEY_F15              : int = 304;
pub const KEY_F16              : int = 305;
pub const KEY_F17              : int = 306;
pub const KEY_F18              : int = 307;
pub const KEY_F19              : int = 308;
pub const KEY_F20              : int = 309;
pub const KEY_F21              : int = 310;
pub const KEY_F22              : int = 311;
pub const KEY_F23              : int = 312;
pub const KEY_F24              : int = 313;
pub const KEY_F25              : int = 314;
pub const KEY_KP_0             : int = 320;
pub const KEY_KP_1             : int = 321;
pub const KEY_KP_2             : int = 322;
pub const KEY_KP_3             : int = 323;
pub const KEY_KP_4             : int = 324;
pub const KEY_KP_5             : int = 325;
pub const KEY_KP_6             : int = 326;
pub const KEY_KP_7             : int = 327;
pub const KEY_KP_8             : int = 328;
pub const KEY_KP_9             : int = 329;
pub const KEY_KP_DECIMAL       : int = 330;
pub const KEY_KP_DIVIDE        : int = 331;
pub const KEY_KP_MULTIPLY      : int = 332;
pub const KEY_KP_SUBTRACT      : int = 333;
pub const KEY_KP_ADD           : int = 334;
pub const KEY_KP_ENTER         : int = 335;
pub const KEY_KP_EQUAL         : int = 336;
pub const KEY_LEFT_SHIFT       : int = 340;
pub const KEY_LEFT_CONTROL     : int = 341;
pub const KEY_LEFT_ALT         : int = 342;
pub const KEY_LEFT_SUPER       : int = 343;
pub const KEY_RIGHT_SHIFT      : int = 344;
pub const KEY_RIGHT_CONTROL    : int = 345;
pub const KEY_RIGHT_ALT        : int = 346;
pub const KEY_RIGHT_SUPER      : int = 347;
pub const KEY_MENU             : int = 348;
pub const KEY_LAST             : int = KEY_MENU;

/* GLFW 2.x key name aliases (deprecated) */
pub const KEY_ESC              : int = KEY_ESCAPE;
pub const KEY_DEL              : int = KEY_DELETE;
pub const KEY_PAGEUP           : int = KEY_PAGE_UP;
pub const KEY_PAGEDOWN         : int = KEY_PAGE_DOWN;
pub const KEY_KP_NUM_LOCK      : int = KEY_NUM_LOCK;
pub const KEY_LCTRL            : int = KEY_LEFT_CONTROL;
pub const KEY_LSHIFT           : int = KEY_LEFT_SHIFT;
pub const KEY_LALT             : int = KEY_LEFT_ALT;
pub const KEY_LSUPER           : int = KEY_LEFT_SUPER;
pub const KEY_RCTRL            : int = KEY_RIGHT_CONTROL;
pub const KEY_RSHIFT           : int = KEY_RIGHT_SHIFT;
pub const KEY_RALT             : int = KEY_RIGHT_ALT;
pub const KEY_RSUPER           : int = KEY_RIGHT_SUPER;

/* Mouse button aliases */
pub const MOUSE_BUTTON_LEFT    : int = MOUSE_BUTTON_1;
pub const MOUSE_BUTTON_RIGHT   : int = MOUSE_BUTTON_2;
pub const MOUSE_BUTTON_MIDDLE  : int = MOUSE_BUTTON_3;

/* Mouse button definitions */
pub const MOUSE_BUTTON_1       : int = 0;
pub const MOUSE_BUTTON_2       : int = 1;
pub const MOUSE_BUTTON_3       : int = 2;
pub const MOUSE_BUTTON_4       : int = 3;
pub const MOUSE_BUTTON_5       : int = 4;
pub const MOUSE_BUTTON_6       : int = 5;
pub const MOUSE_BUTTON_7       : int = 6;
pub const MOUSE_BUTTON_8       : int = 7;
pub const MOUSE_BUTTON_LAST    : int = MOUSE_BUTTON_8;

/* Joystick identifiers */
pub const JOYSTICK_1           : int = 0;
pub const JOYSTICK_2           : int = 1;
pub const JOYSTICK_3           : int = 2;
pub const JOYSTICK_4           : int = 3;
pub const JOYSTICK_5           : int = 4;
pub const JOYSTICK_6           : int = 5;
pub const JOYSTICK_7           : int = 6;
pub const JOYSTICK_8           : int = 7;
pub const JOYSTICK_9           : int = 8;
pub const JOYSTICK_10          : int = 9;
pub const JOYSTICK_11          : int = 10;
pub const JOYSTICK_12          : int = 11;
pub const JOYSTICK_13          : int = 12;
pub const JOYSTICK_14          : int = 13;
pub const JOYSTICK_15          : int = 14;
pub const JOYSTICK_16          : int = 15;
pub const JOYSTICK_LAST        : int = JOYSTICK_16;

/*************************************************************************
 * Other definitions
 *************************************************************************/

/* glfwCreateWindow modes */
pub const WINDOWED                     : int = 0x00010001;
pub const FULLSCREEN                   : int = 0x00010002;

/* glfwGetWindowParam tokens */
pub const ACTIVE                       : int = 0x00020001;
pub const ICONIFIED                    : int = 0x00020002;
pub const CLOSE_REQUESTED              : int = 0x00020003;
pub const OPENGL_REVISION              : int = 0x00020004;

/* glfwWindowHint tokens */
pub const RED_BITS                     : int = 0x00021000;
pub const GREEN_BITS                   : int = 0x00021001;
pub const BLUE_BITS                    : int = 0x00021002;
pub const ALPHA_BITS                   : int = 0x00021003;
pub const DEPTH_BITS                   : int = 0x00021004;
pub const STENCIL_BITS                 : int = 0x00021005;
pub const REFRESH_RATE                 : int = 0x00021006;
pub const ACCUM_RED_BITS               : int = 0x00021007;
pub const ACCUM_GREEN_BITS             : int = 0x00021008;
pub const ACCUM_BLUE_BITS              : int = 0x00021009;
pub const ACCUM_ALPHA_BITS             : int = 0x0002100A;
pub const AUX_BUFFERS                  : int = 0x0002100B;
pub const STEREO                       : int = 0x0002100C;
pub const WINDOW_RESIZABLE             : int = 0x0002100D;
pub const FSAA_SAMPLES                 : int = 0x0002100E;

/* The following constants are used with both glfwGetWindowParam
 * and glfwWindowHint
 */
pub const CLIENT_API                   : int = 0x00022000;
pub const OPENGL_VERSION_MAJOR         : int = 0x00022001;
pub const OPENGL_VERSION_MINOR         : int = 0x00022002;
pub const OPENGL_FORWARD_COMPAT        : int = 0x00022003;
pub const OPENGL_DEBUG_CONTEXT         : int = 0x00022004;
pub const OPENGL_PROFILE               : int = 0x00022005;
pub const OPENGL_ROBUSTNESS            : int = 0x00022006;
pub const RESIZABLE                    : int = 0x00022007;
pub const VISIBLE                      : int = 0x00022008;
 
/* GLFW_CLIENT_API tokens */
pub const OPENGL_API                   : int = 0x00000001;
pub const OPENGL_ES_API                : int = 0x00000002;

/* GLFW_OPENGL_ROBUSTNESS mode tokens */
pub const OPENGL_NO_ROBUSTNESS         : int = 0x00000000;
pub const OPENGL_NO_RESET_NOTIFICATION : int = 0x00000001;
pub const OPENGL_LOSE_CONTEXT_ON_RESET : int = 0x00000002;

/* GLFW_OPENGL_PROFILE bit tokens */
pub const OPENGL_NO_PROFILE            : int = 0x00000000;
pub const OPENGL_CORE_PROFILE          : int = 0x00000001;
pub const OPENGL_COMPAT_PROFILE        : int = 0x00000002;

/* glfwGetInputMode/glfwSetInputMode tokens */
pub const CURSOR_MODE                  : int = 0x00030001;
pub const STICKY_KEYS                  : int = 0x00030002;
pub const STICKY_MOUSE_BUTTONS         : int = 0x00030003;
pub const SYSTEM_KEYS                  : int = 0x00030004;
pub const KEY_REPEAT                   : int = 0x00030005;

/* GLFW_CURSOR_MODE values */
pub const CURSOR_NORMAL                : int = 0x00040001;
pub const CURSOR_HIDDEN                : int = 0x00040002;
pub const CURSOR_CAPTURED              : int = 0x00040003;

/* glfwGetJoystickParam tokens */
pub const PRESENT                      : int = 0x00050001;
pub const AXES                         : int = 0x00050002;
pub const BUTTONS                      : int = 0x00050003;

/* glfwGetError/glfwErrorString tokens */
pub const NO_ERROR                     : int = 0;
pub const NOT_INITIALIZED              : int = 0x00070001;
pub const NO_CURRENT_CONTEXT           : int = 0x00070002;
pub const INVALID_ENUM                 : int = 0x00070003;
pub const INVALID_VALUE                : int = 0x00070004;
pub const OUT_OF_MEMORY                : int = 0x00070005;
pub const OPENGL_UNAVAILABLE           : int = 0x00070006;
pub const VERSION_UNAVAILABLE          : int = 0x00070007;
pub const PLATFORM_ERROR               : int = 0x00070008;
pub const WINDOW_NOT_ACTIVE            : int = 0x00070009;
pub const FORMAT_UNAVAILABLE           : int = 0x0007000A;

/* Gamma ramps */
pub const GAMMA_RAMP_SIZE              : int = 256;

/*************************************************************************
 * Typedefs
 *************************************************************************/

// /* OpenGL function pointer type */
// Will have to be changed once we can do external C callbacks nicely
pub pub type GLProc = *u8;              // typedef void (*GLFWglproc)(void);

/* Window handle type */
pub type WindowPtr = *c_void;      // typedef void* GLFWwindow;

// Wraps * pointer in a struct for safety 
pub struct Window {
    mut ptr: WindowPtr
}

/* Function pointer types */
// Will have to be changed once we can do external C callbacks nicely
pub type ErrorFun           = *u8;  // typedef void (* GLFWerrorfun)(int,const char*);
pub type WindowSizeFun      = *u8;  // typedef void (* GLFWwindowsizefun)(GLFWwindow,int,int);
pub type WindowCloseFun     = *u8;  // typedef int  (* GLFWwindowclosefun)(GLFWwindow);
pub type WindowRefreshFun   = *u8;  // typedef void (* GLFWwindowrefreshfun)(GLFWwindow);
pub type WindowFocusFun     = *u8;  // typedef void (* GLFWwindowfocusfun)(GLFWwindow,int);
pub type WindowIconifyFun   = *u8;  // typedef void (* GLFWwindowiconifyfun)(GLFWwindow,int);
pub type MouseButtonFun     = *u8;  // typedef void (* GLFWmousebuttonfun)(GLFWwindow,int,int);
pub type CursorPosFun       = *u8;  // typedef void (* GLFWcursorposfun)(GLFWwindow,int,int);
pub type CursorEnterFun     = *u8;  // typedef void (* GLFWcursorenterfun)(GLFWwindow,int);
pub type ScrollFun          = *u8;  // typedef void (* GLFWscrollfun)(GLFWwindow,double,double);
pub type KeyFun             = *u8;  // typedef void (* GLFWkeyfun)(GLFWwindow,int,int);
pub type CharFun            = *u8;  // typedef void (* GLFWcharfun)(GLFWwindow,int);

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
pub extern mod api {
    
    /* GLFW initialization, termination and version querying */
    fn glfwInit() -> c_int;                                                             // GLFWAPI int  glfwInit(void);
    fn glfwTerminate();                                                                 // GLFWAPI void glfwTerminate(void);
    fn glfwGetVersion(major: &mut c_int, minor: &mut c_int, rev: &mut c_int);           // GLFWAPI void glfwGetVersion(int* major, int* minor, int* rev);
    fn glfwGetVersionString() -> *c_char;                                               // GLFWAPI const char* glfwGetVersionString(void);

    /* Error handling */
    fn glfwGetError() -> c_int;                                                         // GLFWAPI int glfwGetError(void);
    fn glfwErrorString(error: c_int) -> *c_char;                                        // GLFWAPI const char* glfwErrorString(int error);
    fn glfwSetErrorCallback(cbfun: ErrorFun);                                           // GLFWAPI void glfwSetErrorCallback(GLFWerrorfun cbfun);
    
    /* Video mode functions */
    fn glfwGetVideoModes(count: &mut c_int) -> *VidMode;                            // GLFWAPI GLFWvidmode* glfwGetVideoModes(int* count);
    fn glfwGetDesktopMode(mode: &mut VidMode);                                      // GLFWAPI void glfwGetDesktopMode(GLFWvidmode* mode);
    
    /* Gamma ramp functions */
    fn glfwSetGamma(gamma: c_float);                                                    // GLFWAPI void glfwSetGamma(float gamma);
    fn glfwGetGammaRamp(ramp: &mut GammaRamp);                                          // GLFWAPI void glfwGetGammaRamp(GLFWgammaramp* ramp);
    fn glfwSetGammaRamp(ramp: &mut GammaRamp);                                          // GLFWAPI void glfwSetGammaRamp(const GLFWgammaramp* ramp);
    
    /* Window handling */
    fn glfwWindowHint(target: c_int, hint: c_int);                                      // GLFWAPI void glfwWindowHint(int target, int hint);
    fn glfwCreateWindow(width: c_int, height: c_int, mode: c_int, title: *c_char, share: WindowPtr) -> WindowPtr; // GLFWAPI GLFWwindow glfwCreateWindow(int width, int height, int mode, const char* title, GLFWwindow share);
    fn glfwDestroyWindow(window: WindowPtr);                                            // GLFWAPI void glfwDestroyWindow(GLFWwindow window);
    fn glfwSetWindowTitle(window: WindowPtr, title: *c_char);                           // GLFWAPI void glfwSetWindowTitle(GLFWwindow window, const char* title);
    fn glfwGetWindowSize(window: WindowPtr, width: &mut c_int, height: &mut c_int);     // GLFWAPI void glfwGetWindowSize(GLFWwindow window, int* width, int* height);
    fn glfwSetWindowSize(window: WindowPtr, width: c_int, height: c_int);               // GLFWAPI void glfwSetWindowSize(GLFWwindow window, int width, int height);
    fn glfwGetWindowPos(window: WindowPtr, xpos: &mut c_int, ypos: &mut c_int);         // GLFWAPI void glfwGetWindowPos(GLFWwindow window, int* xpos, int* ypos);
    fn glfwSetWindowPos(window: WindowPtr, xpos: c_int, ypos: c_int);                   // GLFWAPI void glfwSetWindowPos(GLFWwindow window, int xpos, int ypos);
    fn glfwIconifyWindow(window: WindowPtr);                                            // GLFWAPI void glfwIconifyWindow(GLFWwindow window);
    fn glfwRestoreWindow(window: WindowPtr);                                            // GLFWAPI void glfwRestoreWindow(GLFWwindow window);
    fn glfwGetWindowParam(window: WindowPtr, param: c_int) -> c_int;                    // GLFWAPI int  glfwGetWindowParam(GLFWwindow window, int param);
    fn glfwSetWindowUserPointer(window: WindowPtr, pointer: *c_void);                   // GLFWAPI void glfwSetWindowUserPointer(GLFWwindow window, void* pointer);
    fn glfwGetWindowUserPointer(window: WindowPtr) -> *c_void;                          // GLFWAPI void* glfwGetWindowUserPointer(GLFWwindow window);
    fn glfwSetWindowSizeCallback(cbfun: WindowSizeFun);                                 // GLFWAPI void glfwSetWindowSizeCallback(GLFWwindowsizefun cbfun);
    fn glfwSetWindowCloseCallback(cbfun: WindowCloseFun);                               // GLFWAPI void glfwSetWindowCloseCallback(GLFWwindowclosefun cbfun);
    fn glfwSetWindowRefreshCallback(cbfun: WindowRefreshFun);                           // GLFWAPI void glfwSetWindowRefreshCallback(GLFWwindowrefreshfun cbfun);
    fn glfwSetWindowFocusCallback(cbfun: WindowFocusFun);                               // GLFWAPI void glfwSetWindowFocusCallback(GLFWwindowfocusfun cbfun);
    fn glfwSetWindowIconifyCallback(cbfun: WindowIconifyFun);                           // GLFWAPI void glfwSetWindowIconifyCallback(GLFWwindowiconifyfun cbfun);

    /* Event handling */
    fn glfwPollEvents();                                                                // GLFWAPI void glfwPollEvents(void);
    fn glfwWaitEvents();                                                                // GLFWAPI void glfwWaitEvents(void);

    /* Input handling */
    fn glfwGetInputMode(window: WindowPtr, mode: c_int) -> c_int;                       // GLFWAPI int  glfwGetInputMode(GLFWwindow window, int mode);
    fn glfwSetInputMode(window: WindowPtr, mode: c_int, value: c_int);                  // GLFWAPI void glfwSetInputMode(GLFWwindow window, int mode, int value);
    fn glfwGetKey(window: WindowPtr, key: c_int) -> c_int;                              // GLFWAPI int  glfwGetKey(GLFWwindow window, int key);
    fn glfwGetMouseButton(window: WindowPtr, button: c_int) -> c_int;                   // GLFWAPI int  glfwGetMouseButton(GLFWwindow window, int button);
    fn glfwGetCursorPos(window: WindowPtr, xpos: &mut c_int, ypos: &mut c_int);         // GLFWAPI void glfwGetCursorPos(GLFWwindow window, int* xpos, int* ypos);
    fn glfwSetCursorPos(window: WindowPtr, xpos: c_int, ypos: c_int);                   // GLFWAPI void glfwSetCursorPos(GLFWwindow window, int xpos, int ypos);
    fn glfwGetScrollOffset(window: WindowPtr, xoffset: &mut c_double, yoffset: &mut c_double); // GLFWAPI void glfwGetScrollOffset(GLFWwindow window, double* xoffset, double* yoffset);
    fn glfwSetKeyCallback(cbfun: KeyFun);                                               // GLFWAPI void glfwSetKeyCallback(GLFWkeyfun cbfun);
    fn glfwSetCharCallback(cbfun: CharFun);                                             // GLFWAPI void glfwSetCharCallback(GLFWcharfun cbfun);
    fn glfwSetMouseButtonCallback(cbfun: MouseButtonFun);                               // GLFWAPI void glfwSetMouseButtonCallback(GLFWmousebuttonfun cbfun);
    fn glfwSetCursorPosCallback(cbfun: CursorPosFun);                                   // GLFWAPI void glfwSetCursorPosCallback(GLFWcursorposfun cbfun);
    fn glfwSetCursorEnterCallback(cbfun: CursorEnterFun);                               // GLFWAPI void glfwSetCursorEnterCallback(GLFWcursorenterfun cbfun);
    fn glfwSetScrollCallback(cbfun: ScrollFun);                                         // GLFWAPI void glfwSetScrollCallback(GLFWscrollfun cbfun);
    
    /* Joystick input */
    fn glfwGetJoystickParam(joy: c_int, param: c_int) -> c_int;                         // GLFWAPI int glfwGetJoystickParam(int joy, int param);
    fn glfwGetJoystickAxes(joy: c_int, axes: *c_float, numaxes: c_int) -> c_int;        // GLFWAPI int glfwGetJoystickAxes(int joy, float* axes, int numaxes);
    fn glfwGetJoystickButtons(joy: c_int, buttons: *c_uchar, numbuttons: c_int) -> c_int; // GLFWAPI int glfwGetJoystickButtons(int joy, unsigned char* buttons, int numbuttons);
    
    /* Clipboard */
    fn glfwSetClipboardString(window: WindowPtr, string: *c_char);                      // GLFWAPI void glfwSetClipboardString(GLFWwindow window, const char* string);
    fn glfwGetClipboardString(window: WindowPtr) -> *c_char;                            // GLFWAPI const char* glfwGetClipboardString(GLFWwindow window);
    
    /* Time */
    fn glfwGetTime() -> c_double;                                                       // GLFWAPI double glfwGetTime(void);
    fn glfwSetTime(time: c_double);                                                     // GLFWAPI void   glfwSetTime(double time);
    
    /* OpenGL support */
    fn glfwMakeContextCurrent(window: WindowPtr);                                       // GLFWAPI void glfwMakeContextCurrent(GLFWwindow window);
    fn glfwGetCurrentContext() -> WindowPtr;                                            // GLFWAPI GLFWwindow glfwGetCurrentContext(void);
    fn glfwSwapBuffers(window: WindowPtr);                                              // GLFWAPI void  glfwSwapBuffers(GLFWwindow window);
    fn glfwSwapInterval(interval: c_int);                                               // GLFWAPI void  glfwSwapInterval(int interval);
    fn glfwExtensionSupported(extension: *c_char) -> c_int;                             // GLFWAPI int   glfwExtensionSupported(const char* extension);
    fn glfwGetProcAddress(procname: *c_char) -> GLProc;                                 // GLFWAPI GLFWglproc glfwGetProcAddress(const char* procname);
    fn glfwCopyContext(src: WindowPtr, dst: WindowPtr, mask: c_ulong);                  // GLFWAPI void  glfwCopyContext(GLFWwindow src, GLFWwindow dst, unsigned long mask);
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

pub fn get_error() -> int {
    api::glfwGetError() as int
}

pub fn error_string(error: int) -> ~str {
    unsafe { str::raw::from_c_str(api::glfwErrorString(error as c_int)) }
}

// TODO: glfwSetErrorCallback

/* Video mode functions */

pub fn get_video_modes() -> ~[VidMode] {
    let mut count: c_int = 0;
    let mut mode_ptr: *VidMode;
    let mut modes: ~[VidMode];
    unsafe {
        mode_ptr = api::glfwGetVideoModes(&mut count);
        modes = vec::from_buf(mode_ptr, count as uint);
    }
    return modes;
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

pub fn window_hint(target: int, hint: int) {
    api::glfwWindowHint(target as c_int, hint as c_int);
}

pub fn create_window(width: int, height: int, mode: int, title: &str) -> Window {
    unsafe {
        Window {
            ptr: api::glfwCreateWindow(width as c_int,
                                         height as c_int,
                                         mode as c_int,
                                         str::as_c_str(title, |a| a),
                                         ptr::null())
        }
    }
}

pub fn create_shared_window(width: int, height: int, mode: int, title: &str, share: &mut Window) -> Window {
    unsafe {
        Window {
            ptr: api::glfwCreateWindow(width as c_int,
                                         height as c_int,
                                         mode as c_int,
                                         str::as_c_str(title, |a| a),
                                         share.ptr)
        }
    }
}

pub fn destroy_window(window: &mut Window) {
    api::glfwDestroyWindow(window.ptr);
}

impl Window {
    pub fn set_title(title: &str) {
        api::glfwSetWindowTitle(self.ptr, str::as_c_str(title, |a| a))
    }

    pub fn get_size() -> (int, int) {
        let mut width = 0, height = 0;
        unsafe { api::glfwGetWindowSize(self.ptr, &mut width, &mut height)}
        return (width as int, height as int);
    }

    pub fn set_size(width: int, height: int) {
        api::glfwSetWindowSize(self.ptr, width as c_int, height as c_int);
    }

    pub fn get_pos() -> (int, int) {
        let mut xpos = 0, ypos = 0;
        unsafe { api::glfwGetWindowPos(self.ptr, &mut xpos, &mut ypos); }
        return (xpos as int, ypos as int);
    }

    pub fn set_pos(xpos: int, ypos: int) {
        api::glfwSetWindowPos(self.ptr, xpos as c_int, ypos as c_int);
    }

    pub fn iconify() {
        api::glfwIconifyWindow(self.ptr);
    }

    pub fn restore() {
        api::glfwRestoreWindow(self.ptr);
    }

    pub fn get_param(param: int) -> int {
        api::glfwGetWindowParam(self.ptr, param as c_int) as int
    }
    
    // TODO: glfwSetWindowUserPointer
    // TODO: glfwGetWindowUserPointer
    // TODO: glfwSetWindowSizeCallback
    // TODO: glfwSetWindowCloseCallback
    // TODO: glfwSetWindowRefreshCallback
    // TODO: glfwSetWindowFocusCallback
    // TODO: glfwSetWindowIconifyCallback
}

/* Event handling */

pub fn poll_events() {    
    api::glfwPollEvents();
}
 
pub fn wait_events() {    
    api::glfwWaitEvents();
}

/* Input handling */

impl Window {
    pub fn get_input_mode(mode: int) -> int {
        api::glfwGetInputMode(self.ptr, mode as c_int) as int
    }

    pub fn set_input_mode(mode: int, value: int) {
        api::glfwSetInputMode(self.ptr, mode as c_int, value as c_int);
    }

    pub fn get_key(key: int) -> int {
        api::glfwGetKey(self.ptr, key as c_int) as int
    }

    pub fn get_mouse_button(button: int) -> int {
        api::glfwGetMouseButton(self.ptr, button as c_int) as int
    }

    pub fn get_cursor_pos() -> (int, int) {
        let mut xpos = 0, ypos = 0;
        unsafe { api::glfwGetCursorPos(self.ptr, &mut xpos, &mut ypos); }
        return (xpos as int, ypos as int);
    }

    pub fn set_cursor_pos(xpos: int, ypos: int) {
        api::glfwSetCursorPos(self.ptr, xpos as c_int, ypos as c_int);
    }

    pub fn get_scroll_offset() -> (f64, f64) {
        let mut xpos = 0f64, ypos = 0f64;
        unsafe { api::glfwGetScrollOffset(self.ptr, &mut xpos, &mut ypos); }
        return (xpos as f64, ypos as f64);
    }
    
    // TODO: glfwSetKeyCallback
    // TODO: glfwSetCharCallback
    // TODO: glfwSetMouseButtonCallback
    // TODO: glfwSetCursorPosCallback
    // TODO: glfwSetCursorEnterCallback
    // TODO: glfwSetScrollCallback
}

/* Joystick input */

pub fn get_joystick_param(joy: int, param: int) -> int {
    api::glfwGetJoystickParam(joy as c_int, param as c_int) as int
}

/**
 * Somebody with a joystick will have to test this. I don't have one, unfortunately.
 *
 * I'm also unsure about whether I've got my pointers right. Use at your own risk - sorry!
 */
pub fn get_joystick_axes(joy: int, numaxes: int) -> Option<~[float]> {
    let axes: ~[float];
    
    unsafe {
        let axes_ptr: *c_float = ptr::null();
        let n = api::glfwGetJoystickAxes(joy as c_int, axes_ptr, numaxes as c_int) as uint;
        axes = vec::from_buf(axes_ptr, n).map(|a| *a as float );   // Could be inefficient
    }
    
    if numaxes > 0 { Some(axes) }
    else           { None }
}

/**
 * Somebody with a joystick will have to test this. I don't have one, unfortunately.
 *
 * I'm also unsure about whether I've got my pointers right. Use at your own risk - sorry!
 */
pub fn get_joystick_buttons(joy: int, numbuttons: int) -> Option<~[char]> {
    let buttons: ~[char];
    
    unsafe {
        let buttons_ptr: *c_uchar = ptr::null();
        let n = api::glfwGetJoystickButtons(joy as c_int, buttons_ptr, numbuttons as c_int) as uint;
        buttons = vec::from_buf(buttons_ptr, n).map(|a| *a as char ); // Could be inefficient
    }
    
    if numbuttons > 0 { Some(buttons) }
    else              { None }
}

/* Clipboard */

impl Window {
    pub fn set_clipboard_string(string: &str) {
        api::glfwSetClipboardString(self.ptr, str::as_c_str(string, |a| a));
    }

    pub fn get_clipboard_string() -> ~str {
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

impl Window {
    pub fn make_context_current() {
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