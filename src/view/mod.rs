use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};

const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 768.0;

const TITLE: &'static str = "Streaming Library Viewer";

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
        unsafe { gl::ClearColor(0.2, 0.3, 0.3, 1.0) }
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) }

        self.windowed_context.swap_buffers().unwrap();
    }
}
