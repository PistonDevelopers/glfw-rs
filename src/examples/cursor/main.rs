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

#[feature(link_args)];

extern mod glfw;

use std::libc;

#[link(name="glfw")]
extern {}

#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
   glfw::set_error_callback(~ErrorContext);

    do glfw::start {
        let window = glfw::Window::create(800, 600, "Hello, I am a window.", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_cursor_mode(glfw::CursorDisabled);
        window.make_context_current();

        window.set_cursor_pos_callback(~CursorPosContext);
        window.set_key_callback(~KeyContext);

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

struct CursorPosContext;
impl glfw::CursorPosCallback for CursorPosContext {
    fn call(&self, _: &glfw::Window, xpos: f64, ypos: f64) {
        println!("Cursor position: ({}, {})", xpos, ypos);
    }
}

struct KeyContext;
impl glfw::KeyCallback for KeyContext {
    fn call(&self, window: &glfw::Window, key: glfw::Key, _: libc::c_int, action: glfw::Action, _: glfw::Modifiers) {
        match (action, key) {
            (glfw::Press, glfw::KeyEscape) => window.set_should_close(true),
            (glfw::Press, glfw::KeySpace) => {
                match window.get_cursor_mode() {
                    glfw::CursorDisabled => window.set_cursor_mode(glfw::CursorNormal),
                    glfw::CursorNormal   => window.set_cursor_mode(glfw::CursorDisabled),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
