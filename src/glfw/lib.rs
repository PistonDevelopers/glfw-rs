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

// TODO: Document differences between GLFW and glfw-rs

use std::cast;
use std::libc::*;
use std::ptr;
use std::str;
use std::vec;

pub mod ffi;
mod extfn;

#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Action {
    Release                      = ffi::RELEASE                     as int,
    Press                        = ffi::PRESS                       as int,
    Repeat                       = ffi::REPEAT                      as int,
}

#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Key {
    KeySpace                    = ffi::KEY_SPACE                    as int,
    KeyApostrophe               = ffi::KEY_APOSTROPHE               as int,
    KeyComma                    = ffi::KEY_COMMA                    as int,
    KeyMinus                    = ffi::KEY_MINUS                    as int,
    KeyPeriod                   = ffi::KEY_PERIOD                   as int,
    KeySlash                    = ffi::KEY_SLASH                    as int,
    Key0                        = ffi::KEY_0                        as int,
    Key1                        = ffi::KEY_1                        as int,
    Key2                        = ffi::KEY_2                        as int,
    Key3                        = ffi::KEY_3                        as int,
    Key4                        = ffi::KEY_4                        as int,
    Key5                        = ffi::KEY_5                        as int,
    Key6                        = ffi::KEY_6                        as int,
    Key7                        = ffi::KEY_7                        as int,
    Key8                        = ffi::KEY_8                        as int,
    Key9                        = ffi::KEY_9                        as int,
    KeySemicolon                = ffi::KEY_SEMICOLON                as int,
    KeyEqual                    = ffi::KEY_EQUAL                    as int,
    KeyA                        = ffi::KEY_A                        as int,
    KeyB                        = ffi::KEY_B                        as int,
    KeyC                        = ffi::KEY_C                        as int,
    KeyD                        = ffi::KEY_D                        as int,
    KeyE                        = ffi::KEY_E                        as int,
    KeyF                        = ffi::KEY_F                        as int,
    KeyG                        = ffi::KEY_G                        as int,
    KeyH                        = ffi::KEY_H                        as int,
    KeyI                        = ffi::KEY_I                        as int,
    KeyJ                        = ffi::KEY_J                        as int,
    KeyK                        = ffi::KEY_K                        as int,
    KeyL                        = ffi::KEY_L                        as int,
    KeyM                        = ffi::KEY_M                        as int,
    KeyN                        = ffi::KEY_N                        as int,
    KeyO                        = ffi::KEY_O                        as int,
    KeyP                        = ffi::KEY_P                        as int,
    KeyQ                        = ffi::KEY_Q                        as int,
    KeyR                        = ffi::KEY_R                        as int,
    KeyS                        = ffi::KEY_S                        as int,
    KeyT                        = ffi::KEY_T                        as int,
    KeyU                        = ffi::KEY_U                        as int,
    KeyV                        = ffi::KEY_V                        as int,
    KeyW                        = ffi::KEY_W                        as int,
    KeyX                        = ffi::KEY_X                        as int,
    KeyY                        = ffi::KEY_Y                        as int,
    KeyZ                        = ffi::KEY_Z                        as int,
    KeyLeftBracket              = ffi::KEY_LEFT_BRACKET             as int,
    KeyBackslash                = ffi::KEY_BACKSLASH                as int,
    KeyRightBracket             = ffi::KEY_RIGHT_BRACKET            as int,
    KeyGraveAccent              = ffi::KEY_GRAVE_ACCENT             as int,
    KeyWorld1                   = ffi::KEY_WORLD_1                  as int,
    KeyWorld2                   = ffi::KEY_WORLD_2                  as int,

    KeyEscape                   = ffi::KEY_ESCAPE                   as int,
    KeyEnter                    = ffi::KEY_ENTER                    as int,
    KeyTab                      = ffi::KEY_TAB                      as int,
    KeyBackspace                = ffi::KEY_BACKSPACE                as int,
    KeyInsert                   = ffi::KEY_INSERT                   as int,
    KeyDelete                   = ffi::KEY_DELETE                   as int,
    KeyRight                    = ffi::KEY_RIGHT                    as int,
    KeyLeft                     = ffi::KEY_LEFT                     as int,
    KeyDown                     = ffi::KEY_DOWN                     as int,
    KeyUp                       = ffi::KEY_UP                       as int,
    KeyPageUp                   = ffi::KEY_PAGE_UP                  as int,
    KeyPageDown                 = ffi::KEY_PAGE_DOWN                as int,
    KeyHome                     = ffi::KEY_HOME                     as int,
    KeyEnd                      = ffi::KEY_END                      as int,
    KeyCapsLock                 = ffi::KEY_CAPS_LOCK                as int,
    KeyScrollLock               = ffi::KEY_SCROLL_LOCK              as int,
    KeyNumLock                  = ffi::KEY_NUM_LOCK                 as int,
    KeyPrintScreen              = ffi::KEY_PRINT_SCREEN             as int,
    KeyPause                    = ffi::KEY_PAUSE                    as int,
    KeyF1                       = ffi::KEY_F1                       as int,
    KeyF2                       = ffi::KEY_F2                       as int,
    KeyF3                       = ffi::KEY_F3                       as int,
    KeyF4                       = ffi::KEY_F4                       as int,
    KeyF5                       = ffi::KEY_F5                       as int,
    KeyF6                       = ffi::KEY_F6                       as int,
    KeyF7                       = ffi::KEY_F7                       as int,
    KeyF8                       = ffi::KEY_F8                       as int,
    KeyF9                       = ffi::KEY_F9                       as int,
    KeyF10                      = ffi::KEY_F10                      as int,
    KeyF11                      = ffi::KEY_F11                      as int,
    KeyF12                      = ffi::KEY_F12                      as int,
    KeyF13                      = ffi::KEY_F13                      as int,
    KeyF14                      = ffi::KEY_F14                      as int,
    KeyF15                      = ffi::KEY_F15                      as int,
    KeyF16                      = ffi::KEY_F16                      as int,
    KeyF17                      = ffi::KEY_F17                      as int,
    KeyF18                      = ffi::KEY_F18                      as int,
    KeyF19                      = ffi::KEY_F19                      as int,
    KeyF20                      = ffi::KEY_F20                      as int,
    KeyF21                      = ffi::KEY_F21                      as int,
    KeyF22                      = ffi::KEY_F22                      as int,
    KeyF23                      = ffi::KEY_F23                      as int,
    KeyF24                      = ffi::KEY_F24                      as int,
    KeyF25                      = ffi::KEY_F25                      as int,
    KeyKp0                      = ffi::KEY_KP_0                     as int,
    KeyKp1                      = ffi::KEY_KP_1                     as int,
    KeyKp2                      = ffi::KEY_KP_2                     as int,
    KeyKp3                      = ffi::KEY_KP_3                     as int,
    KeyKp4                      = ffi::KEY_KP_4                     as int,
    KeyKp5                      = ffi::KEY_KP_5                     as int,
    KeyKp6                      = ffi::KEY_KP_6                     as int,
    KeyKp7                      = ffi::KEY_KP_7                     as int,
    KeyKp8                      = ffi::KEY_KP_8                     as int,
    KeyKp9                      = ffi::KEY_KP_9                     as int,
    KeyKpDecimal                = ffi::KEY_KP_DECIMAL               as int,
    KeyKpDivide                 = ffi::KEY_KP_DIVIDE                as int,
    KeyKpMultiply               = ffi::KEY_KP_MULTIPLY              as int,
    KeyKpSubtract               = ffi::KEY_KP_SUBTRACT              as int,
    KeyKpAdd                    = ffi::KEY_KP_ADD                   as int,
    KeyKpEnter                  = ffi::KEY_KP_ENTER                 as int,
    KeyKpEqual                  = ffi::KEY_KP_EQUAL                 as int,
    KeyLeftShift                = ffi::KEY_LEFT_SHIFT               as int,
    KeyLeftControl              = ffi::KEY_LEFT_CONTROL             as int,
    KeyLeftAlt                  = ffi::KEY_LEFT_ALT                 as int,
    KeyLeftSuper                = ffi::KEY_LEFT_SUPER               as int,
    KeyRightShift               = ffi::KEY_RIGHT_SHIFT              as int,
    KeyRightControl             = ffi::KEY_RIGHT_CONTROL            as int,
    KeyRightAlt                 = ffi::KEY_RIGHT_ALT                as int,
    KeyRightSuper               = ffi::KEY_RIGHT_SUPER              as int,
    KeyMenu                     = ffi::KEY_MENU                     as int,
}

#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum MouseButton {
    MouseButtonLeft             = ffi::MOUSE_BUTTON_LEFT            as int,
    MouseButtonRight            = ffi::MOUSE_BUTTON_RIGHT           as int,
    MouseButtonMiddle           = ffi::MOUSE_BUTTON_MIDDLE          as int,
    // MouseButton1                = ffi::MOUSE_BUTTON_1               as int,
    // MouseButton2                = ffi::MOUSE_BUTTON_2               as int,
    // MouseButton3                = ffi::MOUSE_BUTTON_3               as int,
    MouseButton4                = ffi::MOUSE_BUTTON_4               as int,
    MouseButton5                = ffi::MOUSE_BUTTON_5               as int,
    MouseButton6                = ffi::MOUSE_BUTTON_6               as int,
    MouseButton7                = ffi::MOUSE_BUTTON_7               as int,
    MouseButton8                = ffi::MOUSE_BUTTON_8               as int,
}

// pub static MouseButtonLeft           : MouseButton = MouseButton1;
// pub static MouseButtonRight          : MouseButton = MouseButton2;
// pub static MouseButtonMiddle         : MouseButton = MouseButton3;

#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Error {
    NotInitialized              = ffi::NOT_INITIALIZED              as int,
    NoCurrentContext            = ffi::NO_CURRENT_CONTEXT           as int,
    InvalidEnum                 = ffi::INVALID_ENUM                 as int,
    InvalidValue                = ffi::INVALID_VALUE                as int,
    OutOfMemory                 = ffi::OUT_OF_MEMORY                as int,
    ApiUnavailable              = ffi::API_UNAVAILABLE              as int,
    VersionUnavailable          = ffi::VERSION_UNAVAILABLE          as int,
    PlatformError               = ffi::PLATFORM_ERROR               as int,
    FormatUnavailable           = ffi::FORMAT_UNAVAILABLE           as int,
}

pub type ErrorFun = ~fn(error: Error, description: ~str);

#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum ClientApi {
    OpenGlApi                   = ffi::OPENGL_API                   as int,
    OpenGlEsApi                 = ffi::OPENGL_ES_API                as int,
}

#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum ContextRobustness {
    NoRobustness                = ffi::NO_ROBUSTNESS                as int,
    NoResetNotification         = ffi::NO_RESET_NOTIFICATION        as int,
    LoseContextOnReset          = ffi::LOSE_CONTEXT_ON_RESET        as int,
}

#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum OpenGlProfile {
    OpenGlAnyProfile            = ffi::OPENGL_ANY_PROFILE           as int,
    OpenGlCoreProfile           = ffi::OPENGL_CORE_PROFILE          as int,
    OpenGlCompatProfile         = ffi::OPENGL_COMPAT_PROFILE        as int,
}

#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum CursorMode {
    CursorNormal                = ffi::CURSOR_NORMAL                as int,
    CursorHidden                = ffi::CURSOR_HIDDEN                as int,
    CursorDisabled              = ffi::CURSOR_DISABLED              as int,
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
                |&ptr| Ok(Monitor { ptr: ptr }))
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
            ffi::glfwGetVideoMode(self.ptr).to_option().map(|&v| VidMode::from_glfw_vid_mode(v))
        }
    }

    /// Wrapper for `glfwSetGamma`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_gamma(&self, gamma: float) {
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

pub enum MonitorEvent {
    Connected                   = ffi::CONNECTED                    as int,
    Disconnected                = ffi::DISCONNECTED                 as int,
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
pub struct Modifiers(c_int);

/// Key modifier tokens
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Modifier {
    Shift       = ffi::MOD_SHIFT        as int,
    Control     = ffi::MOD_CONTROL      as int,
    Alt         = ffi::MOD_ALT          as int,
    Super       = ffi::MOD_SUPER        as int,
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
        **self & (modifier as c_int) != ffi::FALSE
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
pub type CursorPosFun = ~fn(window: &Window, xpos: float, ypos: float);
pub type CursorEnterFun = ~fn(window: &Window, entered: bool);
pub type ScrollFun = ~fn(window: &Window, xpos: float, ypos: float);
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
    pub fn create(width: uint, height: uint, title: &str, mode: WindowMode) -> Result<Window,()> {
        Window::create_intern(width, height, title, mode, None)
    }

    /// Wrapper for `glfwCreateWindow`.
    pub fn create_shared(&self, width: uint, height: uint, title: &str, mode: WindowMode) -> Result<Window,()> {
        Window::create_intern(width, height, title, mode, Some(self))
    }

    /// Internal wrapper for `glfwCreateWindow`.
    #[fixed_stack_segment] #[inline(never)]
    fn create_intern(width: uint, height: uint, title: &str, mode: WindowMode, share: Option<&Window>) -> Result<Window,()> {
        unsafe {
            do title.with_c_str |title| {
                ffi::glfwCreateWindow(
                    width as c_int,
                    height as c_int,
                    title,
                    mode.to_ptr(),
                    match share { Some(w) => w.ptr, None => ptr::null() }
                )
            }.to_option().map_default(Err(()),
                |&ptr| {
                    let windowfns = WindowFns::new();
                    ffi::glfwSetWindowUserPointer(ptr, cast::transmute(~windowfns));
                    let window = ~Window {
                        ptr: ptr::to_unsafe_ptr(ptr),
                        is_shared: share.is_none(),
                    };
                    Ok(*window)
                }
            )
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
        unsafe { cast::transmute(ffi::glfwGetInputMode(self.ptr, ffi::CURSOR) as int) }
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
        unsafe { cast::transmute(ffi::glfwGetKey(self.ptr, key as c_int) as int) }
    }

    /// Wrapper for `glfwGetMouseButton`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_mouse_button(&self, button: MouseButton) -> Action {
        unsafe { cast::transmute(ffi::glfwGetMouseButton(self.ptr, button as c_int) as int) }
    }

    /// Wrapper for `glfwGetCursorPos`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_cursor_pos(&self) -> (float, float) {
        unsafe {
            let mut xpos = 0.0;
            let mut ypos = 0.0;
            ffi::glfwGetCursorPos(self.ptr, &mut xpos, &mut ypos);
            (xpos as float, ypos as float)
        }
    }

    /// Wrapper for `glfwSetCursorPos`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_cursor_pos(&self, xpos: float, ypos: float) {
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

#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Joystick {
    Joystick1       = ffi::JOYSTICK_1       as int,
    Joystick2       = ffi::JOYSTICK_2       as int,
    Joystick3       = ffi::JOYSTICK_3       as int,
    Joystick4       = ffi::JOYSTICK_4       as int,
    Joystick5       = ffi::JOYSTICK_5       as int,
    Joystick6       = ffi::JOYSTICK_6       as int,
    Joystick7       = ffi::JOYSTICK_7       as int,
    Joystick8       = ffi::JOYSTICK_8       as int,
    Joystick9       = ffi::JOYSTICK_9       as int,
    Joystick10      = ffi::JOYSTICK_10      as int,
    Joystick11      = ffi::JOYSTICK_11      as int,
    Joystick12      = ffi::JOYSTICK_12      as int,
    Joystick13      = ffi::JOYSTICK_13      as int,
    Joystick14      = ffi::JOYSTICK_14      as int,
    Joystick15      = ffi::JOYSTICK_15      as int,
    Joystick16      = ffi::JOYSTICK_16      as int,
}

impl Joystick {
    /// Wrapper for `glfwJoystickPresent`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_present(&self) -> bool {
        unsafe { ffi::glfwJoystickPresent(*self as c_int) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetJoystickAxes`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_axes(&self) -> ~[float] {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetJoystickAxes(*self as c_int, &mut count);
            vec::from_buf(ptr, count as uint).map(|&a| a as float)
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
pub fn get_time() -> float {
    unsafe { ffi::glfwGetTime() as float }
}

/// Wrapper for `glfwSetTime`.
#[fixed_stack_segment] #[inline(never)]
pub fn set_time(time: float) {
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
