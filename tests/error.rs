use glfw::{ffi, Error};

#[test]
fn all_glfw_3_4_error_codes_round_trip() {
    let known = [
        (ffi::GLFW_NO_ERROR, Error::NoError),
        (ffi::GLFW_NOT_INITIALIZED, Error::NotInitialized),
        (ffi::GLFW_NO_CURRENT_CONTEXT, Error::NoCurrentContext),
        (ffi::GLFW_INVALID_ENUM, Error::InvalidEnum),
        (ffi::GLFW_INVALID_VALUE, Error::InvalidValue),
        (ffi::GLFW_OUT_OF_MEMORY, Error::OutOfMemory),
        (ffi::GLFW_API_UNAVAILABLE, Error::ApiUnavailable),
        (ffi::GLFW_VERSION_UNAVAILABLE, Error::VersionUnavailable),
        (ffi::GLFW_PLATFORM_ERROR, Error::PlatformError),
        (ffi::GLFW_FORMAT_UNAVAILABLE, Error::FormatUnavailable),
        (ffi::GLFW_NO_WINDOW_CONTEXT, Error::NoWindowContext),
        (ffi::GLFW_CURSOR_UNAVAILABLE, Error::CursorUnavailable),
        (ffi::GLFW_FEATURE_UNAVAILABLE, Error::FeatureUnavailable),
        (ffi::GLFW_FEATURE_UNIMPLEMENTED, Error::FeatureUnimplemented),
        (ffi::GLFW_PLATFORM_UNAVAILABLE, Error::PlatformUnavailable),
    ];

    for (raw, expected) in known {
        assert_eq!(Error::from_raw(raw), expected);
        assert_eq!(expected.as_raw(), raw);
    }
}

#[test]
fn unknown_error_codes_preserve_the_raw_value() {
    for raw in [i32::MIN, -1, 1, 0x0001_0000, 0x0001_000f, i32::MAX] {
        assert_eq!(Error::from_raw(raw), Error::Unknown(raw));
        assert_eq!(Error::from_raw(raw).as_raw(), raw);
    }
}

#[test]
fn unknown_error_display_includes_the_raw_value() {
    assert_eq!(Error::Unknown(-42).to_string(), "Unknown(-42)");
}
