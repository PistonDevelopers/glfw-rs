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

RUSTC               = rustc
RUSTDOC             = rustdoc

GLFW_LIB_DIR        ?=
LINK_ARGS           = $(shell sh etc/glfw-link-args.sh)

SRC_DIR             = src
LIB_FILE            = $(SRC_DIR)/glfw.rs
EXAMPLE_FILES       = $(SRC_DIR)/examples/*.rs

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(LIB_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(LIB_FILE))

DOC_DIR             = doc
EXAMPLES_DIR        = examples
LIB_DIR             = lib

all: lib examples doc

link:
	sh etc/link-rs.sh "$(LINK_ARGS)" > $(SRC_DIR)/link.rs

lib: link
	mkdir -p $(LIB_DIR)
	$(RUSTC) -C extra-filename=-rs $(if $(GLFW_LIB_DIR),-L $(GLFW_LIB_DIR)) --out-dir=$(LIB_DIR) -O $(LIB_FILE)

doc: link
	mkdir -p $(DOC_DIR)
	$(RUSTDOC) -o $(DOC_DIR) $(LIB_FILE)

examples-dir:
	mkdir -p $(EXAMPLES_DIR)

$(EXAMPLE_FILES): lib examples-dir
	$(RUSTC) -L $(LIB_DIR) --out-dir=$(EXAMPLES_DIR) $@

examples: $(EXAMPLE_FILES)

clean:
	rm -rf $(LIB_DIR)
	rm -rf $(EXAMPLES_DIR)
	rm -rf $(DOC_DIR)
	rm -f $(SRC_DIR)/lib/link.rs

.PHONY: \
	all \
	link \
	lib \
	doc \
	examples \
	examples-dir \
	$(EXAMPLE_FILES) \
	clean
