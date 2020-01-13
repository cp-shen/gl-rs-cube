mod gl;
use glfw::Context;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const vertexShaderSource: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn main() {
    // init glfw
    let mut _glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    _glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    _glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // window creation
    let (mut window, events) = _glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "", glfw::WindowMode::Windowed)
        .expect("failed to create glfw window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
}
