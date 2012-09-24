extern mod std;
extern mod glfw3;
use glfw3::*;

fn main() {
    
    if (glfwInit() == 0) {
        fail(~"glfwInit() failed\n");
    }
    
    let mut time = 0f64;
    glfwSetTime(time);
    
    for 40.times {
        let newTime = glfwGetTime();
        let delta = newTime - time;
        time = newTime;
        io::println(fmt!("dt: %?", delta));
    }
    
    glfwTerminate();
}
