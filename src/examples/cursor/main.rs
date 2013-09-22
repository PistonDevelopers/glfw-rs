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

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    do glfw::set_error_callback |_, description| {
        println!("GLFW Error: {:s}", description);
    }

    do glfw::start {
        let window = glfw::Window::create(800, 600, "Hello, I am a window.", glfw::Windowed).unwrap();

        window.set_cursor_mode(glfw::CursorDisabled);
        window.make_context_current();

        do window.set_cursor_pos_callback |_, xpos, ypos| {
            println!("Cursor position: ({}, {})", xpos, ypos);
        }

        do window.set_key_callback |window, key, _, action, _| {
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

        while !window.should_close() {
            glfw::poll_events();
        }
    }
}
