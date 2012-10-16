extern mod std;
extern mod glfw3;

// let mut done = false;

fn main() {
    
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::task().sched_mode(task::PlatformThread).spawn {
        if (glfw3::init() == 0) {
            glfw3::terminate();
            fail(~"glfwInit() failed\n");
        }
        
        // Register event callbacks
        unsafe {
            glfw3::cfns::glfwSetErrorCallback(errorCallback);
            glfw3::cfns::glfwSetKeyCallback(keyCallback);
            glfw3::cfns::glfwSetMouseButtonCallback(mouseButtonCallback);
        }
        
        let mut window = glfw3::create_window(800, 600, glfw3::WINDOWED, ~"Hello, I am a window.");
        
        io::println(fmt!("Window ptr: %d", window.ptr as int));
        
        if (ptr::is_null(window.ptr)) {
            glfw3::terminate();
            io::println(~"Error: " + glfw3::error_string(glfw3::get_error()));
            fail(~"glfwOpenWindow() failed\n");
        }
        
        window.set_input_mode(glfw3::STICKY_KEYS, 1);
        
        let mut done = false;
        
        while (!done) {
            glfw3::poll_events();
            if (window.get_key(glfw3::KEY_ESC) == glfw3::PRESS || window.get_param(glfw3::CLOSE_REQUESTED) != 0) {
                done = true;
            }
        } 
        
        glfw3::terminate();
    }
}

// For now you have use external functions for the callbacks. This will be changed in the future.

extern fn errorCallback(error: libc::c_int, name: *libc::c_char) {
    unsafe { io::println(fmt!("GLFW Error: %s", str::raw::from_c_str(name))); }
}

extern fn keyCallback(window: glfw3::Window, key: libc::c_int, action: libc::c_int) {
    io::println(fmt!("Key %s: %s", keyString(key as int), buttonStateString(action as int)));
}

extern fn mouseButtonCallback(window: glfw3::Window, btn: libc::c_int, action: libc::c_int) {
    io::println(fmt!("Mouse Button %s: %s", mouseButtonString(btn as int), buttonStateString(action as int)));
}

fn buttonStateString(state: int) -> ~str {
    if      state == glfw3::RELEASE { ~"Released" }
    else if state == glfw3::PRESS   { ~"Pressed"  }
    else                            { ~"Unknown"  }
}

fn keyString(key: int) -> ~str {
    /* Printable keys */
    if      key == glfw3::KEY_SPACE           { ~"Space"         }
    else if key == glfw3::KEY_APOSTROPHE      { ~"Apostrophe"    }
    else if key == glfw3::KEY_COMMA           { ~"Comma"         }
    else if key == glfw3::KEY_MINUS           { ~"Minus"         }
    else if key == glfw3::KEY_PERIOD          { ~"Period"        }
    else if key == glfw3::KEY_SLASH           { ~"Slash"         }
    else if key == glfw3::KEY_0               { ~"0"             }
    else if key == glfw3::KEY_1               { ~"1"             }
    else if key == glfw3::KEY_2               { ~"2"             }
    else if key == glfw3::KEY_3               { ~"3"             }
    else if key == glfw3::KEY_4               { ~"4"             }
    else if key == glfw3::KEY_5               { ~"5"             }
    else if key == glfw3::KEY_6               { ~"6"             }
    else if key == glfw3::KEY_7               { ~"7"             }
    else if key == glfw3::KEY_8               { ~"8"             }
    else if key == glfw3::KEY_9               { ~"9"             }
    else if key == glfw3::KEY_SEMICOLON       { ~"Semicolon"     }
    else if key == glfw3::KEY_EQUAL           { ~"Equal"         }
    else if key == glfw3::KEY_A               { ~"A"             }
    else if key == glfw3::KEY_B               { ~"B"             }
    else if key == glfw3::KEY_C               { ~"C"             }
    else if key == glfw3::KEY_D               { ~"D"             }
    else if key == glfw3::KEY_E               { ~"E"             }
    else if key == glfw3::KEY_F               { ~"F"             }
    else if key == glfw3::KEY_G               { ~"G"             }
    else if key == glfw3::KEY_H               { ~"H"             }
    else if key == glfw3::KEY_I               { ~"I"             }
    else if key == glfw3::KEY_J               { ~"J"             }
    else if key == glfw3::KEY_K               { ~"K"             }
    else if key == glfw3::KEY_L               { ~"L"             }
    else if key == glfw3::KEY_M               { ~"M"             }
    else if key == glfw3::KEY_N               { ~"N"             }
    else if key == glfw3::KEY_O               { ~"O"             }
    else if key == glfw3::KEY_P               { ~"P"             }
    else if key == glfw3::KEY_Q               { ~"Q"             }
    else if key == glfw3::KEY_R               { ~"R"             }
    else if key == glfw3::KEY_S               { ~"S"             }
    else if key == glfw3::KEY_T               { ~"T"             }
    else if key == glfw3::KEY_U               { ~"U"             }
    else if key == glfw3::KEY_V               { ~"V"             }
    else if key == glfw3::KEY_W               { ~"W"             }
    else if key == glfw3::KEY_X               { ~"X"             }
    else if key == glfw3::KEY_Y               { ~"Y"             }
    else if key == glfw3::KEY_Z               { ~"Z"             }
    else if key == glfw3::KEY_LEFT_BRACKET    { ~"Left Bracket"  }
    else if key == glfw3::KEY_BACKSLASH       { ~"Backslash"     }
    else if key == glfw3::KEY_RIGHT_BRACKET   { ~"Right Bracket" }
    else if key == glfw3::KEY_GRAVE_ACCENT    { ~"Grave Accent"  }
    else if key == glfw3::KEY_WORLD_1         { ~"World 1"       }
    else if key == glfw3::KEY_WORLD_2         { ~"World 2"       }

    /* Function keys */
    else if key == glfw3::KEY_ESCAPE          { ~"Escape"        }
    else if key == glfw3::KEY_ENTER           { ~"Enter"         }
    else if key == glfw3::KEY_TAB             { ~"Tab"           }
    else if key == glfw3::KEY_BACKSPACE       { ~"Backspace"     }
    else if key == glfw3::KEY_INSERT          { ~"Insert"        }
    else if key == glfw3::KEY_DELETE          { ~"Delete"        }
    else if key == glfw3::KEY_RIGHT           { ~"Right"         }
    else if key == glfw3::KEY_LEFT            { ~"Left"          }
    else if key == glfw3::KEY_DOWN            { ~"Down"          }
    else if key == glfw3::KEY_UP              { ~"Up"            }
    else if key == glfw3::KEY_PAGE_UP         { ~"Page Up"       }
    else if key == glfw3::KEY_PAGE_DOWN       { ~"Page Down"     }
    else if key == glfw3::KEY_HOME            { ~"Home"          }
    else if key == glfw3::KEY_END             { ~"End"           }
    else if key == glfw3::KEY_CAPS_LOCK       { ~"Caps Lock"     }
    else if key == glfw3::KEY_SCROLL_LOCK     { ~"Scroll Lock"   }
    else if key == glfw3::KEY_NUM_LOCK        { ~"Num Lock"      }
    else if key == glfw3::KEY_PRINT_SCREEN    { ~"Print Screen"  }
    else if key == glfw3::KEY_PAUSE           { ~"Pause"         }
    else if key == glfw3::KEY_F1              { ~"F1"            }
    else if key == glfw3::KEY_F2              { ~"F2"            }
    else if key == glfw3::KEY_F3              { ~"F3"            }
    else if key == glfw3::KEY_F4              { ~"F4"            }
    else if key == glfw3::KEY_F5              { ~"F5"            }
    else if key == glfw3::KEY_F6              { ~"F6"            }
    else if key == glfw3::KEY_F7              { ~"F7"            }
    else if key == glfw3::KEY_F8              { ~"F8"            }
    else if key == glfw3::KEY_F9              { ~"F9"            }
    else if key == glfw3::KEY_F10             { ~"F10"           }
    else if key == glfw3::KEY_F11             { ~"F11"           }
    else if key == glfw3::KEY_F12             { ~"F12"           }
    else if key == glfw3::KEY_F13             { ~"F13"           }
    else if key == glfw3::KEY_F14             { ~"F14"           }
    else if key == glfw3::KEY_F15             { ~"F15"           }
    else if key == glfw3::KEY_F16             { ~"F16"           }
    else if key == glfw3::KEY_F17             { ~"F17"           }
    else if key == glfw3::KEY_F18             { ~"F18"           }
    else if key == glfw3::KEY_F19             { ~"F19"           }
    else if key == glfw3::KEY_F20             { ~"F20"           }
    else if key == glfw3::KEY_F21             { ~"F21"           }
    else if key == glfw3::KEY_F22             { ~"F22"           }
    else if key == glfw3::KEY_F23             { ~"F23"           }
    else if key == glfw3::KEY_F24             { ~"F24"           }
    else if key == glfw3::KEY_F25             { ~"F25"           }
    else if key == glfw3::KEY_KP_0            { ~"Kp 0"          }
    else if key == glfw3::KEY_KP_1            { ~"Kp 1"          }
    else if key == glfw3::KEY_KP_2            { ~"Kp 2"          }
    else if key == glfw3::KEY_KP_3            { ~"Kp 3"          }
    else if key == glfw3::KEY_KP_4            { ~"Kp 4"          }
    else if key == glfw3::KEY_KP_5            { ~"Kp 5"          }
    else if key == glfw3::KEY_KP_6            { ~"Kp 6"          }
    else if key == glfw3::KEY_KP_7            { ~"Kp 7"          }
    else if key == glfw3::KEY_KP_8            { ~"Kp 8"          }
    else if key == glfw3::KEY_KP_9            { ~"Kp 9"          }
    else if key == glfw3::KEY_KP_DECIMAL      { ~"Kp Decimal"    }
    else if key == glfw3::KEY_KP_DIVIDE       { ~"Kp Divide"     }
    else if key == glfw3::KEY_KP_MULTIPLY     { ~"Kp Multiply"   }
    else if key == glfw3::KEY_KP_SUBTRACT     { ~"Kp Subtract"   }
    else if key == glfw3::KEY_KP_ADD          { ~"Kp Add"        }
    else if key == glfw3::KEY_KP_ENTER        { ~"Kp Enter"      }
    else if key == glfw3::KEY_KP_EQUAL        { ~"Kp Equal"      }
    else if key == glfw3::KEY_LEFT_SHIFT      { ~"Left Shift"    }
    else if key == glfw3::KEY_LEFT_CONTROL    { ~"Left Control"  }
    else if key == glfw3::KEY_LEFT_ALT        { ~"Left Alt"      }
    else if key == glfw3::KEY_LEFT_SUPER      { ~"Left Super"    }
    else if key == glfw3::KEY_RIGHT_SHIFT     { ~"Right Shift"   }
    else if key == glfw3::KEY_RIGHT_CONTROL   { ~"Right Control" }
    else if key == glfw3::KEY_RIGHT_ALT       { ~"Right Alt"     }
    else if key == glfw3::KEY_RIGHT_SUPER     { ~"Right Super"   }
    else if key == glfw3::KEY_MENU            { ~"Menu"          }
    else                                      { ~"Unknown"       }
}

fn mouseButtonString(btn: int) -> ~str {
    if      btn == glfw3::MOUSE_BUTTON_LEFT   { ~"Left"    }
    else if btn == glfw3::MOUSE_BUTTON_RIGHT  { ~"Right"   }
    else if btn == glfw3::MOUSE_BUTTON_MIDDLE { ~"Middle"  }
    else                                      { ~"Unknown" }
}