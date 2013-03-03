extern mod glfw;

fn main() {
    glfw::set_error_callback(error_callback);
    
    do glfw::spawn {
        let window = glfw::Window::create(300, 300, "Clipboard Test", glfw::Windowed).unwrap();
        
        window.make_context_current();
        window.set_key_callback(key_callback);
        glfw::set_swap_interval(1);
        
        while !window.should_close() {
            glfw::wait_events();
        }
    }
}

#[cfg(target_os = "macos")]
fn control_is_down(window: &glfw::Window) -> bool {
    window.get_key(glfw::KEY_LEFT_SUPER) as bool
    || window.get_key(glfw::KEY_RIGHT_SUPER) as bool
}

#[cfg(target_os = "linux")]
fn control_is_down(window: &glfw::Window) -> bool {
    window.get_key(glfw::KEY_LEFT_CONTROL) as bool
    || window.get_key(glfw::KEY_RIGHT_CONTROL) as bool
}

#[cfg(target_os = "win32")]
fn control_is_down(window: &glfw::Window) -> bool {
    window.get_key(glfw::KEY_LEFT_CONTROL) as bool
    || window.get_key(glfw::KEY_RIGHT_CONTROL) as bool
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int) {
    if action == glfw::PRESS {
        match key {
            k if k == glfw::KEY_ESCAPE => {
                window.set_should_close(true);
            }
            k if k == glfw::KEY_V && control_is_down(window) => {
                match window.get_clipboard_string() {
                    ref s if !s.is_empty() => io::println(fmt!("Clipboard contains %?", s)),
                    _                      => io::println("Clipboard does not contain a string"),
                }
            }
            k if k == glfw::KEY_C && control_is_down(window) => {
                let s = "Hello GLFW World!";
                window.set_clipboard_string(s);
                io::println(fmt!("Setting clipboard to %?", s));
            }
            _ => {}
        }
    }
}