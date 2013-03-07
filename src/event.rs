macro_rules! callback(
    (
        mod $mod_name:ident {
            $extfun_ty:ty ( $($extarg_id:ident : $extarg_ty:ty),+ )
                => $cbfun_ty:ty ( $($args:expr),+ )
        }
    ) => (
        mod $mod_name {
            /**
             * A key for setting and retrieving the callback from task-
             * local storage
             */
            fn tls_key(_: @$cbfun_ty) {}
            
            /**
             * Stores the callback in task-local storage and then calls
             * `f` with  with `extfun` as the argument.
             */
            pub fn set_callback(cbfun: $cbfun_ty, f: &fn($extfun_ty) ) {
                unsafe {
                    task::local_data::local_data_set(tls_key, @cbfun);
                    f(extfun);
                }
            }
            
            /**
             * An external function that invokes the callback currently stored
             * in task-local storage, if it exists.
             */
            pub extern fn extfun( $( $extarg_id : $extarg_ty ),+ ) {
                unsafe {
                    do task::local_data::local_data_get(tls_key).map |&f| {
                        (*f)( $($args),+ );
                    };
                }
            }
        }
    )
)


// Error Callback
pub callback!(
    mod error {
        ::api::GLFWerrorfun(err: libc::c_int,
                            format: *libc::c_char)
            => ::ErrorFun(err, str::raw::from_c_str(format))
    }
)

// Monitor Callback
pub callback!(
    mod monitor {
        ::api::GLFWmonitorfun(monitor: *::api::GLFWmonitor,
                              event: libc::c_int)
            => ::MonitorFun(&::Monitor(monitor), event)
    }
)

// Window Position Callback
pub callback!(
    mod windowpos {
        ::api::GLFWwindowposfun(window: *::api::GLFWwindow,
                                x: libc::c_int,
                                y: libc::c_int)
            => ::WindowPosFun(&::Window(window),
                               x as int, y as int)
    }
)

// Window Size Callback
pub callback!(
    mod windowsize {
        ::api::GLFWwindowsizefun(window: *::api::GLFWwindow,
                                 width: libc::c_int,
                                 height: libc::c_int)
            => ::WindowSizeFun(&::Window(window),
                               width as int, height as int)
    }
)

// Window Close Callback
pub callback!(
    mod windowclose {
        ::api::GLFWwindowclosefun(window: *::api::GLFWwindow)
            => ::WindowCloseFun(&::Window(window))
    }
)

// Window Refresh Callback
pub callback!(
    mod windowrefresh {
        ::api::GLFWwindowrefreshfun(window: *::api::GLFWwindow)
            => ::WindowRefreshFun(&::Window(window))
    }
)

// Window Focus Callback
pub callback!(
    mod windowfocus {
        ::api::GLFWwindowfocusfun(window: *::api::GLFWwindow,
                                  activated: libc::c_int)
            => ::WindowFocusFun(&::Window(window), activated as bool)
    }
)

// Window Iconify Callback
pub callback!(
    mod windowiconify {
        ::api::GLFWwindowiconifyfun(window: *::api::GLFWwindow,
                                    iconified: libc::c_int)
            => ::WindowIconifyFun(&::Window(window), iconified as bool)
    }
)

// Key Callback
pub callback!(
    mod key {
        ::api::GLFWkeyfun(window: *::api::GLFWwindow,
                          key: libc::c_int,
                          action: libc::c_int)
            => ::KeyFun(&::Window(window), key, action)
    }
)

// Character Callback
pub callback!(
    mod char {
        ::api::GLFWcharfun(window: *::api::GLFWwindow,
                           character: libc::c_uint)
            => ::CharFun(&::Window(window),
                         // TODO: tempory fix for the lack of a bounds check
                         // on X11 and Win32 in GLFW
                         if character == -1 { '0' }
                         else { character as char })
    }
)

// Mouse Button Callback
pub callback!(
    mod mousebutton {
        ::api::GLFWmousebuttonfun(window: *::api::GLFWwindow,
                                  button: libc::c_int,
                                  action: libc::c_int)
            => ::MouseButtonFun(&::Window(window), button, action)
    }
)

// Cursor Position Callback
pub callback!(
    mod cursorpos {
        ::api::GLFWcursorposfun(window: *::api::GLFWwindow,
                                x: libc::c_int, y: libc::c_int)
            => ::CursorPosFun(&::Window(window), x as int, y as int)
    }
)

// Cursor Enter Callback
pub callback!(
    mod cursorenter {
        ::api::GLFWcursorenterfun(window: *::api::GLFWwindow,
                                  entered: libc::c_int)
            => ::CursorEnterFun(&::Window(window), entered as bool)
    }
)

// Scroll Callback
pub callback!(
    mod scroll {
        ::api::GLFWscrollfun(window: *::api::GLFWwindow,
                             x: libc::c_double, y: libc::c_double)
            => ::ScrollFun(&::Window(window), x as f64, y as f64)
    }
)
