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

#[crate_id = "github.com/bjz/glfw-rs#glfw:0.1"];
#[comment = "Bindings and wrapper functions for glfw3."];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];

#[no_uv];
extern mod native;
extern mod green;
extern mod rustuv;

use std::any::*;
use std::cast;
use std::comm::{Chan, Port, SharedChan};
use std::hashmap::HashMap;
use std::libc::*;
use std::ptr;
use std::str;
use std::task::TaskOpts;
use std::vec;

pub mod ffi;
mod callbacks;

////////////////////////////////////////////////////////////////////////////////
// Version information
////////////////////////////////////////////////////////////////////////////////

// These functions can be called from anywhere.

/// Returns the version numbers of the underlying GLFW library.
///
/// # Example
///
/// ~~~rust
/// let (major, minor, rev) = glfw::get_version();
/// println!("GLFW version: {}.{}.{}", major, minor, rev);
/// ~~~
pub fn get_version() -> (u32, u32, u32) {
    let (mut major, mut minor, mut rev) = (0, 0, 0);
    unsafe { ffi::glfwGetVersion(&mut major, &mut minor, &mut rev) };
    (major as u32, minor as u32, rev as u32 )
}

/// Returns a string representation of the version information. Along with the
/// version numbers, this also includes (in order): the window system API, the
/// context creation API, and any additional options or APIs.
pub fn get_version_string() -> ~str {
    unsafe { str::raw::from_c_str(ffi::glfwGetVersionString()) }
}

////////////////////////////////////////////////////////////////////////////////
// Error handling
////////////////////////////////////////////////////////////////////////////////

// TODO

////////////////////////////////////////////////////////////////////////////////
// Lifetime functions
////////////////////////////////////////////////////////////////////////////////

/// Initializes the glfw library on a new native thread, spawning `f` in a new
/// task. This should be called from the `#[start]` function.
///
/// # Example
///
/// ~~~rust
/// #[start]
/// fn start(argc: int, argv: **u8) -> int {
///     glfw::start(argc, argv, main)
/// }
/// 
/// fn main(glfw: Option<glfw::Glfw>) {
///     // ...
/// }
/// ~~~
pub fn start(argc: int, argv: **u8, f: proc(Option<Glfw>)) -> int {
    native::start(argc, argv, proc() {
        let f = f;
        let mut pool = green::SchedPool::new(green::PoolConfig::new());
        let task_opts = TaskOpts::new();
        let (request_port, request_chan) = SharedChan::new();
        let (reply_port, reply_chan) = Chan::new();

        if unsafe { ffi::glfwInit() == ffi::FALSE } {
            // There was an error initialising GLFW, return early without starting
            // the request server.
            pool.spawn(task_opts, proc() {
                let f = f;
                f(None);
            });
        } else {
            let glfw_request_chan = request_chan.clone();
            // A task where user defined functionality will be performed
            pool.spawn(task_opts, proc() {
                // Perform user defined operations with a context. Messages will
                // be sent to the request sever so that the foreign functions
                // will be called on the main thread.
                let f = f;
                f(Some(Glfw {
                    reply_port: reply_port,
                    request_chan: glfw_request_chan,
                }))
            });

            // Begin running a request server in the current task
            let mut server = RequestServer {
                request_chan: request_chan,
                request_port: request_port,
                reply_chan: reply_chan,
                window_reply_chans: HashMap::new(),
            };
            loop {
                match server.request_port.recv_opt() {
                    Some(r) => if !server.handle_request(r) { break },
                    None => break,
                }
            }
        }
        pool.shutdown();
    })
}

pub struct Monitor {
    ptr: *ffi::GLFWmonitor
}

/// Describes the gamma ramp of a monitor.
pub struct GammaRamp {
    red:    ~[c_ushort],
    green:  ~[c_ushort],
    blue:   ~[c_ushort],
}

pub type GLProc = ffi::GLFWglproc;

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

////////////////////////////////////////////////////////////////////////////////
// Request Messages
////////////////////////////////////////////////////////////////////////////////

enum RequestMsg {
    GetMonitors,
    GetPrimaryMonitor,
    GetMonitorPos(*ffi::GLFWmonitor),
    GetMonitorPhysicalSize(*ffi::GLFWmonitor),
    GetMonitorName(*ffi::GLFWmonitor),
    GetVideoModes(*ffi::GLFWmonitor),
    GetVideoMode(*ffi::GLFWmonitor),
    GetGammaRamp(*ffi::GLFWmonitor),
    CreateWindow(u32, u32, ~str/*, WindowMode */),
    // CreateSharedWindow(u32, u32, ~str/*, WindowMode, *ffi::GLFWwindow */),
    JoystickPresent(Joystick),
    GetJoystickAxes(Joystick),
    GetJoystickButtons(Joystick),
    GetJoystickName(Joystick),
    GetCurrentContext,

    Terminate,
    SetErrorEventPolling(bool),
    SetMonitorEventPolling(bool),
    SetGamma(*ffi::GLFWmonitor, f32),
    SetGammaRamp(*ffi::GLFWmonitor, GammaRamp),
    DefaultWindowHints,
    WindowHint(WindowHint),
    PollEvents,
    WaitEvents,
    DestroyWindow(*ffi::GLFWwindow),
}

/// Describes a single video mode.
pub struct VidMode {
    width:        u32,
    height:       u32,
    red_bits:     u32,
    green_bits:   u32,
    blue_bits:    u32,
    refresh_rate: u32,
}

impl VidMode {
    fn from_glfw_vid_mode(mode: &ffi::GLFWvidmode) -> VidMode {
        VidMode {
            width:        mode.width as u32,
            height:       mode.height as u32,
            red_bits:     mode.redBits as u32,
            green_bits:   mode.greenBits as u32,
            blue_bits:    mode.blueBits as u32,
            refresh_rate: mode.refreshRate as u32,
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

////////////////////////////////////////////////////////////////////////////////
// Communication Types
////////////////////////////////////////////////////////////////////////////////

struct RequestServer {
    priv request_chan: SharedChan<RequestMsg>,
    priv request_port: Port<RequestMsg>,
    priv reply_chan: Chan<~Any>,
    priv window_reply_chans: HashMap<*ffi::GLFWwindow, Chan<~Any>>,
}

impl RequestServer {
    /// Handles requests that must be called from the main thread
    fn handle_request(&mut self, request: RequestMsg) -> bool {
        macro_rules! send(
            ($e:expr) => (self.reply_chan.send(~$e as ~Any))
        )
        match request {
            GetMonitors => send!(
                unsafe {
                    let mut count = 0;
                    let ptr = ffi::glfwGetMonitors(&mut count);
                    vec::from_buf(ptr, count as uint).map(|&m| Monitor { ptr: m })
                }
            ),
            GetPrimaryMonitor => send!(
                unsafe {
                    ffi::glfwGetPrimaryMonitor().to_option()
                        .map_default(Err(()), |ptr| Ok(Monitor { ptr: ptr }))
                }
            ),
            GetMonitorPos(ptr) => send!(
                unsafe {
                    let (mut xpos, mut ypos) = (0, 0);
                    ffi::glfwGetMonitorPos(ptr, &mut xpos, &mut ypos);
                    (xpos as i32, ypos as i32)
                }
            ),
            GetMonitorPhysicalSize(ptr) => send!(
                unsafe {
                    let (mut width, mut height) = (0, 0);
                    ffi::glfwGetMonitorPhysicalSize(ptr, &mut width, &mut height);
                    (width as i32, height as i32)
                }
            ),
            GetMonitorName(ptr) => send!(
                unsafe {
                    str::raw::from_c_str(ffi::glfwGetMonitorName(ptr))
                }
            ),
            GetVideoModes(ptr) => send!(
                unsafe {
                    let mut count = 0;
                    let ptr = ffi::glfwGetVideoModes(ptr, &mut count);
                    vec::from_buf(ptr, count as uint).map(VidMode::from_glfw_vid_mode)
                }
            ),
            GetVideoMode(ptr) => send!(
                unsafe {
                    ffi::glfwGetVideoMode(ptr).to_option()
                        .map(|v| VidMode::from_glfw_vid_mode(v))
                }
            ),
            GetGammaRamp(ptr) => send!(
                unsafe {
                    let llramp = ffi::glfwGetGammaRamp(ptr);
                    GammaRamp {
                        red:    vec::from_buf((*llramp).red,   (*llramp).size as uint),
                        green:  vec::from_buf((*llramp).green, (*llramp).size as uint),
                        blue:   vec::from_buf((*llramp).blue,  (*llramp).size as uint),
                    }
                }
            ),
            CreateWindow(width, height, title/*, mode*/) => send!({
                let ptr = unsafe {
                    title.with_c_str(|title| {
                        ffi::glfwCreateWindow(
                            width as c_int,
                            height as c_int,
                            title,
                            ptr::null(),
                            ptr::null()
                        )
                    })
                };
                if ptr.is_null() { None }
                else {
                    let (reply_port, reply_chan) = Chan::<~Any>::new();
                    let (event_port, event_chan) = Chan::<WindowEvent>::new();
                    unsafe {
                        ffi::glfwSetWindowUserPointer(ptr, cast::transmute(~event_chan));
                    }
                    Some(Window {
                        ptr: ptr,
                        request_chan: self.request_chan.clone(),
                        reply_port: reply_port,
                        event_port: event_port,
                    })
                }
            }),
            JoystickPresent(joy) => send!(
                unsafe {
                    ffi::glfwJoystickPresent(joy as c_int) == ffi::TRUE
                }
            ),
            GetJoystickAxes(joy) => send!(
                unsafe {
                    let mut count = 0;
                    let ptr = ffi::glfwGetJoystickAxes(joy as c_int, &mut count);
                    vec::from_buf(ptr, count as uint).map(|&a| a as f32)
                }
            ),
            GetJoystickButtons(joy) => send!(
                unsafe {
                    let mut count = 0;
                    let ptr = ffi::glfwGetJoystickButtons(joy as c_int, &mut count);
                    vec::from_buf(ptr, count as uint).map(|&b| b as c_int)
                }
            ),
            GetJoystickName(joy) => send!(
                unsafe {
                    str::raw::from_c_str(ffi::glfwGetJoystickName(joy as c_int))
                }
            ),
            GetCurrentContext => send!(
                unsafe {
                    ffi::glfwGetCurrentContext()
                }
            ),
            Terminate => return false,
            SetErrorEventPolling(should_poll) => {
                if should_poll { unsafe { ffi::glfwSetErrorCallback(None); } } // TODO
                else           { unsafe { ffi::glfwSetErrorCallback(None); } }
            }
            SetMonitorEventPolling(should_poll) => {
                if should_poll { unsafe { ffi::glfwSetMonitorCallback(None); } } // TODO
                else           { unsafe { ffi::glfwSetMonitorCallback(None); } }
            }
            SetGamma(ptr, gamma) => unsafe { ffi::glfwSetGamma(ptr, gamma as c_float) },
            SetGammaRamp(ptr, ramp) => {
                unsafe {
                    ffi::glfwSetGammaRamp(
                        ptr, &ffi::GLFWgammaramp {
                            red:    ramp.red.as_ptr(),
                            green:  ramp.green.as_ptr(),
                            blue:   ramp.blue.as_ptr(),
                            size:   ramp.red.len() as c_uint,
                        }
                    );
                }
            }
            DefaultWindowHints => unsafe { ffi::glfwDefaultWindowHints() },
            WindowHint(hint) => hint.apply(),
            PollEvents => unsafe { ffi::glfwPollEvents() },
            WaitEvents => unsafe { ffi::glfwWaitEvents() },
            DestroyWindow(ptr) => unsafe { ffi::glfwDestroyWindow(ptr) },
        }
        true
    }
}

impl Drop for RequestServer {
    fn drop(&mut self) {
        unsafe { ffi::glfwTerminate() }
    }
}

pub struct Glfw {
    priv reply_port: Port<~Any>,
    priv request_chan: SharedChan<RequestMsg>,
}

impl Glfw {
    /// A syncronous request to the GLFW library for a value of type `T`.
    fn call<T: Send>(&self, request: RequestMsg) -> Option<~T> {
        self.request_chan.send(request);
        self.reply_port.recv().move().ok()
    }
    
    /// An asyncrouns request to the GLFW library.
    fn cast(&self, request: RequestMsg) {
        self.request_chan.send(request);
    }

    pub fn get_monitors(&self) -> ~[Monitor] {
        *self.call(GetMonitors).unwrap()
    }

    pub fn get_primary_monitor(&self) -> Option<Monitor> {
        *self.call(GetPrimaryMonitor).unwrap()
    }

    pub fn get_monitor_pos(&self, monitor: &Monitor) -> (i32, i32) {
        *self.call(GetMonitorPos(monitor.ptr)).unwrap()
    }

    pub fn get_monitor_physical_size(&self, monitor: &Monitor) -> (i32, i32) {
        *self.call(GetMonitorPhysicalSize(monitor.ptr)).unwrap()
    }

    pub fn get_monitor_name(&self, monitor: &Monitor) -> ~str {
        *self.call(GetMonitorName(monitor.ptr)).unwrap()
    }

    pub fn get_videomodes(&self, monitor: &Monitor) -> ~[VidMode] {
        *self.call(GetVideoModes(monitor.ptr)).unwrap()
    }

    pub fn get_videomode(&self, monitor: &Monitor) -> VidMode {
        *self.call(GetVideoMode(monitor.ptr)).unwrap()
    }

    pub fn get_gammaramp(&self, monitor: &Monitor) -> GammaRamp {
        *self.call(GetGammaRamp(monitor.ptr)).unwrap()
    }

    pub fn create_window(&self, width: u32, height: u32, title: ~str/*, mode: WindowMode */) -> Option<Window> {
        *self.call(CreateWindow(width, height, title/*, mode*/)).unwrap()
    }

    pub fn joystick_present(&self, joy: Joystick) -> bool {
        *self.call(JoystickPresent(joy)).unwrap()
    }

    pub fn get_joystick_axes(&self, joy: Joystick) -> ~[f32] {
        *self.call(GetJoystickAxes(joy)).unwrap()
    }

    pub fn get_joystick_buttons(&self, joy: Joystick) -> ~[c_int] {
        *self.call(GetJoystickButtons(joy)).unwrap()
    }

    pub fn get_joystick_name(&self, joy: Joystick) -> ~str {
        *self.call(GetJoystickName(joy)).unwrap()
    }

    pub fn get_current_context(&self) -> () {
        *self.call(GetCurrentContext).unwrap()
    }

    pub fn terminate(&self) {
        self.cast(Terminate);
    }

    pub fn set_error_event_polling(&self, should_poll: bool) {
        self.cast(SetErrorEventPolling(should_poll));
    }

    pub fn set_monitor_event_polling(&self, should_poll: bool) {
        self.cast(SetMonitorEventPolling(should_poll));
    }

    pub fn set_gamma(&self, monitor: &Monitor, gamma: f32) {
        self.cast(SetGamma(monitor.ptr, gamma));
    }

    pub fn set_gamma_ramp(&self, monitor: &Monitor, ramp: GammaRamp) {
        self.cast(SetGammaRamp(monitor.ptr, ramp));
    }

    pub fn default_window_hints(&self) {
        self.cast(DefaultWindowHints);
    }

    pub fn window_hint(&self, hint: WindowHint) {
        self.cast(WindowHint(hint));
    }

    pub fn poll_events(&self) {
        self.cast(PollEvents);
    }

    pub fn wait_events(&self) {
        self.cast(WaitEvents);
    }

    pub fn get_time(&self) -> f64 {
        unsafe { ffi::glfwGetTime() as f64 }
    }

    pub fn set_time(&self, time: f64) {
        unsafe { ffi::glfwSetTime(time as c_double); }
    }

    pub fn swap_interval(&self, interval: i32) {
        unsafe { ffi::glfwSwapInterval(interval as c_int); }
    }

    pub fn extension_supported(&self, extension: ~str) -> bool {
        unsafe {
            extension.with_c_str(|extension| {
                ffi::glfwExtensionSupported(extension) == ffi::TRUE
            })
        }
    }

    pub fn get_proc_address(&self, procname: ~str) -> Option<GLProc> {
        unsafe {
            procname.with_c_str(|procname| {
                ffi::glfwGetProcAddress(procname)
            })
        }
    }

    // fn iter_events(&self) -> EventIterator {}
}

////////////////////////////////////////////////////////////////////////////////
// Window Hints
////////////////////////////////////////////////////////////////////////////////

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

pub enum WindowHint {
    RedBits(u32),
    GreenBits(u32),
    BlueBits(u32),
    AlphaBits(u32),
    DepthBits(u32),
    StencilBits(u32),
    AccumRedBits(u32),
    AccumGreenBits(u32),
    AccumBlueBits(u32),
    AccumAlphaBits(u32),
    AuxBuffers(u32),
    Stereo(bool),
    Samples(u32),
    SrgbCapable(bool),
    RefreshRate(u32),
    ClientApi(ClientApi),
    ContextVersionMajor(u32),
    ContextVersionMinor(u32),
    ContextVersion(u32, u32),
    ContextRobustness(ContextRobustness),
    OpenglForwardCompat(bool),
    OpenglDebugContext(bool),
    OpenglProfile(OpenGlProfile),
    Resizable(bool),
    Visible(bool),
    Decorated(bool),
}

impl WindowHint {
    fn apply(self) {
        unsafe {
            match self {
                RedBits(bits)               => ffi::glfwWindowHint(ffi::RED_BITS, bits as c_int),
                GreenBits(bits)             => ffi::glfwWindowHint(ffi::GREEN_BITS, bits as c_int),
                BlueBits(bits)              => ffi::glfwWindowHint(ffi::BLUE_BITS, bits as c_int),
                AlphaBits(bits)             => ffi::glfwWindowHint(ffi::ALPHA_BITS, bits as c_int),
                DepthBits(bits)             => ffi::glfwWindowHint(ffi::DEPTH_BITS, bits as c_int),
                StencilBits(bits)           => ffi::glfwWindowHint(ffi::STENCIL_BITS, bits as c_int),
                AccumRedBits(bits)          => ffi::glfwWindowHint(ffi::ACCUM_RED_BITS, bits as c_int),
                AccumGreenBits(bits)        => ffi::glfwWindowHint(ffi::ACCUM_GREEN_BITS, bits as c_int),
                AccumBlueBits(bits)         => ffi::glfwWindowHint(ffi::ACCUM_BLUE_BITS, bits as c_int),
                AccumAlphaBits(bits)        => ffi::glfwWindowHint(ffi::ACCUM_ALPHA_BITS, bits as c_int),
                AuxBuffers(buffers)         => ffi::glfwWindowHint(ffi::AUX_BUFFERS, buffers as c_int),
                Stereo(value)               => ffi::glfwWindowHint(ffi::STEREO, value as c_int),
                Samples(samples)            => ffi::glfwWindowHint(ffi::SAMPLES, samples as c_int),
                SrgbCapable(value)          => ffi::glfwWindowHint(ffi::SRGB_CAPABLE, value as c_int),
                RefreshRate(rate)           => ffi::glfwWindowHint(ffi::REFRESH_RATE, rate as c_int),
                ClientApi(api)              => ffi::glfwWindowHint(ffi::CLIENT_API, api as c_int),
                ContextVersionMajor(major)  => ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, major as c_int),
                ContextVersionMinor(minor)  => ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, minor as c_int),
                ContextVersion(major, minor) => {
                    ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, major as c_int);
                    ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, minor as c_int);
                }
                ContextRobustness(value)    => ffi::glfwWindowHint(ffi::CONTEXT_ROBUSTNESS, value as c_int),
                OpenglForwardCompat(value)  => ffi::glfwWindowHint(ffi::OPENGL_FORWARD_COMPAT, value as c_int),
                OpenglDebugContext(value)   => ffi::glfwWindowHint(ffi::OPENGL_DEBUG_CONTEXT, value as c_int),
                OpenglProfile(profile)      => ffi::glfwWindowHint(ffi::OPENGL_PROFILE, profile as c_int),
                Resizable(value)            => ffi::glfwWindowHint(ffi::RESIZABLE, value as c_int),
                Visible(value)              => ffi::glfwWindowHint(ffi::VISIBLE, value as c_int),
                Decorated(value)            => ffi::glfwWindowHint(ffi::DECORATED, value as c_int),
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Window Events
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Action {
    Release                      = ffi::RELEASE,
    Press                        = ffi::PRESS,
    Repeat                       = ffi::REPEAT,
}

#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum CursorMode {
    CursorNormal                = ffi::CURSOR_NORMAL,
    CursorHidden                = ffi::CURSOR_HIDDEN,
    CursorDisabled              = ffi::CURSOR_DISABLED,
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

pub enum WindowEvent {
    WindowPos(i32, i32),
    WindowSize(i32, i32),
    WindowClose,
    WindowRefresh,
    WindowFocus(bool),
    WindowIconify(bool),
    FramebufferSize(i32, i32),
    MouseButton(MouseButton, Action, Modifiers),
    CursorPos(f64, f64),
    CursorEnter(bool),
    Scroll(f64, f64),
    Key(Key, Scancode, Action, Modifiers),
    Char(char),
}

pub struct Scancode(c_int);

/// Key modifier tokens
#[repr(C)]
#[deriving(Clone, Eq, IterBytes, ToStr)]
pub enum Modifier {
    Shift       = ffi::MOD_SHIFT,
    Control     = ffi::MOD_CONTROL,
    Alt         = ffi::MOD_ALT,
    Super       = ffi::MOD_SUPER,
}

/// A group of key modifiers
pub struct Modifiers {
    values: c_int,
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

////////////////////////////////////////////////////////////////////////////////
// Window
////////////////////////////////////////////////////////////////////////////////

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

pub struct Window {
    priv ptr: *ffi::GLFWwindow,
    priv request_chan: SharedChan<RequestMsg>,
    priv reply_port: Port<~Any>,
    priv event_port: Port<WindowEvent>,
}

pub struct WindowEventIterator<'a> {
    priv window: &'a Window,
}

impl<'a> Iterator<WindowEvent> for WindowEventIterator<'a> {
    fn next(&mut self) -> Option<WindowEvent> {
        fail!()
    }
}

impl Window {
    fn call<T: Send>(&self, request: RequestMsg) -> Option<~T> {
        self.request_chan.send(request);
        self.reply_port.recv().move().ok()
    }
    
    fn cast(&self, request: RequestMsg) {
        self.request_chan.send(request);
    }

    pub fn get_shared_window(&self, width: u32, height: u32, title: ~str/*, mode: WindowMode */) -> Option<Window> {
        #[allow(unused_variable)]; fail!()
    }

    pub fn should_close(&self) -> bool {
        unsafe { ffi::glfwWindowShouldClose(self.ptr) == ffi::TRUE }
    }

    pub fn set_should_close(&self, value: bool) {
        unsafe { ffi::glfwSetWindowShouldClose(self.ptr, value as c_int) }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            title.with_c_str(|title| {
                ffi::glfwSetWindowTitle(self.ptr, title);
            });
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        unsafe {
            let (mut xpos, mut ypos) = (0, 0);
            ffi::glfwGetWindowPos(self.ptr, &mut xpos, &mut ypos);
            (xpos as i32, ypos as i32)
        }
    }

    pub fn set_pos(&self, xpos: i32, ypos: i32) {
        unsafe { ffi::glfwSetWindowPos(self.ptr, xpos as c_int, ypos as c_int); }
    }

    pub fn get_size(&self) -> (i32, i32) {
        unsafe {
            let (mut width, mut height) = (0, 0);
            ffi::glfwGetWindowSize(self.ptr, &mut width, &mut height);
            (width as i32, height as i32)
        }
    }

    pub fn set_size(&self, width: i32, height: i32) {
        unsafe { ffi::glfwSetWindowSize(self.ptr, width as c_int, height as c_int); }
    }

    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        unsafe {
            let (mut width, mut height) = (0, 0);
            ffi::glfwGetFramebufferSize(self.ptr, &mut width, &mut height);
            (width as i32, height as i32)
        }
    }

    pub fn iconify(&self) {
        unsafe { ffi::glfwIconifyWindow(self.ptr); }
    }

    pub fn restore(&self) {
        unsafe { ffi::glfwRestoreWindow(self.ptr); }
    }

    pub fn show(&self) {
        unsafe { ffi::glfwShowWindow(self.ptr); }
    }

    pub fn hide(&self) {
        unsafe { ffi::glfwHideWindow(self.ptr); }
    }

    pub fn get_window_mode(&self) -> WindowMode {
        WindowMode::from_ptr(
            unsafe { ffi::glfwGetWindowMonitor(self.ptr) }
        )
    }

    pub fn is_focused(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::FOCUSED) == ffi::TRUE }
    }

    pub fn is_iconified(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::ICONIFIED) == ffi::TRUE }
    }

    pub fn get_client_api(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::CLIENT_API) }
    }

    pub fn get_context_version(&self) -> (i32, i32, i32) {
        unsafe {(
            ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_VERSION_MAJOR) as i32,
            ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_VERSION_MINOR) as i32,
            ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_REVISION) as i32
        )}
    }

    pub fn get_context_robustness(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_ROBUSTNESS) }
    }

    pub fn is_opengl_forward_compat(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::OPENGL_FORWARD_COMPAT) == ffi::TRUE }
    }

    pub fn is_opengl_debug_context(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::OPENGL_DEBUG_CONTEXT) == ffi::TRUE }
    }

    pub fn get_opengl_profile(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::OPENGL_PROFILE) }
    }

    pub fn is_resizable(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::RESIZABLE) == ffi::TRUE }
    }

    pub fn is_visible(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::VISIBLE) == ffi::TRUE }
    }

    pub fn is_decorated(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::DECORATED) == ffi::TRUE }
    }

    pub fn set_pos_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_size_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_close_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_refresh_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_focus_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_iconify_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_framebuffer_size_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn get_cursor_mode(&self) -> CursorMode {
        unsafe { cast::transmute(ffi::glfwGetInputMode(self.ptr, ffi::CURSOR)) }
    }

    pub fn set_cursor_mode(&self, mode: CursorMode) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::CURSOR, mode as c_int); }
    }

    pub fn has_sticky_keys(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_KEYS) == ffi::TRUE }
    }

    pub fn set_sticky_keys(&self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::STICKY_KEYS, value as c_int); }
    }

    pub fn has_sticky_mouse_buttons(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS) == ffi::TRUE }
    }

    pub fn set_sticky_mouse_buttons(&self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS, value as c_int); }
    }

    pub fn get_key(&self, key: Key) -> Action {
        unsafe { cast::transmute(ffi::glfwGetKey(self.ptr, key as c_int)) }
    }

    pub fn get_mouse_button(&self, button: MouseButton) -> Action {
        unsafe { cast::transmute(ffi::glfwGetMouseButton(self.ptr, button as c_int)) }
    }

    pub fn get_cursor_pos(&self) -> (f64, f64) {
        unsafe {
            let (mut xpos, mut ypos) = (0.0, 0.0);
            ffi::glfwGetCursorPos(self.ptr, &mut xpos, &mut ypos);
            (xpos as f64, ypos as f64)
        }
    }

    pub fn set_cursor_pos(&self, xpos: f64, ypos: f64) {
        unsafe { ffi::glfwSetCursorPos(self.ptr, xpos as c_double, ypos as c_double); }
    }

    pub fn set_key_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_char_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_mouse_button_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_cursor_pos_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_cursor_enter_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_scroll_event_polling(&self, should_poll: bool) {
        #[allow(unused_variable)]; // ...
    }

    pub fn set_clipboard_string(&self, string: &str) {
        unsafe {
            string.with_c_str(|string| {
                ffi::glfwSetClipboardString(self.ptr, string);
            });
        }
    }

    pub fn get_clipboard_string(&self) -> ~str {
        unsafe { str::raw::from_c_str(ffi::glfwGetClipboardString(self.ptr)) }
    }

    pub fn is_current_context(&self) -> bool {
        self.ptr == unsafe { ffi::glfwGetCurrentContext() }
    }

    pub fn swap_buffers(&self) {
        unsafe { ffi::glfwSwapBuffers(self.ptr); }
    }

    pub fn destroy(self) {
        // empty
    }

    // fn iter_events(&self) -> WindowEventIterator {}

    // Native APIs

    #[cfg(target_os="win32")]
    pub fn get_win32_window(&self) -> *c_void {
        unsafe { ffi::glfwGetWin32Window(self.ptr) }
    }

    #[cfg(target_os="win32")]
    pub fn get_wgl_context(&self) -> *c_void {
        unsafe { ffi::glfwGetWGLContext(self.ptr) }
    }

    #[cfg(target_os="macos")]
    pub fn get_cocoa_window(&self) -> *c_void {
        unsafe { ffi::glfwGetCocoaWindow(self.ptr) }
    }

    #[cfg(target_os="macos")]
    pub fn get_nsgl_context(&self) -> *c_void {
        unsafe { ffi::glfwGetNSGLContext(self.ptr) }
    }

    #[cfg(target_os="linux")]
    pub fn get_x11_window(&self) -> *c_void {
        unsafe { ffi::glfwGetX11Window(self.ptr) }
    }

    #[cfg(target_os="linux")]
    pub fn get_glx_context(&self) -> *c_void {
        unsafe { ffi::glfwGetGLXContext(self.ptr) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // Close the port
        let _: ~Chan<WindowEvent> = unsafe {
            cast::transmute(ffi::glfwGetWindowUserPointer(self.ptr))
        };
        self.cast(DestroyWindow(self.ptr));
    }
}
