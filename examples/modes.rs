extern mod glfw;

fn main() {
        
    if !glfw::init() {
        glfw::terminate();
        die!(~"Failed to initialize GLFW");
    }
    
    io::println(fmt!("Primary monitor: %s\n", glfw::get_primary_monitor().get_name()));
    
    io::println(~"Available monitors\n\
                  ------------------");
    
    glfw::get_monitors().map(|monitor| {
        io::println(fmt!("%s:", monitor.get_name()));
        
        do monitor.get_video_modes().map |mode| {
            io::println(fmt!("  %s", mode.to_str()));
        }
    });
    
    glfw::terminate();
}