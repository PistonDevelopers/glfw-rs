extern mod glfw3;

fn main() {
    do task::task().sched_mode(task::PlatformThread).spawn {
        
        glfw3::set_error_callback(error_callback);
        
        if !glfw3::init() {
            glfw3::terminate();
            die!(~"Failed to initialize GLFW");
        }
        
        let window =
            match glfw3::Window::create(300, 300, "Clipboard Test", glfw3::Windowed) {
                Some(w) => { w }
                None => {
                    glfw3::terminate();
                    die!(~"Failed to open GLFW window");
                }
            };
        
        window.make_context_current();
        glfw3::set_swap_interval(1);
        
        window.set_key_callback(key_callback);
        
        while window.get_param(glfw3::SHOULD_CLOSE) == 0 {
            glfw3::wait_events();
        }
        
        window.destroy();
        glfw3::terminate();
    }
}

#[cfg(target_os = "macos")]
fn control_is_down(window: &glfw3::Window) -> bool {
    window.get_key(glfw3::KEY_LEFT_SUPER) as bool
    || window.get_key(glfw3::KEY_RIGHT_SUPER) as bool
}

#[cfg(target_os = "linux")]
fn control_is_down(window: &glfw3::Window) -> bool {
    window.get_key(glfw3::KEY_LEFT_CONTROL) as bool
    || window.get_key(glfw3::KEY_RIGHT_CONTROL) as bool
}

#[cfg(target_os = "win32")]
fn control_is_down(window: &glfw3::Window) -> bool {
    window.get_key(glfw3::KEY_LEFT_CONTROL) as bool
    || window.get_key(glfw3::KEY_RIGHT_CONTROL) as bool
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}

fn key_callback(window: &glfw3::Window, key: libc::c_int, action: libc::c_int) {
    if action == glfw3::PRESS {
        match key {
            // k if k == glfw3::KEY_ESCAPE => {}
            k if k == glfw3::KEY_V && control_is_down(window) => {
                match window.get_clipboard_string() {
                    ref s if !s.is_empty() => io::println(fmt!("Clipboard contains %?", s)),
                    _                      => io::println("Clipboard does not contain a string"),
                }
            }
            k if k == glfw3::KEY_C && control_is_down(window) => {
                let s = "Hello GLFW World!";
                window.set_clipboard_string(s);
                io::println(fmt!("Setting clipboard to %?", s));
            }
            _ => {}
        }
    }
}