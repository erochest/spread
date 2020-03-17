use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(PURPLE);
}
