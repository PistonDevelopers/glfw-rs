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

extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (window, events) = glfw.create_window(300, 300, "Clipboard Test", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    glfw.set_swap_interval(1);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }
    }
}

#[cfg(target_os = "macos")]
static NATIVE_MOD: glfw::Modifiers = glfw::Super;

#[cfg(not(target_os = "macos"))]
static NATIVE_MOD: glfw::Modifiers = glfw::Control;

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(key, _, action, mods) => {
            if action == Action::Press {
                if key == Key::Escape {
                    window.set_should_close(true);
                }
                if (key == Key::V) && mods.contains(NATIVE_MOD) {
                    match window.get_clipboard_string() {
                        ref s if !s.is_empty() => println!("Clipboard contains \"{}\"", *s),
                        _                      => println!("Clipboard does not contain a string"),
                    }
                }
                if (key == Key::C) && mods.contains(NATIVE_MOD) {
                    let s = "Hello GLFW World!";
                    window.set_clipboard_string(s);
                    println!("Setting clipboard to {}", s);
                }
            }
        }
        _ => {}
    }
}
