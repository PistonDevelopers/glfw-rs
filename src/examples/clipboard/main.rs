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
    glfw::set_error_callback(error_callback);

    do glfw::start {
        let window = glfw::Window::create(300, 300, "Clipboard Test", glfw::Windowed).unwrap();

        window.make_context_current();
        window.set_key_callback(key_callback);
        glfw::set_swap_interval(1);

        while !window.should_close() {
            glfw::poll_events();
        }
    }
}

#[cfg(target_os = "macos")]
static NATIVE_MOD: glfw::Modifier = glfw::Super;

#[cfg(not(target_os = "macos"))]
static NATIVE_MOD: glfw::Modifier = glfw::Control;

fn error_callback(_: glfw::Error, description: ~str) {
    println!("GLFW Error: {:s}", description);
}

fn key_callback(window: &glfw::Window, key: glfw::Key, _: libc::c_int, action: glfw::Action, mods: glfw::Modifiers) {
    if action == glfw::Press {
        if key == glfw::KeyEscape {
            window.set_should_close(true);
        }
        if (key == glfw::KeyV) && mods.contains(NATIVE_MOD) {
            match window.get_clipboard_string() {
                ref s if !s.is_empty() => println!("Clipboard contains \"{:s}\"", *s),
                _                      => println("Clipboard does not contain a string"),
            }
        }
        if (key == glfw::KeyC) && mods.contains(NATIVE_MOD) {
            let s = "Hello GLFW World!";
            window.set_clipboard_string(s);
            println!("Setting clipboard to {:s}", s);
        }
    }
}
