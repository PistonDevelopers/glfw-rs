# Copyright 2013-2014 The GLFW-RS Developers. For a full listing of the authors,
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

RUSTC               ?= rustc
RUSTDOC             ?= rustdoc

SRC_DIR             = src
LIB_FILE            = $(SRC_DIR)/lib/lib.rs
EXAMPLE_FILES       = $(SRC_DIR)/examples/*.rs
TEST_FILES          = $(SRC_DIR)/tests/*.rs

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(LIB_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(LIB_FILE))

DEPS_DIR            = deps
GLFW_BUILD_DIR      = $(DEPS_DIR)/glfw-build
GLFW_BUILD_SRC_DIR  = $(GLFW_BUILD_DIR)/src

DOC_DIR             = doc
EXAMPLES_DIR        = examples
LIB_DIR             = lib
TESTS_DIR           = tests

all: lib examples doc

deps:
	mkdir -p $(GLFW_BUILD_DIR)
	cd $(GLFW_BUILD_DIR) && \
	cmake -DGLFW_BUILD_EXAMPLES=OFF -DGLFW_BUILD_TESTS=OFF -DGLFW_BUILD_DOCS=OFF -Wno-dev ../glfw
	make -C $(GLFW_BUILD_DIR)

link: deps
	sh etc/link-rs.sh "$(shell sh etc/glfw-link-args.sh $(GLFW_BUILD_SRC_DIR))" > $(SRC_DIR)/lib/link.rs

lib: deps link
	mkdir -p $(LIB_DIR)
	$(RUSTC) -L $(GLFW_BUILD_SRC_DIR) --out-dir=$(LIB_DIR) -O $(LIB_FILE)

doc: link
	mkdir -p $(DOC_DIR)
	$(RUSTDOC) -o $(DOC_DIR) $(LIB_FILE)

tests-dir:
	mkdir -p $(TESTS_DIR)

$(TEST_FILES): lib tests-dir
	@ $(RUSTC) -L $(LIB_DIR) --out-dir=$(TESTS_DIR) $@
	@ echo "testing $@" && ./$(TESTS_DIR)/$(shell $(RUSTC) --crate-name $@)

check: $(TEST_FILES)
	@ echo "tests passed"

examples-dir:
	mkdir -p $(EXAMPLES_DIR)

$(EXAMPLE_FILES): lib examples-dir
	$(RUSTC) -L $(LIB_DIR) --out-dir=$(EXAMPLES_DIR) $@

examples: $(EXAMPLE_FILES)

clean:
	rm -rf $(LIB_DIR)
	rm -rf $(EXAMPLES_DIR)
	rm -rf $(DOC_DIR)
	rm -rf $(TESTS_DIR)
	rm -rf $(GLFW_BUILD_DIR)
	rm -f $(SRC_DIR)/lib/link.rs

.PHONY: \
	all \
	deps \
	link \
	lib \
	doc \
	tests-dir \
	$(TEST_FILES) \
	check \
	examples \
	examples-dir \
	$(EXAMPLE_FILES) \
	clean
