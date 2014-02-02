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

//! Outputs the correct link args for the platform.
//!
//! Requires pkg-config on non-windows platforms
//!
//! See http://www.glfw.org/docs/latest/build.html

#[cfg(target_os = "win32")]
fn main() {
    println!("-lglfw3 -lopengl32 -lgdi32");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("-lglfw3 -framework Cocoa -framework OpenGL -framework IOKit -framework CoreFoundation");
}

#[cfg(not(target_os = "win32"), not(target_os = "macos"))]
fn main() {
    use std::io::{IoError, FileNotFound, io_error};
    use std::run::{Process, ProcessOptions};

    io_error::cond.trap(|e| {
        match e {
            IoError { kind: FileNotFound, .. } => fail!("Failed to locate pkg-config in current environment."),
            e => fail!("Unexpected error when locating locate pkg-config: {}", e.to_str()),
        }
    }).inside(|| {
        println!("{}", Process::new("pkg-config", [~"--static", ~"--libs", ~"glfw3"], ProcessOptions::new())
            .expect("Failed to run pkg-config.")
            .output().read_to_str());
    });
}
