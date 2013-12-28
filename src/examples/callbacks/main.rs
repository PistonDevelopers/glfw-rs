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

extern mod glfw;

use std::libc;

#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    glfw::set_error_callback(~ErrorContext);

    do glfw::start {
        glfw::window_hint::resizable(true);

        let window = glfw::Window::create(800, 600, "Hello, I am a window.", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_sticky_keys(true);

        // Register event callbacks

        window.set_pos_callback(~WindowPosContext);
        window.set_size_callback(~WindowSizeContext);
        window.set_close_callback(~WindowCloseContext);
        window.set_refresh_callback(~WindowRefreshContext);
        window.set_focus_callback(~WindowFocusContext);
        window.set_iconify_callback(~WindowIconifyContext);
        window.set_framebuffer_size_callback(~FramebufferSizeContext);

        window.set_key_callback(~KeyContext);
        window.set_char_callback(~CharContext);
        window.set_mouse_button_callback(~MouseButtonContext);
        window.set_cursor_pos_callback(~CursorPosContext);
        window.set_cursor_enter_callback(~CursorEnterContext);
        window.set_scroll_callback(~ScrollContext);

        window.make_context_current();

        while !window.should_close() {
            glfw::poll_events();
        }
    }
}

struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}

struct WindowPosContext;
impl glfw::WindowPosCallback for WindowPosContext {
    fn call(&self, window: &glfw::Window, x: i32, y: i32) {
        window.set_title(format!("Window pos: ({}, {})", x, y));
    }
}

struct WindowSizeContext;
impl glfw::WindowSizeCallback for WindowSizeContext {
    fn call(&self, window: &glfw::Window, width: i32, height: i32) {
        window.set_title(format!("Window size: ({}, {})", width, height));
    }
}

struct WindowCloseContext;
impl glfw::WindowCloseCallback for WindowCloseContext {
    fn call(&self, _: &glfw::Window) {
        println("Window close requested.");
    }
}

struct WindowRefreshContext;
impl glfw::WindowRefreshCallback for WindowRefreshContext {
    fn call(&self, _: &glfw::Window) {
        println("Window refresh callback triggered.");
    }
}

struct WindowFocusContext;
impl glfw::WindowFocusCallback for WindowFocusContext {
    fn call(&self, _: &glfw::Window, activated: bool) {
        if activated { println("Window focus gained."); }
        else         { println("Window focus lost.");   }
    }
}

struct WindowIconifyContext;
impl glfw::WindowIconifyCallback for WindowIconifyContext {
    fn call(&self, _: &glfw::Window, iconified: bool) {
        if iconified { println("Window was minimised");  }
        else         { println("Window was maximised."); }
    }
}

struct FramebufferSizeContext;
impl glfw::FramebufferSizeCallback for FramebufferSizeContext {
    fn call(&self, _: &glfw::Window, width: i32, height: i32) {
        println!("Framebuffer size: {} {}", width, height);
    }
}

struct KeyContext;
impl glfw::KeyCallback for KeyContext {
    fn call(&self, window: &glfw::Window, key: glfw::Key, scancode: libc::c_int, action: glfw::Action, mods: glfw::Modifiers) {
        println!("{} {:s}{:s} (scan code : {})",
                 key.to_str(),
                 action.to_str(),
                 match mods.to_str() {
                    ~"" => ~"",
                    s => format!(" with: {:s}", s),
                 },
                 scancode);

        match (key, action) {
            (glfw::KeyEscape, glfw::Press) => {
                window.set_should_close(true);
            }

            (glfw::KeyR, glfw::Press) => {
                // Resize should cause the window to "refresh"
                let (window_width, window_height) = window.get_size();
                window.set_size(window_width + 1, window_height);
                window.set_size(window_width, window_height);
            }

            _ => ()
        }
    }
}

struct CharContext;
impl glfw::CharCallback for CharContext {
    fn call(&self, _: &glfw::Window, character: char) {
        println!("Character: {}", character);
    }
}

struct MouseButtonContext;
impl glfw::MouseButtonCallback for MouseButtonContext {
    fn call(&self, _: &glfw::Window, button: glfw::MouseButton, action: glfw::Action, mods: glfw::Modifiers) {
        println!("{} {:s}{:s}",
                 button.to_str(),
                 action.to_str(),
                 match mods.to_str() {
                    ~"" => ~"",
                    s => format!(" with: {:s}", s),
                 });
    }
}

struct CursorPosContext;
impl glfw::CursorPosCallback for CursorPosContext {
    fn call(&self, window: &glfw::Window, xpos: f64, ypos: f64) {
        window.set_title(format!("Cursor position: ({}, {})", xpos, ypos));
    }
}

struct CursorEnterContext;
impl glfw::CursorEnterCallback for CursorEnterContext {
    fn call(&self, _: &glfw::Window, entered: bool) {
        if entered { println("Cursor entered window."); }
        else       { println("Cursor left window.");    }
    }
}

struct ScrollContext;
impl glfw::ScrollCallback for ScrollContext {
    fn call(&self, window: &glfw::Window, x: f64, y: f64) {
        window.set_title(format!("Scroll offset: ({}, {})", x, y));
    }
}
