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
use vec::raw::from_buf;
use vec::map;

#[nolink]
#[cfg(target_os = "macos")]
#[link_args="-framework Cocoa -framework OpenGL -framework IOKit -framework CoreFoundation"]
extern mod osx_frameworks {}

/*************************************************************************
 * GLFW version
 *************************************************************************/
 
const GLFW_VERSION_MAJOR        : int = 3;
const GLFW_VERSION_MINOR        : int = 0;
const GLFW_VERSION_REVISION     : int = 0;

/*************************************************************************
 * Input handling definitions
 *************************************************************************/

/* Key and button state/action definitions */
const GLFW_RELEASE              : int = 0;
const GLFW_PRESS                : int = 1;

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
const GLFW_KEY_SPACE            : int = 32;
const GLFW_KEY_APOSTROPHE       : int = 39;  /* ' */
const GLFW_KEY_COMMA            : int = 44;  /* , */
const GLFW_KEY_MINUS            : int = 45;  /* - */
const GLFW_KEY_PERIOD           : int = 46;  /* . */
const GLFW_KEY_SLASH            : int = 47;  /* / */
const GLFW_KEY_0                : int = 48;
const GLFW_KEY_1                : int = 49;
const GLFW_KEY_2                : int = 50;
const GLFW_KEY_3                : int = 51;
const GLFW_KEY_4                : int = 52;
const GLFW_KEY_5                : int = 53;
const GLFW_KEY_6                : int = 54;
const GLFW_KEY_7                : int = 55;
const GLFW_KEY_8                : int = 56;
const GLFW_KEY_9                : int = 57;
const GLFW_KEY_SEMICOLON        : int = 59;  /* ; */
const GLFW_KEY_EQUAL            : int = 61;  /* = */
const GLFW_KEY_A                : int = 65;
const GLFW_KEY_B                : int = 66;
const GLFW_KEY_C                : int = 67;
const GLFW_KEY_D                : int = 68;
const GLFW_KEY_E                : int = 69;
const GLFW_KEY_F                : int = 70;
const GLFW_KEY_G                : int = 71;
const GLFW_KEY_H                : int = 72;
const GLFW_KEY_I                : int = 73;
const GLFW_KEY_J                : int = 74;
const GLFW_KEY_K                : int = 75;
const GLFW_KEY_L                : int = 76;
const GLFW_KEY_M                : int = 77;
const GLFW_KEY_N                : int = 78;
const GLFW_KEY_O                : int = 79;
const GLFW_KEY_P                : int = 80;
const GLFW_KEY_Q                : int = 81;
const GLFW_KEY_R                : int = 82;
const GLFW_KEY_S                : int = 83;
const GLFW_KEY_T                : int = 84;
const GLFW_KEY_U                : int = 85;
const GLFW_KEY_V                : int = 86;
const GLFW_KEY_W                : int = 87;
const GLFW_KEY_X                : int = 88;
const GLFW_KEY_Y                : int = 89;
const GLFW_KEY_Z                : int = 90;
const GLFW_KEY_LEFT_BRACKET     : int = 91;  /* [ */
const GLFW_KEY_BACKSLASH        : int = 92;  /* \ */
const GLFW_KEY_RIGHT_BRACKET    : int = 93;  /* ] */
const GLFW_KEY_GRAVE_ACCENT     : int = 96;  /* ` */
const GLFW_KEY_WORLD_1          : int = 161; /* non-US #1 */
const GLFW_KEY_WORLD_2          : int = 162; /* non-US #2 */

/* Function keys */
const GLFW_KEY_ESCAPE           : int = 256;
const GLFW_KEY_ENTER            : int = 257;
const GLFW_KEY_TAB              : int = 258;
const GLFW_KEY_BACKSPACE        : int = 259;
const GLFW_KEY_INSERT           : int = 260;
const GLFW_KEY_DELETE           : int = 261;
const GLFW_KEY_RIGHT            : int = 262;
const GLFW_KEY_LEFT             : int = 263;
const GLFW_KEY_DOWN             : int = 264;
const GLFW_KEY_UP               : int = 265;
const GLFW_KEY_PAGE_UP          : int = 266;
const GLFW_KEY_PAGE_DOWN        : int = 267;
const GLFW_KEY_HOME             : int = 268;
const GLFW_KEY_END              : int = 269;
const GLFW_KEY_CAPS_LOCK        : int = 280;
const GLFW_KEY_SCROLL_LOCK      : int = 281;
const GLFW_KEY_NUM_LOCK         : int = 282;
const GLFW_KEY_PRINT_SCREEN     : int = 283;
const GLFW_KEY_PAUSE            : int = 284;
const GLFW_KEY_F1               : int = 290;
const GLFW_KEY_F2               : int = 291;
const GLFW_KEY_F3               : int = 292;
const GLFW_KEY_F4               : int = 293;
const GLFW_KEY_F5               : int = 294;
const GLFW_KEY_F6               : int = 295;
const GLFW_KEY_F7               : int = 296;
const GLFW_KEY_F8               : int = 297;
const GLFW_KEY_F9               : int = 298;
const GLFW_KEY_F10              : int = 299;
const GLFW_KEY_F11              : int = 300;
const GLFW_KEY_F12              : int = 301;
const GLFW_KEY_F13              : int = 302;
const GLFW_KEY_F14              : int = 303;
const GLFW_KEY_F15              : int = 304;
const GLFW_KEY_F16              : int = 305;
const GLFW_KEY_F17              : int = 306;
const GLFW_KEY_F18              : int = 307;
const GLFW_KEY_F19              : int = 308;
const GLFW_KEY_F20              : int = 309;
const GLFW_KEY_F21              : int = 310;
const GLFW_KEY_F22              : int = 311;
const GLFW_KEY_F23              : int = 312;
const GLFW_KEY_F24              : int = 313;
const GLFW_KEY_F25              : int = 314;
const GLFW_KEY_KP_0             : int = 320;
const GLFW_KEY_KP_1             : int = 321;
const GLFW_KEY_KP_2             : int = 322;
const GLFW_KEY_KP_3             : int = 323;
const GLFW_KEY_KP_4             : int = 324;
const GLFW_KEY_KP_5             : int = 325;
const GLFW_KEY_KP_6             : int = 326;
const GLFW_KEY_KP_7             : int = 327;
const GLFW_KEY_KP_8             : int = 328;
const GLFW_KEY_KP_9             : int = 329;
const GLFW_KEY_KP_DECIMAL       : int = 330;
const GLFW_KEY_KP_DIVIDE        : int = 331;
const GLFW_KEY_KP_MULTIPLY      : int = 332;
const GLFW_KEY_KP_SUBTRACT      : int = 333;
const GLFW_KEY_KP_ADD           : int = 334;
const GLFW_KEY_KP_ENTER         : int = 335;
const GLFW_KEY_KP_EQUAL         : int = 336;
const GLFW_KEY_LEFT_SHIFT       : int = 340;
const GLFW_KEY_LEFT_CONTROL     : int = 341;
const GLFW_KEY_LEFT_ALT         : int = 342;
const GLFW_KEY_LEFT_SUPER       : int = 343;
const GLFW_KEY_RIGHT_SHIFT      : int = 344;
const GLFW_KEY_RIGHT_CONTROL    : int = 345;
const GLFW_KEY_RIGHT_ALT        : int = 346;
const GLFW_KEY_RIGHT_SUPER      : int = 347;
const GLFW_KEY_MENU             : int = 348;
const GLFW_KEY_LAST             : int = GLFW_KEY_MENU;

/* GLFW 2.x key name aliases (deprecated) */
const GLFW_KEY_ESC              : int = GLFW_KEY_ESCAPE;
const GLFW_KEY_DEL              : int = GLFW_KEY_DELETE;
const GLFW_KEY_PAGEUP           : int = GLFW_KEY_PAGE_UP;
const GLFW_KEY_PAGEDOWN         : int = GLFW_KEY_PAGE_DOWN;
const GLFW_KEY_KP_NUM_LOCK      : int = GLFW_KEY_NUM_LOCK;
const GLFW_KEY_LCTRL            : int = GLFW_KEY_LEFT_CONTROL;
const GLFW_KEY_LSHIFT           : int = GLFW_KEY_LEFT_SHIFT;
const GLFW_KEY_LALT             : int = GLFW_KEY_LEFT_ALT;
const GLFW_KEY_LSUPER           : int = GLFW_KEY_LEFT_SUPER;
const GLFW_KEY_RCTRL            : int = GLFW_KEY_RIGHT_CONTROL;
const GLFW_KEY_RSHIFT           : int = GLFW_KEY_RIGHT_SHIFT;
const GLFW_KEY_RALT             : int = GLFW_KEY_RIGHT_ALT;
const GLFW_KEY_RSUPER           : int = GLFW_KEY_RIGHT_SUPER;

/* Mouse button aliases */
const GLFW_MOUSE_BUTTON_LEFT    : int = GLFW_MOUSE_BUTTON_1;
const GLFW_MOUSE_BUTTON_RIGHT   : int = GLFW_MOUSE_BUTTON_2;
const GLFW_MOUSE_BUTTON_MIDDLE  : int = GLFW_MOUSE_BUTTON_3;

/* Mouse button definitions */
const GLFW_MOUSE_BUTTON_1       : int = 0;
const GLFW_MOUSE_BUTTON_2       : int = 1;
const GLFW_MOUSE_BUTTON_3       : int = 2;
const GLFW_MOUSE_BUTTON_4       : int = 3;
const GLFW_MOUSE_BUTTON_5       : int = 4;
const GLFW_MOUSE_BUTTON_6       : int = 5;
const GLFW_MOUSE_BUTTON_7       : int = 6;
const GLFW_MOUSE_BUTTON_8       : int = 7;
const GLFW_MOUSE_BUTTON_LAST    : int = GLFW_MOUSE_BUTTON_8;

/* Joystick identifiers */
const GLFW_JOYSTICK_1           : int = 0;
const GLFW_JOYSTICK_2           : int = 1;
const GLFW_JOYSTICK_3           : int = 2;
const GLFW_JOYSTICK_4           : int = 3;
const GLFW_JOYSTICK_5           : int = 4;
const GLFW_JOYSTICK_6           : int = 5;
const GLFW_JOYSTICK_7           : int = 6;
const GLFW_JOYSTICK_8           : int = 7;
const GLFW_JOYSTICK_9           : int = 8;
const GLFW_JOYSTICK_10          : int = 9;
const GLFW_JOYSTICK_11          : int = 10;
const GLFW_JOYSTICK_12          : int = 11;
const GLFW_JOYSTICK_13          : int = 12;
const GLFW_JOYSTICK_14          : int = 13;
const GLFW_JOYSTICK_15          : int = 14;
const GLFW_JOYSTICK_16          : int = 15;
const GLFW_JOYSTICK_LAST        : int = GLFW_JOYSTICK_16;

/*************************************************************************
 * Other definitions
 *************************************************************************/

/* glfwCreateWindow modes */
const GLFW_WINDOWED                     : int = 0x00010001;
const GLFW_FULLSCREEN                   : int = 0x00010002;

/* glfwGetWindowParam tokens */
const GLFW_ACTIVE                       : int = 0x00020001;
const GLFW_ICONIFIED                    : int = 0x00020002;
const GLFW_CLOSE_REQUESTED              : int = 0x00020003;
const GLFW_OPENGL_REVISION              : int = 0x00020004;

/* glfwWindowHint tokens */
const GLFW_RED_BITS                     : int = 0x00021000;
const GLFW_GREEN_BITS                   : int = 0x00021001;
const GLFW_BLUE_BITS                    : int = 0x00021002;
const GLFW_ALPHA_BITS                   : int = 0x00021003;
const GLFW_DEPTH_BITS                   : int = 0x00021004;
const GLFW_STENCIL_BITS                 : int = 0x00021005;
const GLFW_REFRESH_RATE                 : int = 0x00021006;
const GLFW_ACCUM_RED_BITS               : int = 0x00021007;
const GLFW_ACCUM_GREEN_BITS             : int = 0x00021008;
const GLFW_ACCUM_BLUE_BITS              : int = 0x00021009;
const GLFW_ACCUM_ALPHA_BITS             : int = 0x0002100A;
const GLFW_AUX_BUFFERS                  : int = 0x0002100B;
const GLFW_STEREO                       : int = 0x0002100C;
const GLFW_WINDOW_RESIZABLE             : int = 0x0002100D;
const GLFW_FSAA_SAMPLES                 : int = 0x0002100E;

/* The following constants are used with both glfwGetWindowParam
 * and glfwWindowHint
 */
const GLFW_OPENGL_VERSION_MAJOR         : int = 0x0002100F;
const GLFW_OPENGL_VERSION_MINOR         : int = 0x00021010;
const GLFW_OPENGL_FORWARD_COMPAT        : int = 0x00021011;
const GLFW_OPENGL_DEBUG_CONTEXT         : int = 0x00021012;
const GLFW_OPENGL_PROFILE               : int = 0x00021013;
const GLFW_OPENGL_ROBUSTNESS            : int = 0x00021014;

/* GLFW_OPENGL_ROBUSTNESS mode tokens */
const GLFW_OPENGL_NO_ROBUSTNESS         : int = 0x00000000;
const GLFW_OPENGL_NO_RESET_NOTIFICATION : int = 0x00000001;
const GLFW_OPENGL_LOSE_CONTEXT_ON_RESET : int = 0x00000002;

/* GLFW_OPENGL_PROFILE bit tokens */
const GLFW_OPENGL_NO_PROFILE            : int = 0x00000000;
const GLFW_OPENGL_CORE_PROFILE          : int = 0x00000001;
const GLFW_OPENGL_COMPAT_PROFILE        : int = 0x00000002;
const GLFW_OPENGL_ES2_PROFILE           : int = 0x00000004;

/* glfwGetInputMode/glfwSetInputMode tokens */
const GLFW_CURSOR_MODE                  : int = 0x00030001;
const GLFW_STICKY_KEYS                  : int = 0x00030002;
const GLFW_STICKY_MOUSE_BUTTONS         : int = 0x00030003;
const GLFW_SYSTEM_KEYS                  : int = 0x00030004;
const GLFW_KEY_REPEAT                   : int = 0x00030005;

/* GLFW_CURSOR_MODE values */
const GLFW_CURSOR_NORMAL                : int = 0x00040001;
const GLFW_CURSOR_HIDDEN                : int = 0x00040002;
const GLFW_CURSOR_CAPTURED              : int = 0x00040003;

/* glfwGetJoystickParam tokens */
const GLFW_PRESENT                      : int = 0x00050001;
const GLFW_AXES                         : int = 0x00050002;
const GLFW_BUTTONS                      : int = 0x00050003;

/* glfwGetError/glfwErrorString tokens */
const GLFW_NO_ERROR                     : int = 0;
const GLFW_NOT_INITIALIZED              : int = 0x00070001;
const GLFW_NO_CURRENT_CONTEXT           : int = 0x00070002;
const GLFW_INVALID_ENUM                 : int = 0x00070003;
const GLFW_INVALID_VALUE                : int = 0x00070004;
const GLFW_OUT_OF_MEMORY                : int = 0x00070005;
const GLFW_OPENGL_UNAVAILABLE           : int = 0x00070006;
const GLFW_VERSION_UNAVAILABLE          : int = 0x00070007;
const GLFW_PLATFORM_ERROR               : int = 0x00070008;
const GLFW_WINDOW_NOT_ACTIVE            : int = 0x00070009;
const GLFW_FORMAT_UNAVAILABLE           : int = 0x0007000A;

/* Gamma ramps */
const GLFW_GAMMA_RAMP_SIZE              : int = 256;

/*************************************************************************
 * Typedefs
 *************************************************************************/

// /* OpenGL function pointer type */
// Will have to be changed once we can do external C callbacks nicely
type GLFWglproc = *u8;              // typedef void (*GLFWglproc)(void);

/* Window handle type */
type GLFWwindowPtr = *c_void;      // typedef void* GLFWwindow;

// Wraps * pointer in a struct for safety 
struct GLFWwindow {
    mut ptr: GLFWwindowPtr
}

/* Function pointer types */
// Will have to be changed once we can do external C callbacks nicely
type GLFWerrorfun           = *u8;  // typedef void (* GLFWerrorfun)(int,const char*);
type GLFWwindowsizefun      = *u8;  // typedef void (* GLFWwindowsizefun)(GLFWwindow,int,int);
type GLFWwindowclosefun     = *u8;  // typedef int  (* GLFWwindowclosefun)(GLFWwindow);
type GLFWwindowrefreshfun   = *u8;  // typedef void (* GLFWwindowrefreshfun)(GLFWwindow);
type GLFWwindowfocusfun     = *u8;  // typedef void (* GLFWwindowfocusfun)(GLFWwindow,int);
type GLFWwindowiconifyfun   = *u8;  // typedef void (* GLFWwindowiconifyfun)(GLFWwindow,int);
type GLFWmousebuttonfun     = *u8;  // typedef void (* GLFWmousebuttonfun)(GLFWwindow,int,int);
type GLFWcursorposfun       = *u8;  // typedef void (* GLFWcursorposfun)(GLFWwindow,int,int);
type GLFWcursorenterfun     = *u8;  // typedef void (* GLFWcursorenterfun)(GLFWwindow,int);
type GLFWscrollfun          = *u8;  // typedef void (* GLFWscrollfun)(GLFWwindow,double,double);
type GLFWkeyfun             = *u8;  // typedef void (* GLFWkeyfun)(GLFWwindow,int,int);
type GLFWcharfun            = *u8;  // typedef void (* GLFWcharfun)(GLFWwindow,int);

/* The video mode structure used by glfwGetVideoModes */
struct GLFWvidmode {
    width      : c_int,
    height     : c_int,
    redBits    : c_int,
    blueBits   : c_int,
    greenBits  : c_int
}

/* Gamma ramp */
/* See https://github.com/mozilla/rust/issues/3469
struct GLFWgammaramp {
    red     : [c_ushort * GLFW_GAMMA_RAMP_SIZE],      // unsigned short red[GLFW_GAMMA_RAMP_SIZE];
    green   : [c_ushort * GLFW_GAMMA_RAMP_SIZE],      // unsigned short green[GLFW_GAMMA_RAMP_SIZE];
    blue    : [c_ushort * GLFW_GAMMA_RAMP_SIZE]       // unsigned short blue[GLFW_GAMMA_RAMP_SIZE];
}
*/
struct GLFWgammaramp {
    red     : [c_ushort * 256],      // unsigned short red[GLFW_GAMMA_RAMP_SIZE];
    green   : [c_ushort * 256],      // unsigned short green[GLFW_GAMMA_RAMP_SIZE];
    blue    : [c_ushort * 256]       // unsigned short blue[GLFW_GAMMA_RAMP_SIZE];
}


extern mod glfw3 {
    
    /* GLFW initialization, termination and version querying */
    fn glfwInit() -> c_int;                                                             // GLFWAPI int  glfwInit(void);
    fn glfwTerminate();                                                                 // GLFWAPI void glfwTerminate(void);
    fn glfwGetVersion(major: &mut c_int, minor: &mut c_int, rev: &mut c_int);           // GLFWAPI void glfwGetVersion(int* major, int* minor, int* rev);
    fn glfwGetVersionString() -> *c_char;                                               // GLFWAPI const char* glfwGetVersionString(void);

    /* Error handling */
    fn glfwGetError() -> c_int;                                                         // GLFWAPI int glfwGetError(void);
    fn glfwErrorString(error: c_int) -> *c_char;                                        // GLFWAPI const char* glfwErrorString(int error);
    fn glfwSetErrorCallback(cbfun: GLFWerrorfun);                                       // GLFWAPI void glfwSetErrorCallback(GLFWerrorfun cbfun);
    
    /* Video mode functions */
    fn glfwGetVideoModes(count: &mut c_int) -> *GLFWvidmode;                            // GLFWAPI GLFWvidmode* glfwGetVideoModes(int* count);
    fn glfwGetDesktopMode(mode: &mut GLFWvidmode);                                      // GLFWAPI void glfwGetDesktopMode(GLFWvidmode* mode);
    
    /* Gamma ramp functions */
    fn glfwSetGamma(gamma: c_float);                                                    // GLFWAPI void glfwSetGamma(float gamma);
    fn glfwGetGammaRamp(ramp: &mut GLFWgammaramp);                                      // GLFWAPI void glfwGetGammaRamp(GLFWgammaramp* ramp);
    fn glfwSetGammaRamp(ramp: &mut GLFWgammaramp);                                      // GLFWAPI void glfwSetGammaRamp(const GLFWgammaramp* ramp);
    
    /* Window handling */
    fn glfwWindowHint(target: c_int, hint: c_int);                                      // GLFWAPI void glfwWindowHint(int target, int hint);
    fn glfwCreateWindow(width: c_int, height: c_int, mode: c_int, title: *c_char, share: GLFWwindowPtr) -> GLFWwindowPtr; // GLFWAPI GLFWwindow glfwCreateWindow(int width, int height, int mode, const char* title, GLFWwindow share);
    fn glfwDestroyWindow(window: GLFWwindowPtr);                                        // GLFWAPI void glfwDestroyWindow(GLFWwindow window);
    fn glfwSetWindowTitle(window: GLFWwindowPtr, title: *c_char);                       // GLFWAPI void glfwSetWindowTitle(GLFWwindow window, const char* title);
    fn glfwGetWindowSize(window: GLFWwindowPtr, width: &mut c_int, height: &mut c_int   // GLFWAPI void glfwGetWindowSize(GLFWwindow window, int* width, int* height);
    fn glfwSetWindowSize(window: GLFWwindowPtr, width: c_int, height: c_int);           // GLFWAPI void glfwSetWindowSize(GLFWwindow window, int width, int height);
    fn glfwGetWindowPos(window: GLFWwindowPtr, xpos: &mut c_int, ypos: &mut c_int);     // GLFWAPI void glfwGetWindowPos(GLFWwindow window, int* xpos, int* ypos);
    fn glfwSetWindowPos(window: GLFWwindowPtr, xpos: c_int, ypos: c_int);               // GLFWAPI void glfwSetWindowPos(GLFWwindow window, int xpos, int ypos);
    fn glfwIconifyWindow(window: GLFWwindowPtr);                                        // GLFWAPI void glfwIconifyWindow(GLFWwindow window);
    fn glfwRestoreWindow(window: GLFWwindowPtr);                                        // GLFWAPI void glfwRestoreWindow(GLFWwindow window);
    fn glfwGetWindowParam(window: GLFWwindowPtr, param: c_int) -> c_int;                // GLFWAPI int  glfwGetWindowParam(GLFWwindow window, int param);
    fn glfwSetWindowUserPointer(window: GLFWwindowPtr, pointer: *c_void);               // GLFWAPI void glfwSetWindowUserPointer(GLFWwindow window, void* pointer);
    fn glfwGetWindowUserPointer(window: GLFWwindowPtr) -> *c_void;                      // GLFWAPI void* glfwGetWindowUserPointer(GLFWwindow window);
    fn glfwSetWindowSizeCallback(cbfun: GLFWwindowsizefun);                             // GLFWAPI void glfwSetWindowSizeCallback(GLFWwindowsizefun cbfun);
    fn glfwSetWindowCloseCallback(cbfun: GLFWwindowclosefun);                           // GLFWAPI void glfwSetWindowCloseCallback(GLFWwindowclosefun cbfun);
    fn glfwSetWindowRefreshCallback(cbfun: GLFWwindowrefreshfun);                       // GLFWAPI void glfwSetWindowRefreshCallback(GLFWwindowrefreshfun cbfun);
    fn glfwSetWindowFocusCallback(cbfun: GLFWwindowfocusfun);                           // GLFWAPI void glfwSetWindowFocusCallback(GLFWwindowfocusfun cbfun);
    fn glfwSetWindowIconifyCallback(cbfun: GLFWwindowiconifyfun);                       // GLFWAPI void glfwSetWindowIconifyCallback(GLFWwindowiconifyfun cbfun);

    /* Event handling */
    fn glfwPollEvents();                                                                // GLFWAPI void glfwPollEvents(void);
    fn glfwWaitEvents();                                                                // GLFWAPI void glfwWaitEvents(void);

    /* Input handling */
    fn glfwGetInputMode(window: GLFWwindowPtr, mode: c_int) -> c_int;                   // GLFWAPI int  glfwGetInputMode(GLFWwindow window, int mode);
    fn glfwSetInputMode(window: GLFWwindowPtr, mode: c_int, value: c_int);              // GLFWAPI void glfwSetInputMode(GLFWwindow window, int mode, int value);
    fn glfwGetKey(window: GLFWwindowPtr, key: c_int) -> c_int;                          // GLFWAPI int  glfwGetKey(GLFWwindow window, int key);
    fn glfwGetMouseButton(window: GLFWwindowPtr, button: c_int) -> c_int;               // GLFWAPI int  glfwGetMouseButton(GLFWwindow window, int button);
    fn glfwGetCursorPos(window: GLFWwindowPtr, xpos: &mut c_int, ypos: &mut c_int);     // GLFWAPI void glfwGetCursorPos(GLFWwindow window, int* xpos, int* ypos);
    fn glfwSetCursorPos(window: GLFWwindowPtr, xpos: c_int, ypos: c_int);               // GLFWAPI void glfwSetCursorPos(GLFWwindow window, int xpos, int ypos);
    fn glfwGetScrollOffset(window: GLFWwindowPtr, xoffset: &mut c_double, yoffset: &mut c_double); // GLFWAPI void glfwGetScrollOffset(GLFWwindow window, double* xoffset, double* yoffset);
    fn glfwSetKeyCallback(cbfun: GLFWkeyfun);                                           // GLFWAPI void glfwSetKeyCallback(GLFWkeyfun cbfun);
    fn glfwSetCharCallback(cbfun: GLFWcharfun);                                         // GLFWAPI void glfwSetCharCallback(GLFWcharfun cbfun);
    fn glfwSetMouseButtonCallback(cbfun: GLFWmousebuttonfun);                           // GLFWAPI void glfwSetMouseButtonCallback(GLFWmousebuttonfun cbfun);
    fn glfwSetCursorPosCallback(cbfun: GLFWcursorposfun);                               // GLFWAPI void glfwSetCursorPosCallback(GLFWcursorposfun cbfun);
    fn glfwSetCursorEnterCallback(cbfun: GLFWcursorenterfun);                           // GLFWAPI void glfwSetCursorEnterCallback(GLFWcursorenterfun cbfun);
    fn glfwSetScrollCallback(cbfun: GLFWscrollfun);                                     // GLFWAPI void glfwSetScrollCallback(GLFWscrollfun cbfun);
    
    /* Joystick input */
    fn glfwGetJoystickParam(joy: c_int, param: c_int) -> c_int;                         // GLFWAPI int glfwGetJoystickParam(int joy, int param);
    fn glfwGetJoystickAxes(joy: c_int, axes: *c_float, numaxes: c_int) -> c_int;        // GLFWAPI int glfwGetJoystickAxes(int joy, float* axes, int numaxes);
    fn glfwGetJoystickButtons(joy: c_int, buttons: *c_uchar, numbuttons: c_int) -> c_int; // GLFWAPI int glfwGetJoystickButtons(int joy, unsigned char* buttons, int numbuttons);
    
    /* Clipboard */
    fn glfwSetClipboardString(window: GLFWwindowPtr, string: *c_char);                  // GLFWAPI void glfwSetClipboardString(GLFWwindow window, const char* string);
    fn glfwGetClipboardString(window: GLFWwindowPtr) -> *c_char;                        // GLFWAPI const char* glfwGetClipboardString(GLFWwindow window);
    
    /* Time */
    fn glfwGetTime() -> c_double;                                                       // GLFWAPI double glfwGetTime(void);
    fn glfwSetTime(time: c_double);                                                     // GLFWAPI void   glfwSetTime(double time);
    
    /* OpenGL support */
    fn glfwMakeContextCurrent(window: GLFWwindowPtr);                                   // GLFWAPI void glfwMakeContextCurrent(GLFWwindow window);
    fn glfwGetCurrentContext() -> GLFWwindowPtr;                                        // GLFWAPI GLFWwindow glfwGetCurrentContext(void);
    fn glfwSwapBuffers(window: GLFWwindowPtr);                                          // GLFWAPI void  glfwSwapBuffers(GLFWwindow window);
    fn glfwSwapInterval(interval: c_int);                                               // GLFWAPI void  glfwSwapInterval(int interval);
    fn glfwExtensionSupported(extension: *c_char) -> c_int;                             // GLFWAPI int   glfwExtensionSupported(const char* extension);
    fn glfwGetProcAddress(procname: *c_char) -> GLFWglproc;                             // GLFWAPI GLFWglproc glfwGetProcAddress(const char* procname);
    fn glfwCopyContext(src: GLFWwindowPtr, dst: GLFWwindowPtr, mask: c_ulong);          // GLFWAPI void  glfwCopyContext(GLFWwindow src, GLFWwindow dst, unsigned long mask);
}


/* GLFW initialization, termination and version querying */
    
fn glfwInit() -> int {    
    unsafe { glfw3::glfwInit() as int }
}

fn glfwTerminate() {
    unsafe { glfw3::glfwTerminate(); }  
}

fn glfwGetVersion() -> (int, int, int) {
    let mut major = 0, minor = 0, rev = 0;
    unsafe { glfw3::glfwGetVersion(&mut major, &mut minor, &mut rev); }
    return (major as int, minor as int, rev as int);
}

fn glfwGetVersionString() -> ~str {
    unsafe { str::raw::from_c_str(glfw3::glfwGetVersionString()) }
}

/* Error handling */

fn glfwGetError() -> int {
    unsafe { glfw3::glfwGetError() as int }
}

fn glfwErrorString(error: int) -> ~str {
    unsafe { str::raw::from_c_str(glfw3::glfwErrorString(error as c_int)) }
}

// TODO: glfwSetErrorCallback

/* Video mode functions */

fn glfwGetVideoModes() -> ~[GLFWvidmode] {
    let mut count: c_int = 0;
    let mut mode_ptr: *GLFWvidmode;
    let mut modes: ~[GLFWvidmode];
    unsafe {
        mode_ptr = glfw3::glfwGetVideoModes(&mut count);
        modes = from_buf(mode_ptr, count as uint);
    }
    return modes;
}

fn glfwGetDesktopMode() -> GLFWvidmode {
    let mut mode = GLFWvidmode { width: 0, height : 0, redBits: 0, blueBits: 0, greenBits: 0 }; // initialisation is necessary
    unsafe { glfw3::glfwGetDesktopMode(&mut mode); }
    return mode;
}

/* Gamma ramp functions */

fn glfwSetGamma(gamma: float) {
    unsafe { glfw3::glfwSetGamma(gamma as c_float); }
}

fn glfwGetGammaRamp() -> GLFWgammaramp {
    let mut ramp = GLFWgammaramp { red: [0, ..256], green: [0, ..256], blue: [0, ..256] }; // initialisation is necessary
    unsafe { glfw3::glfwGetGammaRamp(&mut ramp); }
    return ramp;
}

fn glfwSetGammaRamp(ramp: &mut GLFWgammaramp) {
    unsafe { glfw3::glfwSetGammaRamp(ramp) }
}

/* Window handling */

fn glfwWindowHint(target: int, hint: int) {
    unsafe { glfw3::glfwWindowHint(target as c_int, hint as c_int); }
}

fn glfwCreateWindow(width: int, height: int, mode: int, title: ~str) -> GLFWwindow {
    unsafe {
        GLFWwindow {
            ptr: glfw3::glfwCreateWindow(width as c_int,
                                         height as c_int,
                                         mode as c_int,
                                         str::as_c_str(title, |p| {p}),
                                         ptr::null())
        }
    }
}

fn glfwCreateWindowShared(width: int, height: int, mode: int, title: ~str, share: &mut GLFWwindow) -> GLFWwindow {
    unsafe {
        GLFWwindow {
            ptr: glfw3::glfwCreateWindow(width as c_int,
                                         height as c_int,
                                         mode as c_int,
                                         str::as_c_str(title, |p| {p}),
                                         share.ptr)
        }
    }
}

fn glfwDestroyWindow(window: &mut GLFWwindow) {
    unsafe {
        glfw3::glfwDestroyWindow(window.ptr)
    }
}

fn glfwSetWindowTitle(window: &GLFWwindow, title: ~str) {
    unsafe {
        glfw3::glfwSetWindowTitle(window.ptr, str::as_c_str(title, |p| {p}))
    }
}

fn glfwGetWindowSize(window: &GLFWwindow) -> (int, int) {
    let mut width = 0, height = 0;
    unsafe { glfw3::glfwGetWindowSize(window.ptr, &mut width, &mut height)}
    return (width as int, height as int);
}

fn glfwSetWindowSize(window: &GLFWwindow, width: int, height: int) {
    unsafe { glfw3::glfwSetWindowSize(window.ptr, width as c_int, height as c_int); }
}

fn glfwGetWindowPos(window: &GLFWwindow) -> (int, int) {
    let mut xpos = 0, ypos = 0;
    unsafe { glfw3::glfwGetWindowPos(window.ptr, &mut xpos, &mut ypos); }
    return (xpos as int, ypos as int);
}

fn glfwSetWindowPos(window: &GLFWwindow, xpos: int, ypos: int) {
    unsafe { glfw3::glfwSetWindowPos(window.ptr, xpos as c_int, ypos as c_int); }
}

fn glfwIconifyWindow(window: &GLFWwindow) {
    unsafe { glfw3::glfwIconifyWindow(window.ptr); }
}

fn glfwRestoreWindow(window: &GLFWwindow) {
    unsafe { glfw3::glfwRestoreWindow(window.ptr); }
}

fn glfwGetWindowParam(window: &GLFWwindow, param: int) -> int {
    unsafe { glfw3::glfwGetWindowParam(window.ptr, param as c_int) as int }
}

// TODO: glfwSetWindowUserPointer
// TODO: glfwGetWindowUserPointer
// TODO: glfwSetWindowSizeCallback
// TODO: glfwSetWindowCloseCallback
// TODO: glfwSetWindowRefreshCallback
// TODO: glfwSetWindowFocusCallback
// TODO: glfwSetWindowIconifyCallback

/* Event handling */

fn glfwPollEvents() {    
    unsafe { glfw3::glfwPollEvents(); }
}
 
fn glfwWaitEvents() {    
    unsafe { glfw3::glfwWaitEvents(); }
}

/* Input handling */

fn glfwGetInputMode(window: &GLFWwindow, mode: int) -> int {
    unsafe { glfw3::glfwGetInputMode(window.ptr, mode as c_int) as int }
}

fn glfwSetInputMode(window: &GLFWwindow, mode: int, value: int) {
    unsafe { glfw3::glfwSetInputMode(window.ptr, mode as c_int, value as c_int); }
}

fn glfwGetKey(window: &GLFWwindow, key: int) -> int {
    unsafe { glfw3::glfwGetKey(window.ptr, key as c_int) as int }
}

fn glfwGetMouseButton(window: &GLFWwindow, button: int) -> int {
    unsafe { glfw3::glfwGetMouseButton(window.ptr, button as c_int) as int }
}

fn glfwGetCursorPos(window: &GLFWwindow) -> (int, int) {
    let mut xpos = 0, ypos = 0;
    unsafe { glfw3::glfwGetCursorPos(window.ptr, &mut xpos, &mut ypos); }
    return (xpos as int, ypos as int);
}

fn glfwSetCursorPos(window: &GLFWwindow, xpos: int, ypos: int) {
    unsafe { glfw3::glfwSetCursorPos(window.ptr, xpos as c_int, ypos as c_int); }
}

fn glfwGetScrollOffset(window: &GLFWwindow) -> (f64, f64) {
    let mut xpos = 0f64, ypos = 0f64;
    unsafe { glfw3::glfwGetScrollOffset(window.ptr, &mut xpos, &mut ypos); }
    return (xpos as f64, ypos as f64);
}

// TODO: glfwSetKeyCallback
// TODO: glfwSetCharCallback
// TODO: glfwSetMouseButtonCallback
// TODO: glfwSetCursorPosCallback
// TODO: glfwSetCursorEnterCallback
// TODO: glfwSetScrollCallback

/* Joystick input */

fn glfwGetJoystickParam(joy: int, param: int) -> int {
    unsafe { glfw3::glfwGetJoystickParam(joy as c_int, param as c_int) as int }
}

/**
 * Somebody with a joystick will have to test this. I don't have one, unfortunately.
 *
 * I'm also unsure about whether I've got my pointers right. Use at your own risk - sorry!
 */
fn glfwGetJoystickAxes(joy: int, numaxes: int) -> Option<~[float]> {
    let axes: ~[float];
    
    unsafe {
        let axes_ptr: *c_float = ptr::null();
        let n = glfw3::glfwGetJoystickAxes(joy as c_int, axes_ptr, numaxes as c_int) as uint;
        axes = from_buf(axes_ptr, n).map(|a| { a as float });   // Could be inefficient
    }
    
    if numaxes > 0 { Some(axes) }
    else           { None }
}

/**
 * Somebody with a joystick will have to test this. I don't have one, unfortunately.
 *
 * I'm also unsure about whether I've got my pointers right. Use at your own risk - sorry!
 */
fn glfwGetJoystickButtons(joy: int, numbuttons: int) -> Option<~[char]> {
    let buttons: ~[char];
    
    unsafe {
        let buttons_ptr: *c_uchar = ptr::null();
        let n = glfw3::glfwGetJoystickButtons(joy as c_int, buttons_ptr, numbuttons as c_int) as uint;
        buttons = from_buf(buttons_ptr, n).map(|a| { a as char });; // Could be inefficient
    }
    
    if numbuttons > 0 { Some(buttons) }
    else              { None }
}

/* Clipboard */

fn glfwSetClipboardString(window: &GLFWwindow, string: ~str) {
    unsafe {
        do str::as_c_str(string) |c_string| {
            glfw3::glfwSetClipboardString(window.ptr, c_string);
        }
    }
}

fn glfwGetClipboardString(window: &GLFWwindow) -> ~str {
    unsafe { str::raw::from_c_str(glfw3::glfwGetClipboardString(window.ptr)) }
}

/* Time */

fn glfwGetTime() -> f64 {
    unsafe { glfw3::glfwGetTime() as f64 }
}

fn glfwSetTime(time: f64) {
    unsafe { glfw3::glfwSetTime(time as c_double); }
}

/* OpenGL support */

fn glfwMakeContextCurrent(window: &GLFWwindow) {
    unsafe { glfw3::glfwMakeContextCurrent(window.ptr); }
}

fn glfwGetCurrentContext() -> GLFWwindow {
    unsafe { GLFWwindow { ptr: glfw3::glfwGetCurrentContext() } }
}

fn glfwSwapBuffers(window: &GLFWwindow) {
    unsafe { glfw3::glfwSwapBuffers(window.ptr); }
}

fn glfwSwapInterval(interval: int) {
    unsafe { glfw3::glfwSwapInterval(interval as c_int); }
}

fn glfwExtensionSupported(extension: ~str) -> int {
    unsafe {
        do str::as_c_str(extension) |c_extension| {
            glfw3::glfwExtensionSupported(c_extension) as int
        }
    }
}

fn glfwGetProcAddress(procname: ~str) -> GLFWglproc {
    unsafe { glfw3::glfwGetProcAddress(str::as_c_str(procname, |p| {p})) }
}

fn glfwCopyContext(src: &GLFWwindow, dst: &mut GLFWwindow, mask: u32) {
    unsafe { glfw3::glfwCopyContext(src.ptr, dst.ptr, mask as c_ulong); }
}