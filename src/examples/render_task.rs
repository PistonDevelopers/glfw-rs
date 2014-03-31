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

extern crate native;
extern crate glfw;

use glfw::Context;
use std::task::task;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let (glfw, errors) = glfw::init().unwrap();
    glfw::fail_on_error(&errors);

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    let render_context = window.render_context().unwrap();
    let (send, recv) = channel();

    let mut render_task = task().named("render task");
    let render_task_done = render_task.future_result();
    render_task.spawn(proc() { render_main(render_context, recv) });

    while !window.should_close() {
        glfw.poll_events();
        glfw::fail_on_error(&errors);
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }
    }

    // tell the render task to exit.
    send.send(());

    // wait for ack
    let _ = render_task_done.recv();
}

fn render_main(context: glfw::RenderContext, info: Receiver<()>)
{
    context.make_context_current();
    loop {
        // are we done?
        if info.try_recv() == std::comm::Data(()) {
            break;
        }

        // do gl calls here

        context.swap_buffers();
    }

    // required on some platforms
    glfw::make_context_current(None);
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
