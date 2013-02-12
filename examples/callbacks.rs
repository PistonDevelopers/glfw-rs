extern mod glfw3;

// let mut done = false;

fn main() {
    
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::task().sched_mode(task::PlatformThread).spawn {
        
        if !glfw3::init() {
            glfw3::terminate();
            die!(~"Failed to initialize GLFW");
        }
        
        glfw3::window_hint(glfw3::RESIZABLE, 1);
        
        let mut window = glfw3::Window::create(800, 600, "Hello, I am a window.", glfw3::Windowed);
            
        if window.ptr.is_null() {
            glfw3::terminate();
            die!(~"Failed to open GLFW window");
        }
        
        window.set_input_mode(glfw3::STICKY_KEYS, 1);
        
        // Register event callbacks
        
        glfw3::set_error_callback(error_callback);
        
        window.set_pos_callback(window_pos_callback);
        window.set_size_callback(window_size_callback);
        window.set_close_callback(window_close_callback);
        window.set_refresh_callback(window_refresh_callback);  // FIXME
        window.set_focus_callback(window_focus_callback);
        window.set_iconify_callback(window_iconify_callback);
        
        window.set_key_callback(key_callback);
        window.set_char_callback(char_callback);
        window.set_mouse_button_callback(mouse_button_callback);
        window.set_cursor_pos_callback(cursor_pos_callback);
        window.set_cursor_enter_callback(cursor_enter_callback);
        window.set_scroll_callback(scroll_callback);  // FIXME
        
        window.make_context_current();
        
        let mut done = false;
        
        while (!done) {
            glfw3::poll_events();
            if (window.get_key(glfw3::KEY_ESC) == glfw3::PRESS || window.get_param(glfw3::SHOULD_CLOSE) != 0) {
                done = true;
            }
        }
        
        window.destroy();
        glfw3::terminate();
    }
}

fn error_callback(_error: libc::c_int, name: ~str) {
    io::println(fmt!("GLFW Error: %s", name));
}

fn window_pos_callback(window: &glfw3::Window, x: int, y: int) {
    window.set_title(fmt!("Window pos: [%d, %d]", x, y));
}

fn window_size_callback(window: &glfw3::Window, width: int, height: int) {
    window.set_title(fmt!("Window size: %d x %d", width, height));
}

// `_window` is preceded with an undescore to silence the unused variable warning
fn window_close_callback(_window: &glfw3::Window) -> bool {
    io::println(~"Window close requested.");
    return true;
}

// FIXME
fn window_refresh_callback(_window: &glfw3::Window) {
    io::println(~"Window refresh callback triggered.");
}

fn window_focus_callback(_window: &glfw3::Window, activated: bool) {
    if activated { io::println(~"Window focus gained."); }
    else         { io::println(~"Window focus lost.");   }
}

fn window_iconify_callback(_window: &glfw3::Window, iconified: bool) {
    if iconified { io::println(~"Window was minimised");  }
    else         { io::println(~"Window was maximised."); }
}

fn key_callback(window: &glfw3::Window, key: libc::c_int, action: libc::c_int) {
    io::println(fmt!("Key %s: %s", to_key_str(key), to_action_str(action)));
    
    // FIXME: this should trigger the window refresh callback.
    if key == glfw3::KEY_R {
        window.swap_buffers();
    }
}

fn char_callback(_window: &glfw3::Window, character: char) {
    io::println(fmt!("Character: '%?'", character));
}

fn mouse_button_callback(_window: &glfw3::Window, button: libc::c_int, action: libc::c_int) {
    io::println(fmt!("Mouse Button %s: %s", to_mouse_button_str(button), to_action_str(action)));
}

fn cursor_pos_callback(window: &glfw3::Window, x: int, y: int) {
    window.set_title(fmt!("Cursor position: [ %d, %d ]", x, y));
}

fn cursor_enter_callback(_window: &glfw3::Window, entered: bool) {
    if entered { io::println(~"Cursor entered window."); }
    else       { io::println(~"Cursor left window.");    }
}

// FIXME
fn scroll_callback(window: &glfw3::Window, x: f64, y: f64) {
    window.set_title(fmt!("Scroll position: [%?, %?]", x, y));
}

fn to_action_str(state: libc::c_int) -> ~str {
    match state {
        s if s == glfw3::RELEASE => { ~"Released" }
        s if s == glfw3::PRESS   => { ~"Pressed"  }
        _                        => { ~"Unknown"  }
    }
}

fn to_key_str(key: libc::c_int) -> ~str {
    match key {
        /* Printable keys */
        k if k == glfw3::KEY_SPACE             => { ~"Space"         }
        k if k == glfw3::KEY_APOSTROPHE        => { ~"Apostrophe"    }
        k if k == glfw3::KEY_COMMA             => { ~"Comma"         }
        k if k == glfw3::KEY_MINUS             => { ~"Minus"         }
        k if k == glfw3::KEY_PERIOD            => { ~"Period"        }
        k if k == glfw3::KEY_SLASH             => { ~"Slash"         }
        k if k == glfw3::KEY_0                 => { ~"0"             }
        k if k == glfw3::KEY_1                 => { ~"1"             }
        k if k == glfw3::KEY_2                 => { ~"2"             }
        k if k == glfw3::KEY_3                 => { ~"3"             }
        k if k == glfw3::KEY_4                 => { ~"4"             }
        k if k == glfw3::KEY_5                 => { ~"5"             }
        k if k == glfw3::KEY_6                 => { ~"6"             }
        k if k == glfw3::KEY_7                 => { ~"7"             }
        k if k == glfw3::KEY_8                 => { ~"8"             }
        k if k == glfw3::KEY_9                 => { ~"9"             }
        k if k == glfw3::KEY_SEMICOLON         => { ~"Semicolon"     }
        k if k == glfw3::KEY_EQUAL             => { ~"Equal"         }
        k if k == glfw3::KEY_A                 => { ~"A"             }
        k if k == glfw3::KEY_B                 => { ~"B"             }
        k if k == glfw3::KEY_C                 => { ~"C"             }
        k if k == glfw3::KEY_D                 => { ~"D"             }
        k if k == glfw3::KEY_E                 => { ~"E"             }
        k if k == glfw3::KEY_F                 => { ~"F"             }
        k if k == glfw3::KEY_G                 => { ~"G"             }
        k if k == glfw3::KEY_H                 => { ~"H"             }
        k if k == glfw3::KEY_I                 => { ~"I"             }
        k if k == glfw3::KEY_J                 => { ~"J"             }
        k if k == glfw3::KEY_K                 => { ~"K"             }
        k if k == glfw3::KEY_L                 => { ~"L"             }
        k if k == glfw3::KEY_M                 => { ~"M"             }
        k if k == glfw3::KEY_N                 => { ~"N"             }
        k if k == glfw3::KEY_O                 => { ~"O"             }
        k if k == glfw3::KEY_P                 => { ~"P"             }
        k if k == glfw3::KEY_Q                 => { ~"Q"             }
        k if k == glfw3::KEY_R                 => { ~"R"             }
        k if k == glfw3::KEY_S                 => { ~"S"             }
        k if k == glfw3::KEY_T                 => { ~"T"             }
        k if k == glfw3::KEY_U                 => { ~"U"             }
        k if k == glfw3::KEY_V                 => { ~"V"             }
        k if k == glfw3::KEY_W                 => { ~"W"             }
        k if k == glfw3::KEY_X                 => { ~"X"             }
        k if k == glfw3::KEY_Y                 => { ~"Y"             }
        k if k == glfw3::KEY_Z                 => { ~"Z"             }
        k if k == glfw3::KEY_LEFT_BRACKET      => { ~"Left Bracket"  }
        k if k == glfw3::KEY_BACKSLASH         => { ~"Backslash"     }
        k if k == glfw3::KEY_RIGHT_BRACKET     => { ~"Right Bracket" }
        k if k == glfw3::KEY_GRAVE_ACCENT      => { ~"Grave Accent"  }
        k if k == glfw3::KEY_WORLD_1           => { ~"World 1"       }
        k if k == glfw3::KEY_WORLD_2           => { ~"World 2"       }

        /* Function keys */
        k if k == glfw3::KEY_ESCAPE            => { ~"Escape"        }
        k if k == glfw3::KEY_ENTER             => { ~"Enter"         }
        k if k == glfw3::KEY_TAB               => { ~"Tab"           }
        k if k == glfw3::KEY_BACKSPACE         => { ~"Backspace"     }
        k if k == glfw3::KEY_INSERT            => { ~"Insert"        }
        k if k == glfw3::KEY_DELETE            => { ~"Delete"        }
        k if k == glfw3::KEY_RIGHT             => { ~"Right"         }
        k if k == glfw3::KEY_LEFT              => { ~"Left"          }
        k if k == glfw3::KEY_DOWN              => { ~"Down"          }
        k if k == glfw3::KEY_UP                => { ~"Up"            }
        k if k == glfw3::KEY_PAGE_UP           => { ~"Page Up"       }
        k if k == glfw3::KEY_PAGE_DOWN         => { ~"Page Down"     }
        k if k == glfw3::KEY_HOME              => { ~"Home"          }
        k if k == glfw3::KEY_END               => { ~"End"           }
        k if k == glfw3::KEY_CAPS_LOCK         => { ~"Caps Lock"     }
        k if k == glfw3::KEY_SCROLL_LOCK       => { ~"Scroll Lock"   }
        k if k == glfw3::KEY_NUM_LOCK          => { ~"Num Lock"      }
        k if k == glfw3::KEY_PRINT_SCREEN      => { ~"Print Screen"  }
        k if k == glfw3::KEY_PAUSE             => { ~"Pause"         }
        k if k == glfw3::KEY_F1                => { ~"F1"            }
        k if k == glfw3::KEY_F2                => { ~"F2"            }
        k if k == glfw3::KEY_F3                => { ~"F3"            }
        k if k == glfw3::KEY_F4                => { ~"F4"            }
        k if k == glfw3::KEY_F5                => { ~"F5"            }
        k if k == glfw3::KEY_F6                => { ~"F6"            }
        k if k == glfw3::KEY_F7                => { ~"F7"            }
        k if k == glfw3::KEY_F8                => { ~"F8"            }
        k if k == glfw3::KEY_F9                => { ~"F9"            }
        k if k == glfw3::KEY_F10               => { ~"F10"           }
        k if k == glfw3::KEY_F11               => { ~"F11"           }
        k if k == glfw3::KEY_F12               => { ~"F12"           }
        k if k == glfw3::KEY_F13               => { ~"F13"           }
        k if k == glfw3::KEY_F14               => { ~"F14"           }
        k if k == glfw3::KEY_F15               => { ~"F15"           }
        k if k == glfw3::KEY_F16               => { ~"F16"           }
        k if k == glfw3::KEY_F17               => { ~"F17"           }
        k if k == glfw3::KEY_F18               => { ~"F18"           }
        k if k == glfw3::KEY_F19               => { ~"F19"           }
        k if k == glfw3::KEY_F20               => { ~"F20"           }
        k if k == glfw3::KEY_F21               => { ~"F21"           }
        k if k == glfw3::KEY_F22               => { ~"F22"           }
        k if k == glfw3::KEY_F23               => { ~"F23"           }
        k if k == glfw3::KEY_F24               => { ~"F24"           }
        k if k == glfw3::KEY_F25               => { ~"F25"           }
        k if k == glfw3::KEY_KP_0              => { ~"Kp 0"          }
        k if k == glfw3::KEY_KP_1              => { ~"Kp 1"          }
        k if k == glfw3::KEY_KP_2              => { ~"Kp 2"          }
        k if k == glfw3::KEY_KP_3              => { ~"Kp 3"          }
        k if k == glfw3::KEY_KP_4              => { ~"Kp 4"          }
        k if k == glfw3::KEY_KP_5              => { ~"Kp 5"          }
        k if k == glfw3::KEY_KP_6              => { ~"Kp 6"          }
        k if k == glfw3::KEY_KP_7              => { ~"Kp 7"          }
        k if k == glfw3::KEY_KP_8              => { ~"Kp 8"          }
        k if k == glfw3::KEY_KP_9              => { ~"Kp 9"          }
        k if k == glfw3::KEY_KP_DECIMAL        => { ~"Kp Decimal"    }
        k if k == glfw3::KEY_KP_DIVIDE         => { ~"Kp Divide"     }
        k if k == glfw3::KEY_KP_MULTIPLY       => { ~"Kp Multiply"   }
        k if k == glfw3::KEY_KP_SUBTRACT       => { ~"Kp Subtract"   }
        k if k == glfw3::KEY_KP_ADD            => { ~"Kp Add"        }
        k if k == glfw3::KEY_KP_ENTER          => { ~"Kp Enter"      }
        k if k == glfw3::KEY_KP_EQUAL          => { ~"Kp Equal"      }
        k if k == glfw3::KEY_LEFT_SHIFT        => { ~"Left Shift"    }
        k if k == glfw3::KEY_LEFT_CONTROL      => { ~"Left Control"  }
        k if k == glfw3::KEY_LEFT_ALT          => { ~"Left Alt"      }
        k if k == glfw3::KEY_LEFT_SUPER        => { ~"Left Super"    }
        k if k == glfw3::KEY_RIGHT_SHIFT       => { ~"Right Shift"   }
        k if k == glfw3::KEY_RIGHT_CONTROL     => { ~"Right Control" }
        k if k == glfw3::KEY_RIGHT_ALT         => { ~"Right Alt"     }
        k if k == glfw3::KEY_RIGHT_SUPER       => { ~"Right Super"   }
        k if k == glfw3::KEY_MENU              => { ~"Menu"          }
        _                                      => { ~"Unknown"       }
    }
}

fn to_mouse_button_str(btn: libc::c_int) -> ~str {
    match btn {
        b if b == glfw3::MOUSE_BUTTON_LEFT     => { ~"Left"     }
        b if b == glfw3::MOUSE_BUTTON_RIGHT    => { ~"Right"    }
        b if b == glfw3::MOUSE_BUTTON_MIDDLE   => { ~"Middle"   }
        _                                      => { ~"Unknown"  }
    }
}