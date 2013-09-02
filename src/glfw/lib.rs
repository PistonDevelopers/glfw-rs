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
use std::comm::{Port, stream};
use std::libc::*;
use std::ptr;
use std::str;
use std::vec;

// re-export constants
pub use consts::*;

pub mod ffi;
pub mod consts;
mod extfn;

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
        TRUE => Ok(()),
        _    => Err(()),
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
        fmt!("%?.%?.%?", self.major, self.minor, self.rev)
    }
}

/// Wrapper for `glfwGetVersion`.
#[fixed_stack_segment] #[inline(never)]
pub fn get_version() -> Version {
    unsafe {
        let (major, minor, rev) = (0, 0, 0);
        ffi::glfwGetVersion(&major, &minor, &rev);
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
            let count = 0;
            let ptr = ffi::glfwGetMonitors(&count);
            vec::from_buf(ptr, count as uint).map(|&m| Monitor { ptr: m })
        }
    }

    /// Wrapper for `glfwGetMonitorPos`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_pos(&self) -> (int, int) {
        unsafe {
            let (xpos, ypos) = (0, 0);
            ffi::glfwGetMonitorPos(self.ptr, &xpos, &ypos);
            (xpos as int, ypos as int)
        }
    }

    /// Wrapper for `glfwGetMonitorPhysicalSize`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_physical_size(&self) -> (int, int) {
        unsafe {
            let (width, height) = (0, 0);
            ffi::glfwGetMonitorPhysicalSize(self.ptr, &width, &height);
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
            let count = 0;
            let ptr = ffi::glfwGetVideoModes(self.ptr, &count);
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
        fmt!("%? x %?, %? (%? %? %?) %? Hz",
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
        unsafe { ffi::glfwWindowHint(RED_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `GREEN_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn green_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(GREEN_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `BLUE_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn blue_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(BLUE_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ALPHA_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn alpha_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ALPHA_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `DEPTH_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn depth_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(DEPTH_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `STENCIL_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn stencil_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(STENCIL_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_RED_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn accum_red_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ACCUM_RED_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_GREEN_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn accum_green_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ACCUM_GREEN_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_BLUE_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn accum_blue_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ACCUM_BLUE_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `ACCUM_ALPHA_BITS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn accum_alpha_bits(bits: uint) {
        unsafe { ffi::glfwWindowHint(ACCUM_ALPHA_BITS, bits as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `AUX_BUFFERS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn aux_buffers(buffers: uint) {
        unsafe { ffi::glfwWindowHint(AUX_BUFFERS, buffers as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `STEREO`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn stereo(value: bool) {
        unsafe { ffi::glfwWindowHint(STEREO, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `SAMPLES`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn samples(samples: uint) {
        unsafe { ffi::glfwWindowHint(SAMPLES, samples as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `SRGB_CAPABLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn srgb_capable(value: bool) {
        unsafe { ffi::glfwWindowHint(SRGB_CAPABLE, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `REFRESH_RATE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn refresh_rate(rate: int) {
        unsafe { ffi::glfwWindowHint(REFRESH_RATE, rate as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CLIENT_API`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn client_api(api: c_int) {
        unsafe { ffi::glfwWindowHint(CLIENT_API, api); }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MAJOR`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn context_version_major(major: uint) {
        unsafe { ffi::glfwWindowHint(CONTEXT_VERSION_MAJOR, major as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MINOR`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn context_version_minor(minor: uint) {
        unsafe { ffi::glfwWindowHint(CONTEXT_VERSION_MINOR, minor as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_VERSION_MAJOR` and
    /// `CONTEXT_VERSION_MINOR`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn context_version(major: uint, minor: uint) {
        unsafe {
            ffi::glfwWindowHint(CONTEXT_VERSION_MAJOR, major as c_int);
            ffi::glfwWindowHint(CONTEXT_VERSION_MINOR, minor as c_int);
        }
    }

    /// Wrapper for `glfwWindowHint` called with `CONTEXT_ROBUSTNESS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn context_robustness(value: bool) {
        unsafe { ffi::glfwWindowHint(CONTEXT_ROBUSTNESS, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_FORWARD_COMPAT`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn opengl_forward_compat(value: bool) {
        unsafe { ffi::glfwWindowHint(OPENGL_FORWARD_COMPAT, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_DEBUG_CONTEXT`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn opengl_debug_context(value: bool) {
        unsafe { ffi::glfwWindowHint(OPENGL_DEBUG_CONTEXT, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `OPENGL_PROFILE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn opengl_profile(profile: c_int) {
        unsafe { ffi::glfwWindowHint(OPENGL_PROFILE, profile); }
    }

    /// Wrapper for `glfwWindowHint` called with `RESIZABLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn resizable(value: bool) {
        unsafe { ffi::glfwWindowHint(RESIZABLE, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `VISIBLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn visible(value: bool) {
        unsafe { ffi::glfwWindowHint(VISIBLE, value as c_int); }
    }

    /// Wrapper for `glfwWindowHint` called with `DECORATED`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn decorated(value: bool) {
        unsafe { ffi::glfwWindowHint(DECORATED, value as c_int); }
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
    priv port: Port<WindowEvent>,
    priv data_map: @mut WindowFns,
}

/// A group of key modifiers
#[deriving(Eq, Clone)]
pub struct KeyMods(c_int);

/// A key modifier token
#[deriving(Eq, Clone)]
pub enum KeyMod {
    Shift,
    Control,
    Alt,
    Super,
}

impl KeyMods {
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
    pub fn contains(&self, key_mod: KeyMod) -> bool {
        match key_mod {
            Shift   => (**self & MOD_SHIFT)   as bool,
            Control => (**self & MOD_CONTROL) as bool,
            Alt     => (**self & MOD_ALT)     as bool,
            Super   => (**self & MOD_SUPER)   as bool,
        }
    }
}

/// Events sent for registered window callback functions
#[deriving(Eq, Clone)]
pub enum WindowEvent {
    Pos { xpos: int, ypos: int },
    Size { width: int, height: int },
    Close,
    Refresh,
    Focus(bool),
    Iconify(bool),
    FrameBufferSize { width: int, height: int },
    MouseButton { button:c_int, action: c_int, mods: KeyMods },
    CursorPos { xpos: float, ypos: float },
    CursorEnter(bool),
    Scroll { xpos: float, ypos: float },
    Key { key: c_int, scancode: c_int, action: c_int, mods: KeyMods },
    Char(char),
}

pub type ErrorFun = @fn(error: c_int, description: ~str);
pub type WindowPosFun = @fn(window: &Window, xpos: int, ypos: int);
pub type WindowSizeFun = @fn(window: &Window, width: int, height: int);
pub type WindowCloseFun = @fn(window: &Window);
pub type WindowRefreshFun = @fn(window: &Window);
pub type WindowFocusFun = @fn(window: &Window, focused: bool);
pub type WindowIconifyFun = @fn(window: &Window, iconified: bool);
pub type FramebufferSizeFun = @fn(window: &Window, width: int, height: int);
pub type MouseButtonFun = @fn(window: &Window, button: c_int, action: c_int, mods: KeyMods);
pub type CursorPosFun = @fn(window: &Window, xpos: float, ypos: float);
pub type CursorEnterFun = @fn(window: &Window, entered: bool);
pub type ScrollFun = @fn(window: &Window, xpos: float, ypos: float);
pub type KeyFun = @fn(window: &Window, key: c_int, scancode: c_int, action: c_int, mods: KeyMods);
pub type CharFun = @fn(window: &Window, character: char);
pub type MonitorFun = @fn(monitor: &Monitor, event: c_int);

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
        self.data_map.$data_field = Some(cbfun);
        unsafe { ffi::$ll_fn(self.ptr, Some(extfn::$ext_fn)); }
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
                    let (port, chan) = stream();
                    ffi::glfwSetWindowUserPointer(ptr, cast::transmute(~chan));
                    Ok(Window {
                        ptr: ptr::to_unsafe_ptr(ptr),
                        is_shared: share.is_none(),
                        port: port,
                        data_map: @mut WindowFns::new()
                    })
                }
            )
        }
    }

    /// Updates all window callbacks if they have been triggered.
    pub fn poll_events(&self) {
        if self.port.peek() {
            match self.port.recv() {
                Pos { xpos, ypos }                      => self.data_map.pos_fun.map(|&cb| cb(self, xpos, ypos)),
                Size { width, height }                  => self.data_map.size_fun.map(|&cb| cb(self, width, height)),
                Close                                   => self.data_map.close_fun.map(|&cb| cb(self)),
                Refresh                                 => self.data_map.refresh_fun.map(|&cb| cb(self)),
                Focus(focused)                          => self.data_map.focus_fun.map(|&cb| cb(self, focused)),
                Iconify(iconified)                      => self.data_map.iconify_fun.map(|&cb| cb(self, iconified)),
                FrameBufferSize { width, height }       => self.data_map.framebuffer_size_fun.map(|&cb| cb(self, width, height)),
                MouseButton { button, action, mods }    => self.data_map.mouse_button_fun.map(|&cb| cb(self, button, action, mods)),
                CursorPos { xpos, ypos }                => self.data_map.cursor_pos_fun.map(|&cb| cb(self, xpos, ypos)),
                CursorEnter(entered)                    => self.data_map.cursor_enter_fun.map(|&cb| cb(self, entered)),
                Scroll { xpos, ypos }                   => self.data_map.scroll_fun.map(|&cb| cb(self, xpos, ypos)),
                Key { key, scancode, action, mods }     => self.data_map.key_fun.map(|&cb| cb(self, key, scancode, action, mods)),
                Char(character)                         => self.data_map.char_fun.map(|&cb| cb(self, character)),
            };
        }
    }

    /// Wrapper for `glfwWindowShouldClose`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn should_close(&self) -> bool {
        unsafe { ffi::glfwWindowShouldClose(self.ptr) as bool }
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
            let (xpos, ypos) = (0, 0);
            ffi::glfwGetWindowPos(self.ptr, &xpos, &ypos);
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
            let (width, height) = (0, 0);
            ffi::glfwGetWindowSize(self.ptr, &width, &height);
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
            let (width, height) = (0, 0);
            ffi::glfwGetFramebufferSize(self.ptr, &width, &height);
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
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, FOCUSED) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `ICONIFIED`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_iconified(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ICONIFIED) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `CLIENT_API`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_client_api(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, CLIENT_API) }
    }

    /// Wrapper for `glfw::ffi::glfwGetWindowAttrib` called with
    /// `CONTEXT_VERSION_MAJOR`, `CONTEXT_VERSION_MINOR` and `CONTEXT_REVISION`.
    ///
    /// # Returns
    ///
    /// The client API version of the window's context in a version struct.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_context_version(&self) -> Version {
        unsafe {
            Version {
                major:  ffi::glfwGetWindowAttrib(self.ptr, CONTEXT_VERSION_MAJOR) as uint,
                minor:  ffi::glfwGetWindowAttrib(self.ptr, CONTEXT_VERSION_MINOR) as uint,
                rev:    ffi::glfwGetWindowAttrib(self.ptr, CONTEXT_REVISION) as uint,
            }
        }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `CONTEXT_ROBUSTNESS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_context_robustness(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, CONTEXT_ROBUSTNESS) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_FORWARD_COMPAT`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_opengl_forward_compat(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, OPENGL_FORWARD_COMPAT) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_DEBUG_CONTEXT`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_opengl_debug_context(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, OPENGL_DEBUG_CONTEXT) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_PROFILE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_opengl_profile(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, OPENGL_PROFILE) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `RESIZABLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_resizable(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, RESIZABLE) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `VISIBLE`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_visible(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, VISIBLE) as bool }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `DECORATED`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_decorated(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, DECORATED) as bool }
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
    pub fn get_cursor_mode(&self) -> c_int {
        unsafe { ffi::glfwGetInputMode(self.ptr, CURSOR) }
    }

    /// Wrapper for `glfwSetInputMode` called with `CURSOR`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_cursor_mode(&self, mode: c_int) {
        unsafe { ffi::glfwSetInputMode(self.ptr, CURSOR, mode); }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_KEYS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn has_sticky_keys(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, STICKY_KEYS) as bool }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_KEYS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_sticky_keys(&self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, STICKY_KEYS, value as c_int); }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn has_sticky_mouse_buttons(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, STICKY_MOUSE_BUTTONS) as bool }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_sticky_mouse_buttons(&self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, STICKY_MOUSE_BUTTONS, value as c_int); }
    }

    /// Wrapper for `glfwGetKey`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_key(&self, key: c_int) -> c_int {
        unsafe { ffi::glfwGetKey(self.ptr, key) }
    }

    /// Wrapper for `glfwGetMouseButton`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_mouse_button(&self, button: c_int) -> c_int {
        unsafe { ffi::glfwGetMouseButton(self.ptr, button) }
    }

    /// Wrapper for `glfwGetCursorPos`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_cursor_pos(&self) -> (float, float) {
        unsafe {
            let (xpos, ypos) = (0.0, 0.0);
            ffi::glfwGetCursorPos(self.ptr, &xpos, &ypos);
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
    fn drop(&self) {
        if !self.is_shared {
            unsafe { ffi::glfwDestroyWindow(self.ptr); }
        }

        if !self.ptr.is_null() {
            // Free the boxed channel
            let _: ~Chan<WindowEvent> = unsafe {
                cast::transmute(ffi::glfwGetWindowUserPointer(self.ptr))
            };
        }
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

pub mod joystick {
    use std::libc::*;
    use std::str;
    use std::vec;

    use ffi;

    /// Wrapper for `glfwJoystickPresent`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_present(joy: c_int) -> bool {
        unsafe { ffi::glfwJoystickPresent(joy) as bool }
    }

    /// Wrapper for `glfwGetJoystickAxes`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_axes(joy: c_int) -> ~[float] {
        unsafe {
            let count = 0;
            let ptr = ffi::glfwGetJoystickAxes(joy, &count);
            vec::from_buf(ptr, count as uint).map(|&a| a as float)
        }
    }

    /// Wrapper for `glfwGetJoystickButtons`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_buttons(joy: c_int) -> ~[c_int] {
        unsafe {
            let count = 0;
            let ptr = ffi::glfwGetJoystickButtons(joy, &count);
            vec::from_buf(ptr, count as uint).map(|&b| b as c_int)
        }
    }

    /// Wrapper for `glfwGetJoystickName`.
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_name(joy: c_int) -> ~str {
        unsafe { str::raw::from_c_str(ffi::glfwGetJoystickName(joy)) }
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
            ffi::glfwExtensionSupported(extension) as bool
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
