// Copyright 2013-2014 The GLFW-RS Developers. For a full listing of the authors,
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
#![crate_name = "glfw"]

#![allow(non_upper_case_globals)]

//! An ideomatic wrapper for the GLFW library.
//!
//! # Example
//!
//! ~~~no_run
//! extern crate glfw;
//!
//! use glfw::{Action, Context, Key};
//!
//! fn main() {
//!    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
//!
//!     // Create a windowed mode window and its OpenGL context
//!     let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
//!         .expect("Failed to create GLFW window.");
//!
//!     // Make the window's context current
//!     window.make_current();
//!     window.set_key_polling(true);
//!
//!     // Loop until the user closes the window
//!     while !window.should_close() {
//!         // Swap front and back buffers
//!         window.swap_buffers();
//!
//!         // Poll for and process events
//!         glfw.poll_events();
//!         for (_, event) in glfw::flush_messages(&events) {
//!             println!("{:?}", event);
//!             match event {
//!                 glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
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
extern crate libc;
extern crate vk_sys;
#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_primitive;
extern crate num;
#[cfg(feature = "image")]
extern crate image;

use libc::{c_char, c_double, c_float, c_int, c_uchar};
use libc::{c_ushort, c_void};
use std::ffi::{CStr, CString};
use std::mem;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::fmt;
use std::marker::Send;
use std::ptr;
use std::slice;
use std::path::PathBuf;
use semver::Version;

/// Alias to `MouseButton1`, supplied for improved clarity.
pub use self::MouseButton::Button1 as MouseButtonLeft;
/// Alias to `MouseButton2`, supplied for improved clarity.
pub use self::MouseButton::Button2 as MouseButtonRight;
/// Alias to `MouseButton3`, supplied for improved clarity.
pub use self::MouseButton::Button3 as MouseButtonMiddle;

pub mod ffi;
mod callbacks;

/// Input actions.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Action {
    Release                      = ffi::RELEASE,
    Press                        = ffi::PRESS,
    Repeat                       = ffi::REPEAT,
}

enum_from_primitive! {
/// Input keys.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
    Space                    = ffi::KEY_SPACE,
    Apostrophe               = ffi::KEY_APOSTROPHE,
    Comma                    = ffi::KEY_COMMA,
    Minus                    = ffi::KEY_MINUS,
    Period                   = ffi::KEY_PERIOD,
    Slash                    = ffi::KEY_SLASH,
    Num0                     = ffi::KEY_0,
    Num1                     = ffi::KEY_1,
    Num2                     = ffi::KEY_2,
    Num3                     = ffi::KEY_3,
    Num4                     = ffi::KEY_4,
    Num5                     = ffi::KEY_5,
    Num6                     = ffi::KEY_6,
    Num7                     = ffi::KEY_7,
    Num8                     = ffi::KEY_8,
    Num9                     = ffi::KEY_9,
    Semicolon                = ffi::KEY_SEMICOLON,
    Equal                    = ffi::KEY_EQUAL,
    A                        = ffi::KEY_A,
    B                        = ffi::KEY_B,
    C                        = ffi::KEY_C,
    D                        = ffi::KEY_D,
    E                        = ffi::KEY_E,
    F                        = ffi::KEY_F,
    G                        = ffi::KEY_G,
    H                        = ffi::KEY_H,
    I                        = ffi::KEY_I,
    J                        = ffi::KEY_J,
    K                        = ffi::KEY_K,
    L                        = ffi::KEY_L,
    M                        = ffi::KEY_M,
    N                        = ffi::KEY_N,
    O                        = ffi::KEY_O,
    P                        = ffi::KEY_P,
    Q                        = ffi::KEY_Q,
    R                        = ffi::KEY_R,
    S                        = ffi::KEY_S,
    T                        = ffi::KEY_T,
    U                        = ffi::KEY_U,
    V                        = ffi::KEY_V,
    W                        = ffi::KEY_W,
    X                        = ffi::KEY_X,
    Y                        = ffi::KEY_Y,
    Z                        = ffi::KEY_Z,
    LeftBracket              = ffi::KEY_LEFT_BRACKET,
    Backslash                = ffi::KEY_BACKSLASH,
    RightBracket             = ffi::KEY_RIGHT_BRACKET,
    GraveAccent              = ffi::KEY_GRAVE_ACCENT,
    World1                   = ffi::KEY_WORLD_1,
    World2                   = ffi::KEY_WORLD_2,

    Escape                   = ffi::KEY_ESCAPE,
    Enter                    = ffi::KEY_ENTER,
    Tab                      = ffi::KEY_TAB,
    Backspace                = ffi::KEY_BACKSPACE,
    Insert                   = ffi::KEY_INSERT,
    Delete                   = ffi::KEY_DELETE,
    Right                    = ffi::KEY_RIGHT,
    Left                     = ffi::KEY_LEFT,
    Down                     = ffi::KEY_DOWN,
    Up                       = ffi::KEY_UP,
    PageUp                   = ffi::KEY_PAGE_UP,
    PageDown                 = ffi::KEY_PAGE_DOWN,
    Home                     = ffi::KEY_HOME,
    End                      = ffi::KEY_END,
    CapsLock                 = ffi::KEY_CAPS_LOCK,
    ScrollLock               = ffi::KEY_SCROLL_LOCK,
    NumLock                  = ffi::KEY_NUM_LOCK,
    PrintScreen              = ffi::KEY_PRINT_SCREEN,
    Pause                    = ffi::KEY_PAUSE,
    F1                       = ffi::KEY_F1,
    F2                       = ffi::KEY_F2,
    F3                       = ffi::KEY_F3,
    F4                       = ffi::KEY_F4,
    F5                       = ffi::KEY_F5,
    F6                       = ffi::KEY_F6,
    F7                       = ffi::KEY_F7,
    F8                       = ffi::KEY_F8,
    F9                       = ffi::KEY_F9,
    F10                      = ffi::KEY_F10,
    F11                      = ffi::KEY_F11,
    F12                      = ffi::KEY_F12,
    F13                      = ffi::KEY_F13,
    F14                      = ffi::KEY_F14,
    F15                      = ffi::KEY_F15,
    F16                      = ffi::KEY_F16,
    F17                      = ffi::KEY_F17,
    F18                      = ffi::KEY_F18,
    F19                      = ffi::KEY_F19,
    F20                      = ffi::KEY_F20,
    F21                      = ffi::KEY_F21,
    F22                      = ffi::KEY_F22,
    F23                      = ffi::KEY_F23,
    F24                      = ffi::KEY_F24,
    F25                      = ffi::KEY_F25,
    Kp0                      = ffi::KEY_KP_0,
    Kp1                      = ffi::KEY_KP_1,
    Kp2                      = ffi::KEY_KP_2,
    Kp3                      = ffi::KEY_KP_3,
    Kp4                      = ffi::KEY_KP_4,
    Kp5                      = ffi::KEY_KP_5,
    Kp6                      = ffi::KEY_KP_6,
    Kp7                      = ffi::KEY_KP_7,
    Kp8                      = ffi::KEY_KP_8,
    Kp9                      = ffi::KEY_KP_9,
    KpDecimal                = ffi::KEY_KP_DECIMAL,
    KpDivide                 = ffi::KEY_KP_DIVIDE,
    KpMultiply               = ffi::KEY_KP_MULTIPLY,
    KpSubtract               = ffi::KEY_KP_SUBTRACT,
    KpAdd                    = ffi::KEY_KP_ADD,
    KpEnter                  = ffi::KEY_KP_ENTER,
    KpEqual                  = ffi::KEY_KP_EQUAL,
    LeftShift                = ffi::KEY_LEFT_SHIFT,
    LeftControl              = ffi::KEY_LEFT_CONTROL,
    LeftAlt                  = ffi::KEY_LEFT_ALT,
    LeftSuper                = ffi::KEY_LEFT_SUPER,
    RightShift               = ffi::KEY_RIGHT_SHIFT,
    RightControl             = ffi::KEY_RIGHT_CONTROL,
    RightAlt                 = ffi::KEY_RIGHT_ALT,
    RightSuper               = ffi::KEY_RIGHT_SUPER,
    Menu                     = ffi::KEY_MENU,
}
}

/// Wrapper around 'glfwGetKeyName`
pub fn key_name(key: Option<Key>, scancode: Option<Scancode>) -> String {
    unsafe {
        string_from_c_str(ffi::glfwGetKeyName(match key {
            Some(k) => k as c_int,
            None => ffi::KEY_UNKNOWN
        }, scancode.unwrap_or(ffi::KEY_UNKNOWN)))
    }
}

impl Key {
    /// Wrapper around 'glfwGetKeyName` without scancode
    pub fn name(&self) -> String {
        key_name(Some(*self), None)
    }
}

/// Mouse buttons. The `MouseButtonLeft`, `MouseButtonRight`, and
/// `MouseButtonMiddle` aliases are supplied for convenience.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    /// The left mouse button. A `MouseButtonLeft` alias is provided to improve clarity.
    Button1                = ffi::MOUSE_BUTTON_1,
    /// The right mouse button. A `MouseButtonRight` alias is provided to improve clarity.
    Button2                = ffi::MOUSE_BUTTON_2,
    /// The middle mouse button. A `MouseButtonMiddle` alias is provided to improve clarity.
    Button3                = ffi::MOUSE_BUTTON_3,
    Button4                = ffi::MOUSE_BUTTON_4,
    Button5                = ffi::MOUSE_BUTTON_5,
    Button6                = ffi::MOUSE_BUTTON_6,
    Button7                = ffi::MOUSE_BUTTON_7,
    Button8                = ffi::MOUSE_BUTTON_8,
}

// We can't use `enum_from_primitive!` for MouseButton due to
// https://github.com/andersk/enum_primitive-rs/issues/2
// We implement FromPrimitive manually instead.
impl num::FromPrimitive for MouseButton {
    fn from_i64(n: i64) -> Option<MouseButton> {
        use MouseButton::*;
        match n {
            0 => Some(Button1),
            1 => Some(Button2),
            2 => Some(Button3),
            3 => Some(Button4),
            4 => Some(Button5),
            5 => Some(Button6),
            6 => Some(Button7),
            7 => Some(Button8),
            _ => None
        }
    }
    fn from_u64(n: u64) -> Option<MouseButton> {
        Self::from_i64(n as i64)
    }
}

/// Formats the type using aliases rather than the default variant names.
///
/// # Example
///
/// ~~~ignore
/// assert_eq(format!("{}", glfw::MouseButtonLeft), "MouseButton1");
/// assert_eq(format!("{}", glfw::DebugAliases(glfw::MouseButtonLeft)), "MouseButtonLeft");
/// ~~~
pub struct DebugAliases<T>(pub T);

impl fmt::Debug for DebugAliases<MouseButton> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let DebugAliases(button) = *self;
        match button {
            MouseButtonLeft     => write!(f, "MouseButtonLeft"),
            MouseButtonRight    => write!(f, "MouseButtonRight"),
            MouseButtonMiddle   => write!(f, "MouseButtonMiddle"),
            button              => button.fmt(f),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Callback<Fn, UserData> {
    pub f: Fn,
    pub data: UserData,
}

/// Tokens corresponding to various error types.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
pub type ErrorCallback<UserData> = Callback<fn(Error, String, &UserData), UserData>;

/// The function to be used with the `FAIL_ON_ERRORS` callback.
pub fn fail_on_errors(_: Error, description: String, _: &()) {
    panic!("GLFW Error: {}", description);
}

/// A callback that triggers a task failure when an error is encountered.
pub static FAIL_ON_ERRORS: Option<ErrorCallback<()>> =
    Some(Callback { f: fail_on_errors as fn(Error, String, &()), data: () });

/// The function to be used with the `LOG_ERRORS` callback.
pub fn log_errors(_: Error, description: String, _: &()) {
    error!("GLFW Error: {}", description);
}

/// A callback that logs each error as it is encountered without triggering a
/// task failure.
pub static LOG_ERRORS: Option<ErrorCallback<()>> =
    Some(Callback { f: log_errors as fn(Error, String, &()), data: () });

/// Cursor modes.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CursorMode {
    Normal                = ffi::CURSOR_NORMAL,
    Hidden                = ffi::CURSOR_HIDDEN,
    Disabled              = ffi::CURSOR_DISABLED,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum StandardCursor {
    Arrow                 = ffi::ARROW_CURSOR,
    IBeam                 = ffi::IBEAM_CURSOR,
    Crosshair             = ffi::CROSSHAIR_CURSOR,
    Hand                  = ffi::HAND_CURSOR,
    HResize               = ffi::HRESIZE_CURSOR,
    VResize               = ffi::VRESIZE_CURSOR
}

pub struct Cursor {
    ptr: *mut ffi::GLFWcursor
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { ffi::glfwDestroyCursor(self.ptr) }
    }
}

impl Cursor {
    pub fn standard(cursor: StandardCursor) -> Cursor {
        Cursor {
            ptr: unsafe { ffi::glfwCreateStandardCursor(cursor as c_int) }
        }
    }

    #[cfg(feature = "image")]
    pub fn create(image: image::RgbaImage, x_hotspot: u32, y_hotspot: u32) -> Cursor {
        let (width, height) = image.dimensions();

        let image_data = image.into_vec();

        let ptr = unsafe {
            let glfw_image = ffi::GLFWimage {
                width: width as c_int,
                height: height as c_int,
                pixels: image_data.as_ptr() as *const c_uchar
            };

            ffi::glfwCreateCursor(&glfw_image as *const ffi::GLFWimage, x_hotspot as c_int, y_hotspot as c_int)
        };

        Cursor {
            ptr: ptr
        }
    }

    pub fn create_from_pixels(pixels: Vec<u32>, width: u32, x_hotspot: u32, y_hotspot: u32) -> Cursor {
        let height = pixels.len() as u32 / width;

        let ptr = unsafe {
            let glfw_image = ffi::GLFWimage {
                width: width as c_int,
                height: height as c_int,
                pixels: pixels.as_ptr() as *const c_uchar
            };

            ffi::glfwCreateCursor(&glfw_image as *const ffi::GLFWimage, x_hotspot as c_int, y_hotspot as c_int)
        };

        Cursor {
            ptr: ptr
        }
    }
}

/// Describes a single video mode.
#[derive(Copy, Clone)]
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
    pub red:    Vec<c_ushort>,
    pub green:  Vec<c_ushort>,
    pub blue:   Vec<c_ushort>,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ContextReleaseBehavior {
    Any                   = ffi::ANY_RELEASE_BEHAVIOR,
    Flush                 = ffi::RELEASE_BEHAVIOR_FLUSH,
    None                  = ffi::RELEASE_BEHAVIOR_NONE
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ContextCreationApi {
    Native                = ffi::NATIVE_CONTEXT_API,
    Egl                   = ffi::EGL_CONTEXT_API
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SwapInterval {
    None,
    Adaptive,
    Sync(u32)
}

/// An OpenGL process address.
pub type GLProc = ffi::GLFWglproc;

/// A token from which to call various GLFW functions. It can be obtained by
/// calling the `init` function. This cannot be sent to other tasks, and should
/// only be initialized on the main platform thread. Whilst this might make
/// performing some operations harder, this is to ensure thread safety is enforced
/// statically. The context can be safely cloned or implicitly copied if need be
/// for convenience.
#[derive(Copy, Clone)]
pub struct Glfw;

/// An error that might be returned when `glfw::init` is called.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum InitError {
    /// The library was already initialized.
    AlreadyInitialized,
    /// An internal error occured when trying to initialize the library.
    Internal,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "InitError: {:?}", *self)
    }
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
/// ~~~no_run
/// extern crate glfw;
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
    // Helper to convert unsafe extern "C" fn to (safe) extern "C" fn.
    extern "C" fn glfw_terminate() {
        unsafe {
            ffi::glfwTerminate();
        }
    }
    use std::sync::{Once, ONCE_INIT};
    static mut INIT: Once = ONCE_INIT;
    let mut result = Err(InitError::AlreadyInitialized);
    unsafe {
        INIT.call_once(|| {
            // Initialize the error callback if it was supplied. This is done
            // before `ffi::glfwInit` because errors could occur during
            // initialization.
            match callback.take() {
                Some(f) => callbacks::error::set(f),
                None    => callbacks::error::unset(),
            }
            if ffi::glfwInit() == ffi::TRUE {
                result = Ok(());
                // TODO: When (if?) std::rt::at_exit() stabilizes, prefer to use it.
                libc::atexit(glfw_terminate);
            } else {
                result = Err(InitError::Internal);
            }
        })
    }
    result.map(|_| Glfw)
}

impl Glfw {
    /// Sets the error callback, overwriting the previous one stored.
    ///
    /// # Example
    ///
    /// ~~~ignore
    /// use std::cell::Cell;
    ///
    /// fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
    ///     println!("GLFW error {}: {}", error_count.get(), description);
    ///     error_count.set(error_count.get() + 1);
    /// }
    ///
    /// // sets a new callback
    /// glfw.set_error_callback(Some(
    ///     glfw::Callback {
    ///         f: error_callback,
    ///         data: Cell::new(0),
    ///     }
    /// ));
    ///
    /// // removes the previously set callback
    /// glfw.set_error_callback(None);
    /// ~~~
    ///
    /// The `FAIL_ON_ERRORS` and `LOG_ERRORS` callbacks are provided for
    /// convenience. For example:
    ///
    /// ~~~ignore
    /// // triggers a task failure when a GLFW error is encountered.
    /// glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    /// ~~~
    pub fn set_error_callback<UserData: 'static>(&mut self, callback: Option<ErrorCallback<UserData>>) {
        match callback {
            Some(f) => callbacks::error::set(f),
            None    => callbacks::error::unset(),
        }
    }

    /// Sets the monitor callback, overwriting the previous one stored.
    pub fn set_monitor_callback<UserData: 'static>(&mut self, callback: Option<MonitorCallback<UserData>>) {
        match callback {
            Some(f) => callbacks::monitor::set(f),
            None    => callbacks::monitor::unset(),
        }
    }

    /// Supplies the primary monitor to the closure provided, if it exists.
    /// This is usually the monitor where elements like the Windows task bar or
    /// the OS X menu bar is located.
    ///
    /// # Example
    ///
    /// ~~~ignore
    /// let (window, events) = glfw.with_primary_monitor(|m| {
    ///     glfw.create_window(300, 300, "Hello this is window",
    ///         m.map_or(glfw::WindowMode::Windowed, |m| glfw::FullScreen(m)))
    /// }).expect("Failed to create GLFW window.");
    /// ~~~
    pub fn with_primary_monitor<T, F>(&mut self, f: F) -> T where F: Fn(&mut Self, Option<&Monitor>) -> T {
        match unsafe { ffi::glfwGetPrimaryMonitor() } {
            ptr if ptr.is_null() => f(self, None),
            ptr => f(self, Some(&Monitor {
                ptr: ptr
            })),
        }
    }

    /// Supplies a vector of the currently connected monitors to the closure
    /// provided.
    ///
    /// # Example
    ///
    /// ~~~ignore
    /// glfw.with_connected_monitors(|monitors| {
    ///     for monitor in monitors.iter() {
    ///         println!("{}: {}", monitor.get_name(), monitor.get_video_mode());
    ///     }
    /// });
    /// ~~~
    pub fn with_connected_monitors<T, F>(&mut self, f: F) -> T where F: Fn(&mut Self, &[Monitor]) -> T {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetMonitors(&mut count);
            f(self,
              &slice::from_raw_parts(ptr as *const _, count as usize).iter().map(|&ptr| {
                Monitor {
                    ptr: ptr
                }
            }).collect::<Vec<Monitor>>())
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
    /// - `glfw::WindowHint::ContextVersion(3, 2)`
    ///
    /// 10.9 supports the following OpenGL versions
    ///
    /// - `glfw::WindowHint::ContextVersion(3, 2)`
    /// - `glfw::WindowHint::ContextVersion(3, 3)`
    /// - `glfw::WindowHint::ContextVersion(4, 1)`
    ///
    /// To create an OS X compatible context, the hints should be specified as
    /// follows:
    ///
    /// ~~~ignore
    /// glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    /// glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    /// glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    /// ~~~
    pub fn window_hint(&mut self, hint: WindowHint) {
        //This is just a simple function to unwrap the option and convert it to `c_int` or use `GLFW_DONT_CARE`,
        //then call `glfwWindowHint` with the result. It was required because `GLFW_DONT_CARE` is signed,
        //so `value.unwrap_or(ffi::DONT_CARE)` wouldn't work because of the type difference.
        #[inline(always)]
        unsafe fn dont_care_hint(hint: c_int, value: Option<u32>) {
            ffi::glfwWindowHint(hint, match value {
                Some(v) => v as c_int,
                None => ffi::DONT_CARE
            })
        }

        match hint {
            WindowHint::RedBits(bits)                    => unsafe { dont_care_hint(ffi::RED_BITS,                      bits) },
            WindowHint::GreenBits(bits)                  => unsafe { dont_care_hint(ffi::GREEN_BITS,                    bits) },
            WindowHint::BlueBits(bits)                   => unsafe { dont_care_hint(ffi::BLUE_BITS,                     bits) },
            WindowHint::AlphaBits(bits)                  => unsafe { dont_care_hint(ffi::ALPHA_BITS,                    bits) },
            WindowHint::DepthBits(bits)                  => unsafe { dont_care_hint(ffi::DEPTH_BITS,                    bits) },
            WindowHint::StencilBits(bits)                => unsafe { dont_care_hint(ffi::STENCIL_BITS,                  bits) },
            WindowHint::AccumRedBits(bits)               => unsafe { dont_care_hint(ffi::ACCUM_RED_BITS,                bits) },
            WindowHint::AccumGreenBits(bits)             => unsafe { dont_care_hint(ffi::ACCUM_GREEN_BITS,              bits) },
            WindowHint::AccumBlueBits(bits)              => unsafe { dont_care_hint(ffi::ACCUM_BLUE_BITS,               bits) },
            WindowHint::AccumAlphaBits(bits)             => unsafe { dont_care_hint(ffi::ACCUM_ALPHA_BITS,              bits) },
            WindowHint::AuxBuffers(num_buffers)          => unsafe { dont_care_hint(ffi::AUX_BUFFERS,                   num_buffers) },
            WindowHint::Samples(num_samples)             => unsafe { dont_care_hint(ffi::SAMPLES,                       num_samples) },
            WindowHint::RefreshRate(rate)                => unsafe { dont_care_hint(ffi::REFRESH_RATE,                  rate) },
            WindowHint::Stereo(is_stereo)                => unsafe { ffi::glfwWindowHint(ffi::STEREO,                   is_stereo as c_int) },
            WindowHint::SRgbCapable(is_capable)          => unsafe { ffi::glfwWindowHint(ffi::SRGB_CAPABLE,             is_capable as c_int) },
            WindowHint::ClientApi(api)                   => unsafe { ffi::glfwWindowHint(ffi::CLIENT_API,               api as c_int) },
            WindowHint::ContextVersionMajor(major)       => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR,    major as c_int) },
            WindowHint::ContextVersionMinor(minor)       => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR,    minor as c_int) },
            WindowHint::ContextVersion(major, minor)     => unsafe {
                ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, major as c_int);
                ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, minor as c_int)
            },
            WindowHint::ContextRobustness(robustness)    => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_ROBUSTNESS,       robustness as c_int) },
            WindowHint::OpenGlForwardCompat(is_compat)   => unsafe { ffi::glfwWindowHint(ffi::OPENGL_FORWARD_COMPAT,    is_compat as c_int) },
            WindowHint::OpenGlDebugContext(is_debug)     => unsafe { ffi::glfwWindowHint(ffi::OPENGL_DEBUG_CONTEXT,     is_debug as c_int) },
            WindowHint::OpenGlProfile(profile)           => unsafe { ffi::glfwWindowHint(ffi::OPENGL_PROFILE,           profile as c_int) },
            WindowHint::Resizable(is_resizable)          => unsafe { ffi::glfwWindowHint(ffi::RESIZABLE,                is_resizable as c_int) },
            WindowHint::Visible(is_visible)              => unsafe { ffi::glfwWindowHint(ffi::VISIBLE,                  is_visible as c_int) },
            WindowHint::Decorated(is_decorated)          => unsafe { ffi::glfwWindowHint(ffi::DECORATED,                is_decorated as c_int) },
            WindowHint::AutoIconify(auto_iconify)        => unsafe { ffi::glfwWindowHint(ffi::AUTO_ICONIFY,             auto_iconify as c_int) },
            WindowHint::Floating(is_floating)            => unsafe { ffi::glfwWindowHint(ffi::FLOATING,                 is_floating as c_int) },
            WindowHint::ContextNoError(is_no_error)      => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_NO_ERROR,         is_no_error as c_int) },
            WindowHint::ContextCreationApi(api)          => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_CREATION_API,     api as c_int) },
            WindowHint::ContextReleaseBehavior(behavior) => unsafe { ffi::glfwWindowHint(ffi::CONTEXT_RELEASE_BEHAVIOR, behavior as c_int) },
            WindowHint::DoubleBuffer(is_dbuffered)       => unsafe { ffi::glfwWindowHint(ffi::DOUBLEBUFFER,             is_dbuffered as c_int) },
        }
    }

    /// Resets the window hints previously set by the `window_hint` function to
    /// their default values.
    ///
    /// Wrapper for `glfwDefaultWindowHints`.
    pub fn default_window_hints(&mut self) {
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
            with_c_str(title, |title| {
                ffi::glfwCreateWindow(
                    width as c_int,
                    height as c_int,
                    title,
                    mode.to_ptr(),
                    match share { Some(w) => w.ptr, None => ptr::null_mut() }
                )
            })
        };
        if ptr.is_null() {
            None
        } else {
            let (drop_sender, drop_receiver) = channel();
            let (sender, receiver) = channel();
            unsafe { ffi::glfwSetWindowUserPointer(ptr, mem::transmute(Box::new(sender))); }
            Some((
                Window {
                    ptr: ptr,
                    glfw: self.clone(),
                    is_shared: share.is_some(),
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
    pub fn make_context_current(&mut self, context: Option<&Window>) {
        match context {
            Some(window) => unsafe { ffi::glfwMakeContextCurrent(window.ptr) },
            None         => unsafe { ffi::glfwMakeContextCurrent(ptr::null_mut()) },
        }
    }

    /// Wrapper for `glfwGetX11Display`
    #[cfg(target_os="linux")]
    pub fn get_x11_display(&self) -> *mut c_void {
        unsafe { ffi::glfwGetX11Display() }
    }

    /// Immediately process the received events.
    ///
    /// Wrapper for `glfwPollEvents`.
    pub fn poll_events(&mut self) {
        unsafe { ffi::glfwPollEvents(); }
    }

    /// Sleep until at least one event has been recieved, and then perform the
    /// equivalent of `Glfw::poll_events`.
    ///
    /// Wrapper for `glfwWaitEvents`.
    pub fn wait_events(&mut self) {
        unsafe { ffi::glfwWaitEvents(); }
    }

    /// Sleep until at least one event has been recieved, or until the specified
    /// timeout is reached, and then perform the equivalent of `Glfw::poll_events`.
    /// Timeout is specified in seconds.
    ///
    /// Wrapper for `glfwWaitEventsTimeout`.
    pub fn wait_events_timeout(&mut self, timeout: f64) {
        unsafe { ffi::glfwWaitEventsTimeout(timeout); }
    }

    /// Posts an empty event from the current thread to the event queue, causing
    /// `wait_events` or `wait_events_timeout` to return.
    /// If no windows exist, this function returns immediately.
    ///
    /// Wrapper for `glfwPostEmptyEvent`.
    pub fn post_empty_event(&mut self) {
        unsafe { ffi::glfwPostEmptyEvent(); }
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
    pub fn set_time(&mut self, time: f64) {
        unsafe { ffi::glfwSetTime(time as c_double); }
    }

    /// Wrapper for `glfwGetTimerValue`.
    pub fn get_timer_value() -> u64 {
        unsafe { ffi::glfwGetTimerValue() as u64 }
    }

    /// Wrapper for `glfwGetTimerFrequency`
    pub fn get_timer_frquency() -> u64 {
        unsafe { ffi::glfwGetTimerFrequency() as u64 }
    }

    /// Sets the number of screen updates to wait before swapping the buffers of
    /// the current context and returning from `Window::swap_buffers`.
    ///
    /// Wrapper for `glfwSwapInterval`.
    pub fn set_swap_interval(&mut self, interval: SwapInterval) {
        unsafe {
            ffi::glfwSwapInterval(match interval {
                SwapInterval::None           =>  0       as c_int,
                SwapInterval::Adaptive       => -1       as c_int,
                SwapInterval::Sync(interval) => interval as c_int
            })
        }
    }

    /// Returns `true` if the specified OpenGL or context creation API extension
    /// is supported by the current context.
    ///
    /// Wrapper for `glfwExtensionSupported`.
    pub fn extension_supported(&self, extension: &str) -> bool {
        unsafe {
            with_c_str(extension, |extension| {
                ffi::glfwExtensionSupported(extension) == ffi::TRUE
            })
        }
    }

    /// Returns the address of the specified client API or extension function if
    /// it is supported by the current context, NULL otherwise.
    ///
    /// Wrapper for `glfwGetProcAddress`.
    pub fn get_proc_address_raw(&self, procname: &str) -> GLProc {
        debug_assert!(unsafe { ffi::glfwGetCurrentContext() } != std::ptr::null_mut());
        with_c_str(procname, |procname| {
            unsafe { ffi::glfwGetProcAddress(procname) }
        })
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
            major: major as u64,
            minor: minor as u64,
            patch: patch as u64,
            pre:   Vec::new(),
            build: Vec::new(),
        }
    }
}

/// Replacement for `String::from_raw_buf`
pub unsafe fn string_from_c_str(c_str: *const c_char) -> String {
    String::from_utf8_lossy(CStr::from_ptr(c_str).to_bytes()).into_owned()
}

/// Replacement for `ToCStr::with_c_str`
pub fn with_c_str<F, T>(s: &str, f: F) -> T where F: FnOnce(*const c_char) -> T {
    let c_str = CString::new(s.as_bytes());
    f(c_str.unwrap().as_bytes_with_nul().as_ptr() as *const _)
}

/// Wrapper for `glfwGetVersionString`.
pub fn get_version_string() -> String {
    unsafe { string_from_c_str(ffi::glfwGetVersionString()) }
}

/// An monitor callback. This can be supplied with some user data to be passed
/// to the callback function when it is triggered.
pub type MonitorCallback<UserData> = Callback<fn(Monitor, MonitorEvent, &UserData), UserData>;

/// A struct that wraps a `*GLFWmonitor` handle.
#[allow(missing_copy_implementations)]
pub struct Monitor {
    ptr: *mut ffi::GLFWmonitor
}

impl std::fmt::Debug for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Monitor({:p})", self.ptr)
    }
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
    pub fn get_name(&self) -> String {
        unsafe { string_from_c_str(ffi::glfwGetMonitorName(self.ptr)) }
    }

    /// Wrapper for `glfwGetVideoModes`.
    pub fn get_video_modes(&self) -> Vec<VidMode> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetVideoModes(self.ptr, &mut count);
            slice::from_raw_parts(ptr, count as usize).iter().map(VidMode::from_glfw_vid_mode).collect()
        }
    }

    /// Wrapper for `glfwGetVideoMode`.
    pub fn get_video_mode(&self) -> Option<VidMode> {
        unsafe {
            // TODO: Can be returned to as_ref + map as in previous commit when (if?) as_ref stabilizes.
            let ptr = ffi::glfwGetVideoMode(self.ptr);
            if ptr.is_null() {
                None
            } else {
                Some(VidMode::from_glfw_vid_mode(&*ptr))
            }
        }
    }

    /// Wrapper for `glfwSetGamma`.
    pub fn set_gamma(&mut self, gamma: f32) {
        unsafe { ffi::glfwSetGamma(self.ptr, gamma as c_float); }
    }

    /// Wrapper for `glfwGetGammaRamp`.
    pub fn get_gamma_ramp(&self) -> GammaRamp {
        unsafe {
            let llramp = *ffi::glfwGetGammaRamp(self.ptr);
            GammaRamp {
                red:    slice::from_raw_parts(llramp.red as *const c_ushort, llramp.size as usize)
                              .iter().map(|&x| x).collect(),
                green:  slice::from_raw_parts(llramp.green as *const c_ushort, llramp.size as usize)
                              .iter().map(|&x| x).collect(),
                blue:   slice::from_raw_parts(llramp.blue as *const c_ushort, llramp.size as usize)
                              .iter().map(|&x| x).collect(),
            }
        }
    }

    /// Wrapper for `glfwSetGammaRamp`.
    pub fn set_gamma_ramp(&mut self, ramp: &mut GammaRamp) {
        unsafe {
            ffi::glfwSetGammaRamp(
                self.ptr,
                &ffi::GLFWgammaramp {
                    red:    ramp.red.as_mut_ptr(),
                    green:  ramp.green.as_mut_ptr(),
                    blue:   ramp.blue.as_mut_ptr(),
                    size:   ramp.red.len() as u32,
                }
            );
        }
    }
}

/// Monitor events.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

impl fmt::Debug for VidMode {
    /// Returns a string representation of the video mode.
    ///
    /// # Returns
    ///
    /// A string in the form:
    ///
    /// ~~~ignore
    /// ~"[width] x [height], [total_bits] ([red_bits] [green_bits] [blue_bits]) [refresh_rate] Hz"
    /// ~~~
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} x {}, {} = {} + {} + {}, {} Hz",
            self.width, self.height,
            self.red_bits + self.green_bits + self.blue_bits,
            self.red_bits, self.green_bits, self.blue_bits,
            self.refresh_rate)
    }
}

/// Window hints that can be set using the `window_hint` function.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WindowHint {
    /// Specifies the desired bit depth of the red component of the default framebuffer.
    RedBits(Option<u32>),
    /// Specifies the desired bit depth of the green component of the default framebuffer.
    GreenBits(Option<u32>),
    /// Specifies the desired bit depth of the blue component of the default framebuffer.
    BlueBits(Option<u32>),
    /// Specifies the desired bit depth of the alpha component of the default framebuffer.
    AlphaBits(Option<u32>),
    /// Specifies the desired bit depth of the depth component of the default framebuffer.
    DepthBits(Option<u32>),
    /// Specifies the desired bit depth of the stencil component of the default framebuffer.
    StencilBits(Option<u32>),
    /// Specifies the desired bit depth of the red component of the accumulation framebuffer.
    AccumRedBits(Option<u32>),
    /// Specifies the desired bit depth of the green component of the accumulation framebuffer.
    AccumGreenBits(Option<u32>),
    /// Specifies the desired bit depth of the blue component of the accumulation framebuffer.
    AccumBlueBits(Option<u32>),
    /// Specifies the desired bit depth of the alpha component of the accumulation framebuffer.
    AccumAlphaBits(Option<u32>),
    /// Specifies the desired number of auxiliary buffers.
    AuxBuffers(Option<u32>),
    /// Specifies whether to use stereoscopic rendering.
    Stereo(bool),
    /// Specifies the desired number of samples to use for multisampling. Zero
    /// disables multisampling.
    Samples(Option<u32>),
    /// Specifies whether the framebuffer should be sRGB capable.
    SRgbCapable(bool),
    /// Specifies the desired refresh rate for full screen windows. If set to `None`,
    /// the highest available refresh rate will be used.
    ///
    /// This hint is ignored for windowed mode windows.
    RefreshRate(Option<u32>),
    /// Specifies which `ClientApi` to create the context for.
    ClientApi(ClientApiHint),
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
    ContextRobustness(ContextRobustnessHint),
    /// Specifies whether the OpenGL context should be forward-compatible, i.e.
    /// one where all functionality deprecated in the requested version of
    /// OpenGL is removed. This may only be used if the requested OpenGL version
    /// is 3.0 or above.
    ///
    /// If another client API is requested, this hint is ignored.
    OpenGlForwardCompat(bool),
    /// Specifies whether to create a debug OpenGL context, which may have
    /// additional error and performance issue reporting functionality.
    ///
    /// If another client API is requested, this hint is ignored.
    OpenGlDebugContext(bool),
    /// Specifies which OpenGL profile to create the context for. If requesting
    /// an OpenGL version below 3.2, `OpenGlAnyProfile` must be used.
    ///
    /// If another client API is requested, this hint is ignored.
    OpenGlProfile(OpenGlProfileHint),
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
    /// Specifies whether the (full screen) window will automatically iconify
    /// and restore the previous video mode on input focus loss.
    ///
    /// This hint is ignored for windowed mode windows.
    AutoIconify(bool),
    /// Specifies whether the window will be floating above other regular
    /// windows, also called topmost or always-on-top.
    ///
    /// This hint is ignored for full screen windows.
    Floating(bool),
    /// Specifies whether the OpenGL or OpenGL ES contexts do not emit errors,
    /// allowing for better performance in some situations.
    ContextNoError(bool),
    /// Specifies which context creation API to use to create the context.
    ContextCreationApi(ContextCreationApi),
    /// Specifies the behavior of the OpenGL pipeline when a context is transferred between threads
    ContextReleaseBehavior(ContextReleaseBehavior),
    /// Specifies whether the framebuffer should be double buffered.
    ///
    /// You nearly always want to use double buffering.
    ///
    /// Note that setting this to false will make `swap_buffers` do nothing useful,
    /// and your scene will have to be displayed some other way.
    DoubleBuffer(bool)
}

/// Client API tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientApiHint {
    NoApi                    = ffi::NO_API,
    OpenGl                   = ffi::OPENGL_API,
    OpenGlEs                 = ffi::OPENGL_ES_API,
}

/// Context robustness tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ContextRobustnessHint {
    NoRobustness                = ffi::NO_ROBUSTNESS,
    NoResetNotification         = ffi::NO_RESET_NOTIFICATION,
    LoseContextOnReset          = ffi::LOSE_CONTEXT_ON_RESET,
}

/// OpenGL profile tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum OpenGlProfileHint {
    Any            = ffi::OPENGL_ANY_PROFILE,
    Core           = ffi::OPENGL_CORE_PROFILE,
    Compat         = ffi::OPENGL_COMPAT_PROFILE,
}

/// Describes the mode of a window
#[derive(Copy, Clone, Debug)]
pub enum WindowMode<'a> {
    /// Full screen mode. Contains the monitor on which the window is displayed.
    FullScreen(&'a Monitor),

    /// Windowed mode.
    Windowed,
}

/// Private conversion methods for `glfw::WindowMode`
impl<'a> WindowMode<'a> {
    /// Returns a pointer to a monitor if the window is fullscreen, otherwise
    /// it returns a null pointer (if it is in windowed mode).
    fn to_ptr(&self) -> *mut ffi::GLFWmonitor {
        match *self {
            WindowMode::FullScreen(ref monitor) => monitor.ptr,
            WindowMode::Windowed                => ptr::null_mut(),
        }
    }
}

bitflags! {
    #[doc = "Key modifiers"]
    pub flags Modifiers: ::libc::c_int {
        const Shift       = ::ffi::MOD_SHIFT,
        const Control     = ::ffi::MOD_CONTROL,
        const Alt         = ::ffi::MOD_ALT,
        const Super       = ::ffi::MOD_SUPER
    }
}

pub type Scancode = c_int;

/// Window event messages.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum WindowEvent {
    Pos(i32, i32),
    Size(i32, i32),
    Close,
    Refresh,
    Focus(bool),
    Iconify(bool),
    FramebufferSize(i32, i32),
    MouseButton(MouseButton, Action, Modifiers),
    CursorPos(f64, f64),
    CursorEnter(bool),
    Scroll(f64, f64),
    Key(Key, Scancode, Action, Modifiers),
    Char(char),
    FileDrop(Vec<PathBuf>),
}

/// Returns an iterator that yields until no more messages are contained in the
/// `Receiver`'s queue. This is useful for event handling where the blocking
/// behaviour of `Receiver::iter` is undesirable.
///
/// # Example
///
/// ~~~ignore
/// for event in glfw::flush_messages(&events) {
///     // handle event
/// }
/// ~~~
pub fn flush_messages<'a, Message: Send>(receiver: &'a Receiver<Message>) -> FlushedMessages<'a, Message> {
    FlushedMessages(receiver)
}

/// An iterator that yields until no more messages are contained in the
/// `Receiver`'s queue.
pub struct FlushedMessages<'a, Message: 'a + Send>(&'a Receiver<Message>);

unsafe impl<'a, Message: 'a + Send> Send for FlushedMessages<'a, Message> {
}

impl<'a, Message: 'static + Send> Iterator for FlushedMessages<'a, Message> {
    type Item = Message;

    fn next(&mut self) -> Option<Message> {
        let FlushedMessages(receiver) = *self;
        match receiver.try_recv() {
            Ok(message) => Some(message),
            _ => None,
        }
    }
}

/// Checks is the Vulkan API is supported by calling `glfwVulkanSupported`
///
/// Note that GLFW must be compiled with `GLFW_INCLUDE_VULKAN`
pub fn vulkan_supported() -> bool {
    unsafe { ffi::glfwVulkanSupported() == ffi::TRUE }
}

/// A struct that wraps a `*GLFWwindow` handle.
pub struct Window {
    ptr: *mut ffi::GLFWwindow,
    pub glfw: Glfw,
    pub is_shared: bool,
    /// A `Sender` that can be cloned out to child `RenderContext`s.
    drop_sender: Option<Sender<()>>,
    /// Once all  child`RenderContext`s have been dropped, calling `try_recv()`
    /// on the `drop_receiver` will result in an `Err(std::comm::Disconnected)`,
    /// indicating that it is safe to drop the `Window`.
    drop_receiver: Receiver<()>,
}

macro_rules! set_window_callback {
    ($window:ident, $should_poll:expr, $ll_fn:ident, $callback:ident) => ({
        if $should_poll {
            unsafe { ffi::$ll_fn($window.ptr, Some(callbacks::$callback)); }
        } else {
            unsafe { ffi::$ll_fn($window.ptr, None); }
        }
    })
}

impl Window {
    /// Returns the address of the specified client API or extension function if
    /// it is supported by the context associated with this Window. If this Window is not the
    /// current context, it will make it the current context.
    ///
    /// Wrapper for `glfwGetProcAddress`.
    pub fn get_proc_address(&mut self, procname: &str) -> GLProc {
        if self.ptr != unsafe { ffi::glfwGetCurrentContext() } {
            self.make_current();
        }

        self.glfw.get_proc_address_raw(procname)
    }

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
    pub fn set_should_close(&mut self, value: bool) {
        unsafe { ffi::glfwSetWindowShouldClose(self.ptr, value as c_int) }
    }

    /// Sets the title of the window.
    ///
    /// Wrapper for `glfwSetWindowTitle`.
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            with_c_str(title, |title| {
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
    pub fn set_pos(&mut self, xpos: i32, ypos: i32) {
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
    pub fn set_size(&mut self, width: i32, height: i32) {
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

    /// Wrapper for `glfwSetWindowAspectRatio`.
    pub fn set_aspect_ratio(&mut self, numer: u32, denum: u32) {
        unsafe { ffi::glfwSetWindowAspectRatio(self.ptr, numer as c_int, denum as c_int) }
    }

    /// Wrapper for `glfwSetWindowSizeLimits`.
    pub fn set_size_limits(&mut self, minwidth: u32, minheight: u32, maxwidth: u32, maxheight: u32) {
        unsafe { ffi::glfwSetWindowSizeLimits(self.ptr , minwidth as c_int, minheight as c_int, maxwidth as c_int, maxheight as c_int) }
    }

    /// Wrapper for `glfwIconifyWindow`.
    pub fn iconify(&mut self) {
        unsafe { ffi::glfwIconifyWindow(self.ptr); }
    }

    /// Wrapper for `glfwRestoreWindow`.
    pub fn restore(&mut self) {
        unsafe { ffi::glfwRestoreWindow(self.ptr); }
    }

    /// Wrapper for `glfwShowWindow`.
    pub fn show(&mut self) {
        unsafe { ffi::glfwShowWindow(self.ptr); }
    }

    /// Wrapper for `glfwHideWindow`.
    pub fn hide(&mut self) {
        unsafe { ffi::glfwHideWindow(self.ptr); }
    }

    /// Returns whether the window is fullscreen or windowed.
    ///
    /// # Example
    ///
    /// ~~~ignore
    /// window.with_window_mode(|mode| {
    ///     match mode {
    ///         glfw::Windowed => println!("Windowed"),
    ///         glfw::FullScreen(m) => println!("FullScreen({})", m.get_name()),
    ///     }
    /// });
    /// ~~~
    pub fn with_window_mode<T, F>(&self, f: F) -> T where F: Fn(WindowMode) -> T {
        let ptr = unsafe { ffi::glfwGetWindowMonitor(self.ptr) };
        if ptr.is_null() {
            f(WindowMode::Windowed)
        } else {
            f(WindowMode::FullScreen(&Monitor {
                ptr: ptr
            }))
        }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `FOCUSED`.
    pub fn is_focused(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::FOCUSED) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `ICONIFIED`.
    pub fn is_iconified(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::ICONIFIED) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowattrib` called with `MAXIMIZED`.
    pub fn is_maximized(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::MAXIMIZED) == ffi::TRUE }
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
                major: ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_VERSION_MAJOR) as u64,
                minor: ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_VERSION_MINOR) as u64,
                patch: ffi::glfwGetWindowAttrib(self.ptr, ffi::CONTEXT_REVISION) as u64,
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
    pub fn set_pos_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetWindowPosCallback, window_pos_callback);
    }

    pub fn set_all_polling(&mut self, should_poll: bool) {
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
        self.set_drag_and_drop_polling(should_poll);
    }

    /// Wrapper for `glfwSetWindowSizeCallback`.
    pub fn set_size_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetWindowSizeCallback, window_size_callback);
    }

    /// Wrapper for `glfwSetWindowCloseCallback`.
    pub fn set_close_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetWindowCloseCallback, window_close_callback);
    }

    /// Wrapper for `glfwSetWindowRefreshCallback`.
    pub fn set_refresh_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetWindowRefreshCallback, window_refresh_callback);
    }

    /// Wrapper for `glfwSetWindowFocusCallback`.
    pub fn set_focus_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetWindowFocusCallback, window_focus_callback);
    }

    /// Wrapper for `glfwSetWindowIconifyCallback`.
    pub fn set_iconify_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetWindowIconifyCallback, window_iconify_callback);
    }

    /// Wrapper for `glfwSetFramebufferSizeCallback`.
    pub fn set_framebuffer_size_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetFramebufferSizeCallback, framebuffer_size_callback);
    }

    /// Wrapper for `glfwSetFramebufferSizeCallback`.
    pub fn set_drag_and_drop_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetDropCallback, drop_callback);
    }

    /// Wrapper for `glfwGetInputMode` called with `CURSOR`.
    pub fn get_cursor_mode(&self) -> CursorMode {
        unsafe { mem::transmute(ffi::glfwGetInputMode(self.ptr, ffi::CURSOR)) }
    }

    /// Wrapper for `glfwSetInputMode` called with `CURSOR`.
    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::CURSOR, mode as c_int); }
    }

    pub fn set_cursor(&mut self, cursor: Option<&Cursor>) {
        unsafe {
            ffi::glfwSetCursor(self.ptr, match cursor {
                Some(ref cursor) => cursor.ptr,
                None => ptr::null_mut()
            })
        }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_KEYS`.
    pub fn has_sticky_keys(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_KEYS) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_KEYS`.
    pub fn set_sticky_keys(&mut self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::STICKY_KEYS, value as c_int); }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn has_sticky_mouse_buttons(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn set_sticky_mouse_buttons(&mut self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS, value as c_int); }
    }

    /// Wrapper for `glfwGetKey`.
    pub fn get_key(&self, key: Key) -> Action {
        unsafe { mem::transmute(ffi::glfwGetKey(self.ptr, key as c_int)) }
    }

    /// Wrapper for `glfwGetMouseButton`.
    pub fn get_mouse_button(&self, button: MouseButton) -> Action {
        unsafe { mem::transmute(ffi::glfwGetMouseButton(self.ptr, button as c_int)) }
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
    pub fn set_cursor_pos(&mut self, xpos: f64, ypos: f64) {
        unsafe { ffi::glfwSetCursorPos(self.ptr, xpos as c_double, ypos as c_double); }
    }

    /// Wrapper for `glfwSetKeyCallback`.
    pub fn set_key_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetKeyCallback, key_callback);
    }

    /// Wrapper for `glfwSetCharCallback`.
    pub fn set_char_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetCharCallback, char_callback);
    }

    /// Wrapper for `glfwSetMouseButtonCallback`.
    pub fn set_mouse_button_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetMouseButtonCallback, mouse_button_callback);
    }

    /// Wrapper for `glfwSetCursorPosCallback`.
    pub fn set_cursor_pos_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetCursorPosCallback, cursor_pos_callback);
    }

    /// Wrapper for `glfwSetCursorEnterCallback`.
    pub fn set_cursor_enter_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetCursorEnterCallback, cursor_enter_callback);
    }

    /// Wrapper for `glfwSetScrollCallback`.
    pub fn set_scroll_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetScrollCallback, scroll_callback);
    }

    /// Wrapper for `glfwGetClipboardString`.
    pub fn set_clipboard_string(&mut self, string: &str) {
        unsafe {
            with_c_str(string, |string| {
                ffi::glfwSetClipboardString(self.ptr, string);
            });
        }
    }

    /// Wrapper for `glfwGetClipboardString`.
    pub fn get_clipboard_string(&self) -> String {
        unsafe { string_from_c_str(ffi::glfwGetClipboardString(self.ptr)) }
    }

    /// Wrapper for `glfwGetWin32Window`
    #[cfg(target_os="windows")]
    pub fn get_win32_window(&self) -> *mut c_void {
        unsafe { ffi::glfwGetWin32Window(self.ptr) }
    }

    /// Wrapper for `glfwGetWGLContext`
    #[cfg(target_os="windows")]
    pub fn get_wgl_context(&self) -> *mut c_void {
        unsafe { ffi::glfwGetWGLContext(self.ptr) }
    }

    /// Wrapper for `glfwGetCocoaWindow`
    #[cfg(target_os="macos")]
    pub fn get_cocoa_window(&self) -> *mut c_void {
        unsafe { ffi::glfwGetCocoaWindow(self.ptr) }
    }

    /// Wrapper for `glfwGetNSGLContext`
    #[cfg(target_os="macos")]
    pub fn get_nsgl_context(&self) -> *mut c_void {
        unsafe { ffi::glfwGetNSGLContext(self.ptr) }
    }

    /// Wrapper for `glfwGetX11Window`
    #[cfg(target_os="linux")]
    pub fn get_x11_window(&self) -> *mut c_void {
        unsafe { ffi::glfwGetX11Window(self.ptr) }
    }

    /// Wrapper for `glfwGetGLXContext`
    #[cfg(target_os="linux")]
    pub fn get_glx_context(&self) -> *mut c_void {
        unsafe { ffi::glfwGetGLXContext(self.ptr) }
    }
}

impl Drop for Window {
    /// Closes the window and performs the necessary cleanups. This will block
    /// until all associated `RenderContext`s were also dropped, and emit a
    /// `debug!` message to that effect.
    ///
    /// Wrapper for `glfwDestroyWindow`.
    fn drop(&mut self) {
        drop(self.drop_sender.take());

        // Check if all senders from the child `RenderContext`s have hung up.
        if self.drop_receiver.try_recv() != Err(std::sync::mpsc::TryRecvError::Disconnected) {
            debug!("Attempted to drop a Window before the `RenderContext` was dropped.");
            debug!("Blocking until the `RenderContext` was dropped.");
            let _ = self.drop_receiver.recv();
        }

        if !self.ptr.is_null() {
            unsafe {
                let _: Box<Sender<(f64, WindowEvent)>> = mem::transmute(ffi::glfwGetWindowUserPointer(self.ptr));
            }
        }

        if !self.is_shared {
            unsafe { ffi::glfwDestroyWindow(self.ptr); }
        }
    }
}

/// A rendering context that can be shared between tasks.
pub struct RenderContext {
    ptr: *mut ffi::GLFWwindow,
    /// As long as this sender is alive, it is not safe to drop the parent
    /// `Window`.
    #[allow(dead_code)]
    drop_sender: Sender<()>,
}

unsafe impl Send for RenderContext {}

/// Methods common to renderable contexts
pub trait Context {
    /// Returns the pointer to the underlying `GLFWwindow`.
    fn window_ptr(&self) -> *mut ffi::GLFWwindow;

    /// Swaps the front and back buffers of the window. If the swap interval is
    /// greater than zero, the GPU driver waits the specified number of screen
    /// updates before swapping the buffers.
    ///
    /// Wrapper for `glfwSwapBuffers`.
    fn swap_buffers(&mut self) {
        let ptr = self.window_ptr();
        unsafe { ffi::glfwSwapBuffers(ptr); }
    }

    /// Returns `true` if the window is the current context.
    fn is_current(&self) -> bool {
        self.window_ptr() == unsafe { ffi::glfwGetCurrentContext() }
    }

    /// Wrapper for `glfwMakeContextCurrent`
    fn make_current(&mut self) {
        let ptr = self.window_ptr();
        unsafe { ffi::glfwMakeContextCurrent(ptr); }
    }
}

impl Context for Window {
    fn window_ptr(&self) -> *mut ffi::GLFWwindow { self.ptr }
}

impl Context for RenderContext {
    fn window_ptr(&self) -> *mut ffi::GLFWwindow { self.ptr }
}

/// Wrapper for `glfwMakeContextCurrent`.
pub fn make_context_current(context: Option<&Context>) {
    match context {
        Some(ctx) => unsafe { ffi::glfwMakeContextCurrent(ctx.window_ptr()) },
        None      => unsafe { ffi::glfwMakeContextCurrent(ptr::null_mut()) },
    }
}

enum_from_primitive! {
/// Joystick identifier tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
}

/// A joystick handle.
#[derive(Copy, Clone)]
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
            slice::from_raw_parts(ptr, count as usize).iter().map(|&a| a as f32).collect()
        }
    }

    /// Wrapper for `glfwGetJoystickButtons`.
    pub fn get_buttons(&self) -> Vec<c_int> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetJoystickButtons(self.id as c_int, &mut count);
            slice::from_raw_parts(ptr, count as usize).iter().map(|&b| b as c_int).collect()
        }
    }

    /// Wrapper for `glfwGetJoystickName`.
    pub fn get_name(&self) -> String {
        unsafe { string_from_c_str(ffi::glfwGetJoystickName(self.id as c_int)) }
    }
}
