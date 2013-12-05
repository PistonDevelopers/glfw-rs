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

#[feature(link_args)];

extern mod glfw;

#[link(name="glfw")]
extern {}

#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    do glfw::start {
        glfw::Monitor::get_primary().map(|monitor| {
                println!("{:s}:", monitor.get_name());
                println!("    {:s}\n", monitor.get_video_mode().unwrap().to_str());
        });

        println("Available monitors\n\
                     ------------------");
        glfw::Monitor::get_connected().map(|monitor| {
            println!("{:s}:", monitor.get_name());

            monitor.get_video_modes().map(|mode| {
                println!("  {:s}", mode.to_str());
            });
        });
    }
}
