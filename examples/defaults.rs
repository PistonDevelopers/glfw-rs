extern mod glfw;

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}

fn main() {
    glfw::set_error_callback(error_callback);
    
    do glfw::spawn {
        
        glfw::window_hint::visible(true);
        
        let window = glfw::Window::create(640, 480, "Defaults", glfw::Windowed).get();
        
        window.make_context_current();
        
        let (width, height) = window.get_size();
        io::println(fmt!("window size: %? x %?", width, height));
        
        println(fmt!("Context version major: %?",     window.get_context_version_major()));
        println(fmt!("Context version minor: %?",     window.get_context_version_minor()));
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
        
        for gl_params.each |&p| {
            let (param, ext, name) = p;
            
            if do ext.map_default(true) |&s| {
                glfw::extension_supported(s)
            } {
                let mut value = 0;
                unsafe { gl::GetIntegerv(param, &value); }
                
                println(fmt!("OpenGL %s: %?", name, value));
            };
        }
    }
}

mod gl {
    #[nolink]
    #[link_args="-framework OpenGL"]
    #[cfg(target_os = "macos")]
    extern mod linkhack {}

    #[nolink]
    #[link_args="-lGL"]
    #[cfg(target_os = "linux")]
    extern mod linkhack {}
    
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
    
    pub extern "C" {
        #[link_name="glGetIntegerv"]
        pub fn GetIntegerv(pname: GLenum, params: *GLint);
    }
}