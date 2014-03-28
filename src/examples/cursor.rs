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
   glfw::set_error_callback(ERROR_CONTEXT);

    glfw::start(proc() {
        let window = glfw::Window::create(800, 600, "Hello, I am a window.", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_cursor_mode(glfw::CursorDisabled);
        window.make_context_current();

        window.set_cursor_pos_polling(true);
        window.set_key_polling(true);

        while !window.should_close() {
            glfw::poll_events();
            for (_, event) in window.flush_events() {
                handle_window_event(&window, event);
            }
        }
    });
}

struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}
static ERROR_CONTEXT: &'static ErrorContext = &'static ErrorContext;

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::CursorPosEvent(xpos, ypos) => println!("Cursor position: ({}, {})", xpos, ypos),
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => window.set_should_close(true),
        glfw::KeyEvent(glfw::KeySpace, _, glfw::Press, _) => {
            match window.get_cursor_mode() {
                glfw::CursorDisabled => window.set_cursor_mode(glfw::CursorNormal),
                glfw::CursorNormal   => window.set_cursor_mode(glfw::CursorDisabled),
                _ => {}
            }
        }
        _ => {}
    }
}
