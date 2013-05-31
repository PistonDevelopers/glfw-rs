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

fn main() {
    glfw::set_error_callback(error_callback);

    do glfw::spawn {
        let window = glfw::Window::create(300, 300, "Clipboard Test", glfw::Windowed).unwrap();

        window.make_context_current();
        window.set_key_callback(key_callback);
        glfw::set_swap_interval(1);

        while !window.should_close() {
            glfw::wait_events();
        }
    }
}

#[cfg(target_os = "macos")]
static NATIVE_MOD: libc::c_int = glfw::MOD_SUPER;

#[cfg(not(target_os = "macos"))]
static NATIVE_MOD: libc::c_int = glfw::MOD_CONTROL;

fn error_callback(_: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int, mods: libc::c_int) {
    if action == glfw::PRESS {
        if key == glfw::KEY_ESCAPE {
            window.set_should_close(true);
        }
        if (key == glfw::KEY_V) && (mods & NATIVE_MOD > 0) {
            match window.get_clipboard_string() {
                ref s if !s.is_empty() => io::println(fmt!("Clipboard contains %?", s)),
                _                      => io::println("Clipboard does not contain a string"),
            }
        }
        if (key == glfw::KEY_C) && (mods & NATIVE_MOD > 0) {
            let s = "Hello GLFW World!";
            window.set_clipboard_string(s);
            io::println(fmt!("Setting clipboard to %?", s));
        }
    }
}
