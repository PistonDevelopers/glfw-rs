extern mod std;
extern mod glfw3;

fn main() {
    
    // get version tuple
    let (major, minor, rev) = glfw3::get_version();
    io::println(fmt!("(%d, %d, %d)", major, minor, rev));
    
    // get version string
    io::println(~"GLFW version: " + glfw3::get_version_string());
    
}