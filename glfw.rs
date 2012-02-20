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
 
 

/************************************************************************
 * GLFW - An OpenGL framework
 * API version: 2.7
 * WWW:         http://www.glfw.org/
 *------------------------------------------------------------------------
 * Copyright (c) 2002-2006 Marcus Geelnard
 * Copyright (c) 2006-2010 Camilla Berglund
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

const GLFW_VERSION_MAJOR : int =    2;
const GLFW_VERSION_MINOR : int =    7;
const GLFW_VERSION_REVISION : int = 2;

/*************************************************************************
 * Input handling definitions
 *************************************************************************/

/* Key and button state/action definitions */
const GLFW_RELEASE : int =             0;
const GLFW_PRESS : int =               1;

/* Keyboard key definitions: 8-bit ISO-8859-1 (Latin 1) encoding is used
 * for printable keys (such as A-Z, 0-9 etc), and values above 256
 * represent special (non-printable) keys (e.g. F1, Page Up etc).
 */
const GLFW_KEY_UNKNOWN : int =       -1;
const GLFW_KEY_SPACE : int =         32;
const GLFW_KEY_SPECIAL : int =       256;
const GLFW_KEY_ESC : int =           257;
const GLFW_KEY_F1 : int =            258;
const GLFW_KEY_F2 : int =            259;
const GLFW_KEY_F3 : int =            260;
const GLFW_KEY_F4 : int =            261;
const GLFW_KEY_F5 : int =            262;
const GLFW_KEY_F6 : int =            263;
const GLFW_KEY_F7 : int =            264;
const GLFW_KEY_F8 : int =            265;
const GLFW_KEY_F9 : int =            266;
const GLFW_KEY_F10 : int =           267;
const GLFW_KEY_F11 : int =           268;
const GLFW_KEY_F12 : int =           269;
const GLFW_KEY_F13 : int =           270;
const GLFW_KEY_F14 : int =           271;
const GLFW_KEY_F15 : int =           272;
const GLFW_KEY_F16 : int =           273;
const GLFW_KEY_F17 : int =           274;
const GLFW_KEY_F18 : int =           275;
const GLFW_KEY_F19 : int =           276;
const GLFW_KEY_F20 : int =           277;
const GLFW_KEY_F21 : int =           278;
const GLFW_KEY_F22 : int =           279;
const GLFW_KEY_F23 : int =           280;
const GLFW_KEY_F24 : int =           281;
const GLFW_KEY_F25 : int =           282;
const GLFW_KEY_UP : int =            283;
const GLFW_KEY_DOWN : int =          284;
const GLFW_KEY_LEFT : int =          285;
const GLFW_KEY_RIGHT : int =         286;
const GLFW_KEY_LSHIFT : int =        287;
const GLFW_KEY_RSHIFT : int =        288;
const GLFW_KEY_LCTRL : int =         289;
const GLFW_KEY_RCTRL : int =         290;
const GLFW_KEY_LALT : int =          291;
const GLFW_KEY_RALT : int =          292;
const GLFW_KEY_TAB : int =           293;
const GLFW_KEY_ENTER : int =         294;
const GLFW_KEY_BACKSPACE : int =     295;
const GLFW_KEY_INSERT  : int =       296;
const GLFW_KEY_DEL : int =           297;
const GLFW_KEY_PAGEUP  : int =       298;
const GLFW_KEY_PAGEDOWN  : int =     299;
const GLFW_KEY_HOME : int =          300;
const GLFW_KEY_END : int =           301;
const GLFW_KEY_KP_0  : int =         302;
const GLFW_KEY_KP_1 : int =          303;
const GLFW_KEY_KP_2 : int =          304;
const GLFW_KEY_KP_3 : int =          305;
const GLFW_KEY_KP_4 : int =          306;
const GLFW_KEY_KP_5 : int =          307;
const GLFW_KEY_KP_6 : int =          308;
const GLFW_KEY_KP_7 : int =          309;
const GLFW_KEY_KP_8 : int =          310;
const GLFW_KEY_KP_9 : int =          311;
const GLFW_KEY_KP_DIVIDE : int =     312;
const GLFW_KEY_KP_MULTIPLY : int =   313;
const GLFW_KEY_KP_SUBTRACT : int =   314;
const GLFW_KEY_KP_ADD : int =        315;
const GLFW_KEY_KP_DECIMAL : int =    316;
const GLFW_KEY_KP_EQUAL : int =      317;
const GLFW_KEY_KP_ENTER : int =      318;
const GLFW_KEY_KP_NUM_LOCK : int =   319;
const GLFW_KEY_CAPS_LOCK : int =     320;
const GLFW_KEY_SCROLL_LOCK : int =   321;
const GLFW_KEY_PAUSE : int =         322;
const GLFW_KEY_LSUPER : int =        323;
const GLFW_KEY_RSUPER : int =        324;
const GLFW_KEY_MENU : int =          325;
const GLFW_KEY_LAST : int =          325;

/* Mouse button definitions */
const GLFW_MOUSE_BUTTON_1 : int =       0;
const GLFW_MOUSE_BUTTON_2 : int =       1;
const GLFW_MOUSE_BUTTON_3 : int =       2;
const GLFW_MOUSE_BUTTON_4 : int =       3;
const GLFW_MOUSE_BUTTON_5 : int =       4;
const GLFW_MOUSE_BUTTON_6 : int =       5;
const GLFW_MOUSE_BUTTON_7 : int =       6;
const GLFW_MOUSE_BUTTON_8 : int =       7;
const GLFW_MOUSE_BUTTON_LAST : int =    7;

/* Mouse button aliases */
const GLFW_MOUSE_BUTTON_LEFT : int =    0;
const GLFW_MOUSE_BUTTON_RIGHT : int =   1;
const GLFW_MOUSE_BUTTON_MIDDLE : int =  2;


/* Joystick identifiers */
const GLFW_JOYSTICK_1 : int =           0;
const GLFW_JOYSTICK_2 : int =           1;
const GLFW_JOYSTICK_3 : int =           2;
const GLFW_JOYSTICK_4 : int =           3;
const GLFW_JOYSTICK_5 : int =           4;
const GLFW_JOYSTICK_6 : int =           5;
const GLFW_JOYSTICK_7 : int =           6;
const GLFW_JOYSTICK_8 : int =           7;
const GLFW_JOYSTICK_9 : int =           8;
const GLFW_JOYSTICK_10 : int =          9;
const GLFW_JOYSTICK_11 : int =          10;
const GLFW_JOYSTICK_12 : int =          11;
const GLFW_JOYSTICK_13 : int =          12;
const GLFW_JOYSTICK_14 : int =          13;
const GLFW_JOYSTICK_15 : int =          14;
const GLFW_JOYSTICK_16 : int =          15;
const GLFW_JOYSTICK_LAST : int =        15;


/*************************************************************************
 * Other definitions
 *************************************************************************/

/* glfwOpenWindow modes */ 
const GLFW_WINDOW : int =                0x00010001;
const GLFW_FULLSCREEN : int =            0x00010002;

/* glfwGetWindowParam tokens */
const GLFW_OPENED : int =                0x00020001;
const GLFW_ACTIVE : int =                0x00020002;
const GLFW_ICONIFIED : int =             0x00020003;
const GLFW_ACCELERATED : int =           0x00020004;
const GLFW_RED_BITS : int =              0x00020005;
const GLFW_GREEN_BITS : int =            0x00020006;
const GLFW_BLUE_BITS : int =             0x00020007;
const GLFW_ALPHA_BITS : int =            0x00020008;
const GLFW_DEPTH_BITS : int =            0x00020009;
const GLFW_STENCIL_BITS : int =          0x0002000A;

/* The following constants are used for both glfwGetWindowParam
 * and glfwOpenWindowHint
 */
const GLFW_REFRESH_RATE : int =          0x0002000B;
const GLFW_ACCUM_RED_BITS : int =        0x0002000C;
const GLFW_ACCUM_GREEN_BITS : int =      0x0002000D;
const GLFW_ACCUM_BLUE_BITS : int =       0x0002000E;
const GLFW_ACCUM_ALPHA_BITS : int =      0x0002000F;
const GLFW_AUX_BUFFERS : int =           0x00020010;
const GLFW_STEREO : int =                0x00020011;
const GLFW_WINDOW_NO_RESIZE : int =      0x00020012;
const GLFW_FSAA_SAMPLES : int =          0x00020013;
const GLFW_OPENGL_VERSION_MAJOR : int =  0x00020014;
const GLFW_OPENGL_VERSION_MINOR : int =  0x00020015;
const GLFW_OPENGL_FORWARD_COMPAT : int = 0x00020016;
const GLFW_OPENGL_DEBUG_CONTEXT : int =  0x00020017;
const GLFW_OPENGL_PROFILE : int =        0x00020018;

/* GLFW_OPENGL_PROFILE tokens */
const GLFW_OPENGL_CORE_PROFILE : int =   0x00050001;
const GLFW_OPENGL_COMPAT_PROFILE : int = 0x00050002;

/* glfwEnable/glfwDisable tokens */ 
const GLFW_MOUSE_CURSOR : int =          0x00030001;
const GLFW_STICKY_KEYS : int =           0x00030002;
const GLFW_STICKY_MOUSE_BUTTONS : int =  0x00030003;
const GLFW_SYSTEM_KEYS : int =           0x00030004;
const GLFW_KEY_REPEAT : int =            0x00030005;
const GLFW_AUTO_POLL_EVENTS : int =      0x00030006;

/* glfwWaitThread wait modes */
const GLFW_WAIT : int =                  0x00040001;
const GLFW_NOWAIT : int =                0x00040002;

/* glfwGetJoystickParam tokens */
const GLFW_PRESENT : int =               0x00050001;
const GLFW_AXES : int =                  0x00050002;
const GLFW_BUTTONS : int =               0x00050003;

/* glfwReadImage/glfwLoadTexture2D flags */
const GLFW_NO_RESCALE_BIT : int =        0x00000001; /* Only for glfwReadImage */
const GLFW_ORIGIN_UL_BIT : int =         0x00000002;
const GLFW_BUILD_MIPMAPS_BIT : int =     0x00000004; /* Only for glfwLoadTexture2D */
const GLFW_ALPHA_MAP_BIT : int =         0x00000008;

/* Time spans longer than this (seconds) are considered to be infinity */
const GLFW_INFINITY : int = 100000;

type GLFWvidmode = {
	Width : int,
	Height : int,
	RedBits : int,
	BlueBits : int,
	GreenBits : int
};

type GLFWimage = {
    Width : int,
	Height : int,
	Format : int, 
	BytesPerPixel : int, 
	Data : *u8
};

type GLFWthread = int;
type GLFWmutex = *uint;
type GLFWcond = *uint;


native mod glfw
{
	/* GLFW initialization, termination and version querying */
	fn glfwInit() -> ctypes::c_int;
	fn glfwTerminate();
	fn glfwGetVersion(major : *ctypes::c_int, minor : *ctypes::c_int, rev : *ctypes::c_int);
	
	/* Window handling */
	fn glfwOpenWindow(width : ctypes::c_int, height : ctypes::c_int, redbits : ctypes::c_int, greenbits : ctypes::c_int, bluebits : ctypes::c_int, alphabits : ctypes::c_int, depthbits : ctypes::c_int, stencilbits : ctypes::c_int, mode : ctypes::c_int) -> ctypes::c_int;
	fn glfwOpenWindowHint(target : ctypes::c_int, hint : ctypes::c_int);
	fn glfwCloseWindow();
	fn glfwSetWindowTitle(title : *u8);
	fn glfwGetWindowSize(width : *ctypes::c_int, height : *ctypes::c_int);
	fn glfwSetWindowSize(width : ctypes::c_int, height : ctypes::c_int);
	fn glfwSetWindowPos(x : ctypes::c_int, y : ctypes::c_int);
	fn glfwIconifyWindow();
	fn glfwRestoreWindow();
	fn glfwSwapBuffers();
	fn glfwSwapInterval(interval : ctypes::c_int);
	fn glfwGetWindowParam(param : ctypes::c_int) -> ctypes::c_int;
	
	/* Video mode functions */
	fn glfwGetVideoModes(list : *GLFWvidmode, maxcount : ctypes::c_int) -> ctypes::c_int;
	fn glfwGetDesktopMode(mode : *GLFWvidmode);
	
	/* Input handling */
	fn glfwPollEvents();
	fn glfwWaitEvents();
	fn glfwGetKey(key : ctypes::c_int) -> ctypes::c_int;
	fn glfwGetMouseButton(button : ctypes::c_int) -> ctypes::c_int;
	fn glfwGetMousePos(xpos : *ctypes::c_int, ypos : *ctypes::c_int);
	fn glfwSetMousePos(xpos : ctypes::c_int, ypos : ctypes::c_int);
	fn glfwGetMouseWheel() -> ctypes::c_int;
	fn glfwSetMouseWheel(pos : ctypes::c_int);
	
	/* Joystick input */
	fn glfwGetJoystickParam(joy : ctypes::c_int, param : ctypes::c_int) -> ctypes::c_int;
	fn glfwGetJoystickPos(joy : ctypes::c_int, pos : *float, numaxes : ctypes::c_int) -> ctypes::c_int;
	fn glfwGetJoystickButtons(joy : ctypes::c_int, buttons : *u8, numbuttons : ctypes::c_int) -> ctypes::c_int;
	
	/* Time */
	fn glfwGetTime() -> float;
	fn glfwSetTime(time : float);
	fn glfwSleep(time : float);
	
	/* Extension support */
	fn glfwExtensionSupported(extenstion : *u8) -> ctypes::c_int;
	fn glfwGetProcAddress(procname : *u8) -> *ctypes::c_uint;
	fn glfwGetGLVersion(major : *ctypes::c_int, minor : *ctypes::c_int, rev : *ctypes::c_int);
	
	/* Enable/disable functions */
	fn glfwEnable(token : ctypes::c_int);
	fn glfwDisable(token : ctypes::c_int);
	
	/* Image/texture I/O support */
	fn glfwReadImage(name : *u8, img : *GLFWimage, flags : ctypes::c_int) -> ctypes::c_int;
	fn glfwReadMemoryImage(data : *ctypes::c_uint, size : ctypes::long, img : *GLFWimage, flags : ctypes::c_int) -> ctypes::c_int;
	fn glfwFreeImage(img : *GLFWimage);
	fn glfwLoadTexture2D(name : *u8, flags : ctypes::c_int) -> ctypes::c_int;
	fn glfwLoadMemoryTexture2D(data : *ctypes::c_uint, size : ctypes::long, flags : ctypes::c_int) -> ctypes::c_int;
	fn glfwLoadTextureImage2D(img : *GLFWimage, flags : ctypes::c_int) -> ctypes::c_int;
}


fn glfwInit() -> int {    
    unsafe { ret glfw::glfwInit() as int; }
}

fn glfwTerminate() {
    unsafe { glfw::glfwTerminate(); }  
}

fn glfwGetVersion(major : @int, minor : @int, rev : @int) {
    unsafe { glfw::glfwGetVersion(ptr::addr_of(*major) as *ctypes::c_int, ptr::addr_of(*minor) as *ctypes::c_int, ptr::addr_of(*rev) as *ctypes::c_int); }
}


fn glfwOpenWindow(width : int, height : int, redbits : int, greenbits : int, bluebits : int, alphabits : int, depthbits : int, stencilbits : int, mode : int) -> int {
    unsafe { ret glfw::glfwOpenWindow(width as ctypes::c_int, height as ctypes::c_int, redbits as ctypes::c_int, greenbits as ctypes::c_int, bluebits as ctypes::c_int, alphabits as ctypes::c_int, depthbits as ctypes::c_int, stencilbits as ctypes::c_int, mode as ctypes::c_int) as int; }
}

fn glfwOpenWindowHint(target : int, hint : int) {
    unsafe { glfw::glfwOpenWindowHint(target as ctypes::c_int, hint as ctypes::c_int); }
}

fn glfwCloseWindow() {
    unsafe { glfw::glfwCloseWindow(); }
}

fn glfwSetWindowTitle(title : str) {
    let bytes = str::bytes(title);
    bytes += [0u8];
    unsafe { glfw::glfwSetWindowTitle(vec::unsafe::to_ptr(bytes)); }
}

fn glfwGetWindowSize(width : @int, height : @int) {
    unsafe { glfw::glfwGetWindowSize(ptr::addr_of(*width) as *ctypes::c_int, ptr::addr_of(*height) as *ctypes::c_int); }
}

fn glfwSetWindowSize(width : int, height : int) {
    unsafe { glfw::glfwSetWindowSize(width as ctypes::c_int, height as ctypes::c_int); }
}

fn glfwSetWindowPos(x : int, y : int) {
    unsafe { glfw::glfwSetWindowPos(x as ctypes::c_int, y as ctypes::c_int); }
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
    unsafe { glfw::glfwSwapInterval(interval as ctypes::c_int); }
}

fn glfwGetWindowParam(param : int) -> int {
    unsafe { ret glfw::glfwGetWindowParam(param as ctypes::c_int) as int; }
}

 
fn glfwGetVideoModes(list : @GLFWvidmode, maxcount : int) -> int {
    unsafe { ret glfw::glfwGetVideoModes(ptr::addr_of(*list), maxcount as ctypes::c_int) as int; }
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
    unsafe { ret glfw::glfwGetKey(key as ctypes::c_int) as int; }
}

fn glfwGetMouseButton(button : int) -> int {
    unsafe { ret glfw::glfwGetMouseButton(button as ctypes::c_int) as int; }
}

fn glfwGetMousePos(xpos : @int, ypos : @int) {
    unsafe { glfw::glfwGetMousePos(ptr::addr_of(*xpos) as *ctypes::c_int, ptr::addr_of(*ypos) as *ctypes::c_int); }
}

fn glfwSetMousePos(xpos : int, ypos : int) {
    unsafe { glfw::glfwSetMousePos(xpos as ctypes::c_int, ypos as ctypes::c_int); }
}

fn glfwGetMouseWheel() -> int {
    unsafe { ret glfw::glfwGetMouseWheel() as int; }
}

fn glfwSetMouseWheel(pos : int) {
    unsafe { glfw::glfwSetMouseWheel(pos as ctypes::c_int); }
}

 
fn glfwGetJoystickParam(joy : int, param : int) -> int {
    unsafe { ret glfw::glfwGetJoystickParam(joy as ctypes::c_int, param as ctypes::c_int) as int; }
}

fn glfwGetJoystickPos(joy : int, pos : @float, numaxes : int) -> int {
    unsafe { ret glfw::glfwGetJoystickPos(joy as ctypes::c_int, ptr::addr_of(*pos), numaxes as ctypes::c_int) as int; }
}

fn glfwGetJoystickButtons(joy : int, buttons : @mutable [u8], numbuttons : int) -> int {
    unsafe { 
        let r = 0;
        let b : [u8] = [];
        vec::grow(b, numbuttons as uint, 0u8);
        r = glfw::glfwGetJoystickButtons(joy as ctypes::c_int, vec::unsafe::to_ptr(b), numbuttons as ctypes::c_int) as int;
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
    let bytes = str::bytes(extenstion);
    bytes += [0u8];
    unsafe { ret glfw::glfwExtensionSupported(vec::unsafe::to_ptr(bytes)) as int; }
}

fn glfwGetProcAddress(procname : str) -> *uint {
    let bytes = str::bytes(procname);
    bytes += [0u8];
    unsafe { ret glfw::glfwGetProcAddress(vec::unsafe::to_ptr(bytes)) as *uint; }
}

fn glfwGetGLVersion(major : @int, minor : @int, rev : @int) {
    unsafe { glfw::glfwGetGLVersion(ptr::addr_of(*major) as *ctypes::c_int, ptr::addr_of(*minor) as *ctypes::c_int, ptr::addr_of(*rev) as *ctypes::c_int); }
}

 
fn glfwEnable(token : int) {
    unsafe { glfw::glfwEnable(token as ctypes::c_int); }
}

fn glfwDisable(token : int) {
    unsafe { glfw::glfwDisable(token as ctypes::c_int); }
}

 
fn glfwReadImage(name : str, img : @GLFWimage, flags : int) -> int {
    let bytes = str::bytes(name);
    unsafe { ret glfw::glfwReadImage(vec::unsafe::to_ptr(bytes), ptr::addr_of(*img), flags as ctypes::c_int) as int; }
}

fn glfwReadMemoryImage(data : *uint, size : uint, img : @GLFWimage, flags : int) -> int {
    unsafe { ret glfw::glfwReadMemoryImage(data as *ctypes::c_uint, size as ctypes::long, ptr::addr_of(*img), flags as ctypes::c_int) as int; }
}

fn glfwFreeImage(img : @GLFWimage) {
    unsafe { glfw::glfwFreeImage(ptr::addr_of(*img)); }
}

fn glfwLoadTexture2D(name : str, flags : int) -> int {
    let bytes = str::bytes(name);
    bytes += [0u8];
    unsafe { ret glfw::glfwLoadTexture2D(vec::unsafe::to_ptr(bytes), flags as ctypes::c_int) as int; }
}

fn glfwLoadMemoryTexture2D(data : *uint, size : uint, flags : int) -> int {
    unsafe { ret glfw::glfwLoadMemoryTexture2D(data as *ctypes::c_uint, size as ctypes::long, flags as ctypes::c_int) as int; }
}

fn glfwLoadTextureImage2D(img : @GLFWimage, flags : int) -> int {
    unsafe { ret glfw::glfwLoadTextureImage2D(ptr::addr_of(*img), flags as ctypes::c_int) as int; }
}
