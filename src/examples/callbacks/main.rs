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
    do glfw::set_error_callback |_, msg| {
        println!("GLFW Error: {:s}", msg);
    }

    do glfw::start {
        glfw::window_hint::resizable(true);

        let window = glfw::Window::create(800, 600, "Hello, I am a window.", glfw::Windowed).unwrap();

        window.set_sticky_keys(true);

        // Register event callbacks

        window.set_pos_callback(window_pos_callback);
        window.set_size_callback(window_size_callback);
        window.set_close_callback(window_close_callback);
        window.set_refresh_callback(window_refresh_callback);
        window.set_focus_callback(window_focus_callback);
        window.set_iconify_callback(window_iconify_callback);
        window.set_framebuffer_size_callback(framebuffer_size_callback);

        window.set_key_callback(key_callback);
        window.set_char_callback(char_callback);
        window.set_mouse_button_callback(mouse_button_callback);
        window.set_cursor_pos_callback(cursor_pos_callback);
        window.set_cursor_enter_callback(cursor_enter_callback);
        window.set_scroll_callback(scroll_callback);

        window.make_context_current();

        while !window.should_close() {
            glfw::poll_events();
        }
    }
}

fn window_pos_callback(window: &glfw::Window, x: int, y: int) {
    window.set_title(format!("Window pos: ({}, {})", x, y));
}

fn window_size_callback(window: &glfw::Window, width: int, height: int) {
    window.set_title(format!("Window size: ({}, {})", width, height));
}

fn window_close_callback(_: &glfw::Window) {
    println("Window close requested.");
}

fn window_refresh_callback(_: &glfw::Window) {
    println("Window refresh callback triggered.");
}

fn window_focus_callback(_: &glfw::Window, activated: bool) {
    if activated { println("Window focus gained."); }
    else         { println("Window focus lost.");   }
}

fn window_iconify_callback(_: &glfw::Window, iconified: bool) {
    if iconified { println("Window was minimised");  }
    else         { println("Window was maximised."); }
}

fn framebuffer_size_callback(_: &glfw::Window, width: int, height: int) {
    println!("Framebuffer size: {} {}", width, height);
}

fn key_callback(window: &glfw::Window, key: glfw::Key, scancode: libc::c_int, action: glfw::Action, mods: glfw::Modifiers) {
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

fn char_callback(_: &glfw::Window, character: char) {
    println!("Character: {}", character);
}

fn mouse_button_callback(_: &glfw::Window, button: glfw::MouseButton, action: glfw::Action, mods: glfw::Modifiers) {
    println!("{} {:s}{:s}",
             button.to_str(),
             action.to_str(),
             match mods.to_str() {
                ~"" => ~"",
                s => format!(" with: {:s}", s),
             });
}

fn cursor_pos_callback(window: &glfw::Window, xpos: f64, ypos: f64) {
    window.set_title(format!("Cursor position: ({}, {})", xpos, ypos));
}

fn cursor_enter_callback(_: &glfw::Window, entered: bool) {
    if entered { println("Cursor entered window."); }
    else       { println("Cursor left window.");    }
}

fn scroll_callback(window: &glfw::Window, x: f64, y: f64) {
    window.set_title(format!("Scroll offset: ({}, {})", x, y));
}
