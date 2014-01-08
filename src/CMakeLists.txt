add_custom_target(
    lib ALL 
    COMMAND mkdir -p ${GLFW_RS_LIB_DIR} && ${RUSTC_COMPILER}
        --link-args="${GLFW_STATIC_LDFLAGS}" --out-dir ${GLFW_RS_LIB_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}/lib.rs
)
