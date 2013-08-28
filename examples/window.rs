extern mod glfw;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

pub struct Wrapper {
    glfw_window: glfw::Window
}

impl Wrapper {
    /// Creates a new window.
    fn new() -> @mut Wrapper {
        // Create the GLFW window.
        let glfw_window = glfw::Window::create(800, 600, "Servo", glfw::Windowed).unwrap();
        glfw_window.make_context_current();

        // Create our window object.
        let window = @mut Wrapper {
            glfw_window:glfw_window,
        };

        // BUG: If the captured use of window is changed to instead use the provided win argument,
        // then there is no crash during the destructor. However, the change in servo is not that
        // simple. as the Wrapper struct has significantly more data and functionality, which are
        // used in the event handlers.
        do window.glfw_window.set_key_callback |_win, key, _scancode, action, _mods| {
            println("Handler");
            if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
                window.glfw_window.set_should_close(true);
            }
        };

        window
    }

    fn recv(&self) -> bool {
        self.glfw_window.poll_events();
        glfw::poll_events();
        if self.glfw_window.should_close() {
            true
        } else {
            false
        }
    }

}

fn main() {
    glfw::init();
    
    let window: @mut Wrapper = Wrapper::new();

    // Enter the main event loop.
    while !window.recv() {
    }
    glfw::terminate();
}
