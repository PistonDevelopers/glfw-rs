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
use std::task;
use std::vec;

// re-export constants
pub use consts::*;

pub mod ll;
pub mod consts;
priv mod private;

/// A struct that wraps a `*GLFWmonitor` handle.
#[deriving(Eq)]
pub struct Monitor {
    ptr: *ll::GLFWmonitor
}

/// A struct that wraps a `*GLFWwindow` handle.
#[deriving(Eq, IterBytes)]
pub struct Window {
    ptr: *ll::GLFWwindow
}

pub type ErrorFun = @fn(error: c_int, description: ~str);
pub type WindowPosFun = @fn(window: &Window, xpos: int, ypos: int);
pub type WindowSizeFun = @fn(window: &Window, width: int, height: int);
pub type WindowCloseFun = @fn(window: &Window);
pub type WindowRefreshFun = @fn(window: &Window);
pub type WindowFocusFun = @fn(window: &Window, focused: bool);
pub type WindowIconifyFun = @fn(window: &Window, iconified: bool);
pub type FramebufferSizeFun = @fn(window: &Window, width: int, height: int);
pub type MouseButtonFun = @fn(window: &Window, button: c_int, action: c_int, mods: c_int);
pub type CursorPosFun = @fn(window: &Window, xpos: float, ypos: float);
pub type CursorEnterFun = @fn(window: &Window, entered: bool);
pub type ScrollFun = @fn(window: &Window, xpos: float, ypos: float);
pub type KeyFun = @fn(window: &Window, key: c_int, scancode: c_int, action: c_int, mods: c_int);
pub type CharFun = @fn(window: &Window, character: char);
pub type MonitorFun = @fn(monitor: &Monitor, event: c_int);

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

pub type GLProc = ll::GLFWglproc;

/// Initialises GLFW on the main platform thread. Fails if the initialisation
/// was unsuccessful.
///
/// Wrapper for `glfwInit` and `glfwTerminate`.
///
/// # Parameters
///
/// - `f`: A closure to be called after the GLFW is initialised.
pub fn spawn(f: ~fn()) {
    do task::spawn_sched(task::PlatformThread) {
        use std::unstable::finally::Finally;

        private::WindowDataMap::init();

        match unsafe { ll::glfwInit() } {
            ll::TRUE => f.finally(|| unsafe { ll::glfwTerminate() }),
            _        => fail!(~"Failed to initialize GLFW"),
        }
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
        fmt!("%?.%?.%?", self.major, self.minor, self.rev)
    }
}

/// Wrapper for `glfwGetVersion`.
pub fn get_version() -> Version {
    unsafe {
        let (major, minor, rev) = (0, 0, 0);
        ll::glfwGetVersion(&major, &minor, &rev);
        Version {
            major: major as uint,
            minor: minor as uint,
            rev:   rev   as uint,
        }
    }
}

/// Wrapper for `glfwGetVersionString`.
pub fn get_version_string() -> ~str {
    unsafe { str::raw::from_c_str(ll::glfwGetVersionString()) }
}

/// Wrapper for `glfwSetErrorCallback`.
pub fn set_error_callback(cbfun: ErrorFun) {
    do private::set_error_fun(cbfun) |ext_cb| {
        unsafe { ll::glfwSetErrorCallback(ext_cb); }
    }
}

impl Monitor {
    /// Wrapper for `glfwGetPrimaryMonitor`.
    pub fn get_primary() -> Option<Monitor> {
        unsafe {
            ll::glfwGetPrimaryMonitor().to_option().map(|&ptr| Monitor { ptr: ptr })
        }
    }

    /// Wrapper for `glfwGetMonitors`.
    pub fn get_connected() -> ~[Monitor] {
        unsafe {
            let count = 0;
            let ptr = ll::glfwGetMonitors(&count);
            vec::from_buf(ptr, count as uint).map(|&m| Monitor { ptr: m })
        }
    }

    /// Wrapper for `glfwGetMonitorPos`.
    pub fn get_pos(&self) -> (int, int) {
        unsafe {
            let (xpos, ypos) = (0, 0);
            ll::glfwGetMonitorPos(self.ptr, &xpos, &ypos);
            (xpos as int, ypos as int)
        }
    }

    /// Wrapper for `glfwGetMonitorPhysicalSize`.
    pub fn get_physical_size(&self) -> (int, int) {
        unsafe {
            let (width, height) = (0, 0);
            ll::glfwGetMonitorPhysicalSize(self.ptr, &width, &height);
            (width as int, height as int)
        }
    }

    /// Wrapper for `glfwGetMonitorName`.
    pub fn get_name(&self) -> ~str {
        unsafe { str::raw::from_c_str(ll::glfwGetMonitorName(self.ptr)) }
    }

    /// Wrapper for `glfwSetMonitorCallback`.
    pub fn set_callback(cbfun: MonitorFun) {
        do private::set_monitor_fun(cbfun) |ext_cb| {
            unsafe { ll::glfwSetMonitorCallback(ext_cb); }
        }
    }

    /// Wrapper for `glfwGetVideoModes`.
    pub fn get_video_modes(&self) -> ~[VidMode] {
        unsafe {
            let count = 0;
            let ptr = ll::glfwGetVideoModes(self.ptr, &count);
            vec::from_buf(ptr, count as uint).map(VidMode::from_glfw_vid_mode)
        }
    }

    /// Wrapper for `glfwGetVideoMode`.
    pub fn get_video_mode(&self) -> Option<VidMode> {
        unsafe {
            ll::glfwGetVideoMode(self.ptr).to_option().map(|&v| VidMode::from_glfw_vid_mode(v))
        }
    }

    /// Wrapper for `glfwSetGamma`.
    pub fn set_gamma(&self, gamma: float) {
        unsafe { ll::glfwSetGamma(self.ptr, gamma as c_float); }
    }

    /// Wrapper for `glfwGetGammaRamp`.
    pub fn get_gamma_ramp(&self) -> GammaRamp {
        unsafe {
            let llramp = *ll::glfwGetGammaRamp(self.ptr);
            GammaRamp {
                red:    vec::from_buf(llramp.red,   llramp.size as uint),
                green:  vec::from_buf(llramp.green, llramp.size as uint),
                blue:   vec::from_buf(llramp.blue,  llramp.size as uint),
            }
        }
    }

    /// Wrapper for `glfwSetGammaRamp`.
    pub fn set_gamma_ramp(&self, ramp: &GammaRamp) {
        unsafe {
            ll::glfwSetGammaRamp(
                self.ptr,
                &ll::GLFWgammaramp {
                    red:    vec::raw::to_ptr(ramp.red),
                    green:  vec::raw::to_ptr(ramp.green),
                    blue:   vec::raw::to_ptr(ramp.blue),
                    size:   ramp.red.len() as c_uint,
                }
            );
        }
    }
}

impl VidMode {
    fn from_glfw_vid_mode(mode: &ll::GLFWvidmode) -> VidMode {
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
        fmt!("%? x %?, %? (%? %? %?) %? Hz",
             self.width, self.height,
             self.red_bits + self.green_bits + self.blue_bits,
             self.red_bits, self.green_bits, self.blue_bits,
             self.refresh_rate)
    }
}

pub mod window_hint {
    use std::libc::c_int;
    use ll;

    /// Wrapper for `glfwDefaultWindowHints`.
    pub fn default() {
        unsafe { ll::glfwDefaultWindowHints(); }
    }

    /// Wrapper for `glfwWindowHint` called with `RED_BITS`.
    pub fn red_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::RED_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `GREEN_BITS`.
    pub fn green_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::GREEN_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `BLUE_BITS`.
    pub fn blue_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::BLUE_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ALPHA_BITS`.
    pub fn alpha_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::ALPHA_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `DEPTH_BITS`.
    pub fn depth_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::DEPTH_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `STENCIL_BITS`.
    pub fn stencil_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::STENCIL_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_RED_BITS`.
    pub fn accum_red_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::ACCUM_RED_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_GREEN_BITS`.
    pub fn accum_green_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::ACCUM_GREEN_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_BLUE_BITS`.
    pub fn accum_blue_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::ACCUM_BLUE_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_ALPHA_BITS`.
    pub fn accum_alpha_bits(bits: uint) {
        unsafe { ll::glfwWindowHint(ll::ACCUM_ALPHA_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `AUX_BUFFERS`.
    pub fn aux_buffers(buffers: uint) {
        unsafe { ll::glfwWindowHint(ll::AUX_BUFFERS, buffers as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `STEREO`.
    pub fn stereo(value: bool) {
        unsafe { ll::glfwWindowHint(ll::STEREO, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `SAMPLES`.
    pub fn samples(samples: uint) {
        unsafe { ll::glfwWindowHint(ll::SAMPLES, samples as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `SRGB_CAPABLE`.
    pub fn srgb_capable(value: bool) {
        unsafe { ll::glfwWindowHint(ll::SRGB_CAPABLE, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `REFRESH_RATE`.
    pub fn refresh_rate(rate: int) {
        unsafe { ll::glfwWindowHint(ll::REFRESH_RATE, rate as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CLIENT_API`.
    pub fn client_api(api: c_int) {
        unsafe { ll::glfwWindowHint(ll::CLIENT_API, api); }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MAJOR`.
    pub fn context_version_major(major: uint) {
        unsafe { ll::glfwWindowHint(ll::CONTEXT_VERSION_MAJOR, major as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MINOR`.
    pub fn context_version_minor(minor: uint) {
        unsafe { ll::glfwWindowHint(ll::CONTEXT_VERSION_MINOR, minor as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MAJOR` and
    /// `CONTEXT_VERSION_MINOR`.
    pub fn context_version(major: uint, minor: uint) {
        unsafe {
            ll::glfwWindowHint(ll::CONTEXT_VERSION_MAJOR, major as c_int);
            ll::glfwWindowHint(ll::CONTEXT_VERSION_MINOR, minor as c_int);
        }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_ROBUSTNESS`.
    pub fn context_robustness(value: bool) {
        unsafe { ll::glfwWindowHint(ll::CONTEXT_ROBUSTNESS, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_FORWARD_COMPAT`.
    pub fn opengl_forward_compat(value: bool) {
        unsafe { ll::glfwWindowHint(ll::OPENGL_FORWARD_COMPAT, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_DEBUG_CONTEXT`.
    pub fn opengl_debug_context(value: bool) {
        unsafe { ll::glfwWindowHint(ll::OPENGL_DEBUG_CONTEXT, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_PROFILE`.
    pub fn opengl_profile(profile: c_int) {
        unsafe { ll::glfwWindowHint(ll::OPENGL_PROFILE, profile); }
    }

    /// Wrapper for `glfwWindowHint` called with `RESIZABLE`.
    pub fn resizable(value: bool) {
        unsafe { ll::glfwWindowHint(ll::RESIZABLE, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `VISIBLE`.
    pub fn visible(value: bool) {
        unsafe { ll::glfwWindowHint(ll::VISIBLE, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `DECORATED`.
    pub fn decorated(value: bool) {
        unsafe { ll::glfwWindowHint(ll::DECORATED, value as c_int); }
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
    priv fn from_ptr(ptr: *ll::GLFWmonitor) -> WindowMode {
        if ptr.is_null() {
            Windowed
        } else {
            FullScreen(Monitor { ptr: ptr })
        }
    }

    /// Returns a pointer to a monitor if the window is fullscreen, otherwise
    /// it returns a null pointer (if it is in windowed mode).
    priv fn to_ptr(&self) -> *ll::GLFWmonitor {
        match *self {
            FullScreen(monitor) => monitor.ptr,
            Windowed => ptr::null()
        }
    }
}

macro_rules! set_window_callback(
    (
        setter:   $ll_fn:ident,
        callback: $ext_fn:ident,
        field:    $data_field:ident
    ) => ({
        self.get_local_data().$data_field = Some(cbfun);
        unsafe { ll::$ll_fn(self.ptr, private::$ext_fn); }
    })
)

impl Window {
    /// Wrapper for `glfwCreateWindow`.
    ///
    /// # Returns
    ///
    /// The created window wrapped in `Some`, or `None` if an error occurred.
    pub fn create(width: uint, height: uint, title: &str, mode: WindowMode) -> Option<Window> {
        unsafe {
            do ll::glfwCreateWindow(
                width as c_int,
                height as c_int,
                str::as_c_str(title, |a| a),
                mode.to_ptr(),
                ptr::null()
            ).to_option().map |&ptr| {
                // Initialize the local data for this window in TLS
                private::WindowDataMap::get().insert(
                    cast::transmute(ptr), @mut private::WindowData::new()
                );
                Window { ptr: cast::transmute(ptr) }
            }
        }
    }

    /// Returns a mutable pointer to the window's local data stored in task-
    /// local storage. Fails if no data is found.
    priv fn get_local_data(&self) -> @mut private::WindowData {
        match private::WindowDataMap::get().find_mut(&self.ptr) {
            Some(&data) => data,
            None => fail!("Could not find local data for this window."),
        }
    }

    /// Wrapper for `glfwWindowShouldClose`.
    pub fn should_close(&self) -> bool {
        unsafe { ll::glfwWindowShouldClose(self.ptr) as bool }
    }

    /// Wrapper for `glfwSetWindowShouldClose`.
    pub fn set_should_close(&self, value: bool) {
        unsafe { ll::glfwSetWindowShouldClose(self.ptr, value as c_int) }
    }

    /// Wrapper for `glfwSetWindowTitle`.
    pub fn set_title(&self, title: &str) {
        unsafe { ll::glfwSetWindowTitle(self.ptr, str::as_c_str(title, |a| a)); }
    }

    /// Wrapper for `glfwGetWindowPos`.
    pub fn get_pos(&self) -> (int, int) {
        unsafe {
            let (xpos, ypos) = (0, 0);
            ll::glfwGetWindowPos(self.ptr, &xpos, &ypos);
            (xpos as int, ypos as int)
        }
    }

    /// Wrapper for `glfwSetWindowPos`.
    pub fn set_pos(&self, xpos: int, ypos: int) {
        unsafe { ll::glfwSetWindowPos(self.ptr, xpos as c_int, ypos as c_int); }
    }

    /// Wrapper for `glfwGetWindowSize`.
    pub fn get_size(&self) -> (int, int) {
        unsafe {
            let (width, height) = (0, 0);
            ll::glfwGetWindowSize(self.ptr, &width, &height);
            (width as int, height as int)
        }
    }

    /// Wrapper for `glfwSetWindowSize`.
    pub fn set_size(&self, width: int, height: int) {
        unsafe { ll::glfwSetWindowSize(self.ptr, width as c_int, height as c_int); }
    }

    /// Wrapper for `glfwGetFramebufferSize`.
    pub fn get_framebuffer_size(&self) -> (int, int) {
        unsafe {
            let (width, height) = (0, 0);
            ll::glfwGetFramebufferSize(self.ptr, &width, &height);
            (width as int, height as int)
        }
    }

    /// Wrapper for `glfwIconifyWindow`.
    pub fn iconify(&self) {
        unsafe { ll::glfwIconifyWindow(self.ptr); }
    }

    /// Wrapper for `glfwRestoreWindow`.
    pub fn restore(&self) {
        unsafe { ll::glfwRestoreWindow(self.ptr); }
    }

    /// Wrapper for `glfwShowWindow`.
    pub fn show(&self) {
        unsafe { ll::glfwShowWindow(self.ptr); }
    }

    /// Wrapper for `glfwHideWindow`.
    pub fn hide(&self) {
        unsafe { ll::glfwHideWindow(self.ptr); }
    }

    /// Wrapper for `glfwGetWindowMonitor`.
    ///
    /// # Returns
    ///
    /// The window mode; either glfw::FullScreen or glfw::Windowed
    pub fn get_window_mode(&self) -> WindowMode {
        WindowMode::from_ptr(
            unsafe { ll::glfwGetWindowMonitor(self.ptr) }
        )
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `FOCUSED`.
    pub fn is_focused(&self) -> bool {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, FOCUSED) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `ICONIFIED`.
    pub fn is_iconified(&self) -> bool {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, ICONIFIED) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `CLIENT_API`.
    pub fn get_client_api(&self) -> c_int {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, CLIENT_API) }
    }

    /// Wrapper for `glfw::ll::glfwGetWindowAttrib` called with
    /// `CONTEXT_VERSION_MAJOR`, `CONTEXT_VERSION_MINOR` and `CONTEXT_REVISION`.
    ///
    /// # Returns
    ///
    /// The client API version of the window's context in a version struct.
    ///
    pub fn get_context_version(&self) -> Version {
        unsafe {
            Version {
                major:  ll::glfwGetWindowAttrib(self.ptr, CONTEXT_VERSION_MAJOR) as uint,
                minor:  ll::glfwGetWindowAttrib(self.ptr, CONTEXT_VERSION_MINOR) as uint,
                rev:    ll::glfwGetWindowAttrib(self.ptr, CONTEXT_REVISION) as uint,
            }
        }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `CONTEXT_ROBUSTNESS`.
    pub fn get_context_robustness(&self) -> c_int {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, CONTEXT_ROBUSTNESS) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_FORWARD_COMPAT`.
    pub fn is_opengl_forward_compat(&self) -> bool {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, OPENGL_FORWARD_COMPAT) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_DEBUG_CONTEXT`.
    pub fn is_opengl_debug_context(&self) -> bool {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, OPENGL_DEBUG_CONTEXT) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_PROFILE`.
    pub fn get_opengl_profile(&self) -> c_int {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, OPENGL_PROFILE) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `RESIZABLE`.
    pub fn is_resizable(&self) -> bool {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, RESIZABLE) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `VISIBLE`.
    pub fn is_visible(&self) -> bool {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, VISIBLE) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `DECORATED`.
    pub fn is_decorated(&self) -> bool {
        unsafe { ll::glfwGetWindowAttrib(self.ptr, DECORATED) as bool }
    }

    /// Wrapper for `glfwSetWindowPosCallback`.
    pub fn set_pos_callback(&self, cbfun: WindowSizeFun) {
        set_window_callback!(setter:   glfwSetWindowPosCallback,
                             callback: window_pos_callback,
                             field:    pos_fun);
    }

    /// Wrapper for `glfwSetWindowSizeCallback`.
    pub fn set_size_callback(&self, cbfun: WindowSizeFun) {
        set_window_callback!(setter:   glfwSetWindowSizeCallback,
                             callback: window_size_callback,
                             field:    size_fun);
    }

    /// Wrapper for `glfwSetWindowCloseCallback`.
    pub fn set_close_callback(&self, cbfun: WindowCloseFun) {
        set_window_callback!(setter:   glfwSetWindowCloseCallback,
                             callback: window_close_callback,
                             field:    close_fun);
    }

    /// Wrapper for `glfwSetWindowRefreshCallback`.
    pub fn set_refresh_callback(&self, cbfun: WindowRefreshFun) {
        set_window_callback!(setter:   glfwSetWindowRefreshCallback,
                             callback: window_refresh_callback,
                             field:    refresh_fun);
    }

    /// Wrapper for `glfwSetWindowFocusCallback`.
    pub fn set_focus_callback(&self, cbfun: WindowFocusFun) {
        set_window_callback!(setter:   glfwSetWindowFocusCallback,
                             callback: window_focus_callback,
                             field:    focus_fun);
    }

    /// Wrapper for `glfwSetWindowIconifyCallback`.
    pub fn set_iconify_callback(&self, cbfun: WindowIconifyFun) {
        set_window_callback!(setter:   glfwSetWindowIconifyCallback,
                             callback: window_iconify_callback,
                             field:    iconify_fun);
    }

    /// Wrapper for `glfwSetFramebufferSizeCallback`.
    pub fn set_framebuffer_size_callback(&self, cbfun: FramebufferSizeFun) {
        set_window_callback!(setter:   glfwSetFramebufferSizeCallback,
                             callback: framebuffer_size_callback,
                             field:    framebuffer_size_fun);
    }

    /// Wrapper for `glfwGetInputMode` called with `CURSOR`.
    pub fn get_cursor_mode(&self) -> c_int {
        unsafe { ll::glfwGetInputMode(self.ptr, CURSOR) }
    }

    /// Wrapper for `glfwSetInputMode` called with `CURSOR`.
    pub fn set_cursor_mode(&self, mode: c_int) {
        unsafe { ll::glfwSetInputMode(self.ptr, CURSOR, mode); }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_KEYS`.
    pub fn has_sticky_keys(&self) -> bool {
        unsafe { ll::glfwGetInputMode(self.ptr, STICKY_KEYS) as bool }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_KEYS`.
    pub fn set_sticky_keys(&self, value: bool) {
        unsafe { ll::glfwSetInputMode(self.ptr, STICKY_KEYS, value as c_int); }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn has_sticky_mouse_buttons(&self) -> bool {
        unsafe { ll::glfwGetInputMode(self.ptr, STICKY_MOUSE_BUTTONS) as bool }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn set_sticky_mouse_buttons(&self, value: bool) {
        unsafe { ll::glfwSetInputMode(self.ptr, STICKY_MOUSE_BUTTONS, value as c_int); }
    }

    /// Wrapper for `glfwGetKey`.
    pub fn get_key(&self, key: c_int) -> c_int {
        unsafe { ll::glfwGetKey(self.ptr, key) }
    }

    /// Wrapper for `glfwGetMouseButton`.
    pub fn get_mouse_button(&self, button: c_int) -> c_int {
        unsafe { ll::glfwGetMouseButton(self.ptr, button) }
    }

    /// Wrapper for `glfwGetCursorPos`.
    pub fn get_cursor_pos(&self) -> (float, float) {
        unsafe {
            let (xpos, ypos) = (0.0, 0.0);
            ll::glfwGetCursorPos(self.ptr, &xpos, &ypos);
            (xpos as float, ypos as float)
        }
    }

    /// Wrapper for `glfwSetCursorPos`.
    pub fn set_cursor_pos(&self, xpos: float, ypos: float) {
        unsafe { ll::glfwSetCursorPos(self.ptr, xpos as c_double, ypos as c_double); }
    }

    /// Wrapper for `glfwSetKeyCallback`.
    pub fn set_key_callback(&self, cbfun: KeyFun) {
        set_window_callback!(setter:   glfwSetKeyCallback,
                             callback: key_callback,
                             field:    key_fun);
    }

    /// Wrapper for `glfwSetCharCallback`.
    pub fn set_char_callback(&self, cbfun: CharFun) {
        set_window_callback!(setter:   glfwSetCharCallback,
                             callback: char_callback,
                             field:    char_fun);
    }

    /// Wrapper for `glfwSetMouseButtonCallback`.
    pub fn set_mouse_button_callback(&self, cbfun: MouseButtonFun) {
        set_window_callback!(setter:   glfwSetMouseButtonCallback,
                             callback: mouse_button_callback,
                             field:    mouse_button_fun);
    }

    /// Wrapper for `glfwSetCursorPosCallback`.
    pub fn set_cursor_pos_callback(&self, cbfun: CursorPosFun) {
        set_window_callback!(setter:   glfwSetCursorPosCallback,
                             callback: cursor_pos_callback,
                             field:    cursor_pos_fun);
    }

    /// Wrapper for `glfwSetCursorEnterCallback`.
    pub fn set_cursor_enter_callback(&self, cbfun: CursorEnterFun) {
        set_window_callback!(setter:   glfwSetCursorEnterCallback,
                             callback: cursor_enter_callback,
                             field:    cursor_enter_fun);
    }

    /// Wrapper for `glfwSetScrollCallback`.
    pub fn set_scroll_callback(&self, cbfun: ScrollFun) {
        set_window_callback!(setter:   glfwSetScrollCallback,
                             callback: scroll_callback,
                             field:    scroll_fun);
    }

    /// Wrapper for `glfwGetClipboardString`.
    pub fn set_clipboard_string(&self, string: &str) {
        unsafe { ll::glfwSetClipboardString(self.ptr, str::as_c_str(string, |a| a)); }
    }

    /// Wrapper for `glfwGetClipboardString`.
    pub fn get_clipboard_string(&self) -> ~str {
        unsafe { str::raw::from_c_str(ll::glfwGetClipboardString(self.ptr)) }
    }

    /// Wrapper for `glfwMakeContextCurrent`.
    pub fn make_context_current(&self) {
        unsafe { ll::glfwMakeContextCurrent(self.ptr); }
    }

    // TODO: documentation
    pub fn is_current_context(&self) -> bool {
        self.ptr == unsafe { ll::glfwGetCurrentContext() }
    }

    /// Wrapper for `glfwSwapBuffers`.
    pub fn swap_buffers(&self) {
        unsafe { ll::glfwSwapBuffers(self.ptr); }
    }
}

/// Wrapper for glfwMakeContextCurrent` called with `null`.
pub fn detach_current_context() {
    unsafe { ll::glfwMakeContextCurrent(ptr::null()); }
}

impl Drop for Window {
    /// Closes the window and removes all associated callbacks.
    ///
    /// Wrapper for `glfwDestroyWindow`.
    pub fn finalize(&self) {
        unsafe { ll::glfwDestroyWindow(self.ptr); }
        // Remove data from task-local storage
        private::WindowDataMap::get().remove(&self.ptr);
    }
}

/// Wrapper for `glfwPollEvents`.
pub fn poll_events() {
    unsafe { ll::glfwPollEvents(); }
}

/// Wrapper for `glfwWaitEvents`.
pub fn wait_events() {
    unsafe { ll::glfwWaitEvents(); }
}

pub mod joystick {
    use std::libc::*;
    use std::str;
    use std::vec;

    use ll;

    /// Wrapper for `glfwJoystickPresent`.
    pub fn is_present(joy: c_int) -> bool {
        unsafe { ll::glfwJoystickPresent(joy) as bool }
    }

    /// Wrapper for `glfwGetJoystickAxes`.
    pub fn get_axes(joy: c_int) -> ~[float] {
        unsafe {
            let count = 0;
            let ptr = ll::glfwGetJoystickAxes(joy, &count);
            vec::from_buf(ptr, count as uint).map(|&a| a as float)
        }
    }

    /// Wrapper for `glfwGetJoystickButtons`.
    pub fn get_buttons(joy: c_int) -> ~[c_int] {
        unsafe {
            let count = 0;
            let ptr = ll::glfwGetJoystickButtons(joy, &count);
            vec::from_buf(ptr, count as uint).map(|&b| b as c_int)
        }
    }

    /// Wrapper for `glfwGetJoystickName`.
    pub fn get_name(joy: c_int) -> ~str {
        unsafe { str::raw::from_c_str(ll::glfwGetJoystickName(joy)) }
    }
}

/// Wrapper for `glfwGetTime`.
pub fn get_time() -> float {
    unsafe { ll::glfwGetTime() as float }
}

/// Wrapper for `glfwSetTime`.
pub fn set_time(time: float) {
    unsafe { ll::glfwSetTime(time as c_double); }
}

/// Wrapper for `glfwSwapInterval`.
pub fn set_swap_interval(interval: int) {
    unsafe { ll::glfwSwapInterval(interval as c_int); }
}

/// Wrapper for `glfwExtensionSupported`.
pub fn extension_supported(extension: &str) -> bool {
    unsafe { ll::glfwExtensionSupported(str::as_c_str(extension, |a| a)) as bool }
}

/// Wrapper for `glfwGetProcAddress`.
pub fn get_proc_address(procname: &str) -> GLProc {
    unsafe { ll::glfwGetProcAddress(str::as_c_str(procname, |a| a)) }
}
