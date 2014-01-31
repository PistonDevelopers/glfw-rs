# Copyright 2013 The GLFW-RS Developers. For a full listing of the authors,
# refer to the AUTHORS file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

all: examples

tools:
	mkdir -p build build/etc
	rustc --out-dir=build/etc src/etc/link-args.rs

lib: tools
	mkdir -p build/lib
	rustc --out-dir=build/lib --link-args="`./build/etc/link-args`" -O src/lib/lib.rs

examples: lib
	mkdir -p build/examples
	rustc --out-dir=build/examples -L ./build/lib --link-args="`./build/etc/link-args`" src/examples/callbacks.rs
	rustc --out-dir=build/examples -L ./build/lib --link-args="`./build/etc/link-args`" src/examples/clipboard.rs
	rustc --out-dir=build/examples -L ./build/lib --link-args="`./build/etc/link-args`" src/examples/cursor.rs
	rustc --out-dir=build/examples -L ./build/lib --link-args="`./build/etc/link-args`" src/examples/defaults.rs
	rustc --out-dir=build/examples -L ./build/lib --link-args="`./build/etc/link-args`" src/examples/manual_init.rs
	rustc --out-dir=build/examples -L ./build/lib --link-args="`./build/etc/link-args`" src/examples/modes.rs
	rustc --out-dir=build/examples -L ./build/lib --link-args="`./build/etc/link-args`" src/examples/title.rs
	rustc --out-dir=build/examples -L ./build/lib --link-args="`./build/etc/link-args`" src/examples/version.rs
	rustc --out-dir=build/examples -L ./build/lib --link-args="`./build/etc/link-args`" src/examples/window.rs

clean:
	rm -rfv ./build
