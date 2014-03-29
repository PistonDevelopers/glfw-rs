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

use std::comm;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // GLFW must run on the main platform thread
    native::start(argc, argv, main)
}

fn main() {
    let errors = glfw::get_errors().unwrap();

    if glfw::init().is_err() {
        fail!(~"Failed to initialize GLFW");
    } else {
        let window = glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_context_current();

        while !window.should_close() {
            glfw::poll_events();
            match errors.try_recv() {
                comm::Data((_, _, description)) => {
                    fail!("GLFW Error: {}", description)
                },
                _ => {},
            }
            for (_, event) in window.flush_events() {
                handle_window_event(&window, event);
            }
        }
    }
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
