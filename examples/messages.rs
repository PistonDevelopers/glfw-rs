extern mod glfw;

fn main() {
    glfw::set_error_callback(error_callback);
    
    do glfw::spawn {
        let window = glfw::Window::create(640, 480, "Use the arrow keys to move the window (↑ ↓ ← →)", glfw::Windowed).unwrap();
        
        window.make_context_current();
        
        do window.spawn_pos_listener |msg| {
            io::println(fmt!("Window pos: [%d, %d]", msg.x, msg.y));
            // window.set_title(fmt!("Window pos: [%d, %d]", msg.x, msg.y));
        }
        
        do window.spawn_key_listener |msg| {
            if msg.action == glfw::PRESS {
                match msg.key {
                    k if k == glfw::KEY_ESCAPE => { window.set_should_close(true); }
                    k if k == glfw::KEY_UP     => { move_window(&window, 0, 5);    }
                    k if k == glfw::KEY_DOWN   => { move_window(&window, 0,-5);    }
                    k if k == glfw::KEY_LEFT   => { move_window(&window,-5, 0);    }
                    k if k == glfw::KEY_RIGHT  => { move_window(&window, 5, 0);    }
                    _ => {}
                }
            }
        }
        
        while !window.should_close() {
            glfw::poll_events();
        }
    }
}

fn move_window(window: &glfw::Window, dx: int, dy: int) {
    let (x, y) = window.get_pos();
    window.set_pos(x + dx, y + dy);
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}