set(GLFW_RS_EXAMPLES
    callbacks
    clipboard
    cursor
    defaults
    manual_init
    modes
    title
    version
    window
)

foreach(example ${GLFW_RS_EXAMPLES})
    add_custom_target(
        ${example} DEPENDS lib
        COMMAND mkdir -p ${GLFW_RS_EXAMPLE_DIR} && ${RUSTC_COMPILER} -L ${GLFW_RS_LIB_DIR}
            --link-args="${GLFW_STATIC_LDFLAGS}" --out-dir ${GLFW_RS_EXAMPLE_DIR}
            ${CMAKE_CURRENT_SOURCE_DIR}/${example}.rs
    )
endforeach()

add_custom_target(
    examples ALL
    DEPENDS lib ${GLFW_RS_EXAMPLES}
)
