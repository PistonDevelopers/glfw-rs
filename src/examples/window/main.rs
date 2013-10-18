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
        let window = glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_callback(key_callback);
        window.make_context_current();

        while !window.should_close() {
            glfw::poll_events();
        }
    }
}

fn key_callback(window: &glfw::Window, key: glfw::Key, _: libc::c_int, action: glfw::Action, _: glfw::Modifiers) {
    if action == glfw::Press && key == glfw::KeyEscape {
        window.set_should_close(true);
    }
}
