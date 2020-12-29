use crate::models::LibraryModel;
use crate::view::GlutinLibraryViewer;
use glutin::event_loop::{ControlFlow, EventLoop};

mod models;
mod view;

fn main() {
    let el = EventLoop::new();

    // create models
    let model = LibraryModel::new();

    // create view (needs models)
    let view = GlutinLibraryViewer::new(&el, Box::new(model));

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        view.update(event, control_flow);
        view.render();
    });
}
