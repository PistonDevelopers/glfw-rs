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

# Sources
LIB_SRC=$(shell find src/glfw -type f -name '*.rs')
EXAMPLES_SRC=$(shell find src/examples -type f -name '*.rs')

LINK_ARGS=$(shell ./etc/link_args.sh)

.SILENT:

.PHONY: all clean

all: .build_examples
	echo "Done!"

.build_lib: .setup_lib
	echo "Building glfw-rs..."
	rustc --out-dir lib src/glfw/lib.rs --link-args="${LINK_ARGS}"
	touch .build_lib

.setup_lib:
	mkdir -p lib
	touch .setup_lib

.build_examples: .build_lib .setup_examples ${EXAMPLES_SRC}
	echo "Building examples..."
	$(foreach file, ${EXAMPLES_SRC}, rustc --out-dir bin -L lib $(file) --link-args="${LINK_ARGS}";)
	touch .build_examples

.setup_examples:
	mkdir -p bin
	touch .setup_examples

clean:
	find . -type f -name '.build_*' | xargs rm -f
	rm -f lib/libglfw*
	echo "Cleaned"
