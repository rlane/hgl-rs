#[no_uv];
extern mod gl;
extern mod hgl;
extern mod glfw;
extern mod extra;
extern mod native;

use std::mem::size_of;

use hgl::{Shader, Program, Triangles, Vbo, Vao};

#[link(name="glfw")]
extern {}

static VERTEX_SHADER: &'static str = "
#version 330

in vec2 position;
in vec3 color;
out vec3 Color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    Color = color;
}";

static FRAGMENT_SHADER: &'static str = "
#version 330
out vec4 out_color;
in vec3 Color;

void main() {
    out_color = vec4(Color, 1.0);
}";

#[start]
fn main(argc: int, argv: **u8) -> int {
    native::start(argc, argv, proc() {
        glfw::set_error_callback(~glfw::LogErrorHandler);
        glfw::start(proc() {
            glfw::window_hint::context_version(3, 3);
            glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);
            let window = glfw::Window::create(800, 600, "HGL", glfw::Windowed).unwrap();
            window.make_context_current();
            gl::load_with(glfw::get_proc_address);

            gl::Viewport(0, 0, 800, 600);

            let vao = Vao::new();
            vao.activate();
            let program = Program::link([Shader::compile(VERTEX_SHADER, hgl::VertexShader).ok().unwrap(),
                                         Shader::compile(FRAGMENT_SHADER, hgl::FragmentShader).ok().unwrap()]).unwrap();
            program.bind_frag(0, "out_color");
            program.activate();

            let vbo = Vbo::from_data([0.0f32,  0.5, 1.0, 0.0, 0.0,
                                      0.5,    -0.5, 0.0, 1.0, 0.0,
                                     -0.5,    -0.5, 0.0, 0.0, 1.0],
                hgl::StaticDraw).unwrap();

            vao.enable_attrib(&program, "position", 2, 5*size_of::<f32>() as i32, 0);
            vao.enable_attrib(&program, "color", 3, 5*size_of::<f32>() as i32, 2*size_of::<f32>());
            vbo.activate();

            while !window.should_close() {
                glfw::poll_events();
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                vao.draw_array(Triangles, 0, 3);
                window.swap_buffers();
            }
        });
    });
    0
}
