use crate::models::BrowsableLibraryModel;
use glutin::dpi::LogicalSize;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};

mod open_gl;

const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 768.0;

const TITLE: &'static str = "Streaming Library Viewer";

pub struct GlutinLibraryViewer {
    windowed_context: ContextWrapper<PossiblyCurrent, Window>,
    model: Box<dyn BrowsableLibraryModel>,
}

impl GlutinLibraryViewer {
    pub(crate) fn new(el: &EventLoop<()>, model: Box<dyn BrowsableLibraryModel>) -> Self {
        // build and configure window
        let wb = WindowBuilder::new()
            .with_title(TITLE)
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT));

        // build and configure context, make current
        let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        // initialize opengl
        let _ = gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

        Self {
            windowed_context,
            model,
        }
    }

    pub(crate) fn update(&self, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => self.windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => self.process_keycode(virtual_code, state, control_flow),
                _ => (),
            },
            _ => (),
        }
    }

    fn process_keycode(
        &self,
        virtual_code: VirtualKeyCode,
        state: ElementState,
        control_flow: &mut ControlFlow,
    ) {
        match (virtual_code, state) {
            (VirtualKeyCode::Escape, _) => *control_flow = ControlFlow::Exit,
            (VirtualKeyCode::Left, ElementState::Pressed) => self.model.previous_title(),
            (VirtualKeyCode::Right, ElementState::Pressed) => self.model.next_title(),
            (VirtualKeyCode::Up, ElementState::Pressed) => self.model.previous_collection(),
            (VirtualKeyCode::Down, ElementState::Pressed) => self.model.next_collection(),
            (VirtualKeyCode::Return, ElementState::Pressed) => self.model.current_title(),
            _ => (),
        }
    }

    pub(crate) fn render(&self) {
        unsafe { open_gl::draw() };

        self.windowed_context.swap_buffers().unwrap();
    }
}
