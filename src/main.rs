use crate::view::GlutinLibraryViewer;
use glutin::event_loop::{ControlFlow, EventLoop};

mod view;

fn main() {
    let el = EventLoop::new();

    // create models
    // create view (needs models)
    let view = GlutinLibraryViewer::new(&el);

    el.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        view.update(event, control_flow);
        view.render();
    });
}
