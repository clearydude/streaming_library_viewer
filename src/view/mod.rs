use gl::types::*;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use std::ffi::CString;
use std::os::raw::c_void;
use std::{mem, ptr};

const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 768.0;

const TITLE: &'static str = "Streaming Library Viewer";

// TODO SHADER ERROR CHECKING
unsafe fn compile_vertex_shader() -> u32 {
    let vertex_shader_source: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        void main() {
           gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }
    "#;

    let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
    let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
    gl::ShaderSource(vertex_shader_id, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertex_shader_id);

    vertex_shader_id
}

unsafe fn compile_fragment_shader() -> u32 {
    let fragment_shader_source: &str = r#"
        #version 330 core
        out vec4 FragColor;
        void main() {
           FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
        }
    "#;

    let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
    gl::ShaderSource(fragment_shader_id, 1, &c_str_frag.as_ptr(), ptr::null());
    gl::CompileShader(fragment_shader_id);

    fragment_shader_id
}

unsafe fn create_shader_program(vertex_shader_id: u32, fragment_shader_id: u32) -> u32 {
    let shader_program_id = gl::CreateProgram();
    gl::AttachShader(shader_program_id, vertex_shader_id);
    gl::AttachShader(shader_program_id, fragment_shader_id);
    gl::LinkProgram(shader_program_id);

    // clean up after we've created it
    gl::DeleteShader(vertex_shader_id);
    gl::DeleteShader(fragment_shader_id);

    shader_program_id
}

unsafe fn create_vertex_array() -> u32 {
    // HINT: type annotation is crucial since default for float literals is f64
    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
    ];
    let indices = [
        // note that we start from 0!
        0, 1, 3, // first Triangle
        1, 2, 3, // second Triangle
    ];

    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &indices[0] as *const i32 as *const c_void,
        gl::STATIC_DRAW,
    );

    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0);

    // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
    // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
    gl::BindVertexArray(0);

    vao
}

pub struct GlutinLibraryViewer {
    windowed_context: ContextWrapper<PossiblyCurrent, Window>,
}

impl GlutinLibraryViewer {
    pub(crate) fn new(el: &EventLoop<()>) -> Self {
        // build and configure window
        let wb = WindowBuilder::new()
            .with_title(TITLE)
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT));

        // build and configure context, make current
        let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        // initialize opengl
        let _ = gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

        Self { windowed_context }
    }

    pub(crate) fn update(&self, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => self.windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    }

    pub(crate) fn render(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw some shit
            gl::UseProgram(create_shader_program(
                compile_vertex_shader(),
                compile_fragment_shader(),
            ));
            gl::BindVertexArray(create_vertex_array());
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        self.windowed_context.swap_buffers().unwrap();
    }
}
