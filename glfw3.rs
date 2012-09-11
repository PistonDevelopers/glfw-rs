/************************************************************************
 * GLFW Bindings for rust
 * 
 * All glfw functions are represented except the following
 * 
 * 
 * The thread functions : glfw has some thread control functions,
 * these are not required to use glfw, and they would almost certainly
 * not play well with rust tasks so they are left out.
 * 
 * The callback function : These will be put in with rust supports
 * C calling rust code.  Luckily glfw offers these functions as an
 * option.  You do not need them to use the library, so with this
 * file and glfw installed you can set up a useful opengl context 
 * with rust 0.1!
 * 
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
 
use std;

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
const GLFW_KEY_SPACE            : int = 32
const GLFW_KEY_APOSTROPHE       : int = 39  /* ' */
const GLFW_KEY_COMMA            : int = 44  /* , */
const GLFW_KEY_MINUS            : int = 45  /* - */
const GLFW_KEY_PERIOD           : int = 46  /* . */
const GLFW_KEY_SLASH            : int = 47  /* / */
const GLFW_KEY_0                : int = 48
const GLFW_KEY_1                : int = 49
const GLFW_KEY_2                : int = 50
const GLFW_KEY_3                : int = 51
const GLFW_KEY_4                : int = 52
const GLFW_KEY_5                : int = 53
const GLFW_KEY_6                : int = 54
const GLFW_KEY_7                : int = 55
const GLFW_KEY_8                : int = 56
const GLFW_KEY_9                : int = 57
const GLFW_KEY_SEMICOLON        : int = 59  /* ; */
const GLFW_KEY_EQUAL            : int = 61  /* = */
const GLFW_KEY_A                : int = 65
const GLFW_KEY_B                : int = 66
const GLFW_KEY_C                : int = 67
const GLFW_KEY_D                : int = 68
const GLFW_KEY_E                : int = 69
const GLFW_KEY_F                : int = 70
const GLFW_KEY_G                : int = 71
const GLFW_KEY_H                : int = 72
const GLFW_KEY_I                : int = 73
const GLFW_KEY_J                : int = 74
const GLFW_KEY_K                : int = 75
const GLFW_KEY_L                : int = 76
const GLFW_KEY_M                : int = 77
const GLFW_KEY_N                : int = 78
const GLFW_KEY_O                : int = 79
const GLFW_KEY_P                : int = 80
const GLFW_KEY_Q                : int = 81
const GLFW_KEY_R                : int = 82
const GLFW_KEY_S                : int = 83
const GLFW_KEY_T                : int = 84
const GLFW_KEY_U                : int = 85
const GLFW_KEY_V                : int = 86
const GLFW_KEY_W                : int = 87
const GLFW_KEY_X                : int = 88
const GLFW_KEY_Y                : int = 89
const GLFW_KEY_Z                : int = 90
const GLFW_KEY_LEFT_BRACKET     : int = 91  /* [ */
const GLFW_KEY_BACKSLASH        : int = 92  /* \ */
const GLFW_KEY_RIGHT_BRACKET    : int = 93  /* ] */
const GLFW_KEY_GRAVE_ACCENT     : int = 96  /* ` */
const GLFW_KEY_WORLD_1          : int = 161 /* non-US #1 */
const GLFW_KEY_WORLD_2          : int = 162 /* non-US #2 */

/* Function keys */
const GLFW_KEY_ESCAPE           : int = 256
const GLFW_KEY_ENTER            : int = 257
const GLFW_KEY_TAB              : int = 258
const GLFW_KEY_BACKSPACE        : int = 259
const GLFW_KEY_INSERT           : int = 260
const GLFW_KEY_DELETE           : int = 261
const GLFW_KEY_RIGHT            : int = 262
const GLFW_KEY_LEFT             : int = 263
const GLFW_KEY_DOWN             : int = 264
const GLFW_KEY_UP               : int = 265
const GLFW_KEY_PAGE_UP          : int = 266
const GLFW_KEY_PAGE_DOWN        : int = 267
const GLFW_KEY_HOME             : int = 268
const GLFW_KEY_END              : int = 269
const GLFW_KEY_CAPS_LOCK        : int = 280
const GLFW_KEY_SCROLL_LOCK      : int = 281
const GLFW_KEY_NUM_LOCK         : int = 282
const GLFW_KEY_PRINT_SCREEN     : int = 283
const GLFW_KEY_PAUSE            : int = 284
const GLFW_KEY_F1               : int = 290
const GLFW_KEY_F2               : int = 291
const GLFW_KEY_F3               : int = 292
const GLFW_KEY_F4               : int = 293
const GLFW_KEY_F5               : int = 294
const GLFW_KEY_F6               : int = 295
const GLFW_KEY_F7               : int = 296
const GLFW_KEY_F8               : int = 297
const GLFW_KEY_F9               : int = 298
const GLFW_KEY_F10              : int = 299
const GLFW_KEY_F11              : int = 300
const GLFW_KEY_F12              : int = 301
const GLFW_KEY_F13              : int = 302
const GLFW_KEY_F14              : int = 303
const GLFW_KEY_F15              : int = 304
const GLFW_KEY_F16              : int = 305
const GLFW_KEY_F17              : int = 306
const GLFW_KEY_F18              : int = 307
const GLFW_KEY_F19              : int = 308
const GLFW_KEY_F20              : int = 309
const GLFW_KEY_F21              : int = 310
const GLFW_KEY_F22              : int = 311
const GLFW_KEY_F23              : int = 312
const GLFW_KEY_F24              : int = 313
const GLFW_KEY_F25              : int = 314
const GLFW_KEY_KP_0             : int = 320
const GLFW_KEY_KP_1             : int = 321
const GLFW_KEY_KP_2             : int = 322
const GLFW_KEY_KP_3             : int = 323
const GLFW_KEY_KP_4             : int = 324
const GLFW_KEY_KP_5             : int = 325
const GLFW_KEY_KP_6             : int = 326
const GLFW_KEY_KP_7             : int = 327
const GLFW_KEY_KP_8             : int = 328
const GLFW_KEY_KP_9             : int = 329
const GLFW_KEY_KP_DECIMAL       : int = 330
const GLFW_KEY_KP_DIVIDE        : int = 331
const GLFW_KEY_KP_MULTIPLY      : int = 332
const GLFW_KEY_KP_SUBTRACT      : int = 333
const GLFW_KEY_KP_ADD           : int = 334
const GLFW_KEY_KP_ENTER         : int = 335
const GLFW_KEY_KP_EQUAL         : int = 336
const GLFW_KEY_LEFT_SHIFT       : int = 340
const GLFW_KEY_LEFT_CONTROL     : int = 341
const GLFW_KEY_LEFT_ALT         : int = 342
const GLFW_KEY_LEFT_SUPER       : int = 343
const GLFW_KEY_RIGHT_SHIFT      : int = 344
const GLFW_KEY_RIGHT_CONTROL    : int = 345
const GLFW_KEY_RIGHT_ALT        : int = 346
const GLFW_KEY_RIGHT_SUPER      : int = 347
const GLFW_KEY_MENU             : int = 348
const GLFW_KEY_LAST             : int = GLFW_KEY_MENU

/* GLFW 2.x key name aliases (deprecated) */
const GLFW_KEY_ESC              : int = GLFW_KEY_ESCAPE
const GLFW_KEY_DEL              : int = GLFW_KEY_DELETE
const GLFW_KEY_PAGEUP           : int = GLFW_KEY_PAGE_UP
const GLFW_KEY_PAGEDOWN         : int = GLFW_KEY_PAGE_DOWN
const GLFW_KEY_KP_NUM_LOCK      : int = GLFW_KEY_NUM_LOCK
const GLFW_KEY_LCTRL            : int = GLFW_KEY_LEFT_CONTROL
const GLFW_KEY_LSHIFT           : int = GLFW_KEY_LEFT_SHIFT
const GLFW_KEY_LALT             : int = GLFW_KEY_LEFT_ALT
const GLFW_KEY_LSUPER           : int = GLFW_KEY_LEFT_SUPER
const GLFW_KEY_RCTRL            : int = GLFW_KEY_RIGHT_CONTROL
const GLFW_KEY_RSHIFT           : int = GLFW_KEY_RIGHT_SHIFT
const GLFW_KEY_RALT             : int = GLFW_KEY_RIGHT_ALT
const GLFW_KEY_RSUPER           : int = GLFW_KEY_RIGHT_SUPER

/* Mouse button aliases */
const GLFW_MOUSE_BUTTON_LEFT    : int = GLFW_MOUSE_BUTTON_1
const GLFW_MOUSE_BUTTON_RIGHT   : int = GLFW_MOUSE_BUTTON_2
const GLFW_MOUSE_BUTTON_MIDDLE  : int = GLFW_MOUSE_BUTTON_3

/* Mouse button definitions */
const GLFW_MOUSE_BUTTON_1       : int = 0
const GLFW_MOUSE_BUTTON_2       : int = 1
const GLFW_MOUSE_BUTTON_3       : int = 2
const GLFW_MOUSE_BUTTON_4       : int = 3
const GLFW_MOUSE_BUTTON_5       : int = 4
const GLFW_MOUSE_BUTTON_6       : int = 5
const GLFW_MOUSE_BUTTON_7       : int = 6
const GLFW_MOUSE_BUTTON_8       : int = 7
const GLFW_MOUSE_BUTTON_LAST    : int = GLFW_MOUSE_BUTTON_8

/* Joystick identifiers */
const GLFW_JOYSTICK_1           : int = 0
const GLFW_JOYSTICK_2           : int = 1
const GLFW_JOYSTICK_3           : int = 2
const GLFW_JOYSTICK_4           : int = 3
const GLFW_JOYSTICK_5           : int = 4
const GLFW_JOYSTICK_6           : int = 5
const GLFW_JOYSTICK_7           : int = 6
const GLFW_JOYSTICK_8           : int = 7
const GLFW_JOYSTICK_9           : int = 8
const GLFW_JOYSTICK_10          : int = 9
const GLFW_JOYSTICK_11          : int = 10
const GLFW_JOYSTICK_12          : int = 11
const GLFW_JOYSTICK_13          : int = 12
const GLFW_JOYSTICK_14          : int = 13
const GLFW_JOYSTICK_15          : int = 14
const GLFW_JOYSTICK_16          : int = 15
const GLFW_JOYSTICK_LAST        : int = GLFW_JOYSTICK_16

/*************************************************************************
 * Other definitions
 *************************************************************************/

/* glfwCreateWindow modes */
const GLFW_WINDOWED                     : int = 0x00010001
const GLFW_FULLSCREEN                   : int = 0x00010002

/* glfwGetWindowParam tokens */
const GLFW_ACTIVE                       : int = 0x00020001
const GLFW_ICONIFIED                    : int = 0x00020002
const GLFW_CLOSE_REQUESTED              : int = 0x00020003
const GLFW_OPENGL_REVISION              : int = 0x00020004

/* glfwWindowHint tokens */
const GLFW_RED_BITS                     : int = 0x00021000
const GLFW_GREEN_BITS                   : int = 0x00021001
const GLFW_BLUE_BITS                    : int = 0x00021002
const GLFW_ALPHA_BITS                   : int = 0x00021003
const GLFW_DEPTH_BITS                   : int = 0x00021004
const GLFW_STENCIL_BITS                 : int = 0x00021005
const GLFW_REFRESH_RATE                 : int = 0x00021006
const GLFW_ACCUM_RED_BITS               : int = 0x00021007
const GLFW_ACCUM_GREEN_BITS             : int = 0x00021008
const GLFW_ACCUM_BLUE_BITS              : int = 0x00021009
const GLFW_ACCUM_ALPHA_BITS             : int = 0x0002100A
const GLFW_AUX_BUFFERS                  : int = 0x0002100B
const GLFW_STEREO                       : int = 0x0002100C
const GLFW_WINDOW_RESIZABLE             : int = 0x0002100D
const GLFW_FSAA_SAMPLES                 : int = 0x0002100E

/* The following constants are used with both glfwGetWindowParam
 * and glfwWindowHint
 */
const GLFW_OPENGL_VERSION_MAJOR         : int = 0x0002100F
const GLFW_OPENGL_VERSION_MINOR         : int = 0x00021010
const GLFW_OPENGL_FORWARD_COMPAT        : int = 0x00021011
const GLFW_OPENGL_DEBUG_CONTEXT         : int = 0x00021012
const GLFW_OPENGL_PROFILE               : int = 0x00021013
const GLFW_OPENGL_ROBUSTNESS            : int = 0x00021014

/* GLFW_OPENGL_ROBUSTNESS mode tokens */
const GLFW_OPENGL_NO_ROBUSTNESS         : int = 0x00000000
const GLFW_OPENGL_NO_RESET_NOTIFICATION : int = 0x00000001
const GLFW_OPENGL_LOSE_CONTEXT_ON_RESET : int = 0x00000002

/* GLFW_OPENGL_PROFILE bit tokens */
const GLFW_OPENGL_NO_PROFILE            : int = 0x00000000
const GLFW_OPENGL_CORE_PROFILE          : int = 0x00000001
const GLFW_OPENGL_COMPAT_PROFILE        : int = 0x00000002
const GLFW_OPENGL_ES2_PROFILE           : int = 0x00000004

/* glfwGetInputMode/glfwSetInputMode tokens */
const GLFW_CURSOR_MODE                  : int = 0x00030001
const GLFW_STICKY_KEYS                  : int = 0x00030002
const GLFW_STICKY_MOUSE_BUTTONS         : int = 0x00030003
const GLFW_SYSTEM_KEYS                  : int = 0x00030004
const GLFW_KEY_REPEAT                   : int = 0x00030005

/* GLFW_CURSOR_MODE values */
const GLFW_CURSOR_NORMAL                : int = 0x00040001
const GLFW_CURSOR_HIDDEN                : int = 0x00040002
const GLFW_CURSOR_CAPTURED              : int = 0x00040003

/* glfwGetJoystickParam tokens */
const GLFW_PRESENT                      : int = 0x00050001
const GLFW_AXES                         : int = 0x00050002
const GLFW_BUTTONS                      : int = 0x00050003

/* glfwGetError/glfwErrorString tokens */
const GLFW_NO_ERROR                     : int = 0
const GLFW_NOT_INITIALIZED              : int = 0x00070001
const GLFW_NO_CURRENT_CONTEXT           : int = 0x00070002
const GLFW_INVALID_ENUM                 : int = 0x00070003
const GLFW_INVALID_VALUE                : int = 0x00070004
const GLFW_OUT_OF_MEMORY                : int = 0x00070005
const GLFW_OPENGL_UNAVAILABLE           : int = 0x00070006
const GLFW_VERSION_UNAVAILABLE          : int = 0x00070007
const GLFW_PLATFORM_ERROR               : int = 0x00070008
const GLFW_WINDOW_NOT_ACTIVE            : int = 0x00070009
const GLFW_FORMAT_UNAVAILABLE           : int = 0x0007000A

/* Gamma ramps */
const GLFW_GAMMA_RAMP_SIZE              : int = 256

/*************************************************************************
 * Typedefs
 *************************************************************************/

// /* OpenGL function pointer type */
// typedef void (*GLFWglproc)(void);

/* Window handle type */
type GLFWwindow = *libc::c_void;        // typedef void* GLFWwindow;

// /* Function pointer types */
// typedef void (* GLFWerrorfun)(int,const char*);
// typedef void (* GLFWwindowsizefun)(GLFWwindow,int,int);
// typedef int  (* GLFWwindowclosefun)(GLFWwindow);
// typedef void (* GLFWwindowrefreshfun)(GLFWwindow);
// typedef void (* GLFWwindowfocusfun)(GLFWwindow,int);
// typedef void (* GLFWwindowiconifyfun)(GLFWwindow,int);
// typedef void (* GLFWmousebuttonfun)(GLFWwindow,int,int);
// typedef void (* GLFWcursorposfun)(GLFWwindow,int,int);
// typedef void (* GLFWcursorenterfun)(GLFWwindow,int);
// typedef void (* GLFWscrollfun)(GLFWwindow,double,double);
// typedef void (* GLFWkeyfun)(GLFWwindow,int,int);
// typedef void (* GLFWcharfun)(GLFWwindow,int);

/* The video mode structure used by glfwGetVideoModes */
type GLFWvidmode = {
    width      : int,
    height     : int,
    redBits    : int,
    blueBits   : int,
    greenBits  : int
};

/* Gamma ramp */
type GLFWgammaramp = {
    red     : [libc::c_ushort * GLFW_GAMMA_RAMP_SIZE],      // unsigned short red[GLFW_GAMMA_RAMP_SIZE];
    green   : [libc::c_ushort * GLFW_GAMMA_RAMP_SIZE],      // unsigned short green[GLFW_GAMMA_RAMP_SIZE];
    blue    : [libc::c_ushort * GLFW_GAMMA_RAMP_SIZE]       // unsigned short blue[GLFW_GAMMA_RAMP_SIZE];
}


extern mod glfw3
{
    /* GLFW initialization, termination and version querying */
    fn glfwInit() -> libc::c_int;                                                          // GLFWAPI int  glfwInit(void);
    fn glfwTerminate();                                                                    // GLFWAPI void glfwTerminate(void);
    fn glfwGetVersion(major: *libc::c_int, minor: *libc::c_int, rev: *libc::c_int);        // GLFWAPI void glfwGetVersion(int* major, int* minor, int* rev);
    fn glfwGetVersionString() -> *libc::c_char;                                            // GLFWAPI const char* glfwGetVersionString(void);

    /* Error handling */
    fn glfwGetError() -> libc::c_int;                                                       // GLFWAPI int glfwGetError(void);
    fn glfwErrorString(error: libc::c_int) -> *libc::c_char;                                // GLFWAPI const char* glfwErrorString(int error);
    // GLFWAPI void glfwSetErrorCallback(GLFWerrorfun cbfun);
    
    /* Video mode functions */
    fn glfwGetVideoModes(count: *lib_c::c_int) -> *GLFWvidmode;                             // GLFWAPI GLFWvidmode* glfwGetVideoModes(int* count);
    fn glfwGetDesktopMode(mode: *GLFWvidmode);                                              // GLFWAPI void glfwGetDesktopMode(GLFWvidmode* mode);
    
    /* Gamma ramp functions */
    fn glfwSetGamma(gamma: libc::c_float);                                                  // GLFWAPI void glfwSetGamma(float gamma);
    fn glfwGetGammaRamp(ramp: *GLFWgammaramp);                                              // GLFWAPI void glfwGetGammaRamp(GLFWgammaramp* ramp);
    fn glfwSetGammaRamp(ramp: *GLFWgammaramp);                                              // GLFWAPI void glfwSetGammaRamp(const GLFWgammaramp* ramp);
    
    /* Window handling */
    fn glfwWindowHint(target: libc::c_int, hint: libc::c_int);                              // GLFWAPI void glfwWindowHint(int target, int hint);
    fn glfwCreateWindow(width: libc::c_int, height: libc::c_int, mode: libc::c_int, title: *libc::c_char, share: GLFWwindow) -> GLFWwindow; // GLFWAPI GLFWwindow glfwCreateWindow(int width, int height, int mode, const char* title, GLFWwindow share);
    fn glfwDestroyWindow(window: GLFWwindow);                                               // GLFWAPI void glfwDestroyWindow(GLFWwindow window);
    fn glfwSetWindowTitle(window: GLFWwindow, title: *libc::c_char);                        // GLFWAPI void glfwSetWindowTitle(GLFWwindow window, const char* title);
    fn glfwGetWindowSize(window: GLFWwindow, width: *libc::c_int, height: *libc::c_int);    // GLFWAPI void glfwGetWindowSize(GLFWwindow window, int* width, int* height);
    fn glfwSetWindowSize(window: GLFWwindow, width: libc::c_int, height: libc::c_int);      // GLFWAPI void glfwSetWindowSize(GLFWwindow window, int width, int height);
    fn glfwGetWindowPos(window: GLFWwindow, xpos: *libc::c_int, ypos: *libc::c_int);        // GLFWAPI void glfwGetWindowPos(GLFWwindow window, int* xpos, int* ypos);
    fn glfwSetWindowPos(window: GLFWwindow, xpos: *libc::c_int, ypos: libc::c_int);         // GLFWAPI void glfwSetWindowPos(GLFWwindow window, int xpos, int ypos);
    fn glfwIconifyWindow(window: GLFWwindow);                                               // GLFWAPI void glfwIconifyWindow(GLFWwindow window);
    fn glfwRestoreWindow(window: GLFWwindow);                                               // GLFWAPI void glfwRestoreWindow(GLFWwindow window);
    fn glfwGetWindowParam(window: GLFWwindow, param: libc::c_int) -> libc::c_int;           // GLFWAPI int  glfwGetWindowParam(GLFWwindow window, int param);
    fn glfwSetWindowUserPointer(window: GLFWwindow, pointer: *libc::c_void);                // GLFWAPI void glfwSetWindowUserPointer(GLFWwindow window, void* pointer);
    fn glfwGetWindowUserPointer(window: GLFWwindow) -> *libc::c_void;                       // GLFWAPI void* glfwGetWindowUserPointer(GLFWwindow window);
    // GLFWAPI void glfwSetWindowSizeCallback(GLFWwindowsizefun cbfun);
    // GLFWAPI void glfwSetWindowCloseCallback(GLFWwindowclosefun cbfun);
    // GLFWAPI void glfwSetWindowRefreshCallback(GLFWwindowrefreshfun cbfun);
    // GLFWAPI void glfwSetWindowFocusCallback(GLFWwindowfocusfun cbfun);
    // GLFWAPI void glfwSetWindowIconifyCallback(GLFWwindowiconifyfun cbfun);

    /* Event handling */
    fn glfwPollEvents();                                                                    // GLFWAPI void glfwPollEvents(void);
    fn glfwWaitEvents();                                                                    // GLFWAPI void glfwWaitEvents(void);

    /* Input handling */
    fn glfwGetInputMode(window: GLFWwindow, mode: libc::c_int) -> libc::c_int;              // GLFWAPI int  glfwGetInputMode(GLFWwindow window, int mode);
    fn glfwSetInputMode(window: GLFWwindow, mode: libc::c_int, value: libc::c_int);         // GLFWAPI void glfwSetInputMode(GLFWwindow window, int mode, int value);
    fn glfwGetKey(window: GLFWwindow, key: libc::c_int) -> libc::c_int;                     // GLFWAPI int  glfwGetKey(GLFWwindow window, int key);
    fn glfwGetMouseButton(window: GLFWwindow, button: libc::c_int) -> libc::c_int;          // GLFWAPI int  glfwGetMouseButton(GLFWwindow window, int button);
    fn glfwGetCursorPos(window: GLFWwindow, xpos: *libc::c_int, ypos: *libc::c_int);        // GLFWAPI void glfwGetCursorPos(GLFWwindow window, int* xpos, int* ypos);
    fn glfwSetCursorPos(window: GLFWwindow, xpos: libc::c_int, ypos: libc::c_int);          // GLFWAPI void glfwSetCursorPos(GLFWwindow window, int xpos, int ypos);
    fn glfwGetScrollOffset(window: GLFWwindow, xoffset: *libc::c_double, yoffset: *libc::c_double); // GLFWAPI void glfwGetScrollOffset(GLFWwindow window, double* xoffset, double* yoffset);
    // GLFWAPI void glfwSetKeyCallback(GLFWkeyfun cbfun);
    // GLFWAPI void glfwSetCharCallback(GLFWcharfun cbfun);
    // GLFWAPI void glfwSetMouseButtonCallback(GLFWmousebuttonfun cbfun);
    // GLFWAPI void glfwSetCursorPosCallback(GLFWcursorposfun cbfun);
    // GLFWAPI void glfwSetCursorEnterCallback(GLFWcursorenterfun cbfun);
    // GLFWAPI void glfwSetScrollCallback(GLFWscrollfun cbfun);

    /* Joystick input */
    fn glfwGetJoystickParam(joy: libc::c_int, param: libc::c_int) -> libc::c_int;           // GLFWAPI int glfwGetJoystickParam(int joy, int param);
    fn glfwGetJoystickAxes(joy: libc::c_int, axes: *libc::c_float, numaxes: libc::c_int) -> libc::c_int; // GLFWAPI int glfwGetJoystickAxes(int joy, float* axes, int numaxes);
    fn glfwGetJoystickButtons(joy: libc::c_int, buttons: *libc::c_uchar, numbuttons::c_int) -> libc::c_int; // GLFWAPI int glfwGetJoystickButtons(int joy, unsigned char* buttons, int numbuttons);

    /* Clipboard */
    fn glfwSetClipboardString(window: GLFWwindow, string: *libc::c_char);                   // GLFWAPI void glfwSetClipboardString(GLFWwindow window, const char* string);
    fn glfwGetClipboardString(window: GLFWwindow) -> *libc::c_char;                         // GLFWAPI const char* glfwGetClipboardString(GLFWwindow window);
    
    /* Time */
    fn glfwGetTime() -> libc::c_double;                                                     // GLFWAPI double glfwGetTime(void);
    fn glfwSetTime(time: libc::c_double);                                                   // GLFWAPI void   glfwSetTime(double time);
    
    /* OpenGL support */
    fn glfwMakeContextCurrent(window: GLFWwindow);                                          // GLFWAPI void glfwMakeContextCurrent(GLFWwindow window);
    fn glfwGetCurrentContext() -> GLFWwindow;                                               // GLFWAPI GLFWwindow glfwGetCurrentContext(void);
    fn glfwSwapBuffers(window: GLFWwindow);                                                 // GLFWAPI void  glfwSwapBuffers(GLFWwindow window);
    fn glfwSwapInterval(interval: libc::c_int);                                             // GLFWAPI void  glfwSwapInterval(int interval);
    fn glfwExtensionSupported(extension: *libc::c_char) -> libc::c_int;                     // GLFWAPI int   glfwExtensionSupported(const char* extension);
    // GLFWAPI GLFWglproc glfwGetProcAddress(const char* procname);
    fn glfwCopyContext(src: GLFWwindow, dst: GLFWwindow, mask: libc::c_ulong);              // GLFWAPI void  glfwCopyContext(GLFWwindow src, GLFWwindow dst, unsigned long mask);
}


fn glfwInit() -> int {    
    unsafe { ret glfw::glfwInit() as int; }
}

fn glfwTerminate() {
    unsafe { glfw::glfwTerminate(); }  
}

fn glfwGetVersion(major : @int, minor : @int, rev : @int) {
    unsafe { glfw::glfwGetVersion(ptr::addr_of(*major) as *libc::c_int, ptr::addr_of(*minor) as *libc::c_int, ptr::addr_of(*rev) as *libc::c_int); }
}


fn glfwOpenWindow(width : int, height : int, redbits : int, greenbits : int, bluebits : int, alphabits : int, depthbits : int, stencilbits : int, mode : int) -> int {
    unsafe { ret glfw::glfwOpenWindow(width as libc::c_int, height as libc::c_int, redbits as libc::c_int, greenbits as libc::c_int, bluebits as libc::c_int, alphabits as libc::c_int, depthbits as libc::c_int, stencilbits as libc::c_int, mode as libc::c_int) as int; }
}

fn glfwOpenWindowHint(target : int, hint : int) {
    unsafe { glfw::glfwOpenWindowHint(target as libc::c_int, hint as libc::c_int); }
}

fn glfwCloseWindow() {
    unsafe { glfw::glfwCloseWindow(); }
}

fn glfwSetWindowTitle(title : str) {
    let mut bytes = str::bytes(title);
    bytes += [0u8];
    unsafe { glfw::glfwSetWindowTitle(vec::unsafe::to_ptr(bytes)); }
}

fn glfwGetWindowSize(width : @int, height : @int) {
    unsafe { glfw::glfwGetWindowSize(ptr::addr_of(*width) as *libc::c_int, ptr::addr_of(*height) as *libc::c_int); }
}

fn glfwSetWindowSize(width : int, height : int) {
    unsafe { glfw::glfwSetWindowSize(width as libc::c_int, height as libc::c_int); }
}

fn glfwSetWindowPos(x : int, y : int) {
    unsafe { glfw::glfwSetWindowPos(x as libc::c_int, y as libc::c_int); }
}

fn glfwIconifyWindow() {
    unsafe { glfw::glfwIconifyWindow(); }
}

fn glfwRestoreWindow() {
    unsafe { glfw::glfwRestoreWindow(); }
}

fn glfwSwapBuffers() {
    unsafe { glfw::glfwSwapBuffers(); }
}

fn glfwSwapInterval(interval : int) {
    unsafe { glfw::glfwSwapInterval(interval as libc::c_int); }
}

fn glfwGetWindowParam(param : int) -> int {
    unsafe { ret glfw::glfwGetWindowParam(param as libc::c_int) as int; }
}

 
fn glfwGetVideoModes(list : @GLFWvidmode, maxcount : int) -> int {
    unsafe { ret glfw::glfwGetVideoModes(ptr::addr_of(*list), maxcount as libc::c_int) as int; }
}

fn glfwGetDesktopMode(mode : @GLFWvidmode) {
    unsafe { glfw::glfwGetDesktopMode(ptr::addr_of(*mode)); }
}

 
fn glfwPollEvents() {
    unsafe { glfw::glfwPollEvents(); }
}

fn glfwWaitEvents() {
    unsafe { glfw::glfwWaitEvents(); }
}

fn glfwGetKey(key : int) -> int {
    unsafe { ret glfw::glfwGetKey(key as libc::c_int) as int; }
}

fn glfwGetMouseButton(button : int) -> int {
    unsafe { ret glfw::glfwGetMouseButton(button as libc::c_int) as int; }
}

fn glfwGetMousePos(xpos : @int, ypos : @int) {
    unsafe { glfw::glfwGetMousePos(ptr::addr_of(*xpos) as *libc::c_int, ptr::addr_of(*ypos) as *libc::c_int); }
}

fn glfwSetMousePos(xpos : int, ypos : int) {
    unsafe { glfw::glfwSetMousePos(xpos as libc::c_int, ypos as libc::c_int); }
}

fn glfwGetMouseWheel() -> int {
    unsafe { ret glfw::glfwGetMouseWheel() as int; }
}

fn glfwSetMouseWheel(pos : int) {
    unsafe { glfw::glfwSetMouseWheel(pos as libc::c_int); }
}

 
fn glfwGetJoystickParam(joy : int, param : int) -> int {
    unsafe { ret glfw::glfwGetJoystickParam(joy as libc::c_int, param as libc::c_int) as int; }
}

fn glfwGetJoystickPos(joy : int, pos : @float, numaxes : int) -> int {
    unsafe { ret glfw::glfwGetJoystickPos(joy as libc::c_int, ptr::addr_of(*pos), numaxes as libc::c_int) as int; }
}

fn glfwGetJoystickButtons(joy : int, buttons : @mut [u8], numbuttons : int) -> int {
    unsafe { 
        let mut r = 0;
        let mut b : [u8] = [];
        vec::grow(b, numbuttons as uint, 0u8);
        r = glfw::glfwGetJoystickButtons(joy as libc::c_int, vec::unsafe::to_ptr(b), numbuttons as libc::c_int) as int;
        *buttons = b;
        ret r;
    }
}

 
fn glfwGetTime() -> float {
    unsafe { ret glfw::glfwGetTime(); }
}

fn glfwSetTime(time : float) {
    unsafe { glfw::glfwSetTime(time); }
}

fn glfwSleep(time : float) {
    unsafe { glfw::glfwSleep(time); }
}

 
fn glfwExtensionSupported(extenstion : str) -> int {
    let mut bytes = str::bytes(extenstion);
    bytes += [0u8];
    unsafe { ret glfw::glfwExtensionSupported(vec::unsafe::to_ptr(bytes)) as int; }
}

fn glfwGetProcAddress(procname : str) -> *uint {
    let mut bytes = str::bytes(procname);
    bytes += [0u8];
    unsafe { ret glfw::glfwGetProcAddress(vec::unsafe::to_ptr(bytes)) as *uint; }
}

fn glfwGetGLVersion(major : @int, minor : @int, rev : @int) {
    unsafe { glfw::glfwGetGLVersion(ptr::addr_of(*major) as *libc::c_int, ptr::addr_of(*minor) as *libc::c_int, ptr::addr_of(*rev) as *libc::c_int); }
}

 
fn glfwEnable(token : int) {
    unsafe { glfw::glfwEnable(token as libc::c_int); }
}

fn glfwDisable(token : int) {
    unsafe { glfw::glfwDisable(token as libc::c_int); }
}

 
fn glfwReadImage(name : str, img : @GLFWimage, flags : int) -> int {
    let bytes = str::bytes(name);
    unsafe { ret glfw::glfwReadImage(vec::unsafe::to_ptr(bytes), ptr::addr_of(*img), flags as libc::c_int) as int; }
}

fn glfwReadMemoryImage(data : *uint, size : uint, img : @GLFWimage, flags : int) -> int {
    unsafe { ret glfw::glfwReadMemoryImage(data as *libc::c_uint, size as libc::c_long, ptr::addr_of(*img), flags as libc::c_int) as int; }
}

fn glfwFreeImage(img : @GLFWimage) {
    unsafe { glfw::glfwFreeImage(ptr::addr_of(*img)); }
}

fn glfwLoadTexture2D(name : str, flags : int) -> int {
    let mut bytes = str::bytes(name);
    bytes += [0u8];
    unsafe { ret glfw::glfwLoadTexture2D(vec::unsafe::to_ptr(bytes), flags as libc::c_int) as int; }
}

fn glfwLoadMemoryTexture2D(data : *uint, size : uint, flags : int) -> int {
    unsafe { ret glfw::glfwLoadMemoryTexture2D(data as *libc::c_uint, size as libc::c_long, flags as libc::c_int) as int; }
}

fn glfwLoadTextureImage2D(img : @GLFWimage, flags : int) -> int {
    unsafe { ret glfw::glfwLoadTextureImage2D(ptr::addr_of(*img), flags as libc::c_int) as int; }
}
