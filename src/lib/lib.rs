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

#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![crate_id = "github.com/bjz/glfw-rs#glfw:0.1"]
#![comment = "Bindings and wrapper functions for glfw3."]

#![feature(globs)]
#![feature(macro_rules)]
#![feature(phase)]

//! An ideomatic wrapper for the GLFW library.
//!
//! # Example
//!
//! ~~~rust
//! extern crate native;
//! extern crate glfw;
//!
//! use glfw::Context;
//! 
//! #[start]
//! fn start(argc: int, argv: **u8) -> int {
//!     // Run GLFW on the main thread
//!     native::start(argc, argv, main)
//! }
//! 
//! fn main() {
//!    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
//! 
//!     // Create a windowed mode window and its OpenGL context
//!     let window = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
//!         .expect("Failed to create GLFW window.");
//! 
//!     // Make the window's context current
//!     window.make_current();
//! 
//!     // Loop until the user closes the window
//!     while !window.should_close() {
//!         // Swap front and back buffers
//!         window.swap_buffers();
//! 
//!         // Poll for and process events
//!         glfw.poll_events();
//!         for (_, event) in glfw::flush_messages(&events) {
//!             println!("{}", event);
//!             match event {
//!                 glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
//!                     window.set_should_close(true)
//!                 },
//!                 _ => {},
//!             }
//!         }
//!     }
//! }
//! ~~~

// TODO: Document differences between GLFW and glfw-rs

extern crate semver;
extern crate sync;
extern crate libc;
#[phase(syntax, link)] extern crate log;

use libc::{c_double, c_float, c_int};
use libc::{c_uint, c_ushort, c_void};
use std::cast;
use std::comm::{channel, Receiver, Sender, Data};
use std::fmt;
use std::kinds::marker;
use std::ptr;
use std::str;
use std::slice;
use semver::Version;

/// Alias to `MouseButton1`, supplied for improved clarity.
pub use MouseButtonLeft     = self::MouseButton1;
/// Alias to `MouseButton2`, supplied for improved clarity.
pub use MouseButtonRight    = self::MouseButton2;
/// Alias to `MouseButton3`, supplied for improved clarity.
pub use MouseButtonMiddle   = self::MouseButton3;

pub mod ffi;

mod callbacks;

/// Platform-specific linking. This module is automatically generated when
/// glfw-rs is compiled.
mod link;

/// Input actions.
#[repr(C)]
#[deriving(Clone, Eq, Hash, Show)]
pub enum Action {
    Release                      = ffi::RELEASE,
    Press                        = ffi::PRESS,
    Repeat                       = ffi::REPEAT,
}

/// Input keys.
#[repr(C)]
#[deriving(Clone, Eq, Hash, Show)]
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

/// Mouse buttons. The `MouseButtonLeft`, `MouseButtonRight`, and
/// `MouseButtonMiddle` aliases are supplied for convenience.
#[repr(C)]
#[deriving(Clone, Eq, Hash, Show)]
pub enum MouseButton {
    /// The left mouse button. A `MouseButtonLeft` alias is provided to improve clarity.
    MouseButton1                = ffi::MOUSE_BUTTON_1,
    /// The right mouse button. A `MouseButtonRight` alias is provided to improve clarity.
    MouseButton2                = ffi::MOUSE_BUTTON_2,
    /// The middle mouse button. A `MouseButtonMiddle` alias is provided to improve clarity.
    MouseButton3                = ffi::MOUSE_BUTTON_3,
    MouseButton4                = ffi::MOUSE_BUTTON_4,
    MouseButton5                = ffi::MOUSE_BUTTON_5,
    MouseButton6                = ffi::MOUSE_BUTTON_6,
    MouseButton7                = ffi::MOUSE_BUTTON_7,
    MouseButton8                = ffi::MOUSE_BUTTON_8,
}

/// Formats the type using aliases rather than the default variant names.
///
/// # Example
///
/// ~~~rust
/// assert_eq(format!("{}", glfw::MouseButtonLeft), ~"MouseButton1");
/// assert_eq(format!("{}", glfw::ShowAliases(glfw::MouseButtonLeft)), ~"MouseButtonLeft");
/// ~~~
pub struct ShowAliases<T>(pub T);

impl fmt::Show for ShowAliases<MouseButton> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ShowAliases(button) = *self;
        match button {
            MouseButtonLeft     => write!(f.buf, "MouseButtonLeft"),
            MouseButtonRight    => write!(f.buf, "MouseButtonRight"),
            MouseButtonMiddle   => write!(f.buf, "MouseButtonMiddle"),
            button              => button.fmt(f),
        }
    }
}

pub struct Callback<Fn, UserData> {
    pub f: Fn,
    pub data: UserData,
}

/// Tokens corresponding to various error types.
#[repr(C)]
#[deriving(Clone, Eq, Hash, Show)]
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

/// An error callback. This can be supplied with some user data to be passed to
/// the callback function when it is triggered.
pub type ErrorCallback<UserData> = Callback<fn(Error, ~str, &UserData), UserData>;

/// The function to be used with the `FAIL_ON_ERRORS` callback.
pub fn fail_on_errors(_: Error, description: ~str, _: &()) {
    fail!("GLFW Error: {}", description);
}

/// A callback that triggers a task failure when an error is encountered.
pub static FAIL_ON_ERRORS: Option<ErrorCallback<()>> =
    Some(Callback { f: fail_on_errors, data: () });

/// The function to be used with the `LOG_ERRORS` callback.
pub fn log_errors(_: Error, description: ~str, _: &()) {
    error!("GLFW Error: {}", description);
}

/// A callback that logs each error as it is encountered without triggering a
/// task failure.
pub static LOG_ERRORS: Option<ErrorCallback<()>> =
    Some(Callback { f: log_errors, data: () });

/// Cursor modes.
#[repr(C)]
#[deriving(Clone, Eq, Hash, Show)]
pub enum CursorMode {
    CursorNormal                = ffi::CURSOR_NORMAL,
    CursorHidden                = ffi::CURSOR_HIDDEN,
    CursorDisabled              = ffi::CURSOR_DISABLED,
}

/// Describes a single video mode.
pub struct VidMode {
    pub width:        u32,
    pub height:       u32,
    pub red_bits:     u32,
    pub green_bits:   u32,
    pub blue_bits:    u32,
    pub refresh_rate: u32,
}

/// Describes the gamma ramp of a monitor.
pub struct GammaRamp {
    pub red:    ~[c_ushort],
    pub green:  ~[c_ushort],
    pub blue:   ~[c_ushort],
}

/// An OpenGL process address.
pub type GLProc = ffi::GLFWglproc;
 
/// A token from which to call various GLFW functions. It can be obtained by
/// calling the `init` function. This cannot be sent to other tasks, and should
/// only be initialized on the main platform thread. Whilst this might make
/// performing some operations harder, this is to ensure thread safety is enforced
/// statically. The context can be safely cloned or implicitly copied if need be
/// for convenience.
#[deriving(Clone)]
pub struct Glfw {
    no_send: marker::NoSend,
    no_share: marker::NoShare,
}

/// An error that might be returned when `glfw::init` is called.
#[deriving(Eq, Show)]
pub enum InitError {
    /// The library was already initialized.
    AlreadyInitialized,
    /// An internal error occured when trying to initialize the library.
    InternalInitError,
}

/// Initializes the GLFW library. This must be called on the main platform
/// thread.
///
/// Wrapper for `glfwInit`.
///
/// # Error callback
///
/// An error callback can be set if desired. This allows for the handling of any
/// errors that occur during initialization. This can subsequently be changed
/// using the `Glfw::set_error_callback` function.
///
/// # Example
///
/// ~~~rust
/// extern crate native;
/// extern crate glfw;
/// 
/// #[start]
/// fn start(argc: int, argv: **u8) -> int {
///     // Run GLFW on the main thread
///     native::start(argc, argv, main)
/// }
/// 
/// fn main() {
///    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
/// }
/// ~~~
///
/// # Returns
///
/// - If initialization was successful a `Glfw` token will be returned along
///   with a `Receiver` from which errors can be intercepted.
/// - Subsequent calls to `init` will return `Err(AlreadyInitialized)`.
/// - If an initialization error occured within the GLFW library
///   `Err(InternalInitError)` will be returned.
pub fn init<UserData: 'static>(mut callback: Option<ErrorCallback<UserData>>) -> Result<Glfw, InitError> {
    use sync::one::{Once, ONCE_INIT};
    static mut INIT: Once = ONCE_INIT;
    let mut result = Err(AlreadyInitialized);
    unsafe {
        INIT.doit(|| {
            // Initialize the error callback if it was supplied. This is done
            // before `ffi::glfwInit` because errors could occur during
            // initialization.
            match callback.take() {
                Some(f) => callbacks::error::set(f),
                None    => callbacks::error::unset(),
            }
            if ffi::glfwInit() == ffi::TRUE {
                result = Ok(());
                std::rt::at_exit(proc() {
                    ffi::glfwTerminate()
                });
            } else {
                result = Err(InternalInitError);
            }
        })
    }
    result.map(|_| Glfw {
        no_send: marker::NoSend,
        no_share: marker::NoShare,
    })
}

impl Glfw {
    /// Sets the error callback, overwriting the previous one stored.
    ///
    /// # Example
    ///
    /// ~~~rust
    /// fn fail_on_errors(_: glfw::Error, description: ~str, prefix: &'static str) {
    ///     fail!("{}{}", prefix, description);
    /// }
    ///
    /// // sets a new callback
    /// glfw.set_error_callback(Some((fail_on_errors, "GLFW Error: ")));
    ///
    /// // removes the previously set callback
    /// glfw.set_error_callback(None);
    /// ~~~
    ///
    /// The `FAIL_ON_ERRORS` and `LOG_ERRORS` callbacks are provided for
    /// convenience. For example:
    ///
    /// ~~~rust
    /// // triggers a task failure when a GLFW error is encountered.
    /// glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    /// ~~~
    pub fn set_error_callback<UserData: 'static>(&self, callback: Option<ErrorCallback<UserData>>) {
        match callback {
            Some(f) => callbacks::error::set(f),
            None    => callbacks::error::unset(),
        }
    }

    /// Sets the error callback, overwriting the previous one stored.
    pub fn set_monitor_callback<UserData: 'static>(&self, callback: Option<MonitorCallback<UserData>>) {
        match callback {
            Some(f) => callbacks::monitor::set(f),
            None    => callbacks::monitor::unset(),
        }
    }

    /// Supplies the primary monitor to the closure provided, if it exists.
    /// This is usually the monitor where elements like the Windows task bar or
    /// the OS X menu bar is located.
    ///
    /// Wrapper for `glfwGetPrimaryMonitor`.
    pub fn get_primary_monitor<T>(&self, f: |Option<&Monitor>| -> T) -> T {
        match unsafe { ffi::glfwGetPrimaryMonitor() } {
            ptr if ptr.is_null() => f(None),
            ptr => f(Some(&Monitor {
                ptr: ptr,
                no_copy: marker::NoCopy,
                no_send: marker::NoSend,
                no_share: marker::NoShare,
            })),
        }
    }

    /// Supplies a vector of the currently connected monitors to the closure
    /// provided.
    ///
    /// Wrapper for `glfwGetMonitors`.
    pub fn get_connected_monitors<T>(&self, f: |&[Monitor]| -> T) -> T {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetMonitors(&mut count);
            f(slice::from_buf(ptr, count as uint).iter().map(|&ptr| {
                Monitor {
                    ptr: ptr,
                    no_copy: marker::NoCopy,
                    no_send: marker::NoSend,
                    no_share: marker::NoShare,
                }
            }).collect::<~[Monitor]>())
        }
    }

    /// This is used to set the window hints for the next call to
    /// `Glfw::create_window`. The hints can be reset to their default values
    /// using calling the `Glfw::default_window_hints` function.
    ///
    /// Wrapper for `glfwWindowHint`
    ///
    /// # OpenGL 3.x and 4.x on Mac OS X
    ///
    /// The only OpenGL 3.x and 4.x contexts supported by OS X are
    /// forward-compatible, core profile contexts.
    ///
    /// 10.7 and 10.8 support the following OpenGL versions:
    ///
    /// - `glfw::ContextVersion(3, 2)`
    ///
    /// 10.9 supports the following OpenGL versions
    ///
    /// - `glfw::ContextVersion(3, 2)`
    /// - `glfw::ContextVersion(3, 3)`
    /// - `glfw::ContextVersion(4, 1)`
    ///
    /// To create an OS X compatible context, the hints should be specified as
    /// follows:
    ///
    /// ~~~rust
    /// glfw.window_hint(glfw::ContextVersion(3, 2));
    /// glfw.window_hint(glfw::OpenglForwardCompat(true));
    /// glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
    /// ~~~
    pub fn window_hint(&self, hint: WindowHint) {
        match hint {
            RedBits(bits)                   => unsafe { ffi::glfwWindowHint(ffi::RED_BITS,              bits as c_int) },
            GreenBits(bits)                 => unsafe { ffi::glfwWindowHint(ffi::GREEN_BITS,            bits as c_int) },
            BlueBits(bits)                  => unsafe { ffi::glfwWindowHint(ffi::BLUE_BITS,             bits as c_int) },
            AlphaBits(bits)                 => unsafe { ffi::glfwWindowHint(ffi::ALPHA_BITS,            bits as c_int) },
            DepthBits(bits)                 => unsafe { ffi::glfwWindowHint(ffi::DEPTH_BITS,            bits as c_int) },
            StencilBits(bits)               => unsafe { ffi::glfwWindowHint(ffi::STENCIL_BITS,          bits as c_int) },
            AccumRedBits(bits)              => unsafe { ffi::glfwWindowHint(ffi::ACCUM_RED_BITS,        bits as c_int) },
            AccumGreenBits(bits)            => unsafe { ffi::glfwWindowHint(ffi::ACCUM_GREEN_BITS,      bits as c_int) },
            AccumBlueBits(bits)             => unsafe { ffi::glfwWindowHint(ffi::ACCUM_BLUE_BITS,       bits as c_int) },
            AccumAlphaBits(bits)            => unsafe { ffi::glfwWindowHint(ffi::ACCUM_ALPHA_BITS,      bits as c_int) },
            AuxBuffers(num_buffers)         => unsafe { ffi::glfwWindowHint(ffi::AUX_BUFFERS,           num_buffers as c_int) },
            Stereo(is_stereo)               => unsafe { ffi::glfwWindowHint(ffi::STEREO,                is_stereo as c_int) },
            Samples(num_samples)            => unsafe { ffi::glfwWindowHint(ffi::SAMPLES,               num_samples as c_int) },
            SRgbCapable(is_capable)         => unsafe { ffi::glfwWindowHint(ffi::SRGB_CAPABLE,          is_capable as c_int) },
            RefreshRate(rate)               => unsafe { ffi::glfwWindowHint(ffi::REFRESH_RATE,          rate as c_int) },
            ClientApi(api)                  => unsafe { ffi::glfwWindowHint(ffi::CLIENT_API,            api as c_int) },
            ContextVersionMajor(major)      => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, major as c_int) },
            ContextVersionMinor(minor)      => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, minor as c_int) },
            ContextVersion(major, minor)    => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, major as c_int);
                                                        ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, minor as c_int) },
            ContextRobustness(robustness)   => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_ROBUSTNESS,    robustness as c_int) },
            OpenglForwardCompat(is_compat)  => unsafe { ffi::glfwWindowHint(ffi::OPENGL_FORWARD_COMPAT, is_compat as c_int) },
            OpenglDebugContext(is_debug)    => unsafe { ffi::glfwWindowHint(ffi::OPENGL_DEBUG_CONTEXT,  is_debug as c_int) },
            OpenglProfile(profile)          => unsafe { ffi::glfwWindowHint(ffi::OPENGL_PROFILE,        profile as c_int) },
            Resizable(is_resizable)         => unsafe { ffi::glfwWindowHint(ffi::RESIZABLE,             is_resizable as c_int) },
            Visible(is_visible)             => unsafe { ffi::glfwWindowHint(ffi::VISIBLE,               is_visible as c_int) },
            Decorated(is_decorated)         => unsafe { ffi::glfwWindowHint(ffi::DECORATED,             is_decorated as c_int) },
        }
    }

    /// Resets the window hints previously set by the `window_hint` function to
    /// their default values.
    ///
    /// Wrapper for `glfwDefaultWindowHints`.
    pub fn default_window_hints(&self) {
        unsafe { ffi::glfwDefaultWindowHints(); }
    }

    /// Creates a new window.
    ///
    /// Wrapper for `glfwCreateWindow`.
    pub fn create_window(&self, width: u32, height: u32, title: &str, mode: WindowMode) -> Option<(Window, Receiver<(f64, WindowEvent)>)> {
        self.create_window_intern(width, height, title, mode, None)
    }

    /// Internal wrapper for `glfwCreateWindow`.
    fn create_window_intern(&self, width: u32, height: u32, title: &str, mode: WindowMode, share: Option<&Window>) -> Option<(Window, Receiver<(f64, WindowEvent)>)> {
        let ptr = unsafe {
            title.with_c_str(|title| {
                ffi::glfwCreateWindow(
                    width as c_int,
                    height as c_int,
                    title,
                    mode.to_ptr(),
                    match share { Some(w) => w.ptr, None => ptr::null() }
                )
            })
        };
        if ptr.is_null() {
            None
        } else {
            let (drop_sender, drop_receiver) = channel();
            let (sender, receiver) = channel();
            unsafe { ffi::glfwSetWindowUserPointer(ptr, cast::transmute(~sender)); }
            Some((
                Window {
                    ptr: ptr,
                    glfw: self.clone(),
                    is_shared: share.is_none(),
                    drop_sender: Some(drop_sender),
                    drop_receiver: drop_receiver
                },
                receiver,
            ))
        }
    }

    /// Makes the context of the specified window current. If no window is given
    /// then the current context is detached.
    ///
    /// Wrapper for `glfwMakeContextCurrent`.
    pub fn make_context_current(&self, context: Option<&Window>) {
        match context {
            Some(window) => unsafe { ffi::glfwMakeContextCurrent(window.ptr) },
            None         => unsafe { ffi::glfwMakeContextCurrent(ptr::null()) },
        }
    }

    /// Wrapper for `glfwGetX11Display`
    #[cfg(target_os="linux")]
    pub fn get_x11_display(&self) -> *c_void {
        unsafe { ffi::glfwGetX11Display() }
    }

    /// Immediately process the received events.
    ///
    /// Wrapper for `glfwPollEvents`.
    pub fn poll_events(&self) {
        unsafe { ffi::glfwPollEvents(); }
    }

    /// Sleep until at least one event has been recieved, and then perform the
    /// equivalent of `Glfw::poll_events`.
    ///
    /// Wrapper for `glfwWaitEvents`.
    pub fn wait_events(&self) {
        unsafe { ffi::glfwWaitEvents(); }
    }

    /// Returns the current value of the GLFW timer. Unless the timer has been
    /// set using `glfw::set_time`, the timer measures time elapsed since GLFW
    /// was initialized.
    ///
    /// Wrapper for `glfwGetTime`.
    pub fn get_time(&self) -> f64 {
        unsafe { ffi::glfwGetTime() as f64 }
    }

    /// Sets the value of the GLFW timer.
    ///
    /// Wrapper for `glfwSetTime`.
    pub fn set_time(&self, time: f64) {
        unsafe { ffi::glfwSetTime(time as c_double); }
    }

    /// Sets the number of screen updates to wait before swapping the buffers of
    /// the current context and returning from `Window::swap_buffers`.
    ///
    /// Wrapper for `glfwSwapInterval`.
    pub fn set_swap_interval(&self, interval: u32) {
        unsafe { ffi::glfwSwapInterval(interval as c_int); }
    }

    /// Returns `true` if the specified OpenGL or context creation API extension
    /// is supported by the current context.
    ///
    /// Wrapper for `glfwExtensionSupported`.
    pub fn extension_supported(&self, extension: &str) -> bool {
        unsafe {
            extension.with_c_str(|extension| {
                ffi::glfwExtensionSupported(extension) == ffi::TRUE
            })
        }
    }

    /// Returns the address of the specified client API or extension function
    /// if it is supported by the current context.
    ///
    /// Wrapper for `glfwGetProcAddress`.
    pub fn get_proc_address(&self, procname: &str) -> Option<GLProc> {
        unsafe {
            procname.with_c_str(|procname| {
                ffi::glfwGetProcAddress(procname)
            })
        }
    }

    /// Constructs a `Joystick` handle corresponding to the supplied `JoystickId`.
    pub fn get_joystick(&self, id: JoystickId) -> Joystick {
        Joystick { id: id, glfw: self.clone() }
    }
}

/// Wrapper for `glfwGetVersion`.
pub fn get_version() -> Version {
    unsafe {
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        ffi::glfwGetVersion(&mut major, &mut minor, &mut patch);
        Version {
            major: major as uint,
            minor: minor as uint,
            patch: patch as uint,
            pre:   Vec::new(),
            build: Vec::new(),
        }
    }
}

/// Wrapper for `glfwGetVersionString`.
pub fn get_version_string() -> ~str {
    unsafe { str::raw::from_c_str(ffi::glfwGetVersionString()) }
}

/// An monitor callback. This can be supplied with some user data to be passed
/// to the callback function when it is triggered.
pub type MonitorCallback<UserData> = Callback<fn(Monitor, MonitorEvent, &UserData), UserData>;

/// A struct that wraps a `*GLFWmonitor` handle.
pub struct Monitor {
    ptr: *ffi::GLFWmonitor,
    no_copy: marker::NoCopy,
    no_send: marker::NoSend,
    no_share: marker::NoShare,
}

impl Monitor {
    /// Wrapper for `glfwGetMonitorPos`.
    pub fn get_pos(&self) -> (i32, i32) {
        unsafe {
            let mut xpos = 0;
            let mut ypos = 0;
            ffi::glfwGetMonitorPos(self.ptr, &mut xpos, &mut ypos);
            (xpos as i32, ypos as i32)
        }
    }

    /// Wrapper for `glfwGetMonitorPhysicalSize`.
    pub fn get_physical_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = 0;
            let mut height = 0;
            ffi::glfwGetMonitorPhysicalSize(self.ptr, &mut width, &mut height);
            (width as i32, height as i32)
        }
    }

    /// Wrapper for `glfwGetMonitorName`.
    pub fn get_name(&self) -> ~str {
        unsafe { str::raw::from_c_str(ffi::glfwGetMonitorName(self.ptr)) }
    }

    /// Wrapper for `glfwGetVideoModes`.
    pub fn get_video_modes(&self) -> Vec<VidMode> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetVideoModes(self.ptr, &mut count);
            slice::from_buf(ptr, count as uint).iter().map(VidMode::from_glfw_vid_mode).collect()
        }
    }

    /// Wrapper for `glfwGetVideoMode`.
    pub fn get_video_mode(&self) -> Option<VidMode> {
        unsafe {
            ffi::glfwGetVideoMode(self.ptr).to_option().map(|v| VidMode::from_glfw_vid_mode(v))
        }
    }

    /// Wrapper for `glfwSetGamma`.
    pub fn set_gamma(&self, gamma: f32) {
        unsafe { ffi::glfwSetGamma(self.ptr, gamma as c_float); }
    }

    /// Wrapper for `glfwGetGammaRamp`.
    pub fn get_gamma_ramp(&self) -> GammaRamp {
        unsafe {
            let llramp = *ffi::glfwGetGammaRamp(self.ptr);
            GammaRamp {
                red:    slice::from_buf(llramp.red,   llramp.size as uint),
                green:  slice::from_buf(llramp.green, llramp.size as uint),
                blue:   slice::from_buf(llramp.blue,  llramp.size as uint),
            }
        }
    }

    /// Wrapper for `glfwSetGammaRamp`.
    pub fn set_gamma_ramp(&self, ramp: &GammaRamp) {
        unsafe {
            ffi::glfwSetGammaRamp(
                self.ptr,
                &ffi::GLFWgammaramp {
                    red:    ramp.red.as_ptr(),
                    green:  ramp.green.as_ptr(),
                    blue:   ramp.blue.as_ptr(),
                    size:   ramp.red.len() as c_uint,
                }
            );
        }
    }
}

/// Monitor events.
#[repr(C)]
pub enum MonitorEvent {
    Connected                   = ffi::CONNECTED,
    Disconnected                = ffi::DISCONNECTED,
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

impl fmt::Show for VidMode {
    /// Returns a string representation of the video mode.
    ///
    /// # Returns
    ///
    /// A string in the form:
    ///
    /// ~~~
    /// ~"[width] x [height], [total_bits] ([red_bits] [green_bits] [blue_bits]) [refresh_rate] Hz"
    /// ~~~
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{} x {}, {} = {} + {} + {}, {} Hz",
            self.width, self.height,
            self.red_bits + self.green_bits + self.blue_bits,
            self.red_bits, self.green_bits, self.blue_bits,
            self.refresh_rate)
    }
}

/// Window hints that can be set using the `window_hint` function.
pub enum WindowHint {
    /// Specifies the desired bit depth of the red component of the default framebuffer.
    RedBits(u32),
    /// Specifies the desired bit depth of the green component of the default framebuffer.
    GreenBits(u32),
    /// Specifies the desired bit depth of the blue component of the default framebuffer.
    BlueBits(u32),
    /// Specifies the desired bit depth of the alpha component of the default framebuffer.
    AlphaBits(u32),
    /// Specifies the desired bit depth of the depth component of the default framebuffer.
    DepthBits(u32),
    /// Specifies the desired bit depth of the stencil component of the default framebuffer.
    StencilBits(u32),
    /// Specifies the desired bit depth of the red component of the accumulation framebuffer.
    AccumRedBits(u32),
    /// Specifies the desired bit depth of the green component of the accumulation framebuffer.
    AccumGreenBits(u32),
    /// Specifies the desired bit depth of the blue component of the accumulation framebuffer.
    AccumBlueBits(u32),
    /// Specifies the desired bit depth of the alpha component of the accumulation framebuffer.
    AccumAlphaBits(u32),
    /// Specifies the desired number of auxiliary buffers.
    AuxBuffers(u32),
    /// Specifies whether to use stereoscopic rendering.
    Stereo(bool),
    /// Specifies the desired number of samples to use for multisampling. Zero
    /// disables multisampling.
    Samples(u32),
    /// Specifies whether the framebuffer should be sRGB capable.
    SRgbCapable(bool),
    /// Specifies the desired refresh rate for full screen windows. If set to
    /// zero, the highest available refresh rate will be used.
    ///
    /// This hint is ignored for windowed mode windows.
    RefreshRate(u32),
    /// Specifies which `ClientApi` to create the context for.
    ClientApi(ClientApi),
    /// Specifies the major client API version that the created context must be
    /// compatible with.
    ///
    /// Window creation will fail if the resulting OpenGL version is less than
    /// the one requested.
    ContextVersionMajor(u32),
    /// Specifies the minor client API version that the created context must be
    /// compatible with.
    ///
    /// Window creation will fail if the resulting OpenGL version is less than
    /// the one requested.
    ContextVersionMinor(u32),
    /// Specifies the client API version that the created context must be
    /// compatible with. This is the same as successive calls to `window_hint`
    /// function with the `ContextVersionMajor` and `ContextVersionMinor` hints.
    ///
    /// Window creation will fail if the resulting OpenGL version is less than
    /// the one requested.
    ///
    /// If `ContextVersion(1, 0)` is requested, _most_ drivers will provide the
    /// highest available context.
    ContextVersion(u32, u32),
    /// Specifies the `ContextRobustness` strategy to be used.
    ContextRobustness(ContextRobustness),
    /// Specifies whether the OpenGL context should be forward-compatible, i.e.
    /// one where all functionality deprecated in the requested version of
    /// OpenGL is removed. This may only be used if the requested OpenGL version
    /// is 3.0 or above.
    ///
    /// If another client API is requested, this hint is ignored.
    OpenglForwardCompat(bool),
    /// Specifies whether to create a debug OpenGL context, which may have
    /// additional error and performance issue reporting functionality.
    ///
    /// If another client API is requested, this hint is ignored.
    OpenglDebugContext(bool),
    /// Specifies which OpenGL profile to create the context for. If requesting
    /// an OpenGL version below 3.2, `OpenGlAnyProfile` must be used.
    ///
    /// If another client API is requested, this hint is ignored.
    OpenglProfile(OpenGlProfile),
    /// Specifies whether the window will be resizable by the user. Even if this
    /// is set to `false`, the window can still be resized using the
    /// `Window::set_size` function.
    ///
    /// This hint is ignored for fullscreen windows.
    Resizable(bool),
    /// Specifies whether the window will be visible on creation.
    ///
    /// This hint is ignored for fullscreen windows.
    Visible(bool),
    /// Specifies whether the window will have platform-specific decorations
    /// such as a border, a close widget, etc.
    ///
    /// This hint is ignored for full screen windows.
    Decorated(bool),
}

/// Client API tokens.
#[repr(C)]
#[deriving(Clone, Eq, Show)]
pub enum ClientApi {
    OpenGlApi                   = ffi::OPENGL_API,
    OpenGlEsApi                 = ffi::OPENGL_ES_API,
}

/// Context robustness tokens.
#[repr(C)]
#[deriving(Clone, Eq, Show)]
pub enum ContextRobustness {
    NoRobustness                = ffi::NO_ROBUSTNESS,
    NoResetNotification         = ffi::NO_RESET_NOTIFICATION,
    LoseContextOnReset          = ffi::LOSE_CONTEXT_ON_RESET,
}

/// OpenGL profile tokens.
#[repr(C)]
#[deriving(Clone, Eq, Show)]
pub enum OpenGlProfile {
    OpenGlAnyProfile            = ffi::OPENGL_ANY_PROFILE,
    OpenGlCoreProfile           = ffi::OPENGL_CORE_PROFILE,
    OpenGlCompatProfile         = ffi::OPENGL_COMPAT_PROFILE,
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
            FullScreen(Monitor {
                ptr: ptr,
                no_copy: marker::NoCopy,
                no_send: marker::NoSend,
                no_share: marker::NoShare,
            })
        }
    }

    /// Returns a pointer to a monitor if the window is fullscreen, otherwise
    /// it returns a null pointer (if it is in windowed mode).
    fn to_ptr(&self) -> *ffi::GLFWmonitor {
        match *self {
            FullScreen(ref monitor) => monitor.ptr,
            Windowed                => ptr::null(),
        }
    }
}

/// A group of key modifiers
pub struct Modifiers {
    pub values: c_int,
}

/// Key modifier tokens
#[repr(C)]
#[deriving(Clone, Eq, Hash, Show)]
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
    ///         println!("Shift detected!")
    ///     }
    /// }
    /// ~~~
    pub fn contains(&self, modifier: Modifier) -> bool {
        self.values & (modifier as c_int) != ffi::FALSE
    }
}

impl fmt::Show for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, x) in [Shift, Control, Alt, Super].iter().filter(|x| self.contains(**x)).enumerate() {
            if i != 0 { try!(write!(f.buf, ", ")) };
            try!(write!(f.buf, "{}", *x));
        }
        Ok(())
    }
}

pub type Scancode = c_int;

/// Window event messages.
#[deriving(Show)]
pub enum WindowEvent {
    PosEvent(i32, i32),
    SizeEvent(i32, i32),
    CloseEvent,
    RefreshEvent,
    FocusEvent(bool),
    IconifyEvent(bool),
    FramebufferSizeEvent(i32, i32),
    MouseButtonEvent(MouseButton, Action, Modifiers),
    CursorPosEvent(f64, f64),
    CursorEnterEvent(bool),
    ScrollEvent(f64, f64),
    KeyEvent(Key, Scancode, Action, Modifiers),
    CharEvent(char),
}

/// Returns an iterator that yeilds until no more messages are contained in the
/// `Receiver`'s queue. This is useful for event handling where the blocking
/// behaviour of `Receiver::iter` is undesirable.
///
/// # Example
///
/// ~~~rust
/// for event in glfw::flush_messages(&events) {
///     // handle event
/// }
/// ~~~
pub fn flush_messages<'a, Message: Send>(receiver: &'a Receiver<Message>) -> FlushedMessages<'a, Message> {
    FlushedMessages(receiver)
}

/// An iterator that yeilds until no more messages are contained in the
/// `Receiver`'s queue.
pub struct FlushedMessages<'a, Message>(&'a Receiver<Message>);

impl<'a, Message: Send> Iterator<Message> for FlushedMessages<'a, Message> {
    fn next(&mut self) -> Option<Message> {
        let FlushedMessages(receiver) = *self;
        match receiver.try_recv() {
            Data(message) => Some(message),
            _ => None,
        }
    }
}

/// A message for notifying a `Window` that a `RenderContext` has been dropped.
#[deriving(Eq)]
struct ContextDropped;

/// A struct that wraps a `*GLFWwindow` handle.
pub struct Window {
    pub ptr: *ffi::GLFWwindow,
    pub glfw: Glfw,
    pub is_shared: bool,
    drop_sender: Option<Sender<ContextDropped>>,
    drop_receiver: Receiver<ContextDropped>
}

macro_rules! set_window_callback(
    ($should_poll:expr, $ll_fn:ident, $callback:ident) => ({
        if $should_poll {
            unsafe { ffi::$ll_fn(self.ptr, Some(callbacks::$callback)); }
        } else {
            unsafe { ffi::$ll_fn(self.ptr, None); }
        }
    })
)

impl Window {
    /// Wrapper for `glfwCreateWindow`.
    pub fn create_shared(&self, width: u32, height: u32, title: &str, mode: WindowMode) -> Option<(Window, Receiver<(f64, WindowEvent)>)> {
        self.glfw.create_window_intern(width, height, title, mode, Some(self))
    }

    /// Calling this method forces the destructor to be called, closing the
    /// window.
    pub fn close(self) {}

    /// Returns a render context that can be shared between tasks, allowing
    /// for concurrent rendering.
    pub fn render_context(&mut self) -> RenderContext {
        RenderContext {
            ptr: self.ptr,
            // this will only be None after dropping so this is safe
            drop_sender: self.drop_sender.as_ref().unwrap().clone()
        }
    }

    /// Wrapper for `glfwWindowShouldClose`.
    pub fn should_close(&self) -> bool {
        unsafe { ffi::glfwWindowShouldClose(self.ptr) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowShouldClose`.
    pub fn set_should_close(&self, value: bool) {
        unsafe { ffi::glfwSetWindowShouldClose(self.ptr, value as c_int) }
    }

    /// Sets the title of the window.
    ///
    /// Wrapper for `glfwSetWindowTitle`.
    pub fn set_title(&self, title: &str) {
        unsafe {
            title.with_c_str(|title| {
                ffi::glfwSetWindowTitle(self.ptr, title);
            });
        }
    }

    /// Wrapper for `glfwGetWindowPos`.
    pub fn get_pos(&self) -> (i32, i32) {
        unsafe {
            let mut xpos = 0;
            let mut ypos = 0;
            ffi::glfwGetWindowPos(self.ptr, &mut xpos, &mut ypos);
            (xpos as i32, ypos as i32)
        }
    }

    /// Wrapper for `glfwSetWindowPos`.
    pub fn set_pos(&self, xpos: i32, ypos: i32) {
        unsafe { ffi::glfwSetWindowPos(self.ptr, xpos as c_int, ypos as c_int); }
    }

    /// Wrapper for `glfwGetWindowSize`.
    pub fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = 0;
            let mut height = 0;
            ffi::glfwGetWindowSize(self.ptr, &mut width, &mut height);
            (width as i32, height as i32)
        }
    }

    /// Wrapper for `glfwSetWindowSize`.
    pub fn set_size(&self, width: i32, height: i32) {
        unsafe { ffi::glfwSetWindowSize(self.ptr, width as c_int, height as c_int); }
    }

    /// Wrapper for `glfwGetFramebufferSize`.
    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = 0;
            let mut height = 0;
            ffi::glfwGetFramebufferSize(self.ptr, &mut width, &mut height);
            (width as i32, height as i32)
        }
    }

    /// Wrapper for `glfwIconifyWindow`.
    pub fn iconify(&self) {
        unsafe { ffi::glfwIconifyWindow(self.ptr); }
    }

    /// Wrapper for `glfwRestoreWindow`.
    pub fn restore(&self) {
        unsafe { ffi::glfwRestoreWindow(self.ptr); }
    }

    /// Wrapper for `glfwShowWindow`.
    pub fn show(&self) {
        unsafe { ffi::glfwShowWindow(self.ptr); }
    }

    /// Wrapper for `glfwHideWindow`.
    pub fn hide(&self) {
        unsafe { ffi::glfwHideWindow(self.ptr); }
    }

    /// Returns whether the window is fullscreen or windowed.
    ///
    /// Wrapper for `glfwGetWindowMonitor`.
    pub fn get_window_mode(&self) -> WindowMode {
        WindowMode::from_ptr(
            unsafe { ffi::glfwGetWindowMonitor(self.ptr) }
        )
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `FOCUSED`.
    pub fn is_focused(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::FOCUSED) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `ICONIFIED`.
    pub fn is_iconified(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::ICONIFIED) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `CLIENT_API`.
    pub fn get_client_api(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::CLIENT_API) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with
    /// `CONTEXT_VERSION_MAJOR`, `CONTEXT_VERSION_MINOR` and `CONTEXT_REVISION`.
    ///
    /// # Returns
    ///
    /// The client API version of the window's context in a version struct.
    pub fn get_context_version(&self) -> Version {
        unsafe {
            Version {
                major: ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_VERSION_MAJOR) as uint,
                minor: ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_VERSION_MINOR) as uint,
                patch: ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_REVISION) as uint,
                pre:   Vec::new(),
                build: Vec::new(),
            }
        }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `CONTEXT_ROBUSTNESS`.
    pub fn get_context_robustness(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_ROBUSTNESS) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_FORWARD_COMPAT`.
    pub fn is_opengl_forward_compat(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::OPENGL_FORWARD_COMPAT) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_DEBUG_CONTEXT`.
    pub fn is_opengl_debug_context(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::OPENGL_DEBUG_CONTEXT) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `OPENGL_PROFILE`.
    pub fn get_opengl_profile(&self) -> c_int {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::OPENGL_PROFILE) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `RESIZABLE`.
    pub fn is_resizable(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::RESIZABLE) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `VISIBLE`.
    pub fn is_visible(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::VISIBLE) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `DECORATED`.
    pub fn is_decorated(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::DECORATED) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowPosCallback`.
    pub fn set_pos_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetWindowPosCallback, window_pos_callback);
    }

    pub fn set_all_polling(&self, should_poll: bool) {
        self.set_pos_polling(should_poll);
        self.set_size_polling(should_poll);
        self.set_close_polling(should_poll);
        self.set_refresh_polling(should_poll);
        self.set_focus_polling(should_poll);
        self.set_iconify_polling(should_poll);
        self.set_framebuffer_size_polling(should_poll);
        self.set_key_polling(should_poll);
        self.set_char_polling(should_poll);
        self.set_mouse_button_polling(should_poll);
        self.set_cursor_pos_polling(should_poll);
        self.set_cursor_enter_polling(should_poll);
        self.set_scroll_polling(should_poll);
    }

    /// Wrapper for `glfwSetWindowSizeCallback`.
    pub fn set_size_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetWindowSizeCallback, window_size_callback);
    }

    /// Wrapper for `glfwSetWindowCloseCallback`.
    pub fn set_close_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetWindowCloseCallback, window_close_callback);
    }

    /// Wrapper for `glfwSetWindowRefreshCallback`.
    pub fn set_refresh_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetWindowRefreshCallback, window_refresh_callback);
    }

    /// Wrapper for `glfwSetWindowFocusCallback`.
    pub fn set_focus_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetWindowFocusCallback, window_focus_callback);
    }

    /// Wrapper for `glfwSetWindowIconifyCallback`.
    pub fn set_iconify_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetWindowIconifyCallback, window_iconify_callback);
    }

    /// Wrapper for `glfwSetFramebufferSizeCallback`.
    pub fn set_framebuffer_size_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetFramebufferSizeCallback, framebuffer_size_callback);
    }

    /// Wrapper for `glfwGetInputMode` called with `CURSOR`.
    pub fn get_cursor_mode(&self) -> CursorMode {
        unsafe { cast::transmute(ffi::glfwGetInputMode(self.ptr, ffi::CURSOR)) }
    }

    /// Wrapper for `glfwSetInputMode` called with `CURSOR`.
    pub fn set_cursor_mode(&self, mode: CursorMode) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::CURSOR, mode as c_int); }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_KEYS`.
    pub fn has_sticky_keys(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_KEYS) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_KEYS`.
    pub fn set_sticky_keys(&self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::STICKY_KEYS, value as c_int); }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn has_sticky_mouse_buttons(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn set_sticky_mouse_buttons(&self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS, value as c_int); }
    }

    /// Wrapper for `glfwGetKey`.
    pub fn get_key(&self, key: Key) -> Action {
        unsafe { cast::transmute(ffi::glfwGetKey(self.ptr, key as c_int)) }
    }

    /// Wrapper for `glfwGetMouseButton`.
    pub fn get_mouse_button(&self, button: MouseButton) -> Action {
        unsafe { cast::transmute(ffi::glfwGetMouseButton(self.ptr, button as c_int)) }
    }

    /// Wrapper for `glfwGetCursorPos`.
    pub fn get_cursor_pos(&self) -> (f64, f64) {
        unsafe {
            let mut xpos = 0.0;
            let mut ypos = 0.0;
            ffi::glfwGetCursorPos(self.ptr, &mut xpos, &mut ypos);
            (xpos as f64, ypos as f64)
        }
    }

    /// Wrapper for `glfwSetCursorPos`.
    pub fn set_cursor_pos(&self, xpos: f64, ypos: f64) {
        unsafe { ffi::glfwSetCursorPos(self.ptr, xpos as c_double, ypos as c_double); }
    }

    /// Wrapper for `glfwSetKeyCallback`.
    pub fn set_key_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetKeyCallback, key_callback);
    }

    /// Wrapper for `glfwSetCharCallback`.
    pub fn set_char_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetCharCallback, char_callback);
    }

    /// Wrapper for `glfwSetMouseButtonCallback`.
    pub fn set_mouse_button_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetMouseButtonCallback, mouse_button_callback);
    }

    /// Wrapper for `glfwSetCursorPosCallback`.
    pub fn set_cursor_pos_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetCursorPosCallback, cursor_pos_callback);
    }

    /// Wrapper for `glfwSetCursorEnterCallback`.
    pub fn set_cursor_enter_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetCursorEnterCallback, cursor_enter_callback);
    }

    /// Wrapper for `glfwSetScrollCallback`.
    pub fn set_scroll_polling(&self, should_poll: bool) {
        set_window_callback!(should_poll, glfwSetScrollCallback, scroll_callback);
    }

    /// Wrapper for `glfwGetClipboardString`.
    pub fn set_clipboard_string(&self, string: &str) {
        unsafe {
            string.with_c_str(|string| {
                ffi::glfwSetClipboardString(self.ptr, string);
            });
        }
    }

    /// Wrapper for `glfwGetClipboardString`.
    pub fn get_clipboard_string(&self) -> ~str {
        unsafe { str::raw::from_c_str(ffi::glfwGetClipboardString(self.ptr)) }
    }

    /// Wrapper for `glfwGetWin32Window`
    #[cfg(target_os="win32")]
    pub fn get_win32_window(&self) -> *c_void {
        unsafe { ffi::glfwGetWin32Window(self.ptr) }
    }

    /// Wrapper for `glfwGetWGLContext`
    #[cfg(target_os="win32")]
    pub fn get_wgl_context(&self) -> *c_void {
        unsafe { ffi::glfwGetWGLContext(self.ptr) }
    }

    /// Wrapper for `glfwGetCocoaWindow`
    #[cfg(target_os="macos")]
    pub fn get_cocoa_window(&self) -> *c_void {
        unsafe { ffi::glfwGetCocoaWindow(self.ptr) }
    }

    /// Wrapper for `glfwGetNSGLContext`
    #[cfg(target_os="macos")]
    pub fn get_nsgl_context(&self) -> *c_void {
        unsafe { ffi::glfwGetNSGLContext(self.ptr) }
    }

    /// Wrapper for `glfwGetX11Window`
    #[cfg(target_os="linux")]
    pub fn get_x11_window(&self) -> *c_void {
        unsafe { ffi::glfwGetX11Window(self.ptr) }
    }

    /// Wrapper for `glfwGetGLXContext`
    #[cfg(target_os="linux")]
    pub fn get_glx_context(&self) -> *c_void {
        unsafe { ffi::glfwGetGLXContext(self.ptr) }
    }
}

#[unsafe_destructor]
impl Drop for Window {
    /// Closes the window and performs the necessary cleanups. This will block
    /// until all associated `RenderContext`s were also dropped.
    ///
    /// Wrapper for `glfwDestroyWindow`.
    fn drop(&mut self) {
        drop(self.drop_sender.take());

        if self.drop_receiver.try_recv() != std::comm::Disconnected {
            error!("Attempted to drop a Window before the `RenderContext` was dropped.");
            error!("Blocking until the `RenderContext` was dropped.");
            let _ = self.drop_receiver.recv_opt();
        }

        if !self.is_shared {
            unsafe { ffi::glfwDestroyWindow(self.ptr); }
        }
        if !self.ptr.is_null() {
            unsafe {
                let _: ~Sender<(f64, WindowEvent)> = cast::transmute(ffi::glfwGetWindowUserPointer(self.ptr));
            }
        }
    }
}

/// A rendering context that can be shared between tasks.
pub struct RenderContext {
    ptr: *ffi::GLFWwindow,
    drop_sender: Sender<ContextDropped>
}

/// Methods common to renderable contexts
pub trait Context {
    /// Returns the pointer to the underlying `GLFWwindow`.
    fn window_ptr(&self) -> *ffi::GLFWwindow;

    /// Swaps the front and back buffers of the window. If the swap interval is
    /// greater than zero, the GPU driver waits the specified number of screen
    /// updates before swapping the buffers.
    ///
    /// Wrapper for `glfwSwapBuffers`.
    fn swap_buffers(&self) {
        let ptr = self.window_ptr();
        unsafe { ffi::glfwSwapBuffers(ptr); }
    }

    /// Returns `true` if the window is the current context.
    fn is_current(&self) -> bool {
        self.window_ptr() == unsafe { ffi::glfwGetCurrentContext() }
    }

    /// Wrapper for `glfwMakeContextCurrent`
    fn make_current(&self) {
        let ptr = self.window_ptr();
        unsafe { ffi::glfwMakeContextCurrent(ptr); }
    }
}

impl Context for Window {
    fn window_ptr(&self) -> *ffi::GLFWwindow { self.ptr }
}

impl Context for RenderContext {
    fn window_ptr(&self) -> *ffi::GLFWwindow { self.ptr }
}

/// Wrapper for `glfwMakeContextCurrent`.
pub fn make_context_current(context: Option<&Context>) {
    match context {
        Some(ctx) => unsafe { ffi::glfwMakeContextCurrent(ctx.window_ptr()) },
        None      => unsafe { ffi::glfwMakeContextCurrent(ptr::null()) },
    }
}

/// Joystick identifier tokens.
#[repr(C)]
#[deriving(Clone, Eq, Hash, Show)]
pub enum JoystickId {
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

/// A joystick handle.
pub struct Joystick {
    pub id: JoystickId,
    pub glfw: Glfw,
}

impl Joystick {
    /// Wrapper for `glfwJoystickPresent`.
    pub fn is_present(&self) -> bool {
        unsafe { ffi::glfwJoystickPresent(self.id as c_int) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetJoystickAxes`.
    pub fn get_axes(&self) -> Vec<f32> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetJoystickAxes(self.id as c_int, &mut count);
            slice::from_buf(ptr, count as uint).iter().map(|&a| a as f32).collect()
        }
    }

    /// Wrapper for `glfwGetJoystickButtons`.
    pub fn get_buttons(&self) -> Vec<c_int> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetJoystickButtons(self.id as c_int, &mut count);
            slice::from_buf(ptr, count as uint).iter().map(|&b| b as c_int).collect()
        }
    }

    /// Wrapper for `glfwGetJoystickName`.
    pub fn get_name(&self) -> ~str {
        unsafe { str::raw::from_c_str(ffi::glfwGetJoystickName(self.id as c_int)) }
    }
}
