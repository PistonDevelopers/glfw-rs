extern mod glfw;

fn main() {
    io::println(glfw::get_version().to_str());
    io::println(~"GLFW version: " + glfw::get_version_string());
}
