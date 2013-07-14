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

RS_FLAGS	:= -L ../lib
# RS_FLAGS	:= -g -L ../lib 		# causes "error: internal compiler error: create_function: unexpected sort of node"
BUILD_DIR	:= bin


all: callbacks clipboard cursor defaults gl-struct manual-init modes title version window

builddir:
	@mkdir -p $(BUILD_DIR)

callbacks: builddir
	rustc callbacks.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

clipboard: builddir
	rustc clipboard.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

cursor: builddir
	rustc cursor.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

defaults: builddir
	rustc defaults.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

gl-struct: builddir
	rustc gl-struct.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

manual-init: builddir
	rustc manual-init.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

modes: builddir
	rustc modes.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

title: builddir
	rustc title.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

version: builddir
	rustc version.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

window: builddir
	rustc window.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

clean:
	rm -R -f $(BUILD_DIR)
