extern mod glfw3;

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}

fn main() {
    do task::task().sched_mode(task::PlatformThread).spawn {
        
        glfw3::set_error_callback(error_callback);
        
        if !glfw3::init() {
            glfw3::terminate();
            fail(~"Failed to initialize GLFW\n");
        }
        
        let window = glfw3::Window::create(400, 400, ~"English 日本語 русский язык 官話", glfw3::Windowed);
        
        if window.ptr.is_null() {
            glfw3::terminate();
            fail(~"Failed to open GLFW window");
        }
        
        window.make_context_current();
        glfw3::set_swap_interval(1);
        
        while window.get_param(glfw3::SHOULD_CLOSE) == 0 {
            glfw3::wait_events();
        }
        
        window.destroy();
        glfw3::terminate();
    }
}