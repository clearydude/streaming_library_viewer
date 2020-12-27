mod controller;
mod view;

use glutin::dpi::LogicalSize;

use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 768.0;

const TITLE: &'static str = "Streaming Library Viewer";

fn main() {
    // create event loop
    let el = EventLoop::new();

    // build and configure window
    let wb = WindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT));

    // build and configure context, make current
    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // initialize opengl
    let _gl = gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    // here's our event loop
    el.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        controller::process_events(&windowed_context, event, control_flow);

        view::render();

        windowed_context.swap_buffers().unwrap();
    });
}
