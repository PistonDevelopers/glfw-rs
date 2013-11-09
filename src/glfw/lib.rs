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

#[link(name = "glfw",
       vers = "0.1",
       uuid = "6199FAD3-6D03-4E29-87E7-7DC1B1B65C2C",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/glfw3-rs")];

#[comment = "Bindings and wrapper functions for glfw3."];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];

// TODO: Document differences between GLFW and glfw-rs

use std::cast;
use std::libc::*;
use std::ptr;
use std::str;
use std::vec;

pub mod ffi;
mod extfn;

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Action {
    Release                      = ffi::RELEASE,
    Press                        = ffi::PRESS,
    Repeat                       = ffi::REPEAT,
}

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Key {
    KeySpace                    = ffi::KEY_SPACE,
    KeyApostrophe               = ffi::KEY_APOSTROPHE,
    KeyComma                    = ffi::KEY_COMMA,
    KeyMinus                    = ffi::KEY_MINUS,
    KeyPeriod                   = ffi::KEY_PERIOD,
    KeySlash                    = ffi::KEY_SLASH,
    Key0                        = ffi::KEY_0,
    Key1                        = ffi::KEY_1,
    Key2                        = ffi::KEY_2,
    Key3                        = ffi::KEY_3,
    Key4                        = ffi::KEY_4,
    Key5                        = ffi::KEY_5,
    Key6                        = ffi::KEY_6,
    Key7                        = ffi::KEY_7,
    Key8                        = ffi::KEY_8,
    Key9                        = ffi::KEY_9,
    KeySemicolon                = ffi::KEY_SEMICOLON,
    KeyEqual                    = ffi::KEY_EQUAL,
    KeyA                        = ffi::KEY_A,
    KeyB                        = ffi::KEY_B,
    KeyC                        = ffi::KEY_C,
    KeyD                        = ffi::KEY_D,
    KeyE                        = ffi::KEY_E,
    KeyF                        = ffi::KEY_F,
    KeyG                        = ffi::KEY_G,
    KeyH                        = ffi::KEY_H,
    KeyI                        = ffi::KEY_I,
    KeyJ                        = ffi::KEY_J,
    KeyK                        = ffi::KEY_K,
    KeyL                        = ffi::KEY_L,
    KeyM                        = ffi::KEY_M,
    KeyN                        = ffi::KEY_N,
    KeyO                        = ffi::KEY_O,
    KeyP                        = ffi::KEY_P,
    KeyQ                        = ffi::KEY_Q,
    KeyR                        = ffi::KEY_R,
    KeyS                        = ffi::KEY_S,
    KeyT                        = ffi::KEY_T,
    KeyU                        = ffi::KEY_U,
    KeyV                        = ffi::KEY_V,
    KeyW                        = ffi::KEY_W,
    KeyX                        = ffi::KEY_X,
    KeyY                        = ffi::KEY_Y,
    KeyZ                        = ffi::KEY_Z,
    KeyLeftBracket              = ffi::KEY_LEFT_BRACKET,
    KeyBackslash                = ffi::KEY_BACKSLASH,
    KeyRightBracket             = ffi::KEY_RIGHT_BRACKET,
    KeyGraveAccent              = ffi::KEY_GRAVE_ACCENT,
    KeyWorld1                   = ffi::KEY_WORLD_1,
    KeyWorld2                   = ffi::KEY_WORLD_2,

    KeyEscape                   = ffi::KEY_ESCAPE,
    KeyEnter                    = ffi::KEY_ENTER,
    KeyTab                      = ffi::KEY_TAB,
    KeyBackspace                = ffi::KEY_BACKSPACE,
    KeyInsert                   = ffi::KEY_INSERT,
    KeyDelete                   = ffi::KEY_DELETE,
    KeyRight                    = ffi::KEY_RIGHT,
    KeyLeft                     = ffi::KEY_LEFT,
    KeyDown                     = ffi::KEY_DOWN,
    KeyUp                       = ffi::KEY_UP,
    KeyPageUp                   = ffi::KEY_PAGE_UP,
    KeyPageDown                 = ffi::KEY_PAGE_DOWN,
    KeyHome                     = ffi::KEY_HOME,
    KeyEnd                      = ffi::KEY_END,
    KeyCapsLock                 = ffi::KEY_CAPS_LOCK,
    KeyScrollLock               = ffi::KEY_SCROLL_LOCK,
    KeyNumLock                  = ffi::KEY_NUM_LOCK,
    KeyPrintScreen              = ffi::KEY_PRINT_SCREEN,
    KeyPause                    = ffi::KEY_PAUSE,
    KeyF1                       = ffi::KEY_F1,
    KeyF2                       = ffi::KEY_F2,
    KeyF3                       = ffi::KEY_F3,
    KeyF4                       = ffi::KEY_F4,
    KeyF5                       = ffi::KEY_F5,
    KeyF6                       = ffi::KEY_F6,
    KeyF7                       = ffi::KEY_F7,
    KeyF8                       = ffi::KEY_F8,
    KeyF9                       = ffi::KEY_F9,
    KeyF10                      = ffi::KEY_F10,
    KeyF11                      = ffi::KEY_F11,
    KeyF12                      = ffi::KEY_F12,
    KeyF13                      = ffi::KEY_F13,
    KeyF14                      = ffi::KEY_F14,
    KeyF15                      = ffi::KEY_F15,
    KeyF16                      = ffi::KEY_F16,
    KeyF17                      = ffi::KEY_F17,
    KeyF18                      = ffi::KEY_F18,
    KeyF19                      = ffi::KEY_F19,
    KeyF20                      = ffi::KEY_F20,
    KeyF21                      = ffi::KEY_F21,
    KeyF22                      = ffi::KEY_F22,
    KeyF23                      = ffi::KEY_F23,
    KeyF24                      = ffi::KEY_F24,
    KeyF25                      = ffi::KEY_F25,
    KeyKp0                      = ffi::KEY_KP_0,
    KeyKp1                      = ffi::KEY_KP_1,
    KeyKp2                      = ffi::KEY_KP_2,
    KeyKp3                      = ffi::KEY_KP_3,
    KeyKp4                      = ffi::KEY_KP_4,
    KeyKp5                      = ffi::KEY_KP_5,
    KeyKp6                      = ffi::KEY_KP_6,
    KeyKp7                      = ffi::KEY_KP_7,
    KeyKp8                      = ffi::KEY_KP_8,
    KeyKp9                      = ffi::KEY_KP_9,
    KeyKpDecimal                = ffi::KEY_KP_DECIMAL,
    KeyKpDivide                 = ffi::KEY_KP_DIVIDE,
    KeyKpMultiply               = ffi::KEY_KP_MULTIPLY,
    KeyKpSubtract               = ffi::KEY_KP_SUBTRACT,
    KeyKpAdd                    = ffi::KEY_KP_ADD,
    KeyKpEnter                  = ffi::KEY_KP_ENTER,
    KeyKpEqual                  = ffi::KEY_KP_EQUAL,
    KeyLeftShift                = ffi::KEY_LEFT_SHIFT,
    KeyLeftControl              = ffi::KEY_LEFT_CONTROL,
    KeyLeftAlt                  = ffi::KEY_LEFT_ALT,
    KeyLeftSuper                = ffi::KEY_LEFT_SUPER,
    KeyRightShift               = ffi::KEY_RIGHT_SHIFT,
    KeyRightControl             = ffi::KEY_RIGHT_CONTROL,
    KeyRightAlt                 = ffi::KEY_RIGHT_ALT,
    KeyRightSuper               = ffi::KEY_RIGHT_SUPER,
    KeyMenu                     = ffi::KEY_MENU,
}

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum MouseButton {
    MouseButtonLeft             = ffi::MOUSE_BUTTON_LEFT,
    MouseButtonRight            = ffi::MOUSE_BUTTON_RIGHT,
    MouseButtonMiddle           = ffi::MOUSE_BUTTON_MIDDLE,
    // MouseButton1                = ffi::MOUSE_BUTTON_1,
    // MouseButton2                = ffi::MOUSE_BUTTON_2,
    // MouseButton3                = ffi::MOUSE_BUTTON_3,
    MouseButton4                = ffi::MOUSE_BUTTON_4,
    MouseButton5                = ffi::MOUSE_BUTTON_5,
    MouseButton6                = ffi::MOUSE_BUTTON_6,
    MouseButton7                = ffi::MOUSE_BUTTON_7,
    MouseButton8                = ffi::MOUSE_BUTTON_8,
}

// pub static MouseButtonLeft           : MouseButton = MouseButton1;
// pub static MouseButtonRight          : MouseButton = MouseButton2;
// pub static MouseButtonMiddle         : MouseButton = MouseButton3;

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Error {
    NotInitialized              = ffi::NOT_INITIALIZED,
    NoCurrentContext            = ffi::NO_CURRENT_CONTEXT,
    InvalidEnum                 = ffi::INVALID_ENUM,
    InvalidValue                = ffi::INVALID_VALUE,
    OutOfMemory                 = ffi::OUT_OF_MEMORY,
    ApiUnavailable              = ffi::API_UNAVAILABLE,
    VersionUnavailable          = ffi::VERSION_UNAVAILABLE,
    PlatformError               = ffi::PLATFORM_ERROR,
    FormatUnavailable           = ffi::FORMAT_UNAVAILABLE,
}

pub type ErrorFun = ~fn(error: Error, description: ~str);

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum ClientApi {
    OpenGlApi                   = ffi::OPENGL_API,
    OpenGlEsApi                 = ffi::OPENGL_ES_API,
}

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum ContextRobustness {
    NoRobustness                = ffi::NO_ROBUSTNESS,
    NoResetNotification         = ffi::NO_RESET_NOTIFICATION,
    LoseContextOnReset          = ffi::LOSE_CONTEXT_ON_RESET,
}

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum OpenGlProfile {
    OpenGlAnyProfile            = ffi::OPENGL_ANY_PROFILE,
    OpenGlCoreProfile           = ffi::OPENGL_CORE_PROFILE,
    OpenGlCompatProfile         = ffi::OPENGL_COMPAT_PROFILE,
}

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum CursorMode {
    CursorNormal                = ffi::CURSOR_NORMAL,
    CursorHidden                = ffi::CURSOR_HIDDEN,
    CursorDisabled              = ffi::CURSOR_DISABLED,
}

/// Describes a single video mode.
pub struct VidMode {
    width:        uint,
    height:       uint,
    red_bits:     uint,
    green_bits:   uint,
    blue_bits:    uint,
    refresh_rate: uint,
}

/// Describes the gamma ramp of a monitor.
pub struct GammaRamp {
    red:    ~[c_ushort],
    green:  ~[c_ushort],
    blue:   ~[c_ushort],
}

pub type GLProc = ffi::GLFWglproc;

/// Initialise glfw. This must be called on the main platform thread.
///
/// Returns `true` if the initialisation was successful, otherwise `false`.
///
/// Wrapper for `glfwInit`.
#[fixed_stack_segment] #[inline(never)]
pub fn init() -> Result<(),()> {
    match unsafe { ffi::glfwInit() } {
        ffi::TRUE => Ok(()),
        _         => Err(()),
    }
}

/// Terminate glfw. This must be called on the main platform thread.
///
/// Wrapper for `glfwTerminate`.
#[fixed_stack_segment] #[inline(never)]
pub fn terminate() {
    unsafe { ffi::glfwTerminate() }
}

/// Initialises GLFW, automatically calling `glfw::terminate` on exit or
/// failure. Fails if the initialisation was unsuccessful.
///
/// # Parameters
///
/// - `f`: A closure to be called after the GLFW is initialised.
pub fn start(f: ~fn()) {
    use std::unstable::finally::Finally;
    if init().is_ok() {
        f.finally(terminate);
    } else {
        fail!(~"Failed to initialize GLFW");
    }
}

/// Holds the version information of the underlying GLFW library
pub struct Version {
    major: uint,
    minor: uint,
    rev:   uint,
}

impl ToStr for Version {
    /// Returns a string representation of the version struct.
    ///
    /// # Returns
    ///
    /// A string in the form:
    ///
    /// ~~~
    /// ~"[major].[minor].[rev]"
    /// ~~~
    fn to_str(&self) -> ~str {
        format!("{}.{}.{}", self.major, self.minor, self.rev)
    }
}

/// Wrapper for `glfwGetVersion`.
#[fixed_stack_segment] #[inline(never)]
pub fn get_version() -> Version {
    unsafe {
        let mut major = 0;
        let mut minor = 0;
        let mut rev = 0;
        ffi::glfwGetVersion(&mut major, &mut minor, &mut rev);
        Version {
            major: major as uint,
            minor: minor as uint,
            rev:   rev   as uint,
        }
    }
}

/// Wrapper for `glfwGetVersionString`.
#[fixed_stack_segment] #[inline(never)]
pub fn get_version_string() -> ~str {
    unsafe { str::raw::from_c_str(ffi::glfwGetVersionString()) }
}

/// Wrapper for `glfwSetErrorCallback`.
#[fixed_stack_segment] #[inline(never)]
pub fn set_error_callback(cbfun: ErrorFun) {
    do extfn::set_error_fun(cbfun) |ext_cb| {
        unsafe { ffi::glfwSetErrorCallback(Some(ext_cb)); }
    }
}

/// A struct that wraps a `*GLFWmonitor` handle.
#[deriving(Eq)]
pub struct Monitor {
    ptr: *ffi::GLFWmonitor
}

impl Monitor {
    /// Wrapper for `glfwGetPrimaryMonitor`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_primary() -> Result<Monitor,()> {
        unsafe {
            ffi::glfwGetPrimaryMonitor()
             .to_option()
             .map_default(Err(()),
                |ptr| Ok(Monitor { ptr: ptr }))
        }
    }

    /// Wrapper for `glfwGetMonitors`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_connected() -> ~[Monitor] {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetMonitors(&mut count);
            vec::from_buf(ptr, count as uint).map(|&m| Monitor { ptr: m })
        }
    }

    /// Wrapper for `glfwGetMonitorPos`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_pos(&self) -> (int, int) {
        unsafe {
            let mut xpos = 0;
            let mut ypos = 0;
            ffi::glfwGetMonitorPos(self.ptr, &mut xpos, &mut ypos);
            (xpos as int, ypos as int)
        }
    }

    /// Wrapper for `glfwGetMonitorPhysicalSize`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_physical_size(&self) -> (int, int) {
        unsafe {
            let mut width = 0;
            let mut height = 0;
            ffi::glfwGetMonitorPhysicalSize(self.ptr, &mut width, &mut height);
            (width as int, height as int)
        }
    }

    /// Wrapper for `glfwGetMonitorName`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_name(&self) -> ~str {
        unsafe { str::raw::from_c_str(ffi::glfwGetMonitorName(self.ptr)) }
    }

    /// Wrapper for `glfwSetMonitorCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_callback(cbfun: MonitorFun) {
        do extfn::set_monitor_fun(cbfun) |ext_cb| {
            unsafe { ffi::glfwSetMonitorCallback(Some(ext_cb)); }
        }
    }

    /// Wrapper for `glfwGetVideoModes`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_video_modes(&self) -> ~[VidMode] {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetVideoModes(self.ptr, &mut count);
            vec::from_buf(ptr, count as uint).map(VidMode::from_glfw_vid_mode)
        }
    }

    /// Wrapper for `glfwGetVideoMode`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_video_mode(&self) -> Option<VidMode> {
        unsafe {
            ffi::glfwGetVideoMode(self.ptr).to_option().map(|v| VidMode::from_glfw_vid_mode(v))
        }
    }

    /// Wrapper for `glfwSetGamma`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_gamma(&self, gamma: f32) {
        unsafe { ffi::glfwSetGamma(self.ptr, gamma as c_float); }
    }

    /// Wrapper for `glfwGetGammaRamp`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_gamma_ramp(&self) -> GammaRamp {
        unsafe {
            let llramp = *ffi::glfwGetGammaRamp(self.ptr);
            GammaRamp {
                red:    vec::from_buf(llramp.red,   llramp.size as uint),
                green:  vec::from_buf(llramp.green, llramp.size as uint),
                blue:   vec::from_buf(llramp.blue,  llramp.size as uint),
            }
        }
    }

    /// Wrapper for `glfwSetGammaRamp`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_gamma_ramp(&self, ramp: &GammaRamp) {
        unsafe {
            ffi::glfwSetGammaRamp(
                self.ptr,
                &ffi::GLFWgammaramp {
                    red:    vec::raw::to_ptr(ramp.red),
                    green:  vec::raw::to_ptr(ramp.green),
                    blue:   vec::raw::to_ptr(ramp.blue),
                    size:   ramp.red.len() as c_uint,
                }
            );
        }
    }
}

pub type MonitorFun = ~fn(monitor: &Monitor, event: MonitorEvent);
#[repr(C)]

pub enum MonitorEvent {
    Connected                   = ffi::CONNECTED,
    Disconnected                = ffi::DISCONNECTED,
}

impl VidMode {
    fn from_glfw_vid_mode(mode: &ffi::GLFWvidmode) -> VidMode {
        VidMode {
            width:        mode.width as uint,
            height:       mode.height as uint,
            red_bits:     mode.redBits as uint,
            green_bits:   mode.greenBits as uint,
            blue_bits:    mode.blueBits as uint,
            refresh_rate: mode.refreshRate as uint,
        }
    }
}

impl ToStr for VidMode {
    /// Returns a string representation of the video mode.
    ///
    /// # Returns
    ///
    /// A string in the form:
    ///
    /// ~~~
    /// ~"[width] x [height], [total_bits] ([red_bits] [green_bits] [blue_bits]) [refresh_rate] Hz"
    /// ~~~
    fn to_str(&self) -> ~str {
        format!("{} x {}, {} ({} {} {}) {} Hz",
                self.width, self.height,
                self.red_bits + self.green_bits + self.blue_bits,
                self.red_bits, self.green_bits, self.blue_bits,
                self.refresh_rate)
    }
}

pub mod window_hint {
    use std::libc::c_int;
    use super::*;

    /// Wrapper for `glfwDefaultWindowHints`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn default() {
        unsafe { ffi::glfwDefaultWindowHints(); }
    }

    /// Wrapper for `glfwWindowHint` called with `RED_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn red_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::RED_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `GREEN_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn green_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::GREEN_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `BLUE_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn blue_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::BLUE_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ALPHA_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn alpha_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::ALPHA_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `DEPTH_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn depth_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::DEPTH_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `STENCIL_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn stencil_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::STENCIL_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_RED_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn accum_red_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::ACCUM_RED_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_GREEN_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn accum_green_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::ACCUM_GREEN_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_BLUE_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn accum_blue_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::ACCUM_BLUE_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_ALPHA_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn accum_alpha_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ffi::ACCUM_ALPHA_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `AUX_BUFFERS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn aux_buffers(buffers: uint) {
        unsafe { ffi::glfwWindowHint(ffi::AUX_BUFFERS, buffers as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `STEREO`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn stereo(value: bool) {
        unsafe { ffi::glfwWindowHint(ffi::STEREO, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `SAMPLES`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn samples(samples: uint) {
        unsafe { ffi::glfwWindowHint(ffi::SAMPLES, samples as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `SRGB_CAPABLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn srgb_capable(value: bool) {
        unsafe { ffi::glfwWindowHint(ffi::SRGB_CAPABLE, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `REFRESH_RATE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn refresh_rate(rate: int) {
        unsafe { ffi::glfwWindowHint(ffi::REFRESH_RATE, rate as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CLIENT_API`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn client_api(api: ClientApi) {
        unsafe { ffi::glfwWindowHint(ffi::CLIENT_API, api as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MAJOR`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn context_version_major(major: uint) {
        unsafe { ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, major as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MINOR`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn context_version_minor(minor: uint) {
        unsafe { ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, minor as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MAJOR` and
    /// `CONTEXT_VERSION_MINOR`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn context_version(major: uint, minor: uint) {
        unsafe {
            ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, major as c_int);
            ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, minor as c_int);
        }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_ROBUSTNESS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn context_robustness(value: ContextRobustness) {
        unsafe { ffi::glfwWindowHint(ffi::CONTEXT_ROBUSTNESS, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_FORWARD_COMPAT`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn opengl_forward_compat(value: bool) {
        unsafe { ffi::glfwWindowHint(ffi::OPENGL_FORWARD_COMPAT, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_DEBUG_CONTEXT`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn opengl_debug_context(value: bool) {
        unsafe { ffi::glfwWindowHint(ffi::OPENGL_DEBUG_CONTEXT, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_PROFILE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn opengl_profile(profile: OpenGlProfile) {
        unsafe { ffi::glfwWindowHint(ffi::OPENGL_PROFILE, profile as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `RESIZABLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn resizable(value: bool) {
        unsafe { ffi::glfwWindowHint(ffi::RESIZABLE, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `VISIBLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn visible(value: bool) {
        unsafe { ffi::glfwWindowHint(ffi::VISIBLE, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `DECORATED`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn decorated(value: bool) {
        unsafe { ffi::glfwWindowHint(ffi::DECORATED, value as c_int); }
    }
}

/// Describes the mode of a window
pub enum WindowMode {
    /// Full screen mode. Contains the monitor on which the window is displayed.
    FullScreen(Monitor),

    /// Windowed mode.
    Windowed,
}

/// Private conversion methods for `glfw::WindowMode`
impl WindowMode {
    /// Extract the window mode from a low-level monitor pointer. If the pointer
    /// is null it assumes the window is in windowed mode and returns `Windowed`,
    /// otherwise it returns the pointer wrapped in `glfw::FullScreen`.
    fn from_ptr(ptr: *ffi::GLFWmonitor) -> WindowMode {
        if ptr.is_null() {
            Windowed
        } else {
            FullScreen(Monitor { ptr: ptr })
        }
    }

    /// Returns a pointer to a monitor if the window is fullscreen, otherwise
    /// it returns a null pointer (if it is in windowed mode).
    fn to_ptr(&self) -> *ffi::GLFWmonitor {
        match *self {
            FullScreen(monitor) => monitor.ptr,
            Windowed => ptr::null()
        }
    }
}

/// A struct that wraps a `*GLFWwindow` handle.
pub struct Window {
    ptr: *ffi::GLFWwindow,
    is_shared: bool,
}

/// A group of key modifiers
pub struct Modifiers {
    values: c_int,
}

/// Key modifier tokens
#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Modifier {
    Shift       = ffi::MOD_SHIFT,
    Control     = ffi::MOD_CONTROL,
    Alt         = ffi::MOD_ALT,
    Super       = ffi::MOD_SUPER,
}

impl Modifiers {
    /// Check to see if a specific key modifier is present
    ///
    /// # Example
    ///
    /// ~~~rust
    /// do window.set_key_callback |_, _, _, _, mods| {
    ///     if mods.contains(glfw::Shift) {
    ///         println("Shift detected!")
    ///     }
    /// }
    /// ~~~
    pub fn contains(&self, modifier: Modifier) -> bool {
        self.values & (modifier as c_int) != ffi::FALSE
    }
}

impl ToStr for Modifiers {
    fn to_str(&self) -> ~str {
        let mut ss = ~[];
        if self.contains(Shift)   { ss.push(Shift.to_str())   }
        if self.contains(Control) { ss.push(Control.to_str()) }
        if self.contains(Alt)     { ss.push(Alt.to_str())     }
        if self.contains(Super)   { ss.push(Super.to_str())   }
        ss.connect(", ")
    }
}

pub type WindowPosFun = ~fn(window: &Window, xpos: int, ypos: int);
pub type WindowSizeFun = ~fn(window: &Window, width: int, height: int);
pub type WindowCloseFun = ~fn(window: &Window);
pub type WindowRefreshFun = ~fn(window: &Window);
pub type WindowFocusFun = ~fn(window: &Window, focused: bool);
pub type WindowIconifyFun = ~fn(window: &Window, iconified: bool);
pub type FramebufferSizeFun = ~fn(window: &Window, width: int, height: int);
pub type MouseButtonFun = ~fn(window: &Window, button: MouseButton, action: Action, modifiers: Modifiers);
pub type CursorPosFun = ~fn(window: &Window, xpos: f64, ypos: f64);
pub type CursorEnterFun = ~fn(window: &Window, entered: bool);
pub type ScrollFun = ~fn(window: &Window, xpos: f64, ypos: f64);
pub type KeyFun = ~fn(window: &Window, key: Key, scancode: c_int, action: Action, modifiers: Modifiers);
pub type CharFun = ~fn(window: &Window, character: char);

/// Holds the callback functions associated with a window
struct WindowFns {
    pos_fun:                Option<WindowPosFun>,
    size_fun:               Option<WindowSizeFun>,
    close_fun:              Option<WindowCloseFun>,
    refresh_fun:            Option<WindowRefreshFun>,
    focus_fun:              Option<WindowFocusFun>,
    iconify_fun:            Option<WindowIconifyFun>,
    framebuffer_size_fun:   Option<FramebufferSizeFun>,
    mouse_button_fun:       Option<MouseButtonFun>,
    cursor_pos_fun:         Option<CursorPosFun>,
    cursor_enter_fun:       Option<CursorEnterFun>,
    scroll_fun:             Option<ScrollFun>,
    key_fun:                Option<KeyFun>,
    char_fun:               Option<CharFun>,
}

impl WindowFns {
    /// Initialize the struct with all callbacks set to `None`.
    fn new() -> WindowFns {
        WindowFns {
            pos_fun:                None,
            size_fun:               None,
            close_fun:              None,
            refresh_fun:            None,
            focus_fun:              None,
            iconify_fun:            None,
            framebuffer_size_fun:   None,
            mouse_button_fun:       None,
            cursor_pos_fun:         None,
            cursor_enter_fun:       None,
            scroll_fun:             None,
            key_fun:                None,
            char_fun:               None,
        }
    }
}

macro_rules! set_window_callback(
    (
        setter:   $ll_fn:ident,
        callback: $ext_fn:ident,
        field:    $data_field:ident
    ) => ({
        unsafe {
            self.get_fns().$data_field = Some(cbfun);
            ffi::$ll_fn(self.ptr, Some(extfn::$ext_fn));
        }
    })
)

impl Window {
    /// Wrapper for `glfwCreateWindow`.
    pub fn create(width: uint, height: uint, title: &str, mode: WindowMode) -> Option<Window> {
        Window::create_intern(width, height, title, mode, None)
    }

    /// Wrapper for `glfwCreateWindow`.
    pub fn create_shared(&self, width: uint, height: uint, title: &str, mode: WindowMode) -> Option<Window> {
        Window::create_intern(width, height, title, mode, Some(self))
    }

    /// Internal wrapper for `glfwCreateWindow`.
    #[fixed_stack_segment] #[inline(never)]
    fn create_intern(width: uint, height: uint, title: &str, mode: WindowMode, share: Option<&Window>) -> Option<Window> {
        let ptr = unsafe {
            do title.with_c_str |title| {
                ffi::glfwCreateWindow(
                    width as c_int,
                    height as c_int,
                    title,
                    mode.to_ptr(),
                    match share { Some(w) => w.ptr, None => ptr::null() }
                )
            }
        };

        if ptr.is_null() {
            None
        } else {
            unsafe {
                ffi::glfwSetWindowUserPointer(ptr, cast::transmute(~WindowFns::new()));
            }
            let window = Window {
                ptr: ptr,
                is_shared: share.is_none(),
            };
            Some(window)
        }
    }

    #[fixed_stack_segment]
    unsafe fn get_fns(&self) -> &mut WindowFns {
        cast::transmute(ffi::glfwGetWindowUserPointer(self.ptr))
    }

    #[fixed_stack_segment]
    unsafe fn free_fns(&self) {
        if !self.ptr.is_null() {
            let _: ~WindowFns =
                cast::transmute(ffi::glfwGetWindowUserPointer(self.ptr));
        }
    }

    pub fn close(self) {
        // Calling this method forces the destructor to be called, closing the window
    }

    /// Wrapper for `glfwWindowShouldClose`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn should_close(&self) -> bool {
        unsafe { ffi::glfwWindowShouldClose(self.ptr) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowShouldClose`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_should_close(&self, value: bool) {
        unsafe { ffi::glfwSetWindowShouldClose(self.ptr, value as c_int) }
    }

    /// Wrapper for `glfwSetWindowTitle`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_title(&self, title: &str) {
        unsafe {
            do title.with_c_str |title| {
                ffi::glfwSetWindowTitle(self.ptr, title);
            }
        }
    }

    /// Wrapper for `glfwGetWindowPos`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_pos(&self) -> (int, int) {
        unsafe {
            let mut xpos = 0;
            let mut ypos = 0;
            ffi::glfwGetWindowPos(self.ptr, &mut xpos, &mut ypos);
            (xpos as int, ypos as int)
        }
    }

    /// Wrapper for `glfwSetWindowPos`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_pos(&self, xpos: int, ypos: int) {
        unsafe { ffi::glfwSetWindowPos(self.ptr, xpos as c_int, ypos as c_int); }
    }

    /// Wrapper for `glfwGetWindowSize`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_size(&self) -> (int, int) {
        unsafe {
            let mut width = 0;
            let mut height = 0;
            ffi::glfwGetWindowSize(self.ptr, &mut width, &mut height);
            (width as int, height as int)
        }
    }

    /// Wrapper for `glfwSetWindowSize`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_size(&self, width: int, height: int) {
        unsafe { ffi::glfwSetWindowSize(self.ptr, width as c_int, height as c_int); }
    }

    /// Wrapper for `glfwGetFramebufferSize`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_framebuffer_size(&self) -> (int, int) {
        unsafe {
            let mut width = 0;
            let mut height = 0;
            ffi::glfwGetFramebufferSize(self.ptr, &mut width, &mut height);
            (width as int, height as int)
        }
    }

    /// Wrapper for `glfwIconifyWindow`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn iconify(&self) {
        unsafe { ffi::glfwIconifyWindow(self.ptr); }
    }

    /// Wrapper for `glfwRestoreWindow`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn restore(&self) {
        unsafe { ffi::glfwRestoreWindow(self.ptr); }
    }

    /// Wrapper for `glfwShowWindow`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn show(&self) {
        unsafe { ffi::glfwShowWindow(self.ptr); }
    }

    /// Wrapper for `glfwHideWindow`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn hide(&self) {
        unsafe { ffi::glfwHideWindow(self.ptr); }
    }

    /// Wrapper for `glfwGetWindowMonitor`.
    ///
    /// # Returns
    ///
    /// The window mode; either glfw::FullScreen or glfw::Windowed
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_window_mode(&self) -> WindowMode {
        WindowMode::from_ptr(
            unsafe { ffi::glfwGetWindowMonitor(self.ptr) }
        )
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `FOCUSED`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_focused(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::FOCUSED) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `ICONIFIED`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_iconified(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::ICONIFIED) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `CLIENT_API`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_client_api(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::CLIENT_API) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with
    /// `CONTEXT_VERSION_MAJOR`, `CONTEXT_VERSION_MINOR` and `CONTEXT_REVISION`.
    ///
    /// # Returns
    ///
    /// The client API version of the window's context in a version struct.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_context_version(&self) -> Version {
        unsafe {
            Version {
                major:  ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_VERSION_MAJOR) as uint,
                minor:  ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_VERSION_MINOR) as uint,
                rev:    ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_REVISION) as uint,
            }
        }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `CONTEXT_ROBUSTNESS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_context_robustness(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_ROBUSTNESS) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_FORWARD_COMPAT`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_opengl_forward_compat(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::OPENGL_FORWARD_COMPAT) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_DEBUG_CONTEXT`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_opengl_debug_context(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::OPENGL_DEBUG_CONTEXT) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_PROFILE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_opengl_profile(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::OPENGL_PROFILE) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `RESIZABLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_resizable(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::RESIZABLE) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `VISIBLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_visible(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::VISIBLE) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `DECORATED`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_decorated(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::DECORATED) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowPosCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_pos_callback(&self, cbfun: WindowSizeFun) {
        set_window_callback!(setter:   glfwSetWindowPosCallback,
                             callback: window_pos_callback,
                             field:    pos_fun);
    }

    /// Wrapper for `glfwSetWindowSizeCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_size_callback(&self, cbfun: WindowSizeFun) {
        set_window_callback!(setter:   glfwSetWindowSizeCallback,
                             callback: window_size_callback,
                             field:    size_fun);
    }

    /// Wrapper for `glfwSetWindowCloseCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_close_callback(&self, cbfun: WindowCloseFun) {
        set_window_callback!(setter:   glfwSetWindowCloseCallback,
                             callback: window_close_callback,
                             field:    close_fun);
    }

    /// Wrapper for `glfwSetWindowRefreshCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_refresh_callback(&self, cbfun: WindowRefreshFun) {
        set_window_callback!(setter:   glfwSetWindowRefreshCallback,
                             callback: window_refresh_callback,
                             field:    refresh_fun);
    }

    /// Wrapper for `glfwSetWindowFocusCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_focus_callback(&self, cbfun: WindowFocusFun) {
        set_window_callback!(setter:   glfwSetWindowFocusCallback,
                             callback: window_focus_callback,
                             field:    focus_fun);
    }

    /// Wrapper for `glfwSetWindowIconifyCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_iconify_callback(&self, cbfun: WindowIconifyFun) {
        set_window_callback!(setter:   glfwSetWindowIconifyCallback,
                             callback: window_iconify_callback,
                             field:    iconify_fun);
    }

    /// Wrapper for `glfwSetFramebufferSizeCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_framebuffer_size_callback(&self, cbfun: FramebufferSizeFun) {
        set_window_callback!(setter:   glfwSetFramebufferSizeCallback,
                             callback: framebuffer_size_callback,
                             field:    framebuffer_size_fun);
    }

    /// Wrapper for `glfwGetInputMode` called with `CURSOR`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_cursor_mode(&self) -> CursorMode {
        unsafe { cast::transmute(ffi::glfwGetInputMode(self.ptr, ffi::CURSOR)) }
    }

    /// Wrapper for `glfwSetInputMode` called with `CURSOR`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_cursor_mode(&self, mode: CursorMode) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::CURSOR, mode as c_int); }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_KEYS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn has_sticky_keys(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_KEYS) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_KEYS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_sticky_keys(&self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::STICKY_KEYS, value as c_int); }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn has_sticky_mouse_buttons(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_sticky_mouse_buttons(&self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS, value as c_int); }
    }

    /// Wrapper for `glfwGetKey`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_key(&self, key: Key) -> Action {
        unsafe { cast::transmute(ffi::glfwGetKey(self.ptr, key as c_int)) }
    }

    /// Wrapper for `glfwGetMouseButton`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_mouse_button(&self, button: MouseButton) -> Action {
        unsafe { cast::transmute(ffi::glfwGetMouseButton(self.ptr, button as c_int)) }
    }

    /// Wrapper for `glfwGetCursorPos`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_cursor_pos(&self) -> (f64, f64) {
        unsafe {
            let mut xpos = 0.0;
            let mut ypos = 0.0;
            ffi::glfwGetCursorPos(self.ptr, &mut xpos, &mut ypos);
            (xpos as f64, ypos as f64)
        }
    }

    /// Wrapper for `glfwSetCursorPos`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_cursor_pos(&self, xpos: f64, ypos: f64) {
        unsafe { ffi::glfwSetCursorPos(self.ptr, xpos as c_double, ypos as c_double); }
    }

    /// Wrapper for `glfwSetKeyCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_key_callback(&self, cbfun: KeyFun) {
        set_window_callback!(setter:   glfwSetKeyCallback,
                             callback: key_callback,
                             field:    key_fun);
    }

    /// Wrapper for `glfwSetCharCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_char_callback(&self, cbfun: CharFun) {
        set_window_callback!(setter:   glfwSetCharCallback,
                             callback: char_callback,
                             field:    char_fun);
    }

    /// Wrapper for `glfwSetMouseButtonCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_mouse_button_callback(&self, cbfun: MouseButtonFun) {
        set_window_callback!(setter:   glfwSetMouseButtonCallback,
                             callback: mouse_button_callback,
                             field:    mouse_button_fun);
    }

    /// Wrapper for `glfwSetCursorPosCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_cursor_pos_callback(&self, cbfun: CursorPosFun) {
        set_window_callback!(setter:   glfwSetCursorPosCallback,
                             callback: cursor_pos_callback,
                             field:    cursor_pos_fun);
    }

    /// Wrapper for `glfwSetCursorEnterCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_cursor_enter_callback(&self, cbfun: CursorEnterFun) {
        set_window_callback!(setter:   glfwSetCursorEnterCallback,
                             callback: cursor_enter_callback,
                             field:    cursor_enter_fun);
    }

    /// Wrapper for `glfwSetScrollCallback`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_scroll_callback(&self, cbfun: ScrollFun) {
        set_window_callback!(setter:   glfwSetScrollCallback,
                             callback: scroll_callback,
                             field:    scroll_fun);
    }

    /// Wrapper for `glfwGetClipboardString`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_clipboard_string(&self, string: &str) {
        unsafe {
            do string.with_c_str |string| {
                ffi::glfwSetClipboardString(self.ptr, string);
            }
        }
    }

    /// Wrapper for `glfwGetClipboardString`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_clipboard_string(&self) -> ~str {
        unsafe { str::raw::from_c_str(ffi::glfwGetClipboardString(self.ptr)) }
    }

    /// Wrapper for `glfwMakeContextCurrent`.
    pub fn make_context_current(&self) {
        make_context_current(Some(self));
    }

    /// Wrapper for `glfwGetCurrentContext`
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_current_context(&self) -> bool {
        self.ptr == unsafe { ffi::glfwGetCurrentContext() }
    }

    /// Wrapper for `glfwSwapBuffers`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn swap_buffers(&self) {
        unsafe { ffi::glfwSwapBuffers(self.ptr); }
    }

    /// Wrapper for `glfwGetWin32Window`
    #[cfg(target_os="win32")]
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_win32_window(&self) -> *c_void {
        unsafe { ffi::glfwGetWin32Window(self.ptr) }
    }

    /// Wrapper for `glfwGetWGLContext`
    #[cfg(target_os="win32")]
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_wgl_context(&self) -> *c_void {
        unsafe { ffi::glfwGetWGLContext(self.ptr) }
    }

    /// Wrapper for `glfwGetCocoaWindow`
    #[cfg(target_os="macos")]
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_cocoa_window(&self) -> *c_void {
        unsafe { ffi::glfwGetCocoaWindow(self.ptr) }
    }

    /// Wrapper for `glfwGetNSGLContext`
    #[cfg(target_os="macos")]
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_nsgl_context(&self) -> *c_void {
        unsafe { ffi::glfwGetNSGLContext(self.ptr) }
    }

    /// Wrapper for `glfwGetX11Window`
    #[cfg(target_os="linux")]
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_x11_window(&self) -> *c_void {
        unsafe { ffi::glfwGetX11Window(self.ptr) }
    }

    /// Wrapper for `glfwGetGLXContext`
    #[cfg(target_os="linux")]
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_glx_context(&self) -> *c_void {
        unsafe { ffi::glfwGetGLXContext(self.ptr) }
    }
}

/// Wrapper for `glfwMakeContextCurrent`.
#[fixed_stack_segment] #[inline(never)]
pub fn make_context_current(context: Option<&Window>) {
    match context {
        Some(window) => unsafe { ffi::glfwMakeContextCurrent(window.ptr) },
        None         => unsafe { ffi::glfwMakeContextCurrent(ptr::null()) },
    }
}

/// Wrapper for `glfwGetX11Display`
#[cfg(target_os="linux")]
#[fixed_stack_segment] #[inline(never)]
pub fn get_x11_display() -> *c_void {
    unsafe { ffi::glfwGetX11Display() }
}

#[unsafe_destructor]
impl Drop for Window {
    /// Closes the window and removes all associated callbacks.
    ///
    /// Wrapper for `glfwDestroyWindow`.
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) {
        if !self.is_shared {
            unsafe { ffi::glfwDestroyWindow(self.ptr); }
        }

        unsafe { self.free_fns() }
    }
}

/// Wrapper for `glfwPollEvents`.
#[fixed_stack_segment] #[inline(never)]
pub fn poll_events() {
    unsafe { ffi::glfwPollEvents(); }
}

/// Wrapper for `glfwWaitEvents`.
#[fixed_stack_segment] #[inline(never)]
pub fn wait_events() {
    unsafe { ffi::glfwWaitEvents(); }
}

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Joystick {
    Joystick1       = ffi::JOYSTICK_1,
    Joystick2       = ffi::JOYSTICK_2,
    Joystick3       = ffi::JOYSTICK_3,
    Joystick4       = ffi::JOYSTICK_4,
    Joystick5       = ffi::JOYSTICK_5,
    Joystick6       = ffi::JOYSTICK_6,
    Joystick7       = ffi::JOYSTICK_7,
    Joystick8       = ffi::JOYSTICK_8,
    Joystick9       = ffi::JOYSTICK_9,
    Joystick10      = ffi::JOYSTICK_10,
    Joystick11      = ffi::JOYSTICK_11,
    Joystick12      = ffi::JOYSTICK_12,
    Joystick13      = ffi::JOYSTICK_13,
    Joystick14      = ffi::JOYSTICK_14,
    Joystick15      = ffi::JOYSTICK_15,
    Joystick16      = ffi::JOYSTICK_16,
}

impl Joystick {
    /// Wrapper for `glfwJoystickPresent`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_present(&self) -> bool {
        unsafe { ffi::glfwJoystickPresent(*self as c_int) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetJoystickAxes`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_axes(&self) -> ~[f32] {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetJoystickAxes(*self as c_int, &mut count);
            vec::from_buf(ptr, count as uint).map(|&a| a as f32)
        }
    }

    /// Wrapper for `glfwGetJoystickButtons`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_buttons(&self) -> ~[c_int] {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetJoystickButtons(*self as c_int, &mut count);
            vec::from_buf(ptr, count as uint).map(|&b| b as c_int)
        }
    }

    /// Wrapper for `glfwGetJoystickName`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_name(&self) -> ~str {
        unsafe { str::raw::from_c_str(ffi::glfwGetJoystickName(*self as c_int)) }
    }
}

/// Wrapper for `glfwGetTime`.
#[fixed_stack_segment] #[inline(never)]
pub fn get_time() -> f64 {
    unsafe { ffi::glfwGetTime() as f64 }
}

/// Wrapper for `glfwSetTime`.
#[fixed_stack_segment] #[inline(never)]
pub fn set_time(time: f64) {
    unsafe { ffi::glfwSetTime(time as c_double); }
}

/// Wrapper for `glfwSwapInterval`.
#[fixed_stack_segment] #[inline(never)]
pub fn set_swap_interval(interval: int) {
    unsafe { ffi::glfwSwapInterval(interval as c_int); }
}

/// Wrapper for `glfwExtensionSupported`.
#[fixed_stack_segment] #[inline(never)]
pub fn extension_supported(extension: &str) -> bool {
    unsafe {
        do extension.with_c_str |extension| {
            ffi::glfwExtensionSupported(extension) == ffi::TRUE
        }
    }
}

/// Wrapper for `glfwGetProcAddress`.
#[fixed_stack_segment] #[inline(never)]
pub fn get_proc_address(procname: &str) -> Option<GLProc> {
    unsafe {
        do procname.with_c_str |procname| {
            ffi::glfwGetProcAddress(procname)
        }
    }
}
