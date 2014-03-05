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

emit() {
    echo "#[link(name=\"$1\")] extern { }"
}

UNAME=$(uname)
case $UNAME in
    "Linux" | "FreeBSD" | "OpenBSD" | "Darwin")
        for lib in $(pkg-config --static --libs-only-l --libs-only-other glfw3); do
            emit $(echo $lib | cut -c3-)
        done
    ;;
    *)
        case $(uname -o) in
            "Cygwin" | "Msys")
                emit glfw3
                emit opengl32
                emit gdi32
            ;;
            *)
                echo "Unsuppported platform: $UNAME";
                exit 1
            ;;
        esac
    ;;
esac
