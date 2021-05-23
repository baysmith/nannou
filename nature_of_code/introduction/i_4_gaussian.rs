// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// I-4: Gaussian distribution
use nannou::prelude::*;
use nannou::rand::distributions::StandardNormal;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    let x = 0.0;
    let y = 0.0;
    let _window = app.new_window().size(640, 360).view(view).build().unwrap();
    Model { x, y }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win_rect = app.window_rect();

    let num = f32::from_f64(StdRng::from_entropy().sample(StandardNormal)).unwrap();

    let sd = 60.0;
    let mean = 0.0;
    model.x = num * sd + mean;

    model.x = model.x.clamp(win_rect.left(), win_rect.right());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(WHITE);
    }

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(16.0, 16.0)
        .rgba(0.0, 0.0, 0.0, 10.0 / 255.0);

    draw.to_frame(app, &frame).unwrap();
}
