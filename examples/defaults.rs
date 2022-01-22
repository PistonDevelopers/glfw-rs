// Copyright 2013 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate glfw;

use glfw::Context;

#[cfg(feature = "vulkan-common")]
fn vulkan_support(glfw: &mut glfw::Glfw) {
    println!("Vulkan supported: {:?}", glfw.vulkan_supported());
    println!(
        "Vulkan required extensions: {:?}",
        glfw.get_required_instance_extensions().unwrap_or(vec![])
    );
}

#[cfg(not(feature = "vulkan"))]
fn vulkan_support(_: &mut glfw::Glfw) {}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Visible(true));

    let (mut window, _) = glfw
        .create_window(640, 480, "Defaults", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();

    let (width, height) = window.get_size();
    println!("window size: ({:?}, {:?})", width, height);

    println!("Context version: {:?}", window.get_context_version());
    println!(
        "OpenGL forward compatible: {:?}",
        window.is_opengl_forward_compat()
    );
    println!(
        "OpenGL debug context: {:?}",
        window.is_opengl_debug_context()
    );
    println!("OpenGL profile: {:?}", window.get_opengl_profile());

    vulkan_support(&mut glfw);

    let gl_params = [
        (gl::RED_BITS, None, "red bits"),
        (gl::GREEN_BITS, None, "green bits"),
        (gl::BLUE_BITS, None, "blue bits"),
        (gl::ALPHA_BITS, None, "alpha bits"),
        (gl::DEPTH_BITS, None, "depth bits"),
        (gl::STENCIL_BITS, None, "stencil bits"),
        (gl::ACCUM_RED_BITS, None, "accum red bits"),
        (gl::ACCUM_GREEN_BITS, None, "accum green bits"),
        (gl::ACCUM_BLUE_BITS, None, "accum blue bits"),
        (gl::ACCUM_ALPHA_BITS, None, "accum alpha bits"),
        (gl::STEREO, None, "stereo"),
        (gl::SAMPLES_ARB, Some("GL_ARB_multisample"), "FSAA samples"),
    ];

    for &(param, ext, name) in gl_params.iter() {
        if ext.map_or(true, |s| glfw.extension_supported(s)) {
            let value = 0;
            unsafe { gl::GetIntegerv(param, &value) };
            println!("OpenGL {:?}: {:?}", name, value);
        };
    }
}

mod gl {
    #[cfg(target_os = "macos")]
    #[link(name = "OpenGL", kind = "framework")]
    extern "C" {}

    #[cfg(target_os = "linux")]
    #[link(name = "GL")]
    extern "C" {}

    pub type GLenum = std::os::raw::c_uint;
    pub type GLint = std::os::raw::c_int;

    pub static RED_BITS: GLenum = 0x0D52;
    pub static GREEN_BITS: GLenum = 0x0D53;
    pub static BLUE_BITS: GLenum = 0x0D54;
    pub static ALPHA_BITS: GLenum = 0x0D55;
    pub static DEPTH_BITS: GLenum = 0x0D56;
    pub static STENCIL_BITS: GLenum = 0x0D57;
    pub static ACCUM_RED_BITS: GLenum = 0x0D58;
    pub static ACCUM_GREEN_BITS: GLenum = 0x0D59;
    pub static ACCUM_BLUE_BITS: GLenum = 0x0D5A;
    pub static ACCUM_ALPHA_BITS: GLenum = 0x0D5B;
    pub static STEREO: GLenum = 0x0C33;
    pub static SAMPLES_ARB: GLenum = 0x80A9;

    #[inline(never)]
    #[allow(non_snake_case)]
    pub unsafe fn GetIntegerv(pname: GLenum, params: *const GLint) {
        glGetIntegerv(pname, params)
    }

    extern "C" {
        fn glGetIntegerv(pname: GLenum, params: *const GLint);
    }
}
