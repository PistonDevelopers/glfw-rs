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

use std::rt;

#[start]
fn main(argc: int, argv: **u8, crate_map: *u8) -> int {
    do rt::start_on_main_thread(argc, argv, crate_map) {
        do glfw::start() {
            do glfw::Monitor::get_primary().map |monitor| {
                    println(fmt!("%s:", monitor.get_name()));
                    println(fmt!("    %s\n", monitor.get_video_mode().get().to_str()));
            };

            println("Available monitors\n\
                         ------------------");
            do glfw::Monitor::get_connected().map |monitor| {
                println(fmt!("%s:", monitor.get_name()));

                do monitor.get_video_modes().map |mode| {
                    println(fmt!("  %s", mode.to_str()));
                }
            };
        }
    }
}
