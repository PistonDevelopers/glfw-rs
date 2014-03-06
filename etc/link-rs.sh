#!/bin/sh

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

ARGS=$1
FRAMEWORK=false

for arg in $ARGS; do
    if [ "$FRAMEWORK" = true ]; then
        echo "#[link(name = \"$arg\", kind = \"framework\")]"
        FRAMEWORK=false
    elif [ "$arg" = "-framework" ]; then
        FRAMEWORK=true
    else
        echo "#[link(name = \"$(echo $arg | cut -c3-)\")]"
    fi
done

echo "extern {}"
