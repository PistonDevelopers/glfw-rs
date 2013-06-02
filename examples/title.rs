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

fn main() {
    glfw::set_error_callback(error_callback);

    do glfw::spawn {
        let window = glfw::Window::create(400, 400, "English 日本語 русский язык 官話", glfw::Windowed).unwrap();

        window.make_context_current();
        window.set_key_callback(key_callback);
        glfw::set_swap_interval(1);

        while !window.should_close() {
            glfw::wait_events();
        }
    }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int, _: libc::c_int) {
    if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
        window.set_should_close(true);
    }
}

fn error_callback(_: libc::c_int, description: ~str) {
    println(fmt!("GLFW Error: %s", description));
}
