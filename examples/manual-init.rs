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
use std::rt;
use std::unstable::finally::Finally;

#[start]
fn main(argc: int, argv: **u8, crate_map: *u8) -> int {
    // GLFW must run on the main platform thread
    do rt::start_on_main_thread(argc, argv, crate_map) {
        glfw::set_error_callback(error_callback);

        if glfw::init().is_err() {
            fail!(~"Failed to initialize GLFW");
        } else {
            (||{
                let window = glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed).unwrap();

                window.set_key_callback(key_callback);
                window.make_context_current();

                while !window.should_close() {
                    glfw::poll_events();
                }
            // Use `finally` to ensure that `glfw::terminate` is called even if a failure occurs
            }).finally(glfw::terminate);
        }
    }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, _: libc::c_int, action: libc::c_int, _: libc::c_int) {
    if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
        window.set_should_close(true);
    }
}

fn error_callback(_: libc::c_int, description: ~str) {
    println(fmt!("GLFW Error: %s", description));
}
