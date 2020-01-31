// Copyright 2016 The GLFW-RS Developers. For a full listing of the authors,
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

// NOTE: Make sure to resize the OpenGL viewport (or whatever else) after any size changes.

extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(
            600,
            400,
            "Press F11 to toggle Fullscreen (it will be blank)",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    //Store fullscreen state
    let mut is_fullscreen = false;

    //Keep track of last position and size so we can restore the window to the originals
    let mut last_pos = (0, 0);
    let mut last_size = (0, 0);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                //F11 is pretty standard for fullscreen
                glfw::WindowEvent::Key(Key::F11, _, Action::Press, _) => {
                    if is_fullscreen {
                        window.set_monitor(
                            glfw::WindowMode::Windowed,
                            last_pos.0,
                            last_pos.1,
                            last_size.0 as u32,
                            last_size.1 as u32,
                            None,
                        );
                        println!(
                            "Window restored to {:?} at location {:?}",
                            last_size, last_pos
                        );
                    } else {
                        last_pos = window.get_pos();
                        last_size = window.get_size();

                        glfw.with_primary_monitor_mut(|_: &mut _, m: Option<&glfw::Monitor>| {
                            let monitor = m.unwrap();

                            let mode = monitor.get_video_mode().unwrap();

                            window.set_monitor(
                                glfw::WindowMode::FullScreen(&monitor),
                                0,
                                0,
                                mode.width,
                                mode.height,
                                Some(mode.refresh_rate),
                            );

                            println!(
                                "{}x{} fullscreen enabled at {}Hz on monitor {}",
                                mode.width,
                                mode.height,
                                mode.refresh_rate,
                                monitor.get_name().unwrap()
                            );
                        });
                    }

                    is_fullscreen = !is_fullscreen;
                }
                _ => {}
            }
        }
    }
}
