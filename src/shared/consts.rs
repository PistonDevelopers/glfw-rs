/* GLFW version */
pub static VERSION_MAJOR                : libc::c_int = 3;
pub static VERSION_MINOR                : libc::c_int = 0;
pub static VERSION_REVISION             : libc::c_int = 0;

/* Not actually defined in GLFW, but very useful */
pub static FALSE                        : libc::c_int = 0;
pub static TRUE                         : libc::c_int = 1;

/* Key and button state/action definitions */
pub static RELEASE                      : libc::c_int = 0;
pub static PRESS                        : libc::c_int = 1;
pub static REPEAT                       : libc::c_int = 2;

/* Printable keys */
pub static KEY_SPACE                    : libc::c_int = 32;
pub static KEY_APOSTROPHE               : libc::c_int = 39;
pub static KEY_COMMA                    : libc::c_int = 44;
pub static KEY_MINUS                    : libc::c_int = 45;
pub static KEY_PERIOD                   : libc::c_int = 46;
pub static KEY_SLASH                    : libc::c_int = 47;
pub static KEY_0                        : libc::c_int = 48;
pub static KEY_1                        : libc::c_int = 49;
pub static KEY_2                        : libc::c_int = 50;
pub static KEY_3                        : libc::c_int = 51;
pub static KEY_4                        : libc::c_int = 52;
pub static KEY_5                        : libc::c_int = 53;
pub static KEY_6                        : libc::c_int = 54;
pub static KEY_7                        : libc::c_int = 55;
pub static KEY_8                        : libc::c_int = 56;
pub static KEY_9                        : libc::c_int = 57;
pub static KEY_SEMICOLON                : libc::c_int = 59;
pub static KEY_EQUAL                    : libc::c_int = 61;
pub static KEY_A                        : libc::c_int = 65;
pub static KEY_B                        : libc::c_int = 66;
pub static KEY_C                        : libc::c_int = 67;
pub static KEY_D                        : libc::c_int = 68;
pub static KEY_E                        : libc::c_int = 69;
pub static KEY_F                        : libc::c_int = 70;
pub static KEY_G                        : libc::c_int = 71;
pub static KEY_H                        : libc::c_int = 72;
pub static KEY_I                        : libc::c_int = 73;
pub static KEY_J                        : libc::c_int = 74;
pub static KEY_K                        : libc::c_int = 75;
pub static KEY_L                        : libc::c_int = 76;
pub static KEY_M                        : libc::c_int = 77;
pub static KEY_N                        : libc::c_int = 78;
pub static KEY_O                        : libc::c_int = 79;
pub static KEY_P                        : libc::c_int = 80;
pub static KEY_Q                        : libc::c_int = 81;
pub static KEY_R                        : libc::c_int = 82;
pub static KEY_S                        : libc::c_int = 83;
pub static KEY_T                        : libc::c_int = 84;
pub static KEY_U                        : libc::c_int = 85;
pub static KEY_V                        : libc::c_int = 86;
pub static KEY_W                        : libc::c_int = 87;
pub static KEY_X                        : libc::c_int = 88;
pub static KEY_Y                        : libc::c_int = 89;
pub static KEY_Z                        : libc::c_int = 90;
pub static KEY_LEFT_BRACKET             : libc::c_int = 91;
pub static KEY_BACKSLASH                : libc::c_int = 92;
pub static KEY_RIGHT_BRACKET            : libc::c_int = 93;
pub static KEY_GRAVE_ACCENT             : libc::c_int = 96;
pub static KEY_WORLD_1                  : libc::c_int = 161;
pub static KEY_WORLD_2                  : libc::c_int = 162;

/* Function keys */
pub static KEY_ESCAPE                   : libc::c_int = 256;
pub static KEY_ENTER                    : libc::c_int = 257;
pub static KEY_TAB                      : libc::c_int = 258;
pub static KEY_BACKSPACE                : libc::c_int = 259;
pub static KEY_INSERT                   : libc::c_int = 260;
pub static KEY_DELETE                   : libc::c_int = 261;
pub static KEY_RIGHT                    : libc::c_int = 262;
pub static KEY_LEFT                     : libc::c_int = 263;
pub static KEY_DOWN                     : libc::c_int = 264;
pub static KEY_UP                       : libc::c_int = 265;
pub static KEY_PAGE_UP                  : libc::c_int = 266;
pub static KEY_PAGE_DOWN                : libc::c_int = 267;
pub static KEY_HOME                     : libc::c_int = 268;
pub static KEY_END                      : libc::c_int = 269;
pub static KEY_CAPS_LOCK                : libc::c_int = 280;
pub static KEY_SCROLL_LOCK              : libc::c_int = 281;
pub static KEY_NUM_LOCK                 : libc::c_int = 282;
pub static KEY_PRINT_SCREEN             : libc::c_int = 283;
pub static KEY_PAUSE                    : libc::c_int = 284;
pub static KEY_F1                       : libc::c_int = 290;
pub static KEY_F2                       : libc::c_int = 291;
pub static KEY_F3                       : libc::c_int = 292;
pub static KEY_F4                       : libc::c_int = 293;
pub static KEY_F5                       : libc::c_int = 294;
pub static KEY_F6                       : libc::c_int = 295;
pub static KEY_F7                       : libc::c_int = 296;
pub static KEY_F8                       : libc::c_int = 297;
pub static KEY_F9                       : libc::c_int = 298;
pub static KEY_F10                      : libc::c_int = 299;
pub static KEY_F11                      : libc::c_int = 300;
pub static KEY_F12                      : libc::c_int = 301;
pub static KEY_F13                      : libc::c_int = 302;
pub static KEY_F14                      : libc::c_int = 303;
pub static KEY_F15                      : libc::c_int = 304;
pub static KEY_F16                      : libc::c_int = 305;
pub static KEY_F17                      : libc::c_int = 306;
pub static KEY_F18                      : libc::c_int = 307;
pub static KEY_F19                      : libc::c_int = 308;
pub static KEY_F20                      : libc::c_int = 309;
pub static KEY_F21                      : libc::c_int = 310;
pub static KEY_F22                      : libc::c_int = 311;
pub static KEY_F23                      : libc::c_int = 312;
pub static KEY_F24                      : libc::c_int = 313;
pub static KEY_F25                      : libc::c_int = 314;
pub static KEY_KP_0                     : libc::c_int = 320;
pub static KEY_KP_1                     : libc::c_int = 321;
pub static KEY_KP_2                     : libc::c_int = 322;
pub static KEY_KP_3                     : libc::c_int = 323;
pub static KEY_KP_4                     : libc::c_int = 324;
pub static KEY_KP_5                     : libc::c_int = 325;
pub static KEY_KP_6                     : libc::c_int = 326;
pub static KEY_KP_7                     : libc::c_int = 327;
pub static KEY_KP_8                     : libc::c_int = 328;
pub static KEY_KP_9                     : libc::c_int = 329;
pub static KEY_KP_DECIMAL               : libc::c_int = 330;
pub static KEY_KP_DIVIDE                : libc::c_int = 331;
pub static KEY_KP_MULTIPLY              : libc::c_int = 332;
pub static KEY_KP_SUBTRACT              : libc::c_int = 333;
pub static KEY_KP_ADD                   : libc::c_int = 334;
pub static KEY_KP_ENTER                 : libc::c_int = 335;
pub static KEY_KP_EQUAL                 : libc::c_int = 336;
pub static KEY_LEFT_SHIFT               : libc::c_int = 340;
pub static KEY_LEFT_CONTROL             : libc::c_int = 341;
pub static KEY_LEFT_ALT                 : libc::c_int = 342;
pub static KEY_LEFT_SUPER               : libc::c_int = 343;
pub static KEY_RIGHT_SHIFT              : libc::c_int = 344;
pub static KEY_RIGHT_CONTROL            : libc::c_int = 345;
pub static KEY_RIGHT_ALT                : libc::c_int = 346;
pub static KEY_RIGHT_SUPER              : libc::c_int = 347;
pub static KEY_MENU                     : libc::c_int = 348;
pub static KEY_LAST                     : libc::c_int = KEY_MENU;

/* Mouse buttons */
pub static MOUSE_BUTTON_1               : libc::c_int = 0;
pub static MOUSE_BUTTON_2               : libc::c_int = 1;
pub static MOUSE_BUTTON_3               : libc::c_int = 2;
pub static MOUSE_BUTTON_4               : libc::c_int = 3;
pub static MOUSE_BUTTON_5               : libc::c_int = 4;
pub static MOUSE_BUTTON_6               : libc::c_int = 5;
pub static MOUSE_BUTTON_7               : libc::c_int = 6;
pub static MOUSE_BUTTON_8               : libc::c_int = 7;
pub static MOUSE_BUTTON_LEFT            : libc::c_int = MOUSE_BUTTON_1;
pub static MOUSE_BUTTON_RIGHT           : libc::c_int = MOUSE_BUTTON_2;
pub static MOUSE_BUTTON_MIDDLE          : libc::c_int = MOUSE_BUTTON_3;
pub static MOUSE_BUTTON_LAST            : libc::c_int = MOUSE_BUTTON_8;

/* Joysticks */
pub static JOYSTICK_1                   : libc::c_int = 0;
pub static JOYSTICK_2                   : libc::c_int = 1;
pub static JOYSTICK_3                   : libc::c_int = 2;
pub static JOYSTICK_4                   : libc::c_int = 3;
pub static JOYSTICK_5                   : libc::c_int = 4;
pub static JOYSTICK_6                   : libc::c_int = 5;
pub static JOYSTICK_7                   : libc::c_int = 6;
pub static JOYSTICK_8                   : libc::c_int = 7;
pub static JOYSTICK_9                   : libc::c_int = 8;
pub static JOYSTICK_10                  : libc::c_int = 9;
pub static JOYSTICK_11                  : libc::c_int = 10;
pub static JOYSTICK_12                  : libc::c_int = 11;
pub static JOYSTICK_13                  : libc::c_int = 12;
pub static JOYSTICK_14                  : libc::c_int = 13;
pub static JOYSTICK_15                  : libc::c_int = 14;
pub static JOYSTICK_16                  : libc::c_int = 15;
pub static JOYSTICK_LAST                : libc::c_int = JOYSTICK_16;

/* glfwGetWindowParam tokens */
pub static FOCUSED                      : libc::c_int = 0x00020001;
pub static ICONIFIED                    : libc::c_int = 0x00020002;
pub static CONTEXT_REVISION             : libc::c_int = 0x00020004;

/* glfwWindowHint tokens */
pub static RED_BITS                     : libc::c_int = 0x00021000;
pub static GREEN_BITS                   : libc::c_int = 0x00021001;
pub static BLUE_BITS                    : libc::c_int = 0x00021002;
pub static ALPHA_BITS                   : libc::c_int = 0x00021003;
pub static DEPTH_BITS                   : libc::c_int = 0x00021004;
pub static STENCIL_BITS                 : libc::c_int = 0x00021005;
pub static ACCUM_RED_BITS               : libc::c_int = 0x00021006;
pub static ACCUM_GREEN_BITS             : libc::c_int = 0x00021007;
pub static ACCUM_BLUE_BITS              : libc::c_int = 0x00021008;
pub static ACCUM_ALPHA_BITS             : libc::c_int = 0x00021009;
pub static AUX_BUFFERS                  : libc::c_int = 0x0002100A;
pub static STEREO                       : libc::c_int = 0x0002100B;
pub static SAMPLES                      : libc::c_int = 0x0002100C;
pub static SRGB_CAPABLE                 : libc::c_int = 0x0002100D;

/* Used with both glfwGetWindowParam and glfwWindowHint */
pub static CLIENT_API                   : libc::c_int = 0x00022000;
pub static CONTEXT_VERSION_MAJOR        : libc::c_int = 0x00022001;
pub static CONTEXT_VERSION_MINOR        : libc::c_int = 0x00022002;
pub static CONTEXT_ROBUSTNESS           : libc::c_int = 0x00022003;
pub static OPENGL_FORWARD_COMPAT        : libc::c_int = 0x00022004;
pub static OPENGL_DEBUG_CONTEXT         : libc::c_int = 0x00022005;
pub static OPENGL_PROFILE               : libc::c_int = 0x00022006;
pub static RESIZABLE                    : libc::c_int = 0x00022007;
pub static VISIBLE                      : libc::c_int = 0x00022008;

/* GLFW_CLIENT_API tokens */
pub static OPENGL_API                   : libc::c_int = 0x00000001;
pub static OPENGL_ES_API                : libc::c_int = 0x00000002;

/* GLFW_CONTEXT_ROBUSTNESS mode tokens */
pub static NO_ROBUSTNESS                : libc::c_int = 0x00000000;
pub static NO_RESET_NOTIFICATION        : libc::c_int = 0x00000001;
pub static LOSE_CONTEXT_ON_RESET        : libc::c_int = 0x00000002;

/* GLFW_OPENGL_PROFILE bit tokens */
pub static OPENGL_NO_PROFILE            : libc::c_int = 0x00000000;
pub static OPENGL_CORE_PROFILE          : libc::c_int = 0x00000001;
pub static OPENGL_COMPAT_PROFILE        : libc::c_int = 0x00000002;

/* glfwGetInputMode/glfwSetInputMode tokens */
pub static CURSOR_MODE                  : libc::c_int = 0x00030001;
pub static STICKY_KEYS                  : libc::c_int = 0x00030002;
pub static STICKY_MOUSE_BUTTONS         : libc::c_int = 0x00030003;

/* GLFW_CURSOR_MODE values */
pub static CURSOR_NORMAL                : libc::c_int = 0x00040001;
pub static CURSOR_HIDDEN                : libc::c_int = 0x00040002;
pub static CURSOR_CAPTURED              : libc::c_int = 0x00040003;

/* glfwGetJoystickParam tokens */
pub static PRESENT                      : libc::c_int = 0x00050001;
pub static AXES                         : libc::c_int = 0x00050002;
pub static BUTTONS                      : libc::c_int = 0x00050003;

/* glfwGetError/glfwErrorString tokens */
pub static NOT_INITIALIZED              : libc::c_int = 0x00070001;
pub static NO_CURRENT_CONTEXT           : libc::c_int = 0x00070002;
pub static INVALID_ENUM                 : libc::c_int = 0x00070003;
pub static INVALID_VALUE                : libc::c_int = 0x00070004;
pub static OUT_OF_MEMORY                : libc::c_int = 0x00070005;
pub static API_UNAVAILABLE              : libc::c_int = 0x00070006;
pub static VERSION_UNAVAILABLE          : libc::c_int = 0x00070007;
pub static PLATFORM_ERROR               : libc::c_int = 0x00070008;
pub static FORMAT_UNAVAILABLE           : libc::c_int = 0x00070009;

/* Gamma ramps */
pub static GAMMA_RAMP_SIZE              : libc::c_int = 256;

/* Monitor event tokens */
pub static CONNECTED                    : libc::c_int = 0x00061000;
pub static DISCONNECTED                 : libc::c_int = 0x00061001;