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
    set(GLFW_STATIC_LDFLAGS -lglfw3;-lopengl32)
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
