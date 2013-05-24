extern mod glfw;

fn main() {
    do glfw::spawn {
        do glfw::Monitor::get_primary().map |monitor| {
                io::println(fmt!("%s:", monitor.get_name()));
                io::println(fmt!("    %s\n", monitor.get_video_mode().get().to_str()));
        };

        io::println("Available monitors\n\
                     ------------------");
        do glfw::Monitor::get_connected().map |monitor| {
            io::println(fmt!("%s:", monitor.get_name()));

            do monitor.get_video_modes().map |mode| {
                io::println(fmt!("  %s", mode.to_str()));
            }
        };
    }
}
