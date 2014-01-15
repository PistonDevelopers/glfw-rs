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

cmake_minimum_required(VERSION 2.8)
project(GLFW_RS NONE)

option(GLFW_RS_BUILD_EXAMPLES "Build the example programs" ON)
set(GLFW_RS_LIB_DIR ${PROJECT_BINARY_DIR}/lib
    CACHE "Directory in which to build the library" PATH)
set(GLFW_RS_EXAMPLE_DIR ${PROJECT_BINARY_DIR}/bin
    CACHE "Directory in which to build the examples" PATH)

find_program(RUSTC_COMPILER rustc)

#-------------------------------------------------------------------------------
# GLFW Linking
#-------------------------------------------------------------------------------
# See http://www.glfw.org/docs/latest/build.html#build_link_cmake_pkgconfig

if(WIN32)
    # Emulate pkg-config behaviour for windows
    # set(GLFW_FOUND 1) - should check if the libs can actually be found
    set(GLFW_LIBRARIES glfw3dll)
    set(GLFW_LDFLAGS -lglfw3dll)
    set(GLFW_STATIC_LIBRARIES glfw3)
    set(GLFW_STATIC_LDFLAGS -lglfw3;-lopengl32;-lgdi32)
else()
    include(FindPkgConfig)
    pkg_search_module(GLFW REQUIRED glfw3)
endif()

#-------------------------------------------------------------------------------
# Add subdirectories
#-------------------------------------------------------------------------------

add_subdirectory(src)

if(GLFW_RS_BUILD_EXAMPLES)
    add_subdirectory(examples)
endif()

#-------------------------------------------------------------------------------
# Add install target
#-------------------------------------------------------------------------------

file(GLOB GLFW_RS_INSTALL_FILES "${GLFW_RS_LIB_DIR}/*")
install(FILES ${GLFW_RS_INSTALL_FILES} DESTINATION lib)
