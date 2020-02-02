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

use glfw::{Action, Key};
use std::sync::mpsc::Receiver;

type WindowInstance = (glfw::Window, Receiver<(f64, glfw::WindowEvent)>);
type WindowVector = Vec<WindowInstance>;

fn add_window(glfw: &glfw::Glfw, window_vector: &mut WindowVector) {
    let (mut window, events) = glfw
        .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window_vector.push((window, events));
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create two windows
    let mut windows = WindowVector::new();
    add_window(&glfw, &mut windows);
    add_window(&glfw, &mut windows);

    // Loop until we no longer have any open windows
    while !windows.is_empty() {
        // Wait for messages
        glfw.wait_events();

        // Process message queues for all windows
        for &mut (ref mut window, ref events) in &mut windows {
            for (_, event) in glfw::flush_messages(events) {
                handle_window_event(window, event);
            }
        }

        // Remove closed windows.
        windows.retain(|&(ref window, _)| !window.should_close());
    }
}
