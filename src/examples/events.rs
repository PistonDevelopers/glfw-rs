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

extern crate native;
extern crate glfw;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let (glfw, errors) = glfw::init().unwrap();
    glfw::fail_on_error(&errors);

    glfw.window_hint(glfw::Resizable(true));

    let (window, events) = glfw.create_window(800, 600, "Hello, I am a window.", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_sticky_keys(true);

    // Polling of events can be turned on and off by the specific event type
    window.set_pos_polling(true);
    window.set_all_polling(true);
    window.set_size_polling(true);
    window.set_close_polling(true);
    window.set_refresh_polling(true);
    window.set_focus_polling(true);
    window.set_iconify_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);
    window.set_char_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_enter_polling(true);
    window.set_scroll_polling(true);

    // Alternatively, all event types may be set to poll at once. Note that
    // in this example, this call is redundant as all events have been set
    // to poll in the above code.
    window.set_all_polling(true);

    window.make_context_current();

    while !window.should_close() {
        glfw.poll_events();
        glfw::fail_on_error(&errors);
        for event in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }
    }
}

fn handle_window_event(window: &glfw::Window, (time, event): (f64, glfw::WindowEvent)) {
    match event {
        glfw::PosEvent(x, y)                => window.set_title(format!("Time: {}, Window pos: ({}, {})", time, x, y)),
        glfw::SizeEvent(w, h)               => window.set_title(format!("Time: {}, Window size: ({}, {})", time, w, h)),
        glfw::CloseEvent                    => println!("Time: {}, Window close requested.", time),
        glfw::RefreshEvent                  => println!("Time: {}, Window refresh callback triggered.", time),
        glfw::FocusEvent(true)              => println!("Time: {}, Window focus gained.", time),
        glfw::FocusEvent(false)             => println!("Time: {}, Window focus lost.", time),
        glfw::IconifyEvent(true)            => println!("Time: {}, Window was minimised", time),
        glfw::IconifyEvent(false)           => println!("Time: {}, Window was maximised.", time),
        glfw::FramebufferSizeEvent(w, h)    => println!("Time: {}, Framebuffer size: ({}, {})", time, w, h),
        glfw::CharEvent(character)          => println!("Time: {}, Character: {}", time, character),
        glfw::MouseButtonEvent(btn, action, mods) => println!("Time: {}, Button: {}, Action: {}, Modifiers: [{}]", time, glfw::ShowAliases(btn), action, mods),
        glfw::CursorPosEvent(xpos, ypos)    => window.set_title(format!("Time: {}, Cursor position: ({}, {})", time, xpos, ypos)),
        glfw::CursorEnterEvent(true)        => println!("Time: {}, Cursor entered window.", time),
        glfw::CursorEnterEvent(false)       => println!("Time: {}, Cursor left window.", time),
        glfw::ScrollEvent(x, y)             => window.set_title(format!("Time: {}, Scroll offset: ({}, {})", time, x, y)),
        glfw::KeyEvent(key, scancode, action, mods) => {
            println!("Time: {}, Key: {}, ScanCode: {}, Action: {}, Modifiers: [{}]", time, key, scancode, action, mods);
            match (key, action) {
                (glfw::KeyEscape, glfw::Press) => window.set_should_close(true),
                (glfw::KeyR, glfw::Press) => {
                    // Resize should cause the window to "refresh"
                    let (window_width, window_height) = window.get_size();
                    window.set_size(window_width + 1, window_height);
                    window.set_size(window_width, window_height);
                }
                _ => {}
            }
        }
    }
}
