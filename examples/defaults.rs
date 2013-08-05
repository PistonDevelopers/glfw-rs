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

extern mod glfw;

use std::libc;

fn error_callback(_: libc::c_int, description: ~str) {
    println(fmt!("GLFW Error: %s", description));
}

fn main() {
    glfw::set_error_callback(error_callback);

    do glfw::spawn {

        glfw::window_hint::visible(true);

        let window = glfw::Window::create(640, 480, "Defaults", glfw::Windowed).unwrap();

        window.make_context_current();

        let (width, height) = window.get_size();
        println(fmt!("window size: %? x %?", width, height));

        println(fmt!("Context version: %s",           window.get_context_version().to_str()));
        println(fmt!("OpenGL forward compatible: %?", window.is_opengl_forward_compat()));
        println(fmt!("OpenGL debug context: %?",      window.is_opengl_debug_context()));
        println(fmt!("OpenGL profile: %?",            window.get_opengl_profile()));

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
            if do ext.map_default(true) |&s| {
                glfw::extension_supported(s)
            } {
                unsafe {
                    let value = 0;
                    gl::GetIntegerv(param, &value);
                    println(fmt!("OpenGL %s: %?", name, value));
                }
            };
        }
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

    extern "C" {
        #[link_name="glGetIntegerv"]
        pub fn GetIntegerv(pname: GLenum, params: *GLint);
    }
}
