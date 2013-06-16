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

TARGET = glfw

ROOT_DIR = .

SRC_DIR        = $(ROOT_DIR)/src
SRC_CRATE      = $(TARGET).rs
BUILD_DIR      = $(ROOT_DIR)/lib

$(TARGET):
	@echo "Building $(TARGET)"
	@mkdir -p $(BUILD_DIR)
	@rustc $(SRC_DIR)/$(SRC_CRATE) --lib --out-dir=$(BUILD_DIR) -L ../glfw/src
	@echo "Success! \o/"

all: $(TARGET)

clean:
	rm -R -f $(BUILD_DIR)
