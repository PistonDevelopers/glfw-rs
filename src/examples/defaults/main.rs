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

#[feature(link_args)];

extern mod glfw;

#[link(name="glfw")]
extern {}

#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    glfw::set_error_callback(~ErrorContext);

    do glfw::start {
        glfw::window_hint::visible(true);

        let window = glfw::Window::create(640, 480, "Defaults", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_context_current();

        let (width, height) = window.get_size();
        println!("window size: ({}, {})", width, height);

        println!("Context version: {:s}",         window.get_context_version().to_str());
        println!("OpenGL forward compatible: {}", window.is_opengl_forward_compat());
        println!("OpenGL debug context: {}",      window.is_opengl_debug_context());
        println!("OpenGL profile: {}",            window.get_opengl_profile());

        let gl_params = [
            (gl::RED_BITS,          None,   "red bits"          ),
            (gl::GREEN_BITS,        None,   "green bits"        ),
            (gl::BLUE_BITS,         None,   "blue bits"         ),
            (gl::ALPHA_BITS,        None,   "alpha bits"        ),
            (gl::DEPTH_BITS,        None,   "depth bits"        ),
            (gl::STENCIL_BITS,      None,   "stencil bits"      ),
            (gl::ACCUM_RED_BITS,    None,   "accum red bits"    ),
            (gl::ACCUM_GREEN_BITS,  None,   "accum green bits"  ),
            (gl::ACCUM_BLUE_BITS,   None,   "accum blue bits"   ),
            (gl::ACCUM_ALPHA_BITS,  None,   "accum alpha bits"  ),
            (gl::STEREO,            None,   "stereo"            ),
            (gl::SAMPLES_ARB,       Some("GL_ARB_multisample"), "FSAA samples" ),
        ];

        for &(param, ext, name) in gl_params.iter() {
            if ext.map_default(true, |s| {
                glfw::extension_supported(s)
            }) {
                let value = 0;
                unsafe { gl::GetIntegerv(param, &value) };
                println!("OpenGL {:s}: {}", name, value);
            };
        }
    }
}

struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}

mod gl {
    use std::libc;

    #[nolink]
    #[link_args="-framework OpenGL"]
    #[cfg(target_os = "macos")]
    extern { }

    #[nolink]
    #[link_args="-lGL"]
    #[cfg(target_os = "linux")]
    extern { }

    pub type GLenum = libc::c_uint;
    pub type GLint  = libc::c_int;

    pub static RED_BITS              : GLenum = 0x0D52;
    pub static GREEN_BITS            : GLenum = 0x0D53;
    pub static BLUE_BITS             : GLenum = 0x0D54;
    pub static ALPHA_BITS            : GLenum = 0x0D55;
    pub static DEPTH_BITS            : GLenum = 0x0D56;
    pub static STENCIL_BITS          : GLenum = 0x0D57;
    pub static ACCUM_RED_BITS        : GLenum = 0x0D58;
    pub static ACCUM_GREEN_BITS      : GLenum = 0x0D59;
    pub static ACCUM_BLUE_BITS       : GLenum = 0x0D5A;
    pub static ACCUM_ALPHA_BITS      : GLenum = 0x0D5B;
    pub static STEREO                : GLenum = 0x0C33;
    pub static SAMPLES_ARB           : GLenum = 0x80A9;

    #[inline(never)]
    pub unsafe fn GetIntegerv(pname: GLenum, params: *GLint) {
        glGetIntegerv(pname, params)
    }

    extern "C" {
        fn glGetIntegerv(pname: GLenum, params: *GLint);
    }
}
