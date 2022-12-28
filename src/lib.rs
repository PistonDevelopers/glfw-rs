// Copyright 2013-2016 The GLFW-RS Developers. For a full listing of the authors,
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
#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    nonstandard_style,
    unused,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_abi,
    clippy::doc_markdown
)]
#![allow(non_upper_case_globals)]

//! An idiomatic wrapper for the GLFW library.
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
//!
//! # Cargo Features
//!
//! Use the `vulkan` feature flag to enable all Vulkan functions and types.
//!
//! Use the `image` feature flag to enable use of the [`image`](https://github.com/PistonDevelopers/image) library for cursors and icons.
//!
//! Use the `all` feature flag to enable both at the same time.
//!

// TODO: Document differences between GLFW and glfw-rs

#[cfg(feature = "vulkan")]
extern crate vk_sys;
#[cfg(feature = "log")]
#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;
#[cfg(feature = "image")]
extern crate image;
#[cfg(all(target_os = "macos"))]
#[macro_use]
extern crate objc;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle, HasRawDisplayHandle, RawDisplayHandle};

use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::Send;
use std::mem;
#[cfg(feature = "vulkan")]
use std::os::raw::c_uint;
use std::os::raw::{c_char, c_double, c_float, c_int};
use std::os::raw::{c_uchar, c_ushort, c_void};
use std::path::PathBuf;
use std::ptr;
use std::slice;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};

#[cfg(feature = "vulkan")]
use vk_sys::{
    self as vk, AllocationCallbacks as VkAllocationCallbacks, Instance as VkInstance,
    PhysicalDevice as VkPhysicalDevice, Result as VkResult, SurfaceKHR as VkSurfaceKHR,
};

/// Alias to `MouseButton1`, supplied for improved clarity.
pub use self::MouseButton::Button1 as MouseButtonLeft;
/// Alias to `MouseButton2`, supplied for improved clarity.
pub use self::MouseButton::Button2 as MouseButtonRight;
/// Alias to `MouseButton3`, supplied for improved clarity.
pub use self::MouseButton::Button3 as MouseButtonMiddle;

mod callbacks;
pub mod ffi;

/// Unique identifier for a `Window`.
pub type WindowId = usize;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

/// Input actions.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Action {
    Release = ffi::RELEASE,
    Press = ffi::PRESS,
    Repeat = ffi::REPEAT,
}

/// Input keys.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
    Space = ffi::KEY_SPACE,
    Apostrophe = ffi::KEY_APOSTROPHE,
    Comma = ffi::KEY_COMMA,
    Minus = ffi::KEY_MINUS,
    Period = ffi::KEY_PERIOD,
    Slash = ffi::KEY_SLASH,
    Num0 = ffi::KEY_0,
    Num1 = ffi::KEY_1,
    Num2 = ffi::KEY_2,
    Num3 = ffi::KEY_3,
    Num4 = ffi::KEY_4,
    Num5 = ffi::KEY_5,
    Num6 = ffi::KEY_6,
    Num7 = ffi::KEY_7,
    Num8 = ffi::KEY_8,
    Num9 = ffi::KEY_9,
    Semicolon = ffi::KEY_SEMICOLON,
    Equal = ffi::KEY_EQUAL,
    A = ffi::KEY_A,
    B = ffi::KEY_B,
    C = ffi::KEY_C,
    D = ffi::KEY_D,
    E = ffi::KEY_E,
    F = ffi::KEY_F,
    G = ffi::KEY_G,
    H = ffi::KEY_H,
    I = ffi::KEY_I,
    J = ffi::KEY_J,
    K = ffi::KEY_K,
    L = ffi::KEY_L,
    M = ffi::KEY_M,
    N = ffi::KEY_N,
    O = ffi::KEY_O,
    P = ffi::KEY_P,
    Q = ffi::KEY_Q,
    R = ffi::KEY_R,
    S = ffi::KEY_S,
    T = ffi::KEY_T,
    U = ffi::KEY_U,
    V = ffi::KEY_V,
    W = ffi::KEY_W,
    X = ffi::KEY_X,
    Y = ffi::KEY_Y,
    Z = ffi::KEY_Z,
    LeftBracket = ffi::KEY_LEFT_BRACKET,
    Backslash = ffi::KEY_BACKSLASH,
    RightBracket = ffi::KEY_RIGHT_BRACKET,
    GraveAccent = ffi::KEY_GRAVE_ACCENT,
    World1 = ffi::KEY_WORLD_1,
    World2 = ffi::KEY_WORLD_2,

    Escape = ffi::KEY_ESCAPE,
    Enter = ffi::KEY_ENTER,
    Tab = ffi::KEY_TAB,
    Backspace = ffi::KEY_BACKSPACE,
    Insert = ffi::KEY_INSERT,
    Delete = ffi::KEY_DELETE,
    Right = ffi::KEY_RIGHT,
    Left = ffi::KEY_LEFT,
    Down = ffi::KEY_DOWN,
    Up = ffi::KEY_UP,
    PageUp = ffi::KEY_PAGE_UP,
    PageDown = ffi::KEY_PAGE_DOWN,
    Home = ffi::KEY_HOME,
    End = ffi::KEY_END,
    CapsLock = ffi::KEY_CAPS_LOCK,
    ScrollLock = ffi::KEY_SCROLL_LOCK,
    NumLock = ffi::KEY_NUM_LOCK,
    PrintScreen = ffi::KEY_PRINT_SCREEN,
    Pause = ffi::KEY_PAUSE,
    F1 = ffi::KEY_F1,
    F2 = ffi::KEY_F2,
    F3 = ffi::KEY_F3,
    F4 = ffi::KEY_F4,
    F5 = ffi::KEY_F5,
    F6 = ffi::KEY_F6,
    F7 = ffi::KEY_F7,
    F8 = ffi::KEY_F8,
    F9 = ffi::KEY_F9,
    F10 = ffi::KEY_F10,
    F11 = ffi::KEY_F11,
    F12 = ffi::KEY_F12,
    F13 = ffi::KEY_F13,
    F14 = ffi::KEY_F14,
    F15 = ffi::KEY_F15,
    F16 = ffi::KEY_F16,
    F17 = ffi::KEY_F17,
    F18 = ffi::KEY_F18,
    F19 = ffi::KEY_F19,
    F20 = ffi::KEY_F20,
    F21 = ffi::KEY_F21,
    F22 = ffi::KEY_F22,
    F23 = ffi::KEY_F23,
    F24 = ffi::KEY_F24,
    F25 = ffi::KEY_F25,
    Kp0 = ffi::KEY_KP_0,
    Kp1 = ffi::KEY_KP_1,
    Kp2 = ffi::KEY_KP_2,
    Kp3 = ffi::KEY_KP_3,
    Kp4 = ffi::KEY_KP_4,
    Kp5 = ffi::KEY_KP_5,
    Kp6 = ffi::KEY_KP_6,
    Kp7 = ffi::KEY_KP_7,
    Kp8 = ffi::KEY_KP_8,
    Kp9 = ffi::KEY_KP_9,
    KpDecimal = ffi::KEY_KP_DECIMAL,
    KpDivide = ffi::KEY_KP_DIVIDE,
    KpMultiply = ffi::KEY_KP_MULTIPLY,
    KpSubtract = ffi::KEY_KP_SUBTRACT,
    KpAdd = ffi::KEY_KP_ADD,
    KpEnter = ffi::KEY_KP_ENTER,
    KpEqual = ffi::KEY_KP_EQUAL,
    LeftShift = ffi::KEY_LEFT_SHIFT,
    LeftControl = ffi::KEY_LEFT_CONTROL,
    LeftAlt = ffi::KEY_LEFT_ALT,
    LeftSuper = ffi::KEY_LEFT_SUPER,
    RightShift = ffi::KEY_RIGHT_SHIFT,
    RightControl = ffi::KEY_RIGHT_CONTROL,
    RightAlt = ffi::KEY_RIGHT_ALT,
    RightSuper = ffi::KEY_RIGHT_SUPER,
    Menu = ffi::KEY_MENU,
    Unknown = ffi::KEY_UNKNOWN,
}

/// Wrapper around `glfwGetKeyName`
pub fn get_key_name(key: Option<Key>, scancode: Option<Scancode>) -> Option<String> {
    unsafe {
        string_from_nullable_c_str(ffi::glfwGetKeyName(
            match key {
                Some(k) => k as c_int,
                None => ffi::KEY_UNKNOWN,
            },
            scancode.unwrap_or(ffi::KEY_UNKNOWN),
        ))
    }
}

/// Wrapper around `glfwGetKeyName`
#[deprecated(
    since = "0.16.0",
    note = "'key_name' can cause a segfault, use 'get_key_name' instead"
)]
pub fn key_name(key: Option<Key>, scancode: Option<Scancode>) -> String {
    unsafe {
        string_from_c_str(ffi::glfwGetKeyName(
            match key {
                Some(k) => k as c_int,
                None => ffi::KEY_UNKNOWN,
            },
            scancode.unwrap_or(ffi::KEY_UNKNOWN),
        ))
    }
}

/// Wrapper around `glfwGetKeyScancode`.
pub fn get_key_scancode(key: Option<Key>) -> Option<Scancode> {
    unsafe {
        match ffi::glfwGetKeyScancode(match key {
            Some(key) => key as c_int,
            None => ffi::KEY_UNKNOWN,
        }) {
            ffi::KEY_UNKNOWN => None,
            scancode => Some(scancode as Scancode),
        }
    }
}

impl Key {
    /// Wrapper around `glfwGetKeyName` without scancode
    #[deprecated(
        since = "0.16.0",
        note = "Key method 'name' can cause a segfault, use 'get_name' instead"
    )]
    pub fn name(&self) -> String {
        #[allow(deprecated)]
        key_name(Some(*self), None)
    }

    /// Wrapper around `glfwGetKeyName` without scancode
    pub fn get_name(&self) -> Option<String> {
        get_key_name(Some(*self), None)
    }

    /// Wrapper around `glfwGetKeyScancode`.
    pub fn get_scancode(&self) -> Option<Scancode> {
        get_key_scancode(Some(*self))
    }
}

/// Mouse buttons. The `MouseButtonLeft`, `MouseButtonRight`, and
/// `MouseButtonMiddle` aliases are supplied for convenience.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    /// The left mouse button. A `MouseButtonLeft` alias is provided to improve clarity.
    Button1 = ffi::MOUSE_BUTTON_1,
    /// The right mouse button. A `MouseButtonRight` alias is provided to improve clarity.
    Button2 = ffi::MOUSE_BUTTON_2,
    /// The middle mouse button. A `MouseButtonMiddle` alias is provided to improve clarity.
    Button3 = ffi::MOUSE_BUTTON_3,
    Button4 = ffi::MOUSE_BUTTON_4,
    Button5 = ffi::MOUSE_BUTTON_5,
    Button6 = ffi::MOUSE_BUTTON_6,
    Button7 = ffi::MOUSE_BUTTON_7,
    Button8 = ffi::MOUSE_BUTTON_8,
}

impl MouseButton {
    /// Converts from `i32`.
    pub fn from_i32(n: i32) -> Option<MouseButton> {
        if (0..=ffi::MOUSE_BUTTON_LAST).contains(&n) {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let DebugAliases(button) = *self;
        match button {
            MouseButtonLeft => write!(f, "MouseButtonLeft"),
            MouseButtonRight => write!(f, "MouseButtonRight"),
            MouseButtonMiddle => write!(f, "MouseButtonMiddle"),
            button => button.fmt(f),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Callback<Fn, UserData> {
    pub f: Fn,
    pub data: UserData,
}

/// Tokens corresponding to various error types.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Error {
    NoError = ffi::NO_ERROR,
    NotInitialized = ffi::NOT_INITIALIZED,
    NoCurrentContext = ffi::NO_CURRENT_CONTEXT,
    InvalidEnum = ffi::INVALID_ENUM,
    InvalidValue = ffi::INVALID_VALUE,
    OutOfMemory = ffi::OUT_OF_MEMORY,
    ApiUnavailable = ffi::API_UNAVAILABLE,
    VersionUnavailable = ffi::VERSION_UNAVAILABLE,
    PlatformError = ffi::PLATFORM_ERROR,
    FormatUnavailable = ffi::FORMAT_UNAVAILABLE,
    NoWindowContext = ffi::NO_WINDOW_CONTEXT,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match *self {
            Error::NoError => "NoError",
            Error::NotInitialized => "NotInitialized",
            Error::NoCurrentContext => "NoCurrentContext",
            Error::InvalidEnum => "InvalidEnum",
            Error::InvalidValue => "InvalidValue",
            Error::OutOfMemory => "OutOfMemory",
            Error::ApiUnavailable => "ApiUnavailable",
            Error::VersionUnavailable => "VersionUnavailable",
            Error::PlatformError => "PlatformError",
            Error::FormatUnavailable => "FormatUnavailable",
            Error::NoWindowContext => "NoWindowContext",
        };

        f.write_str(description)
    }
}

impl error::Error for Error {}

/// An error callback. This can be supplied with some user data to be passed to
/// the callback function when it is triggered.
pub type ErrorCallback<UserData> = Callback<fn(Error, String, &UserData), UserData>;

/// The function to be used with the `FAIL_ON_ERRORS` callback.
pub fn fail_on_errors(_: Error, description: String, _: &()) {
    panic!("GLFW Error: {}", description);
}

/// A callback that triggers a task failure when an error is encountered.
pub static FAIL_ON_ERRORS: Option<ErrorCallback<()>> = Some(Callback {
    f: fail_on_errors as fn(Error, String, &()),
    data: (),
});

#[cfg(feature = "log")]
/// The function to be used with the `LOG_ERRORS` callback.
pub fn log_errors(_: Error, description: String, _: &()) {
    error!("GLFW Error: {}", description);
}

#[cfg(not(feature = "log"))]
/// The function to be used with the `LOG_ERRORS` callback.
pub fn log_errors(_: Error, description: String, _: &()) {
    eprintln!("GLFW Error: {}", description);
}

/// A callback that logs each error as it is encountered without triggering a
/// task failure.
pub static LOG_ERRORS: Option<ErrorCallback<()>> = Some(Callback {
    f: log_errors as fn(Error, String, &()),
    data: (),
});

/// When not using the `image` library, or if you just want to,
/// you can specify an image from its raw pixel data using this structure.
#[derive(Debug)]
pub struct PixelImage {
    /// Width of the image in pixels
    pub width: u32,
    /// Height of the image in pixels
    pub height: u32,
    /// Pixels are 4 bytes each, one byte for each RGBA subpixel.
    pub pixels: Vec<u32>,
}

/// Cursor modes.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CursorMode {
    Normal = ffi::CURSOR_NORMAL,
    Hidden = ffi::CURSOR_HIDDEN,
    Disabled = ffi::CURSOR_DISABLED,
}

/// Standard cursors provided by GLFW
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum StandardCursor {
    /// The regular arrow cursor shape.
    Arrow = ffi::ARROW_CURSOR,
    /// The text input I-beam cursor shape.
    IBeam = ffi::IBEAM_CURSOR,
    /// The crosshair shape.
    Crosshair = ffi::CROSSHAIR_CURSOR,
    /// The hand shape.
    Hand = ffi::HAND_CURSOR,
    /// The horizontal resize arrow shape.
    HResize = ffi::HRESIZE_CURSOR,
    /// The vertical resize arrow shape.
    VResize = ffi::VRESIZE_CURSOR,
}

/// Represents a window cursor that can be used to display any
/// of the standard cursors or load a custom cursor from an image.
///
/// Note that the cursor object has a lifetime and will not display
/// correctly after it has been dropped.
#[derive(Debug)]
pub struct Cursor {
    ptr: *mut ffi::GLFWcursor,
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { ffi::glfwDestroyCursor(self.ptr) }
    }
}

impl Cursor {
    /// Create a new cursor using `glfwCreateStandardCursor`
    pub fn standard(cursor: StandardCursor) -> Cursor {
        Cursor {
            ptr: unsafe { ffi::glfwCreateStandardCursor(cursor as c_int) },
        }
    }

    /// Creates a new cursor from the image provided via `glfwCreateCursor`
    ///
    /// Note that the cursor image will be the same size as the image provided,
    /// so scaling it beforehand may be required.
    ///
    /// The cursor hotspot is specified in pixels, relative to the upper-left
    /// corner of the cursor image. Like all other coordinate systems in GLFW,
    /// the X-axis points to the right and the Y-axis points down.
    #[cfg(feature = "image")]
    pub fn create(image: image::RgbaImage, x_hotspot: u32, y_hotspot: u32) -> Cursor {
        let (width, height) = image.dimensions();

        let image_data = image.into_vec();

        let glfw_image = ffi::GLFWimage {
            width: width as c_int,
            height: height as c_int,
            pixels: image_data.as_ptr() as *const c_uchar,
        };

        Cursor {
            ptr: unsafe {
                ffi::glfwCreateCursor(
                    &glfw_image as *const ffi::GLFWimage,
                    x_hotspot as c_int,
                    y_hotspot as c_int,
                )
            },
        }
    }

    /// Creates a new cursor from the `PixelImage` provided via `glfwCreateCursor`
    ///
    /// Note that the cursor image will be the same size as the image provided,
    /// so scaling it beforehand may be required.
    ///
    /// The cursor hotspot is specified in pixels, relative to the upper-left
    /// corner of the cursor image. Like all other coordinate systems in GLFW,
    /// the X-axis points to the right and the Y-axis points down.
    pub fn create_from_pixels(image: PixelImage, x_hotspot: u32, y_hotspot: u32) -> Cursor {
        let glfw_image = ffi::GLFWimage {
            width: image.width as c_int,
            height: image.height as c_int,
            pixels: image.pixels.as_ptr() as *const c_uchar,
        };

        Cursor {
            ptr: unsafe {
                ffi::glfwCreateCursor(
                    &glfw_image as *const ffi::GLFWimage,
                    x_hotspot as c_int,
                    y_hotspot as c_int,
                )
            },
        }
    }
}

/// Describes a single video mode.
#[derive(Copy, Clone)]
pub struct VidMode {
    pub width: u32,
    pub height: u32,
    pub red_bits: u32,
    pub green_bits: u32,
    pub blue_bits: u32,
    pub refresh_rate: u32,
}

/// Describes the gamma ramp of a monitor.
#[derive(Debug)]
pub struct GammaRamp {
    pub red: Vec<c_ushort>,
    pub green: Vec<c_ushort>,
    pub blue: Vec<c_ushort>,
}

/// `ContextReleaseBehavior` specifies the release behavior to be used by the context.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ContextReleaseBehavior {
    Any = ffi::ANY_RELEASE_BEHAVIOR,
    /// `Flush` tells the context to flush the pipeline whenever the context is released from being the current one.
    Flush = ffi::RELEASE_BEHAVIOR_FLUSH,
    /// `None` tells the context to NOT flush the pipeline on release
    None = ffi::RELEASE_BEHAVIOR_NONE,
}

/// Specifies the API to use to create the context
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ContextCreationApi {
    Native = ffi::NATIVE_CONTEXT_API,
    Egl = ffi::EGL_CONTEXT_API,
    OsMesa = ffi::OSMESA_CONTEXT_API,
}

/// Specifies how the context should handle swapping the buffers.
///
/// i.e. the number of screen updates to wait from the time
/// `glfwSwapBuffers`/`context.swap_buffers`
/// was called before swapping the buffers and returning.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SwapInterval {
    /// Specifies no waits
    None,
    /// If either of the `WGL_EXT_swap_control_tear` and `GLX_EXT_swap_control_tear` extensions
    /// are enabled, allows the adaptively swap the frame. Sometimes called Adaptive V-sync
    Adaptive,
    /// Synchronizes the buffers every N frames. Set to 1 for V-sync
    Sync(u32),
}

/// An OpenGL process address.
pub type GLProc = ffi::GLFWglproc;

/// A Vulkan process address
#[cfg(feature = "vulkan")]
pub type VkProc = ffi::GLFWvkproc;

/// Counts for (Calling glfwInit) - (Calling glfwTerminate)
/// It uses for "global" refference counting for Glfw.
static REF_COUNT_FOR_GLFW: AtomicUsize = AtomicUsize::new(0);

/// A token from which to call various GLFW functions. It can be obtained by
/// calling the `init` function. This cannot be sent to other tasks, and should
/// only be initialized on the main platform thread. Whilst this might make
/// performing some operations harder, this is to ensure thread safety is enforced
/// statically.
#[non_exhaustive]
#[derive(Debug)]
pub struct Glfw {
    phantom: std::marker::PhantomData<*const ()>,
}

/// An error that might be returned when `glfw::init` is called.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum InitError {
    /// Deprecated. Does not occur.
    AlreadyInitialized,
    /// An internal error occurred when trying to initialize the library.
    Internal,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match *self {
            InitError::AlreadyInitialized => "Already Initialized",
            InitError::Internal => "Internal Initialization Error",
        };

        f.write_str(description)
    }
}

impl error::Error for InitError {}

/// Initialization hints that can be set using the `init_hint` function.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum InitHint {
    /// Specifies whether to also expose joystick hats as buttons, for compatibility with earlier
    /// versions of GLFW that did not have `glfwGetJoystickHats`.
    JoystickHatButtons(bool),
    /// Specifies whether to set the current directory to the application to the `Contents/Resources`
    /// subdirectory of the application's bundle, if present.
    ///
    /// This is ignored on platforms besides macOS.
    CocoaChdirResources(bool),
    /// Specifies whether to create a basic menu bar, either from a nib or manually, when the first
    /// window is created, which is when AppKit is initialized.
    ///
    /// This is ignored on platforms besides macOS.
    CocoaMenubar(bool),
}

/// Sets hints for the next initialization of GLFW.
///
/// The values you set hints to are never reset by GLFW, but they only take effect during
/// initialization. Once GLFW has been initialized, any values you set will be ignored until the
/// library is terminated and initialized again.
///
/// Wrapper for `glfwInitHint`.
pub fn init_hint(hint: InitHint) {
    match hint {
        InitHint::JoystickHatButtons(joystick_hat_buttons) => unsafe {
            ffi::glfwInitHint(ffi::JOYSTICK_HAT_BUTTONS, joystick_hat_buttons as c_int)
        },
        InitHint::CocoaChdirResources(chdir) => unsafe {
            ffi::glfwInitHint(ffi::COCOA_CHDIR_RESOURCES, chdir as c_int)
        },
        InitHint::CocoaMenubar(menubar) => unsafe {
            ffi::glfwInitHint(ffi::COCOA_MENUBAR, menubar as c_int)
        },
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
/// - Subsequent calls to `init` will return `Glfw` token immediately.
/// - If an initialization error occurred within the GLFW library
///   `Err(InternalInitError)` will be returned.
pub fn init<UserData: 'static>(
    mut callback: Option<ErrorCallback<UserData>>,
) -> Result<Glfw, InitError> {
    // Initialize the error callback if it was supplied. This is done
    // before `ffi::glfwInit` because errors could occur during
    // initialization.
    match callback.take() {
        Some(f) => callbacks::error::set(f),
        None => callbacks::error::unset(),
    }
    // initialize GLFW.
    // FYI: multiple not terminated ffi::glfwInit() returns ffi::TRUE immediately.
    // https://www.glfw.org/docs/latest/group__init.html#ga317aac130a235ab08c6db0834907d85e
    if unsafe { ffi::glfwInit() } == ffi::TRUE {
        REF_COUNT_FOR_GLFW.fetch_add(1, Ordering::SeqCst);
        Ok(Glfw {phantom: std::marker::PhantomData})
    } else {
        Err(InitError::Internal)
    }
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
    pub fn set_error_callback<UserData: 'static>(
        &mut self,
        callback: Option<ErrorCallback<UserData>>,
    ) {
        match callback {
            Some(f) => callbacks::error::set(f),
            None => callbacks::error::unset(),
        }
    }

    /// Sets the monitor callback, overwriting the previous one stored.
    pub fn set_monitor_callback<UserData: 'static>(
        &mut self,
        callback: Option<MonitorCallback<UserData>>,
    ) {
        match callback {
            Some(f) => callbacks::monitor::set(f),
            None => callbacks::monitor::unset(),
        }
    }

    /// Sets the joystick callback, overwriting the previous one stored
    pub fn set_joystick_callback<UserData: 'static>(
        &mut self,
        callback: Option<JoystickCallback<UserData>>,
    ) {
        match callback {
            Some(f) => callbacks::joystick::set(f),
            None => callbacks::joystick::unset(),
        }
    }

    /// Supplies the primary monitor to the closure provided, if it exists.
    /// This is usually the monitor where elements like the Windows task bar or
    /// the OS X menu bar is located.
    ///
    /// # Example
    ///
    /// ~~~ignore
    /// let (window, events) = glfw.with_primary_monitor(|_, m| {
    ///     glfw.create_window(300, 300, "Hello this is window",
    ///         m.map_or(glfw::WindowMode::Windowed, |m| glfw::FullScreen(m)))
    /// }).expect("Failed to create GLFW window.");
    /// ~~~
    pub fn with_primary_monitor<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Self, Option<&Monitor>) -> T,
    {
        match unsafe { ffi::glfwGetPrimaryMonitor() } {
            ptr if ptr.is_null() => f(self, None),
            ptr => f(self, Some(&Monitor { ptr })),
        }
    }

    /// Supplies a vector of the currently connected monitors to the closure
    /// provided.
    ///
    /// # Example
    ///
    /// ~~~ignore
    /// glfw.with_connected_monitors(|_, monitors| {
    ///     for monitor in monitors.iter() {
    ///         println!("{}: {}", monitor.get_name(), monitor.get_video_mode());
    ///     }
    /// });
    /// ~~~
    pub fn with_connected_monitors<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Self, &[Monitor]) -> T,
    {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetMonitors(&mut count);
            f(
                self,
                &slice::from_raw_parts(ptr as *const _, count as usize)
                    .iter()
                    .map(|&ptr| Monitor { ptr })
                    .collect::<Vec<Monitor>>(),
            )
        }
    }

    /// Queries Vulkan support via `glfwVulkanSupported`
    #[cfg(feature = "vulkan")]
    pub fn vulkan_supported(&self) -> bool {
        unsafe { ffi::glfwVulkanSupported() == ffi::TRUE }
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
            ffi::glfwWindowHint(hint, unwrap_dont_care(value))
        }

        #[inline(always)]
        unsafe fn string_hint(hint: c_int, value: Option<String>) {
            let value = if let Some(value) = &value {
                value.as_str()
            } else {
                ""
            };
            with_c_str(value, |value| ffi::glfwWindowHintString(hint, value))
        }

        match hint {
            WindowHint::RedBits(bits) => unsafe { dont_care_hint(ffi::RED_BITS, bits) },
            WindowHint::GreenBits(bits) => unsafe { dont_care_hint(ffi::GREEN_BITS, bits) },
            WindowHint::BlueBits(bits) => unsafe { dont_care_hint(ffi::BLUE_BITS, bits) },
            WindowHint::AlphaBits(bits) => unsafe { dont_care_hint(ffi::ALPHA_BITS, bits) },
            WindowHint::DepthBits(bits) => unsafe { dont_care_hint(ffi::DEPTH_BITS, bits) },
            WindowHint::StencilBits(bits) => unsafe { dont_care_hint(ffi::STENCIL_BITS, bits) },
            WindowHint::AccumRedBits(bits) => unsafe { dont_care_hint(ffi::ACCUM_RED_BITS, bits) },
            WindowHint::AccumGreenBits(bits) => unsafe {
                dont_care_hint(ffi::ACCUM_GREEN_BITS, bits)
            },
            WindowHint::AccumBlueBits(bits) => unsafe {
                dont_care_hint(ffi::ACCUM_BLUE_BITS, bits)
            },
            WindowHint::AccumAlphaBits(bits) => unsafe {
                dont_care_hint(ffi::ACCUM_ALPHA_BITS, bits)
            },
            WindowHint::AuxBuffers(num_buffers) => unsafe {
                dont_care_hint(ffi::AUX_BUFFERS, num_buffers)
            },
            WindowHint::Samples(num_samples) => unsafe {
                dont_care_hint(ffi::SAMPLES, num_samples)
            },
            WindowHint::RefreshRate(rate) => unsafe { dont_care_hint(ffi::REFRESH_RATE, rate) },
            WindowHint::Stereo(is_stereo) => unsafe {
                ffi::glfwWindowHint(ffi::STEREO, is_stereo as c_int)
            },
            WindowHint::SRgbCapable(is_capable) => unsafe {
                ffi::glfwWindowHint(ffi::SRGB_CAPABLE, is_capable as c_int)
            },
            WindowHint::ClientApi(api) => unsafe {
                ffi::glfwWindowHint(ffi::CLIENT_API, api as c_int)
            },
            WindowHint::ContextVersionMajor(major) => unsafe {
                ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, major as c_int)
            },
            WindowHint::ContextVersionMinor(minor) => unsafe {
                ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, minor as c_int)
            },
            WindowHint::ContextVersion(major, minor) => unsafe {
                ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, major as c_int);
                ffi::glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, minor as c_int)
            },
            WindowHint::ContextRobustness(robustness) => unsafe {
                ffi::glfwWindowHint(ffi::CONTEXT_ROBUSTNESS, robustness as c_int)
            },
            WindowHint::OpenGlForwardCompat(is_compat) => unsafe {
                ffi::glfwWindowHint(ffi::OPENGL_FORWARD_COMPAT, is_compat as c_int)
            },
            WindowHint::OpenGlDebugContext(is_debug) => unsafe {
                ffi::glfwWindowHint(ffi::OPENGL_DEBUG_CONTEXT, is_debug as c_int)
            },
            WindowHint::OpenGlProfile(profile) => unsafe {
                ffi::glfwWindowHint(ffi::OPENGL_PROFILE, profile as c_int)
            },
            WindowHint::Resizable(is_resizable) => unsafe {
                ffi::glfwWindowHint(ffi::RESIZABLE, is_resizable as c_int)
            },
            WindowHint::Visible(is_visible) => unsafe {
                ffi::glfwWindowHint(ffi::VISIBLE, is_visible as c_int)
            },
            WindowHint::Decorated(is_decorated) => unsafe {
                ffi::glfwWindowHint(ffi::DECORATED, is_decorated as c_int)
            },
            WindowHint::AutoIconify(auto_iconify) => unsafe {
                ffi::glfwWindowHint(ffi::AUTO_ICONIFY, auto_iconify as c_int)
            },
            WindowHint::Floating(is_floating) => unsafe {
                ffi::glfwWindowHint(ffi::FLOATING, is_floating as c_int)
            },
            WindowHint::Focused(is_focused) => unsafe {
                ffi::glfwWindowHint(ffi::FOCUSED, is_focused as c_int)
            },
            WindowHint::ContextNoError(is_no_error) => unsafe {
                ffi::glfwWindowHint(ffi::CONTEXT_NO_ERROR, is_no_error as c_int)
            },
            WindowHint::ContextCreationApi(api) => unsafe {
                ffi::glfwWindowHint(ffi::CONTEXT_CREATION_API, api as c_int)
            },
            WindowHint::ContextReleaseBehavior(behavior) => unsafe {
                ffi::glfwWindowHint(ffi::CONTEXT_RELEASE_BEHAVIOR, behavior as c_int)
            },
            WindowHint::DoubleBuffer(is_dbuffered) => unsafe {
                ffi::glfwWindowHint(ffi::DOUBLEBUFFER, is_dbuffered as c_int)
            },
            WindowHint::CenterCursor(center_cursor) => unsafe {
                ffi::glfwWindowHint(ffi::CENTER_CURSOR, center_cursor as c_int)
            },
            WindowHint::TransparentFramebuffer(is_transparent) => unsafe {
                ffi::glfwWindowHint(ffi::TRANSPARENT_FRAMEBUFFER, is_transparent as c_int)
            },
            WindowHint::FocusOnShow(focus) => unsafe {
                ffi::glfwWindowHint(ffi::FOCUS_ON_SHOW, focus as c_int)
            },
            WindowHint::ScaleToMonitor(scale) => unsafe {
                ffi::glfwWindowHint(ffi::SCALE_TO_MONITOR, scale as c_int)
            },
            WindowHint::CocoaRetinaFramebuffer(retina_fb) => unsafe {
                ffi::glfwWindowHint(ffi::COCOA_RETINA_FRAMEBUFFER, retina_fb as c_int)
            },
            WindowHint::CocoaFrameName(name) => unsafe { string_hint(ffi::COCOA_FRAME_NAME, name) },
            WindowHint::CocoaGraphicsSwitching(graphics_switching) => unsafe {
                ffi::glfwWindowHint(ffi::COCOA_GRAPHICS_SWITCHING, graphics_switching as c_int)
            },
            WindowHint::X11ClassName(class_name) => unsafe {
                string_hint(ffi::X11_CLASS_NAME, class_name)
            },
            WindowHint::X11InstanceName(instance_name) => unsafe {
                string_hint(ffi::X11_INSTANCE_NAME, instance_name)
            },
        }
    }

    /// Resets the window hints previously set by the `window_hint` function to
    /// their default values.
    ///
    /// Wrapper for `glfwDefaultWindowHints`.
    pub fn default_window_hints(&mut self) {
        unsafe {
            ffi::glfwDefaultWindowHints();
        }
    }

    /// Creates a new window.
    ///
    /// Wrapper for `glfwCreateWindow`.
    pub fn create_window(
        &mut self,
        width: u32,
        height: u32,
        title: &str,
        mode: WindowMode<'_>,
    ) -> Option<(Window, Receiver<(f64, WindowEvent)>)> {
        #[cfg(feature = "wayland")]
        {
            // Has to be set otherwise wayland refuses to open window.
            self.window_hint(WindowHint::Focused(false));
        }
        self.create_window_intern(width, height, title, mode, None)
    }

    /// Internal wrapper for `glfwCreateWindow`.
    fn create_window_intern(
        &self,
        width: u32,
        height: u32,
        title: &str,
        mode: WindowMode<'_>,
        share: Option<&Window>,
    ) -> Option<(Window, Receiver<(f64, WindowEvent)>)> {
        let ptr = unsafe {
            with_c_str(title, |title| {
                ffi::glfwCreateWindow(
                    width as c_int,
                    height as c_int,
                    title,
                    mode.to_ptr(),
                    match share {
                        Some(w) => w.ptr,
                        None => ptr::null_mut(),
                    },
                )
            })
        };
        if ptr.is_null() {
            None
        } else {
            let (drop_sender, drop_receiver) = channel();
            let (sender, receiver) = channel();
            unsafe {
                ffi::glfwSetWindowUserPointer(ptr, mem::transmute(Box::new(sender)));
            }
            Some((
                Window {
                    ptr,
                    glfw: self.clone(),
                    is_shared: share.is_some(),
                    drop_sender: Some(drop_sender),
                    drop_receiver,
                    current_cursor: None,
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
            None => unsafe { ffi::glfwMakeContextCurrent(ptr::null_mut()) },
        }
    }

    /// Wrapper for `glfwGetX11Display`
    #[cfg(all(target_os = "linux", not(feature = "wayland")))]
    pub fn get_x11_display(&self) -> *mut c_void {
        unsafe { ffi::glfwGetX11Display() }
    }

    /// Wrapper for `glfwGetWaylandDisplay`
    #[cfg(all(target_os = "linux", feature = "wayland"))]
    pub fn get_wayland_display(&self) -> *mut c_void {
        unsafe { ffi::glfwGetWaylandDisplay() }
    }

    /// Immediately process the received events.
    ///
    /// Wrapper for `glfwPollEvents`.
    pub fn poll_events(&mut self) {
        unsafe {
            ffi::glfwPollEvents();
        }
    }

    /// Immediately process the received events. The *unbuffered* variant differs by allowing
    /// inspection of events *prior* to their associated native callback returning. This also
    /// provides a way to synchronously respond to the event. Events returned by the closure
    /// are delivered to the channel receiver just as if `poll_events` was called. Returning
    /// `None` from the closure will drop the event.
    ///
    /// Wrapper for `glfwPollEvents`.
    pub fn poll_events_unbuffered<F>(&mut self, mut f: F)
    where
        F: FnMut(WindowId, (f64, WindowEvent)) -> Option<(f64, WindowEvent)>,
    {
        let _unset_handler_guard = unsafe { crate::callbacks::unbuffered::set_handler(&mut f) };
        self.poll_events();
    }

    /// Sleep until at least one event has been received, and then perform the
    /// equivalent of `Glfw::poll_events`.
    ///
    /// Wrapper for `glfwWaitEvents`.
    pub fn wait_events(&mut self) {
        unsafe {
            ffi::glfwWaitEvents();
        }
    }

    /// Sleep until at least one event has been received, and then perform the
    /// equivalent of `Glfw::poll_events_unbuffered`.
    ///
    /// Wrapper for `glfwWaitEvents`.
    pub fn wait_events_unbuffered<F>(&mut self, mut f: F)
    where
        F: FnMut(WindowId, (f64, WindowEvent)) -> Option<(f64, WindowEvent)>,
    {
        let _unset_handler_guard = unsafe { crate::callbacks::unbuffered::set_handler(&mut f) };
        self.wait_events();
    }

    /// Sleep until at least one event has been received, or until the specified
    /// timeout is reached, and then perform the equivalent of `Glfw::poll_events`.
    /// Timeout is specified in seconds.
    ///
    /// Wrapper for `glfwWaitEventsTimeout`.
    pub fn wait_events_timeout(&mut self, timeout: f64) {
        unsafe {
            ffi::glfwWaitEventsTimeout(timeout);
        }
    }

    /// Sleep until at least one event has been received, or until the specified
    /// timeout is reached, and then perform the equivalent of `Glfw::poll_events_unbuffered`.
    /// Timeout is specified in seconds.
    ///
    /// Wrapper for `glfwWaitEventsTimeout`.
    pub fn wait_events_timeout_unbuffered<F>(&mut self, timeout: f64, mut f: F)
    where
        F: FnMut(WindowId, (f64, WindowEvent)) -> Option<(f64, WindowEvent)>,
    {
        let _unset_handler_guard = unsafe { crate::callbacks::unbuffered::set_handler(&mut f) };
        self.wait_events_timeout(timeout);
    }

    /// Posts an empty event from the current thread to the event queue, causing
    /// `wait_events` or `wait_events_timeout` to return.
    /// If no windows exist, this function returns immediately.
    ///
    /// Wrapper for `glfwPostEmptyEvent`.
    pub fn post_empty_event(&mut self) {
        unsafe {
            ffi::glfwPostEmptyEvent();
        }
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
        unsafe {
            ffi::glfwSetTime(time as c_double);
        }
    }

    /// Wrapper for `glfwGetTimerValue`.
    pub fn get_timer_value() -> u64 {
        unsafe { ffi::glfwGetTimerValue() as u64 }
    }

    /// Wrapper for `glfwGetTimerFrequency`
    pub fn get_timer_frequency() -> u64 {
        unsafe { ffi::glfwGetTimerFrequency() as u64 }
    }

    /// Sets the number of screen updates to wait before swapping the buffers of
    /// the current context and returning from `Window::swap_buffers`.
    ///
    /// Wrapper for `glfwSwapInterval`.
    pub fn set_swap_interval(&mut self, interval: SwapInterval) {
        unsafe {
            ffi::glfwSwapInterval(match interval {
                SwapInterval::None => 0_i32,
                SwapInterval::Adaptive => -1_i32,
                SwapInterval::Sync(interval) => interval as c_int,
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

    /// Wrapper for `glfwGetRequiredInstanceExtensions`
    ///
    /// This function returns a Vector of names of Vulkan instance extensions
    /// required by GLFW for creating Vulkan surfaces for GLFW windows. If successful,
    /// the list will always contains `VK_KHR_surface`, so if you don't require any
    /// additional extensions you can pass this list directly to the `VkInstanceCreateInfo` struct.
    ///
    /// Will return `None` if the API is unavailable.
    #[cfg(feature = "vulkan")]
    pub fn get_required_instance_extensions(&self) -> Option<Vec<String>> {
        let mut len: c_uint = 0;

        unsafe {
            let raw_extensions: *const *const c_char =
                ffi::glfwGetRequiredInstanceExtensions(&mut len as *mut c_uint);

            if !raw_extensions.is_null() {
                return Some(
                    slice::from_raw_parts(raw_extensions, len as usize)
                        .iter()
                        .map(|extensions| string_from_c_str(*extensions))
                        .collect(),
                );
            }
        }

        None
    }

    /// Returns the address of the specified client API or extension function if
    /// it is supported by the current context, NULL otherwise.
    ///
    /// Wrapper for `glfwGetProcAddress`.
    pub fn get_proc_address_raw(&self, procname: &str) -> GLProc {
        debug_assert!(unsafe { ffi::glfwGetCurrentContext() } != std::ptr::null_mut());
        with_c_str(procname, |procname| unsafe {
            ffi::glfwGetProcAddress(procname)
        })
    }

    /// This function returns the address of the specified Vulkan core or extension function
    /// for the specified instance. If instance is set to NULL it can return any function
    /// exported from the Vulkan loader, including at least the following functions:
    ///
    /// * `vkEnumerateInstanceExtensionProperties`
    /// * `vkEnumerateInstanceLayerProperties`
    /// * `vkCreateInstance`
    /// * `vkGetInstanceProcAddr`
    ///
    /// If Vulkan is not available on the machine, this function returns `NULL`
    ///
    /// Wrapper for `glfwGetInstanceProcAddress`
    #[cfg(feature = "vulkan")]
    pub fn get_instance_proc_address_raw(&self, instance: VkInstance, procname: &str) -> VkProc {
        with_c_str(procname, |procname| unsafe {
            ffi::glfwGetInstanceProcAddress(instance, procname)
        })
    }

    /// This function returns whether the specified queue family of the specified
    /// physical device supports presentation to the platform GLFW was built for.
    ///
    /// Wrapper for `glfwGetPhysicalDevicePresentationSupport`
    #[cfg(feature = "vulkan")]
    pub fn get_physical_device_presentation_support_raw(
        &self,
        instance: VkInstance,
        device: VkPhysicalDevice,
        queue_family: u32,
    ) -> bool {
        vk::TRUE
            == unsafe {
                ffi::glfwGetPhysicalDevicePresentationSupport(
                    instance,
                    device,
                    queue_family as c_uint,
                ) as u32
            }
    }

    /// Constructs a `Joystick` handle corresponding to the supplied `JoystickId`.
    pub fn get_joystick(&self, id: JoystickId) -> Joystick {
        Joystick {
            id,
            glfw: self.clone(),
        }
    }

    /// Wrapper for `glfwRawMouseMotionSupported`.
    pub fn supports_raw_motion(&self) -> bool {
        unsafe { ffi::glfwRawMouseMotionSupported() == ffi::TRUE }
    }

    /// Parses the specified ASCII encoded string and updates the internal list with any gamepad
    /// mappings it finds. This string may contain either a single gamepad mapping or many mappings
    /// separated by newlines. The parser supports the full format of the `gamecontrollerdb.txt`
    /// source file including empty lines and comments.
    ///
    /// Wrapper for `glfwUpdateGamepadMappings`.
    ///
    /// # Returns
    ///
    /// `true` if successful, or `false` if an error occurred.
    pub fn update_gamepad_mappings(&self, mappings: &str) -> bool {
        unsafe {
            with_c_str(mappings, |mappings| {
                ffi::glfwUpdateGamepadMappings(mappings) == ffi::TRUE
            })
        }
    }
}

impl Clone for Glfw {
    fn clone(&self) -> Self {
        REF_COUNT_FOR_GLFW.fetch_add(1, Ordering::SeqCst);
        Glfw {phantom: std::marker::PhantomData}
    }
}

impl Drop for Glfw {
    fn drop(&mut self) {
        let old_diff = REF_COUNT_FOR_GLFW.fetch_sub(1, Ordering::SeqCst);
        if old_diff == 1 {
            unsafe {
                ffi::glfwTerminate();
            }
        }
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
        }
    }
}

/// Replacement for `String::from_raw_buf`
pub unsafe fn string_from_c_str(c_str: *const c_char) -> String {
    String::from_utf8_lossy(CStr::from_ptr(c_str).to_bytes()).into_owned()
}

/// Like `string_from_c_str`, but handles null pointers correctly
pub unsafe fn string_from_nullable_c_str(c_str: *const c_char) -> Option<String> {
    if c_str.is_null() {
        None
    } else {
        Some(string_from_c_str(c_str))
    }
}

/// Replacement for `ToCStr::with_c_str`
pub fn with_c_str<F, T>(s: &str, f: F) -> T
where
    F: FnOnce(*const c_char) -> T,
{
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
    ptr: *mut ffi::GLFWmonitor,
}

impl std::fmt::Debug for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    pub fn get_name(&self) -> Option<String> {
        unsafe { string_from_nullable_c_str(ffi::glfwGetMonitorName(self.ptr)) }
    }

    /// Wrapper for `glfwGetVideoModes`.
    pub fn get_video_modes(&self) -> Vec<VidMode> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetVideoModes(self.ptr, &mut count);
            slice::from_raw_parts(ptr, count as usize)
                .iter()
                .map(VidMode::from_glfw_vid_mode)
                .collect()
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
        unsafe {
            ffi::glfwSetGamma(self.ptr, gamma as c_float);
        }
    }

    /// Wrapper for `glfwGetGammaRamp`.
    pub fn get_gamma_ramp(&self) -> GammaRamp {
        unsafe {
            let llramp = *ffi::glfwGetGammaRamp(self.ptr);
            GammaRamp {
                red: slice::from_raw_parts(llramp.red as *const c_ushort, llramp.size as usize)
                    .iter()
                    .copied()
                    .collect(),
                green: slice::from_raw_parts(llramp.green as *const c_ushort, llramp.size as usize)
                    .iter()
                    .copied()
                    .collect(),
                blue: slice::from_raw_parts(llramp.blue as *const c_ushort, llramp.size as usize)
                    .iter()
                    .copied()
                    .collect(),
            }
        }
    }

    /// Wrapper for `glfwSetGammaRamp`.
    pub fn set_gamma_ramp(&mut self, ramp: &mut GammaRamp) {
        unsafe {
            ffi::glfwSetGammaRamp(
                self.ptr,
                &ffi::GLFWgammaramp {
                    red: ramp.red.as_mut_ptr(),
                    green: ramp.green.as_mut_ptr(),
                    blue: ramp.blue.as_mut_ptr(),
                    size: ramp.red.len() as u32,
                },
            );
        }
    }

    /// Wrapper for `glfwGetMonitorContentScale`.
    pub fn get_content_scale(&self) -> (f32, f32) {
        unsafe {
            let mut xscale = 0.0_f32;
            let mut yscale = 0.0_f32;
            ffi::glfwGetMonitorContentScale(self.ptr, &mut xscale, &mut yscale);
            (xscale, yscale)
        }
    }

    /// Wrapper for `glfwGetMonitorWorkarea`.
    pub fn get_workarea(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut xpos = 0;
            let mut ypos = 0;
            let mut width = 0;
            let mut height = 0;
            ffi::glfwGetMonitorWorkarea(self.ptr, &mut xpos, &mut ypos, &mut width, &mut height);
            (xpos, ypos, width, height)
        }
    }
}

/// Monitor events.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MonitorEvent {
    Connected = ffi::CONNECTED,
    Disconnected = ffi::DISCONNECTED,
}

impl VidMode {
    fn from_glfw_vid_mode(mode: &ffi::GLFWvidmode) -> VidMode {
        VidMode {
            width: mode.width as u32,
            height: mode.height as u32,
            red_bits: mode.redBits as u32,
            green_bits: mode.greenBits as u32,
            blue_bits: mode.blueBits as u32,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} x {}, {} = {} + {} + {}, {} Hz",
            self.width,
            self.height,
            self.red_bits + self.green_bits + self.blue_bits,
            self.red_bits,
            self.green_bits,
            self.blue_bits,
            self.refresh_rate
        )
    }
}

/// Window hints that can be set using the `window_hint` function.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
    /// Specifies whether the windowed mode window will be given input focus when created.
    ///
    /// This hint is ignored for full screen and initially hidden windows.
    Focused(bool),
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
    DoubleBuffer(bool),
    /// Speficies whether the cursor should be centered over newly created full screen windows.
    ///
    /// This hint is ignored for windowed mode windows.
    CenterCursor(bool),
    /// Specifies whether the window framebuffer will be transparent.
    ///
    /// If enabled and supported by the system, the window framebuffer alpha channel will be used to
    /// combine the framebuffer with the background. This does not affect window decorations.
    TransparentFramebuffer(bool),
    /// Specifies whether the window will be given input focus when `Window::show` is called.
    FocusOnShow(bool),
    /// Specifies whether the window content area should be resized based on the monitor current scale
    /// of any monitor it is placed on.
    ///
    /// This includes the initial placement when the window is created.
    ScaleToMonitor(bool),
    /// Specifies whether to use full resolution framebuffers on Retina displays.
    ///
    /// This is ignored on platforms besides macOS.
    CocoaRetinaFramebuffer(bool),
    /// Specifies the UTF-8 encoded name to use for autosaving the window frame, or if empty disables
    /// frame autosaving for the window.
    ///
    /// This is ignored on platforms besides macOS.
    CocoaFrameName(Option<String>),
    /// Specifies whether to in participate in Automatic Graphics Switching, i.e. to allow the system
    /// to choose the integrated GPU for the OpenGL context and move it between GPUs if necessary or
    /// whether to force it to always run on the discrete GPU.
    ///
    /// Simpler programs and tools may want to enable this to save power, while games and other
    /// applications performing advanced rendering will want to leave it disabled.
    //
    //  A bundled application that wishes to participate in Automatic Graphics Switching should also
    // declare this in its `Info.plist` by setting the `NSSupportsAutomaticGraphicsSwitching` key to
    // `true`.
    ///
    /// This only affects systems with both integrated and discrete GPUs. This is ignored on platforms
    /// besides macOS.
    CocoaGraphicsSwitching(bool),
    /// Specifies the desired ASCII-encoded class part of the ICCCM `WM_CLASS` window property.
    X11ClassName(Option<String>),
    /// Specifies the desired ASCII-encoded instance part of the ICCCM `WM_CLASS` window property.
    X11InstanceName(Option<String>),
}

/// Client API tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ClientApiHint {
    NoApi = ffi::NO_API,
    OpenGl = ffi::OPENGL_API,
    OpenGlEs = ffi::OPENGL_ES_API,
}

/// Context robustness tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ContextRobustnessHint {
    NoRobustness = ffi::NO_ROBUSTNESS,
    NoResetNotification = ffi::NO_RESET_NOTIFICATION,
    LoseContextOnReset = ffi::LOSE_CONTEXT_ON_RESET,
}

/// OpenGL profile tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum OpenGlProfileHint {
    Any = ffi::OPENGL_ANY_PROFILE,
    Core = ffi::OPENGL_CORE_PROFILE,
    Compat = ffi::OPENGL_COMPAT_PROFILE,
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
            WindowMode::FullScreen(monitor) => monitor.ptr,
            WindowMode::Windowed => ptr::null_mut(),
        }
    }
}

bitflags! {
    #[doc = "Key modifiers (e.g., Shift, Control, Alt, Super)"]
    pub struct Modifiers: ::std::os::raw::c_int {
        const Shift       = crate::ffi::MOD_SHIFT;
        const Control     = crate::ffi::MOD_CONTROL;
        const Alt         = crate::ffi::MOD_ALT;
        const Super       = crate::ffi::MOD_SUPER;
        const CapsLock    = crate::ffi::MOD_CAPS_LOCK;
        const NumLock     = crate::ffi::MOD_NUM_LOCK;
    }
}

/// Keyboard code returned by the OS
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
    CharModifiers(char, Modifiers),
    FileDrop(Vec<PathBuf>),
    Maximize(bool),
    ContentScale(f32, f32),
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
pub fn flush_messages<Message: Send>(receiver: &Receiver<Message>) -> FlushedMessages<'_, Message> {
    FlushedMessages(receiver)
}

/// An iterator that yields until no more messages are contained in the
/// `Receiver`'s queue.
#[derive(Debug)]
pub struct FlushedMessages<'a, Message: Send>(&'a Receiver<Message>);

unsafe impl<'a, Message: 'a + Send> Send for FlushedMessages<'a, Message> {}

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

/// A struct that wraps a `*GLFWwindow` handle.
#[derive(Debug)]
pub struct Window {
    ptr: *mut ffi::GLFWwindow,
    pub is_shared: bool,
    /// A `Sender` that can be cloned out to child `RenderContext`s.
    drop_sender: Option<Sender<()>>,
    /// Once all  child`RenderContext`s have been dropped, calling `try_recv()`
    /// on the `drop_receiver` will result in an `Err(std::comm::Disconnected)`,
    /// indicating that it is safe to drop the `Window`.
    #[allow(unused)]
    drop_receiver: Receiver<()>,
    /// This is here to allow owning the current Cursor object instead
    /// of forcing the user to take care of its lifetime.
    current_cursor: Option<Cursor>,
    pub glfw: Glfw,
}

macro_rules! set_window_callback {
    ($window:ident, $should_poll:expr, $ll_fn:ident, $callback:ident) => {{
        if $should_poll {
            unsafe {
                ffi::$ll_fn($window.ptr, Some(callbacks::$callback));
            }
        } else {
            unsafe {
                ffi::$ll_fn($window.ptr, None);
            }
        }
    }};
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

    /// This function returns the address of the specified Vulkan core or extension function
    /// for the specified instance. If instance is set to NULL it can return any function
    /// exported from the Vulkan loader, including at least the following functions:
    ///
    /// * `vkEnumerateInstanceExtensionProperties`
    /// * `vkEnumerateInstanceLayerProperties`
    /// * `vkCreateInstance`
    /// * `vkGetInstanceProcAddr`
    ///
    /// If Vulkan is not available on the machine, this function returns `NULL`
    ///
    /// Wrapper for `glfwGetInstanceProcAddress`
    #[cfg(feature = "vulkan")]
    pub fn get_instance_proc_address(&mut self, instance: VkInstance, procname: &str) -> VkProc {
        self.glfw.get_instance_proc_address_raw(instance, procname)
    }

    /// This function returns whether the specified queue family of the specified
    /// physical device supports presentation to the platform GLFW was built for.
    ///
    /// Wrapper for `glfwGetPhysicalDevicePresentationSupport`
    #[cfg(feature = "vulkan")]
    pub fn get_physical_device_presentation_support(
        &self,
        instance: VkInstance,
        device: VkPhysicalDevice,
        queue_family: u32,
    ) -> bool {
        self.glfw
            .get_physical_device_presentation_support_raw(instance, device, queue_family)
    }

    /// wrapper for `glfwCreateWindowSurface`
    #[cfg(feature = "vulkan")]
    pub fn create_window_surface(
        &self,
        instance: VkInstance,
        allocator: *const VkAllocationCallbacks,
        surface: *mut VkSurfaceKHR,
    ) -> VkResult {
        unsafe { ffi::glfwCreateWindowSurface(instance, self.ptr, allocator, surface) }
    }
    /// Wrapper for `glfwCreateWindow`.
    pub fn create_shared(
        &self,
        width: u32,
        height: u32,
        title: &str,
        mode: WindowMode<'_>,
    ) -> Option<(Window, Receiver<(f64, WindowEvent)>)> {
        self.glfw
            .create_window_intern(width, height, title, mode, Some(self))
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
            drop_sender: self.drop_sender.as_ref().unwrap().clone(),
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
        unsafe {
            ffi::glfwSetWindowPos(self.ptr, xpos as c_int, ypos as c_int);
        }
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
        unsafe {
            ffi::glfwSetWindowSize(self.ptr, width as c_int, height as c_int);
        }
    }

    /// Wrapper for `glfwGetWindowFrameSize`
    ///
    /// Returns `(left, top, right, bottom)` edge window frame sizes, in screen coordinates.
    pub fn get_frame_size(&self) -> (i32, i32, i32, i32) {
        let (mut left, mut top, mut right, mut bottom): (i32, i32, i32, i32) = (0, 0, 0, 0);

        unsafe {
            ffi::glfwGetWindowFrameSize(
                self.ptr,
                &mut left as *mut c_int,
                &mut top as *mut c_int,
                &mut right as *mut c_int,
                &mut bottom as *mut c_int,
            );
        }

        (left, top, right, bottom)
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
    ///
    /// A value of `None` is equivalent to `GLFW_DONT_CARE`.
    /// If `minwidth` or `minheight` are `None`, no minimum size is enforced.
    /// If `maxwidth` or `maxheight` are `None`, no maximum size is enforced.
    pub fn set_size_limits(
        &mut self,
        minwidth: Option<u32>,
        minheight: Option<u32>,
        maxwidth: Option<u32>,
        maxheight: Option<u32>,
    ) {
        unsafe {
            ffi::glfwSetWindowSizeLimits(
                self.ptr,
                unwrap_dont_care(minwidth),
                unwrap_dont_care(minheight),
                unwrap_dont_care(maxwidth),
                unwrap_dont_care(maxheight),
            )
        }
    }

    /// Wrapper for `glfwIconifyWindow`.
    pub fn iconify(&mut self) {
        unsafe {
            ffi::glfwIconifyWindow(self.ptr);
        }
    }

    /// Wrapper for `glfwRestoreWindow`.
    pub fn restore(&mut self) {
        unsafe {
            ffi::glfwRestoreWindow(self.ptr);
        }
    }

    /// Wrapper for `glfwMaximizeWindow`
    pub fn maximize(&mut self) {
        unsafe { ffi::glfwMaximizeWindow(self.ptr) }
    }

    /// Wrapper for `glfwShowWindow`.
    pub fn show(&mut self) {
        unsafe {
            ffi::glfwShowWindow(self.ptr);
        }
    }

    /// Wrapper for `glfwHideWindow`.
    pub fn hide(&mut self) {
        unsafe {
            ffi::glfwHideWindow(self.ptr);
        }
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
    pub fn with_window_mode<T, F>(&self, f: F) -> T
    where
        F: FnOnce(WindowMode<'_>) -> T,
    {
        let ptr = unsafe { ffi::glfwGetWindowMonitor(self.ptr) };
        if ptr.is_null() {
            f(WindowMode::Windowed)
        } else {
            f(WindowMode::FullScreen(&Monitor { ptr }))
        }
    }

    /// Wrapper for `glfwSetWindowMonitor`
    pub fn set_monitor(
        &mut self,
        mode: WindowMode<'_>,
        xpos: i32,
        ypos: i32,
        width: u32,
        height: u32,
        refresh_rate: Option<u32>,
    ) {
        let monitor_ptr = if let WindowMode::FullScreen(monitor) = mode {
            monitor.ptr
        } else {
            ptr::null_mut()
        };

        unsafe {
            ffi::glfwSetWindowMonitor(
                self.ptr,
                monitor_ptr,
                xpos as c_int,
                ypos as c_int,
                width as c_int,
                height as c_int,
                unwrap_dont_care(refresh_rate),
            )
        }
    }

    /// Wrapper for `glfwFocusWindow`
    ///
    /// It is NOT recommended to use this function, as it steals focus from other applications
    /// and can be extremely disruptive to the user.
    pub fn focus(&mut self) {
        unsafe { ffi::glfwFocusWindow(self.ptr) }
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

    /// Wrapper for `glfwSetWindowAttrib` called with `RESIZABLE`.
    pub fn set_resizable(&mut self, resizable: bool) {
        unsafe { ffi::glfwSetWindowAttrib(self.ptr, ffi::RESIZABLE, resizable as c_int) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `VISIBLE`.
    pub fn is_visible(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::VISIBLE) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `DECORATED`.
    pub fn is_decorated(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::DECORATED) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowAttrib` called with `DECORATED`.
    pub fn set_decorated(&mut self, decorated: bool) {
        unsafe { ffi::glfwSetWindowAttrib(self.ptr, ffi::DECORATED, decorated as c_int) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `AUTO_ICONIFY`.
    pub fn is_auto_iconify(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::AUTO_ICONIFY) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowAttrib` called with `AUTO_ICONIFY`.
    pub fn set_auto_iconify(&mut self, auto_iconify: bool) {
        unsafe { ffi::glfwSetWindowAttrib(self.ptr, ffi::AUTO_ICONIFY, auto_iconify as c_int) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `FLOATING`.
    pub fn is_floating(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::FLOATING) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowAttrib` called with `FLOATING`.
    pub fn set_floating(&mut self, floating: bool) {
        unsafe { ffi::glfwSetWindowAttrib(self.ptr, ffi::FLOATING, floating as c_int) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `TRANSPARENT_FRAMEBUFFER`.
    pub fn is_framebuffer_transparent(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::TRANSPARENT_FRAMEBUFFER) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `FOCUS_ON_SHOW`.
    pub fn is_focus_on_show(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::FOCUS_ON_SHOW) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowAttrib` called with `FOCUS_ON_SHOW`.
    pub fn set_focus_on_show(&mut self, focus_on_show: bool) {
        unsafe { ffi::glfwSetWindowAttrib(self.ptr, ffi::FOCUS_ON_SHOW, focus_on_show as c_int) }
    }

    /// Wrapper for `glfwGetWindowAttrib` called with `HOVERED`.
    pub fn is_hovered(&self) -> bool {
        unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::HOVERED) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowPosCallback`.
    pub fn set_pos_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetWindowPosCallback,
            window_pos_callback
        );
    }

    /// Starts or stops polling for all available events
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
        self.set_char_mods_polling(should_poll);
        self.set_mouse_button_polling(should_poll);
        self.set_cursor_pos_polling(should_poll);
        self.set_cursor_enter_polling(should_poll);
        self.set_scroll_polling(should_poll);
        self.set_drag_and_drop_polling(should_poll);
        self.set_maximize_polling(should_poll);
        self.set_content_scale_polling(should_poll);
    }

    /// Wrapper for `glfwSetWindowSizeCallback`.
    pub fn set_size_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetWindowSizeCallback,
            window_size_callback
        );
    }

    /// Wrapper for `glfwSetWindowCloseCallback`.
    pub fn set_close_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetWindowCloseCallback,
            window_close_callback
        );
    }

    /// Wrapper for `glfwSetWindowRefreshCallback`.
    pub fn set_refresh_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetWindowRefreshCallback,
            window_refresh_callback
        );
    }

    /// Wrapper for `glfwSetWindowFocusCallback`.
    pub fn set_focus_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetWindowFocusCallback,
            window_focus_callback
        );
    }

    /// Wrapper for `glfwSetWindowIconifyCallback`.
    pub fn set_iconify_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetWindowIconifyCallback,
            window_iconify_callback
        );
    }

    /// Wrapper for `glfwSetFramebufferSizeCallback`.
    pub fn set_framebuffer_size_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetFramebufferSizeCallback,
            framebuffer_size_callback
        );
    }

    /// Wrapper for `glfwSetDropCallback`.
    pub fn set_drag_and_drop_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetDropCallback, drop_callback);
    }

    /// Wrapper for `glfwSetWindowMaximizeCallback`.
    pub fn set_maximize_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetWindowMaximizeCallback,
            window_maximize_callback
        );
    }

    /// Wrapper for `glfwSetWindowContentScaleCallback`.
    pub fn set_content_scale_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetWindowContentScaleCallback,
            window_content_scale_callback
        );
    }

    /// Wrapper for `glfwGetInputMode` called with `CURSOR`.
    pub fn get_cursor_mode(&self) -> CursorMode {
        unsafe { mem::transmute(ffi::glfwGetInputMode(self.ptr, ffi::CURSOR)) }
    }

    /// Wrapper for `glfwSetInputMode` called with `CURSOR`.
    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        unsafe {
            ffi::glfwSetInputMode(self.ptr, ffi::CURSOR, mode as c_int);
        }
    }

    /// Wrapper for `glfwSetCursor` using `Cursor`
    ///
    /// The window will take ownership of the cursor, and will not Drop it
    /// until it is replaced or the window itself is destroyed.
    ///
    /// Returns the previously set Cursor or None if no cursor was set.
    pub fn set_cursor(&mut self, cursor: Option<Cursor>) -> Option<Cursor> {
        let previous = mem::replace(&mut self.current_cursor, cursor);

        unsafe {
            ffi::glfwSetCursor(
                self.ptr,
                match self.current_cursor {
                    Some(ref cursor) => cursor.ptr,
                    None => ptr::null_mut(),
                },
            )
        }

        previous
    }

    /// Sets the window icon from the given images by called `glfwSetWindowIcon`
    ///
    /// Multiple images can be specified for allowing the OS to choose the best size where necessary.
    ///
    /// Example:
    ///
    /// ```ignore
    ///if let DynamicImage::ImageRgba8(icon) = image::open("examples/icon.png").unwrap() {
    ///    window.set_icon(vec![
    ///        imageops::resize(&icon, 16, 16, image::imageops::Lanczos3),
    ///        imageops::resize(&icon, 32, 32, image::imageops::Lanczos3),
    ///        imageops::resize(&icon, 48, 48, image::imageops::Lanczos3)
    ///    ]);
    ///}
    /// ```
    #[cfg(feature = "image")]
    pub fn set_icon(&mut self, images: Vec<image::RgbaImage>) {
        // When the images are turned into Vecs, the lifetimes of them go into the Vec lifetime
        // So they need to be kept until the function ends.
        let image_data: Vec<(Vec<_>, u32, u32)> = images
            .into_iter()
            .map(|image| {
                let (width, height) = image.dimensions();

                (image.into_vec(), width, height)
            })
            .collect();

        let glfw_images: Vec<ffi::GLFWimage> = image_data
            .iter()
            .map(|data| ffi::GLFWimage {
                width: data.1 as c_int,
                height: data.2 as c_int,
                pixels: data.0.as_ptr() as *const c_uchar,
            })
            .collect();

        unsafe {
            ffi::glfwSetWindowIcon(
                self.ptr,
                glfw_images.len() as c_int,
                glfw_images.as_ptr() as *const ffi::GLFWimage,
            )
        }
    }

    /// Sets the window icon via `glfwSetWindowIcon` from a set a set of vectors
    /// containing pixels in RGBA format (one pixel per 32-bit integer)
    pub fn set_icon_from_pixels(&mut self, images: Vec<PixelImage>) {
        let glfw_images: Vec<ffi::GLFWimage> = images
            .iter()
            .map(|image: &PixelImage| ffi::GLFWimage {
                width: image.width as c_int,
                height: image.height as c_int,
                pixels: image.pixels.as_ptr() as *const c_uchar,
            })
            .collect();

        unsafe {
            ffi::glfwSetWindowIcon(
                self.ptr,
                glfw_images.len() as c_int,
                glfw_images.as_ptr() as *const ffi::GLFWimage,
            )
        }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_KEYS`.
    pub fn has_sticky_keys(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_KEYS) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_KEYS`.
    pub fn set_sticky_keys(&mut self, value: bool) {
        unsafe {
            ffi::glfwSetInputMode(self.ptr, ffi::STICKY_KEYS, value as c_int);
        }
    }

    /// Wrapper for `glfwGetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn has_sticky_mouse_buttons(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `STICKY_MOUSE_BUTTONS`.
    pub fn set_sticky_mouse_buttons(&mut self, value: bool) {
        unsafe {
            ffi::glfwSetInputMode(self.ptr, ffi::STICKY_MOUSE_BUTTONS, value as c_int);
        }
    }

    /// Wrapper for `glfwGetInputMode` called with `LOCK_KEY_MODS`
    pub fn does_store_lock_key_mods(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::LOCK_KEY_MODS) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `LOCK_KEY_MODS`
    pub fn set_store_lock_key_mods(&mut self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::LOCK_KEY_MODS, value as c_int) }
    }

    /// Wrapper for `glfwGetInputMode` called with `RAW_MOUSE_MOTION`
    pub fn uses_raw_mouse_motion(&self) -> bool {
        unsafe { ffi::glfwGetInputMode(self.ptr, ffi::RAW_MOUSE_MOTION) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetInputMode` called with `RAW_MOUSE_MOTION`
    pub fn set_raw_mouse_motion(&mut self, value: bool) {
        unsafe { ffi::glfwSetInputMode(self.ptr, ffi::RAW_MOUSE_MOTION, value as c_int) }
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
        unsafe {
            ffi::glfwSetCursorPos(self.ptr, xpos as c_double, ypos as c_double);
        }
    }

    /// Wrapper for `glfwSetKeyCallback`.
    pub fn set_key_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetKeyCallback, key_callback);
    }

    /// Wrapper for `glfwSetCharCallback`.
    pub fn set_char_polling(&mut self, should_poll: bool) {
        set_window_callback!(self, should_poll, glfwSetCharCallback, char_callback);
    }

    /// Wrapper for `glfwSetCharModsCallback`
    pub fn set_char_mods_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetCharModsCallback,
            char_mods_callback
        );
    }

    /// Wrapper for `glfwSetMouseButtonCallback`.
    pub fn set_mouse_button_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetMouseButtonCallback,
            mouse_button_callback
        );
    }

    /// Wrapper for `glfwSetCursorPosCallback`.
    pub fn set_cursor_pos_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetCursorPosCallback,
            cursor_pos_callback
        );
    }

    /// Wrapper for `glfwSetCursorEnterCallback`.
    pub fn set_cursor_enter_polling(&mut self, should_poll: bool) {
        set_window_callback!(
            self,
            should_poll,
            glfwSetCursorEnterCallback,
            cursor_enter_callback
        );
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
    pub fn get_clipboard_string(&self) -> Option<String> {
        unsafe { string_from_nullable_c_str(ffi::glfwGetClipboardString(self.ptr)) }
    }

    /// Wrapper for `glfwGetWindowOpacity`.
    pub fn get_opacity(&self) -> f32 {
        unsafe { ffi::glfwGetWindowOpacity(self.ptr) }
    }

    /// Wrapper for `glfwSetWindowOpacity`.
    pub fn set_opacity(&mut self, opacity: f32) {
        unsafe { ffi::glfwSetWindowOpacity(self.ptr, opacity) }
    }

    /// Wrapper for `glfwRequestWindowAttention`.
    pub fn request_attention(&mut self) {
        unsafe { ffi::glfwRequestWindowAttention(self.ptr) }
    }

    /// Wrapper for `glfwGetWindowContentScale`.
    pub fn get_content_scale(&self) -> (f32, f32) {
        unsafe {
            let mut xscale = 0.0_f32;
            let mut yscale = 0.0_f32;
            ffi::glfwGetWindowContentScale(self.ptr, &mut xscale, &mut yscale);
            (xscale, yscale)
        }
    }

    /// Wrapper for `glfwGetWin32Window`
    #[cfg(target_os = "windows")]
    pub fn get_win32_window(&self) -> *mut c_void {
        unsafe { ffi::glfwGetWin32Window(self.ptr) }
    }

    /// Wrapper for `glfwGetWGLContext`
    #[cfg(target_os = "windows")]
    pub fn get_wgl_context(&self) -> *mut c_void {
        unsafe { ffi::glfwGetWGLContext(self.ptr) }
    }

    /// Wrapper for `glfwGetCocoaWindow`
    #[cfg(target_os = "macos")]
    pub fn get_cocoa_window(&self) -> *mut c_void {
        unsafe { ffi::glfwGetCocoaWindow(self.ptr) }
    }

    /// Wrapper for `glfwGetNSGLContext`
    #[cfg(target_os = "macos")]
    pub fn get_nsgl_context(&self) -> *mut c_void {
        unsafe { ffi::glfwGetNSGLContext(self.ptr) }
    }

    /// Wrapper for `glfwGetX11Window`
    #[cfg(all(target_os = "linux", not(feature = "wayland")))]
    pub fn get_x11_window(&self) -> *mut c_void {
        unsafe { ffi::glfwGetX11Window(self.ptr) }
    }

    /// Wrapper for `glfwGetWaylandWindow`
    #[cfg(all(target_os = "linux", feature = "wayland"))]
    pub fn get_wayland_window(&self) -> *mut c_void {
        unsafe { ffi::glfwGetWaylandWindow(self.ptr) }
    }

    /// Wrapper for `glfwGetGLXContext`
    #[cfg(target_os = "linux")]
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
        #[cfg(feature = "log")]
        if self.drop_receiver.try_recv() != Err(std::sync::mpsc::TryRecvError::Disconnected) {
            debug!("Attempted to drop a Window before the `RenderContext` was dropped.");
            debug!("Blocking until the `RenderContext` was dropped.");
            let _ = self.drop_receiver.recv();
        }

        if !self.ptr.is_null() {
            unsafe {
                let _: Box<Sender<(f64, WindowEvent)>> =
                    mem::transmute(ffi::glfwGetWindowUserPointer(self.ptr));
            }
        }

        if !self.is_shared {
            unsafe {
                ffi::glfwDestroyWindow(self.ptr);
            }
        }
    }
}

/// A rendering context that can be shared between tasks.
#[derive(Debug)]
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

    /// Returns the unique identifier for this window.
    fn window_id(&self) -> WindowId {
        self.window_ptr() as WindowId
    }

    /// Swaps the front and back buffers of the window. If the swap interval is
    /// greater than zero, the GPU driver waits the specified number of screen
    /// updates before swapping the buffers.
    ///
    /// Wrapper for `glfwSwapBuffers`.
    fn swap_buffers(&mut self) {
        let ptr = self.window_ptr();
        unsafe {
            ffi::glfwSwapBuffers(ptr);
        }
    }

    /// Returns `true` if the window is the current context.
    fn is_current(&self) -> bool {
        self.window_ptr() == unsafe { ffi::glfwGetCurrentContext() }
    }

    /// Wrapper for `glfwMakeContextCurrent`
    fn make_current(&mut self) {
        let ptr = self.window_ptr();
        unsafe {
            ffi::glfwMakeContextCurrent(ptr);
        }
    }

    /// Wrapper for `glfwWindowShouldClose`.
    fn should_close(&self) -> bool {
        let ptr = self.window_ptr();
        unsafe { ffi::glfwWindowShouldClose(ptr) == ffi::TRUE }
    }

    /// Wrapper for `glfwSetWindowShouldClose`.
    fn set_should_close(&mut self, value: bool) {
        let ptr = self.window_ptr();
        unsafe {
            ffi::glfwSetWindowShouldClose(ptr, value as c_int);
        }
    }

    /// Wrapper for `glfwPostEmptyEvent`.
    fn post_empty_event(&self) {
        unsafe { ffi::glfwPostEmptyEvent() }
    }
}

impl Context for Window {
    fn window_ptr(&self) -> *mut ffi::GLFWwindow {
        self.ptr
    }
}

impl Context for RenderContext {
    fn window_ptr(&self) -> *mut ffi::GLFWwindow {
        self.ptr
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        raw_window_handle(self)
    }
}

unsafe impl HasRawWindowHandle for RenderContext {
    fn raw_window_handle(&self) -> RawWindowHandle {
        raw_window_handle(self)
    }
}

unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        raw_display_handle()
    }
}

unsafe impl HasRawDisplayHandle for RenderContext {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        raw_display_handle()
    }
}

fn raw_window_handle<C: Context>(context: &C) -> RawWindowHandle {
    #[cfg(target_family = "windows")]
    {
        use raw_window_handle::Win32WindowHandle;
        let (hwnd, hinstance) = unsafe {
            let hwnd = ffi::glfwGetWin32Window(context.window_ptr());
            let hinstance = winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null());
            (hwnd, hinstance as _)
        };
        let mut handle = Win32WindowHandle::empty();
        handle.hwnd = hwnd;
        handle.hinstance = hinstance;
        RawWindowHandle::Win32(handle)
    }
    #[cfg(all(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly"), not(feature = "wayland")))]
    {
        use raw_window_handle::XlibWindowHandle;
        let mut handle = XlibWindowHandle::empty();
        handle.window = unsafe { ffi::glfwGetX11Window(context.window_ptr()) as std::os::raw::c_ulong };
        RawWindowHandle::Xlib(handle)
    }
    #[cfg(all(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly"), feature = "wayland"))]
    {
        use raw_window_handle::WaylandWindowHandle;
        let mut handle = WaylandWindowHandle::empty();
        handle.surface = unsafe { ffi::glfwGetWaylandWindow(context.window_ptr()) };
        RawWindowHandle::Wayland(handle)
    }
    #[cfg(target_os = "macos")]
    {
        use raw_window_handle::AppKitWindowHandle;
        let (ns_window, ns_view) = unsafe {
            let ns_window: *mut objc::runtime::Object =
                ffi::glfwGetCocoaWindow(context.window_ptr()) as *mut _;
            let ns_view: *mut objc::runtime::Object = objc::msg_send![ns_window, contentView];
            assert_ne!(ns_view, std::ptr::null_mut());
            (
                ns_window as *mut std::ffi::c_void,
                ns_view as *mut std::ffi::c_void,
            )
        };
        let mut handle = AppKitWindowHandle::empty();
        handle.ns_window = ns_window;
        handle.ns_view = ns_view;
        RawWindowHandle::AppKit(handle)
    }
}

fn raw_display_handle() -> RawDisplayHandle {
    #[cfg(target_family = "windows")]
    {
        use raw_window_handle::WindowsDisplayHandle;
        RawDisplayHandle::Windows(WindowsDisplayHandle::empty())
    }
    #[cfg(all(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly"), not(feature = "wayland")))]
    {
        use raw_window_handle::XlibDisplayHandle;
        let mut handle = XLibDisplayHandle::empty();
        handle.display = unsafe { ffi::glfwGetX11Display() };
        RawDisplayHandle::Xlib(handle)
    }
    #[cfg(all(any(target_os = "linux", target_os = "freebsd", target_os = "dragonfly"), feature = "wayland"))]
    {
        use raw_window_handle::WaylandDisplayHandle;
        let mut handle = WaylandDisplayHandle::empty();
        handle.display = unsafe { ffi::glfwGetWaylandDisplay() };
        RawDisplayHandle::Wayland(handle)
    }
    #[cfg(target_os = "macos")]
    {
        use raw_window_handle::AppKitDisplayHandle;
        RawDisplayHandle::AppKit(AppKitDisplayHandle::empty())
    }
}

/// Wrapper for `glfwMakeContextCurrent`.
pub fn make_context_current(context: Option<&dyn Context>) {
    match context {
        Some(ctx) => unsafe { ffi::glfwMakeContextCurrent(ctx.window_ptr()) },
        None => unsafe { ffi::glfwMakeContextCurrent(ptr::null_mut()) },
    }
}

/// Joystick identifier tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum JoystickId {
    Joystick1 = ffi::JOYSTICK_1,
    Joystick2 = ffi::JOYSTICK_2,
    Joystick3 = ffi::JOYSTICK_3,
    Joystick4 = ffi::JOYSTICK_4,
    Joystick5 = ffi::JOYSTICK_5,
    Joystick6 = ffi::JOYSTICK_6,
    Joystick7 = ffi::JOYSTICK_7,
    Joystick8 = ffi::JOYSTICK_8,
    Joystick9 = ffi::JOYSTICK_9,
    Joystick10 = ffi::JOYSTICK_10,
    Joystick11 = ffi::JOYSTICK_11,
    Joystick12 = ffi::JOYSTICK_12,
    Joystick13 = ffi::JOYSTICK_13,
    Joystick14 = ffi::JOYSTICK_14,
    Joystick15 = ffi::JOYSTICK_15,
    Joystick16 = ffi::JOYSTICK_16,
}

impl JoystickId {
    /// Converts from `i32`.
    pub fn from_i32(n: i32) -> Option<JoystickId> {
        if (0..=ffi::JOYSTICK_LAST).contains(&n) {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }
}

/// Button identifier tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum GamepadButton {
    ButtonA = ffi::GAMEPAD_BUTTON_A,
    ButtonB = ffi::GAMEPAD_BUTTON_B,
    ButtonX = ffi::GAMEPAD_BUTTON_X,
    ButtonY = ffi::GAMEPAD_BUTTON_Y,
    ButtonLeftBumper = ffi::GAMEPAD_BUTTON_LEFT_BUMPER,
    ButtonRightBumper = ffi::GAMEPAD_BUTTON_RIGHT_BUMPER,
    ButtonBack = ffi::GAMEPAD_BUTTON_BACK,
    ButtonStart = ffi::GAMEPAD_BUTTON_START,
    ButtonGuide = ffi::GAMEPAD_BUTTON_GUIDE,
    ButtonLeftThumb = ffi::GAMEPAD_BUTTON_LEFT_THUMB,
    ButtonRightThumb = ffi::GAMEPAD_BUTTON_RIGHT_THUMB,
    ButtonDpadUp = ffi::GAMEPAD_BUTTON_DPAD_UP,
    ButtonDpadRight = ffi::GAMEPAD_BUTTON_DPAD_RIGHT,
    ButtonDpadDown = ffi::GAMEPAD_BUTTON_DPAD_DOWN,
    ButtonDpadLeft = ffi::GAMEPAD_BUTTON_DPAD_LEFT,
}

impl GamepadButton {
    /// Converts from `i32`.
    pub fn from_i32(n: i32) -> Option<GamepadButton> {
        if (0..=ffi::GAMEPAD_BUTTON_LAST).contains(&n) {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }
}

/// Axis identifier tokens.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum GamepadAxis {
    AxisLeftX = ffi::GAMEPAD_AXIS_LEFT_X,
    AxisLeftY = ffi::GAMEPAD_AXIS_LEFT_Y,
    AxisRightX = ffi::GAMEPAD_AXIS_RIGHT_X,
    AxisRightY = ffi::GAMEPAD_AXIS_RIGHT_Y,
    AxisLeftTrigger = ffi::GAMEPAD_AXIS_LEFT_TRIGGER,
    AxisRightTrigger = ffi::GAMEPAD_AXIS_RIGHT_TRIGGER,
}

impl GamepadAxis {
    /// Converts from `i32`.
    pub fn from_i32(n: i32) -> Option<GamepadAxis> {
        if (0..=ffi::GAMEPAD_AXIS_LAST).contains(&n) {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }
}

bitflags! {
    #[doc = "Joystick hats."]
    pub struct JoystickHats: ::std::os::raw::c_int {
        const Centered = crate::ffi::HAT_CENTERED;
        const Up       = crate::ffi::HAT_UP;
        const Right    = crate::ffi::HAT_RIGHT;
        const Down     = crate::ffi::HAT_DOWN;
        const Left     = crate::ffi::HAT_LEFT;
    }
}

/// A joystick handle.
#[derive(Clone, Debug)]
pub struct Joystick {
    pub id: JoystickId,
    pub glfw: Glfw,
}

/// State of a gamepad.
#[derive(Copy, Clone, Debug)]
pub struct GamepadState {
    buttons: [Action; (ffi::GAMEPAD_BUTTON_LAST + 1) as usize],
    axes: [f32; (ffi::GAMEPAD_AXIS_LAST + 1) as usize],
}

/// Joystick events.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum JoystickEvent {
    Connected = ffi::CONNECTED,
    Disconnected = ffi::DISCONNECTED,
}

/// An joystick callback. This can be supplied with some user data to be passed
/// to the callback function when it is triggered.
pub type JoystickCallback<UserData> = Callback<fn(JoystickId, JoystickEvent, &UserData), UserData>;

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
            slice::from_raw_parts(ptr, count as usize)
                .iter()
                .map(|&a| a as f32)
                .collect()
        }
    }

    /// Wrapper for `glfwGetJoystickButtons`.
    pub fn get_buttons(&self) -> Vec<c_int> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetJoystickButtons(self.id as c_int, &mut count);
            slice::from_raw_parts(ptr, count as usize)
                .iter()
                .map(|&b| b as c_int)
                .collect()
        }
    }

    /// Wrapper for `glfwGetJoystickHats`.
    pub fn get_hats(&self) -> Vec<JoystickHats> {
        unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetJoystickHats(self.id as c_int, &mut count);
            slice::from_raw_parts(ptr, count as usize)
                .iter()
                .map(|&b| mem::transmute(b as c_int))
                .collect()
        }
    }

    /// Wrapper for `glfwGetJoystickName`.
    pub fn get_name(&self) -> Option<String> {
        unsafe { string_from_nullable_c_str(ffi::glfwGetJoystickName(self.id as c_int)) }
    }

    /// Wrapper for `glfwGetJoystickGUID`.
    pub fn get_guid(&self) -> Option<String> {
        unsafe { string_from_nullable_c_str(ffi::glfwGetJoystickGUID(self.id as c_int)) }
    }

    /// Wrapper for `glfwJoystickIsGamepad`.
    pub fn is_gamepad(&self) -> bool {
        unsafe { ffi::glfwJoystickIsGamepad(self.id as c_int) == ffi::TRUE }
    }

    /// Wrapper for `glfwGetGamepadName`.
    pub fn get_gamepad_name(&self) -> Option<String> {
        unsafe { string_from_nullable_c_str(ffi::glfwGetGamepadName(self.id as c_int)) }
    }

    /// Wrapper for `glfwGetGamepadState`.
    pub fn get_gamepad_state(&self) -> Option<GamepadState> {
        unsafe {
            let mut state = ffi::GLFWgamepadstate {
                buttons: [0; (ffi::GAMEPAD_BUTTON_LAST + 1) as usize],
                axes: [0_f32; (ffi::GAMEPAD_AXIS_LAST + 1) as usize],
            };
            if ffi::glfwGetGamepadState(self.id as c_int, &mut state) == ffi::TRUE {
                Some(state.into())
            } else {
                None
            }
        }
    }
}

impl From<ffi::GLFWgamepadstate> for GamepadState {
    fn from(state: ffi::GLFWgamepadstate) -> Self {
        let mut buttons = [Action::Release; (ffi::GAMEPAD_BUTTON_LAST + 1) as usize];
        let mut axes = [0_f32; (ffi::GAMEPAD_AXIS_LAST + 1) as usize];
        unsafe {
            state
                .buttons
                .iter()
                .map(|&b| mem::transmute(b as c_int))
                .zip(buttons.iter_mut())
                .for_each(|(a, b)| *b = a);
        }
        state
            .axes
            .iter()
            .map(|&f| f as f32)
            .zip(axes.iter_mut())
            .for_each(|(a, b)| *b = a);
        Self { buttons, axes }
    }
}

impl GamepadState {
    pub fn get_button_state(&self, button: GamepadButton) -> Action {
        self.buttons[button as usize]
    }

    pub fn get_axis(&self, axis: GamepadAxis) -> f32 {
        self.axes[axis as usize]
    }
}

#[inline(always)]
fn unwrap_dont_care(value: Option<u32>) -> c_int {
    match value {
        Some(v) => v as c_int,
        None => ffi::DONT_CARE,
    }
}
