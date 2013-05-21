extern mod glfw;

fn main() {
    glfw::set_error_callback(error_callback);

    do glfw::spawn {

        glfw::window_hint::resizable(true);

        let window = glfw::Window::create(800, 600, "Hello, I am a window.", glfw::Windowed).unwrap();

        window.set_sticky_keys(true);

        // Register event callbacks

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

        while !window.should_close() {
            glfw::poll_events();
        }
    }
}

fn error_callback(_error: libc::c_int, name: ~str) {
    io::println(fmt!("GLFW Error: %s", name));
}

fn window_pos_callback(window: &glfw::Window, x: int, y: int) {
    window.set_title(fmt!("Window pos: [%d, %d]", x, y));
}

fn window_size_callback(window: &glfw::Window, width: int, height: int) {
    window.set_title(fmt!("Window size: %d x %d", width, height));
}

// `_window` is preceded with an undescore to silence the unused variable warning
fn window_close_callback(_window: &glfw::Window) {
    io::println(~"Window close requested.");
}

// FIXME
fn window_refresh_callback(_window: &glfw::Window) {
    io::println(~"Window refresh callback triggered.");
}

fn window_focus_callback(_window: &glfw::Window, activated: bool) {
    if activated { io::println(~"Window focus gained."); }
    else         { io::println(~"Window focus lost.");   }
}

fn window_iconify_callback(_window: &glfw::Window, iconified: bool) {
    if iconified { io::println(~"Window was minimised");  }
    else         { io::println(~"Window was maximised."); }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int) {
    io::println(fmt!("Key %s: %s", to_key_str(key), to_action_str(action)));

    if action == glfw::PRESS {
        if key == glfw::KEY_ESCAPE {
            window.set_should_close(true);
        }

        // FIXME: this should trigger the window refresh callback.
        if key == glfw::KEY_R {
            window.swap_buffers();
        }
    }
}

fn char_callback(_window: &glfw::Window, character: char) {
    io::println(fmt!("Character: %?", character));
}

fn mouse_button_callback(_window: &glfw::Window, button: libc::c_int, action: libc::c_int) {
    io::println(fmt!("Mouse Button %s: %s", to_mouse_button_str(button), to_action_str(action)));
}

fn cursor_pos_callback(window: &glfw::Window, xpos: float, ypos: float) {
    window.set_title(fmt!("Cursor position: [ %f, %f ]", xpos, ypos));
}

fn cursor_enter_callback(_window: &glfw::Window, entered: bool) {
    if entered { io::println(~"Cursor entered window."); }
    else       { io::println(~"Cursor left window.");    }
}

fn scroll_callback(window: &glfw::Window, xpos: float, ypos: float) {
    window.set_title(fmt!("Scroll position: [%f, %f]", xpos, ypos));
}

fn to_action_str(state: libc::c_int) -> ~str {
    match state {
        glfw::RELEASE => { ~"Released" }
        glfw::PRESS   => { ~"Pressed"  }
        glfw::REPEAT  => { ~"Repeated" }
        _             => { ~"Unknown"  }
    }
}

fn to_key_str(key: libc::c_int) -> ~str {
    match key {
        /* Printable keys */
        glfw::KEY_SPACE             => { ~"Space"         }
        glfw::KEY_APOSTROPHE        => { ~"Apostrophe"    }
        glfw::KEY_COMMA             => { ~"Comma"         }
        glfw::KEY_MINUS             => { ~"Minus"         }
        glfw::KEY_PERIOD            => { ~"Period"        }
        glfw::KEY_SLASH             => { ~"Slash"         }
        glfw::KEY_0                 => { ~"0"             }
        glfw::KEY_1                 => { ~"1"             }
        glfw::KEY_2                 => { ~"2"             }
        glfw::KEY_3                 => { ~"3"             }
        glfw::KEY_4                 => { ~"4"             }
        glfw::KEY_5                 => { ~"5"             }
        glfw::KEY_6                 => { ~"6"             }
        glfw::KEY_7                 => { ~"7"             }
        glfw::KEY_8                 => { ~"8"             }
        glfw::KEY_9                 => { ~"9"             }
        glfw::KEY_SEMICOLON         => { ~"Semicolon"     }
        glfw::KEY_EQUAL             => { ~"Equal"         }
        glfw::KEY_A                 => { ~"A"             }
        glfw::KEY_B                 => { ~"B"             }
        glfw::KEY_C                 => { ~"C"             }
        glfw::KEY_D                 => { ~"D"             }
        glfw::KEY_E                 => { ~"E"             }
        glfw::KEY_F                 => { ~"F"             }
        glfw::KEY_G                 => { ~"G"             }
        glfw::KEY_H                 => { ~"H"             }
        glfw::KEY_I                 => { ~"I"             }
        glfw::KEY_J                 => { ~"J"             }
        glfw::KEY_K                 => { ~"K"             }
        glfw::KEY_L                 => { ~"L"             }
        glfw::KEY_M                 => { ~"M"             }
        glfw::KEY_N                 => { ~"N"             }
        glfw::KEY_O                 => { ~"O"             }
        glfw::KEY_P                 => { ~"P"             }
        glfw::KEY_Q                 => { ~"Q"             }
        glfw::KEY_R                 => { ~"R"             }
        glfw::KEY_S                 => { ~"S"             }
        glfw::KEY_T                 => { ~"T"             }
        glfw::KEY_U                 => { ~"U"             }
        glfw::KEY_V                 => { ~"V"             }
        glfw::KEY_W                 => { ~"W"             }
        glfw::KEY_X                 => { ~"X"             }
        glfw::KEY_Y                 => { ~"Y"             }
        glfw::KEY_Z                 => { ~"Z"             }
        glfw::KEY_LEFT_BRACKET      => { ~"Left Bracket"  }
        glfw::KEY_BACKSLASH         => { ~"Backslash"     }
        glfw::KEY_RIGHT_BRACKET     => { ~"Right Bracket" }
        glfw::KEY_GRAVE_ACCENT      => { ~"Grave Accent"  }
        glfw::KEY_WORLD_1           => { ~"World 1"       }
        glfw::KEY_WORLD_2           => { ~"World 2"       }

        /* Function keys */
        glfw::KEY_ESCAPE            => { ~"Escape"        }
        glfw::KEY_ENTER             => { ~"Enter"         }
        glfw::KEY_TAB               => { ~"Tab"           }
        glfw::KEY_BACKSPACE         => { ~"Backspace"     }
        glfw::KEY_INSERT            => { ~"Insert"        }
        glfw::KEY_DELETE            => { ~"Delete"        }
        glfw::KEY_RIGHT             => { ~"Right"         }
        glfw::KEY_LEFT              => { ~"Left"          }
        glfw::KEY_DOWN              => { ~"Down"          }
        glfw::KEY_UP                => { ~"Up"            }
        glfw::KEY_PAGE_UP           => { ~"Page Up"       }
        glfw::KEY_PAGE_DOWN         => { ~"Page Down"     }
        glfw::KEY_HOME              => { ~"Home"          }
        glfw::KEY_END               => { ~"End"           }
        glfw::KEY_CAPS_LOCK         => { ~"Caps Lock"     }
        glfw::KEY_SCROLL_LOCK       => { ~"Scroll Lock"   }
        glfw::KEY_NUM_LOCK          => { ~"Num Lock"      }
        glfw::KEY_PRINT_SCREEN      => { ~"Print Screen"  }
        glfw::KEY_PAUSE             => { ~"Pause"         }
        glfw::KEY_F1                => { ~"F1"            }
        glfw::KEY_F2                => { ~"F2"            }
        glfw::KEY_F3                => { ~"F3"            }
        glfw::KEY_F4                => { ~"F4"            }
        glfw::KEY_F5                => { ~"F5"            }
        glfw::KEY_F6                => { ~"F6"            }
        glfw::KEY_F7                => { ~"F7"            }
        glfw::KEY_F8                => { ~"F8"            }
        glfw::KEY_F9                => { ~"F9"            }
        glfw::KEY_F10               => { ~"F10"           }
        glfw::KEY_F11               => { ~"F11"           }
        glfw::KEY_F12               => { ~"F12"           }
        glfw::KEY_F13               => { ~"F13"           }
        glfw::KEY_F14               => { ~"F14"           }
        glfw::KEY_F15               => { ~"F15"           }
        glfw::KEY_F16               => { ~"F16"           }
        glfw::KEY_F17               => { ~"F17"           }
        glfw::KEY_F18               => { ~"F18"           }
        glfw::KEY_F19               => { ~"F19"           }
        glfw::KEY_F20               => { ~"F20"           }
        glfw::KEY_F21               => { ~"F21"           }
        glfw::KEY_F22               => { ~"F22"           }
        glfw::KEY_F23               => { ~"F23"           }
        glfw::KEY_F24               => { ~"F24"           }
        glfw::KEY_F25               => { ~"F25"           }
        glfw::KEY_KP_0              => { ~"Kp 0"          }
        glfw::KEY_KP_1              => { ~"Kp 1"          }
        glfw::KEY_KP_2              => { ~"Kp 2"          }
        glfw::KEY_KP_3              => { ~"Kp 3"          }
        glfw::KEY_KP_4              => { ~"Kp 4"          }
        glfw::KEY_KP_5              => { ~"Kp 5"          }
        glfw::KEY_KP_6              => { ~"Kp 6"          }
        glfw::KEY_KP_7              => { ~"Kp 7"          }
        glfw::KEY_KP_8              => { ~"Kp 8"          }
        glfw::KEY_KP_9              => { ~"Kp 9"          }
        glfw::KEY_KP_DECIMAL        => { ~"Kp Decimal"    }
        glfw::KEY_KP_DIVIDE         => { ~"Kp Divide"     }
        glfw::KEY_KP_MULTIPLY       => { ~"Kp Multiply"   }
        glfw::KEY_KP_SUBTRACT       => { ~"Kp Subtract"   }
        glfw::KEY_KP_ADD            => { ~"Kp Add"        }
        glfw::KEY_KP_ENTER          => { ~"Kp Enter"      }
        glfw::KEY_KP_EQUAL          => { ~"Kp Equal"      }
        glfw::KEY_LEFT_SHIFT        => { ~"Left Shift"    }
        glfw::KEY_LEFT_CONTROL      => { ~"Left Control"  }
        glfw::KEY_LEFT_ALT          => { ~"Left Alt"      }
        glfw::KEY_LEFT_SUPER        => { ~"Left Super"    }
        glfw::KEY_RIGHT_SHIFT       => { ~"Right Shift"   }
        glfw::KEY_RIGHT_CONTROL     => { ~"Right Control" }
        glfw::KEY_RIGHT_ALT         => { ~"Right Alt"     }
        glfw::KEY_RIGHT_SUPER       => { ~"Right Super"   }
        glfw::KEY_MENU              => { ~"Menu"          }
        _                           => { ~"Unknown"       }
    }
}

fn to_mouse_button_str(btn: libc::c_int) -> ~str {
    match btn {
        glfw::MOUSE_BUTTON_LEFT     => { ~"Left"     }
        glfw::MOUSE_BUTTON_RIGHT    => { ~"Right"    }
        glfw::MOUSE_BUTTON_MIDDLE   => { ~"Middle"   }
        _                           => { ~"Unknown"  }
    }
}
