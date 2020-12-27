use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};

pub(crate) fn process_events(
    windowed_context: &ContextWrapper<PossiblyCurrent, Window>,
    event: Event<()>,
    control_flow: &mut ControlFlow,
) {
    match event {
        Event::LoopDestroyed => return,
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::RedrawRequested(_) => {}
        _ => (),
    }
}
