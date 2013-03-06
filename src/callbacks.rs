macro_rules! callback(
    (
        mod $mod_name:ident {
            $extfun:ty ( $($extarg_id:ident : $extarg_ty:ty),+ )
                => $cbfun:ty ( $($args:expr),+ )
        }
    ) => (
        mod $mod_name {
            /**
             * A key for setting and retrieving the callback from
             * task-local storage
             */
            fn tls_key(_: @ $cbfun) {}
            _setter!(tls_key, $cbfun, $extfun)
            _extern_wrapper!(
                tls_key, ($($extarg_id : $extarg_ty),+) => $cbfun ($($args),+)
            )
        }
    );
    
    // Invoked there is a return type and fallback value supplied
    //
    // `$ext_ret_type`: The return type of the external callback wrapper
    // `$ret_val_if_none`: A fallback value to return if the external callback
    //                     wrapper function returned `None`
    (
        mod $mod_name:ident {
            $extfun:ty ( $($extarg_id:ident : $extarg_ty:ty),+ )
                => $cbfun:ty ( $($args:expr),+ )
                    -> (Some: $ext_ret_type:ty | None: $ret_val_if_none:expr)
        }
    ) => (
        mod $mod_name {
            /**
             * A key for setting and retrieving the callback from
             * task-local storage
             */
            fn tls_key(_: @ $cbfun) {}
            _setter!(tls_key, $cbfun, $extfun)
            _extern_wrapper!(
                tls_key, ($($extarg_id : $extarg_ty),+) => $cbfun ($($args),+)
                        -> (Some: $ext_ret_type | None: $ret_val_if_none)
            )
        }
    )
)

// Generates a function that stores a callback in task local storage
macro_rules! _setter(
    ($tls_key:ident, $cbfun:ty, $extfun:ty) => (
        pub fn set(cbfun: $cbfun, f: &fn($extfun) ) {
            /*!
            Stores `cbfun` in task-local storage. `f` is called after storing 
            `cbfun` has been stored. Eg.
           
            ~~~
            do callbacks::errorfun::set(cbfun) |ext| {
                unsafe { api::glfwSetErrorCallback(ext); }
            }
            ~~~
            */
            unsafe {
                task::local_data::local_data_set($tls_key, @cbfun);
                f(ext);
            }
        }
    )
)

// Generates an external function that fetches and calls a function from
// task local storage.
macro_rules! _extern_wrapper(
    (
        $tls_key:ident, ( $($extarg_id:ident : $extarg_ty:ty),+ )
            => $cbfun:ty ( $($args:expr),+ )
    ) => (
        extern fn ext( $( $extarg_id : $extarg_ty ),+ ) {
            unsafe {
                do task::local_data::local_data_get($tls_key).map |&f| {
                    (*f)( $($args),+ );
                }
            };
        }
    );
    
    // Invoked there is a return type and fallback value supplied
    //
    // `$ext_ret_type`: The return type of the external callback wrapper
    // `$ret_val_if_none`: A fallback value to return if the external callback
    //                     wrapper function returned `None`
    (
        $tls_key:ident, ( $($extarg_id:ident : $extarg_ty:ty),+ )
            => $cbfun:ty ( $($args:expr),+ )
                -> (Some: $ext_ret_type:ty | None: $ret_val_if_none:expr)
    ) => (
        extern fn ext( $( $extarg_id : $extarg_ty ),+ ) -> $ext_ret_type {
            unsafe {
                do task::local_data::local_data_get($tls_key)
                    .map_default($ret_val_if_none) |&f| {
                    (*f)( $($args),+ ) as $ext_ret_type
                }
            }
        }
    );
)

// Error Callback
pub callback!(
    mod errorfun {
        ::api::GLFWerrorfun(err: libc::c_int,
                            format: *libc::c_char)
            => ::ErrorFun(err, str::raw::from_c_str(format))
    }
)

// Monitor Callback
pub callback!(
    mod monitorfun {
        ::api::GLFWmonitorfun(monitor: *::api::GLFWmonitor,
                              event: libc::c_int)
            => ::MonitorFun(&::Monitor(monitor), event)
    }
)

// Window Position Callback
pub callback!(
    mod windowposfun {
        ::api::GLFWwindowposfun(window: *::api::GLFWwindow,
                                x: libc::c_int,
                                y: libc::c_int)
            => ::WindowPosFun(&::Window(window),
                               x as int, y as int)
    }
)

// Window Size Callback
pub callback!(
    mod windowsizefun {
        ::api::GLFWwindowsizefun(window: *::api::GLFWwindow,
                                 width: libc::c_int,
                                 height: libc::c_int)
            => ::WindowSizeFun(&::Window(window),
                               width as int, height as int)
    }
)

// Window Close Callback
pub callback!(
    mod windowclosefun {
        ::api::GLFWwindowclosefun(window: *::api::GLFWwindow)
            => ::WindowCloseFun(&::Window(window)) 
                -> (Some: libc::c_int | None: ::FALSE)
    }
)

// Window Refresh Callback
pub callback!(
    mod windowrefreshfun {
        ::api::GLFWwindowrefreshfun(window: *::api::GLFWwindow)
            => ::WindowRefreshFun(&::Window(window))
    }
)

// Window Focus Callback
pub callback!(
    mod windowfocusfun {
        ::api::GLFWwindowfocusfun(window: *::api::GLFWwindow,
                                  activated: libc::c_int)
            => ::WindowFocusFun(&::Window(window), activated as bool)
    }
)

// Window Iconify Callback
pub callback!(
    mod windowiconifyfun {
        ::api::GLFWwindowiconifyfun(window: *::api::GLFWwindow,
                                    iconified: libc::c_int)
            => ::WindowIconifyFun(&::Window(window), iconified as bool)
    }
)

// Key Callback
pub callback!(
    mod keyfun {
        ::api::GLFWkeyfun(window: *::api::GLFWwindow,
                          key: libc::c_int,
                          action: libc::c_int)
            => ::KeyFun(&::Window(window), key, action)
    }
)

// Character Callback
pub callback!(
    mod charfun {
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
    mod mousebuttonfun {
        ::api::GLFWmousebuttonfun(window: *::api::GLFWwindow,
                                  button: libc::c_int,
                                  action: libc::c_int)
            => ::MouseButtonFun(&::Window(window), button, action)
    }
)

// Cursor Position Callback
pub callback!(
    mod cursorposfun {
        ::api::GLFWcursorposfun(window: *::api::GLFWwindow,
                                x: libc::c_int, y: libc::c_int)
            => ::CursorPosFun(&::Window(window), x as int, y as int)
    }
)

// Cursor Enter Callback
pub callback!(
    mod cursorenterfun {
        ::api::GLFWcursorenterfun(window: *::api::GLFWwindow,
                                  entered: libc::c_int)
            => ::CursorEnterFun(&::Window(window), entered as bool)
    }
)

// Scroll Callback
pub callback!(
    mod scrollfun {
        ::api::GLFWscrollfun(window: *::api::GLFWwindow,
                             x: libc::c_double, y: libc::c_double)
            => ::ScrollFun(&::Window(window), x as f64, y as f64)
    }
)
