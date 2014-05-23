// Copyright 2014 The GLFW-RS Developers. For a full listing of the authors,
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

//! Demonstrates how custom error callbacks with user data can be created

#![feature(phase)]

extern crate native;
extern crate glfw;
#[phase(syntax, link)] extern crate log;

use std::cell::Cell;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let glfw = glfw::init(Some(
        glfw::Callback {
            f: error_callback,
            data: Cell::new(0),
        }
    )).unwrap();

    // Force the error callback to be triggered
    glfw.window_hint(glfw::ContextVersion(40000, 3000)); // Ridiculous!
    let _ = glfw.create_window(300, 300, "Hey this won't work.", glfw::Windowed);
    let _ = glfw.create_window(300, 300, "Nope, not working.",   glfw::Windowed);
    let _ = glfw.create_window(300, 300, "Stop it! :(",          glfw::Windowed);
}

fn error_callback(_: glfw::Error, description: StrBuf, error_count: &Cell<uint>) {
    error!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}
