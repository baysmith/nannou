// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
    noise_x: [f64; 2],
    noise_y: [f64; 2],
    noise: Perlin,
}

fn model(app: &App) -> Model {
    let x = -400.0;
    let y = -100.0;
    let noise_x = [random_range(0.0, 1000.0), random_range(0.0, 1000.0)];
    let noise_y = [random_range(0.0, 1000.0), random_range(0.0, 1000.0)];

    let _window = app.new_window().size(800, 200).view(view).build().unwrap();
    Model {
        x,
        y,
        noise_x,
        noise_y,
        noise: Perlin::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win_rect = app.window_rect();

    let n = model.noise.get(model.noise_x);
    model.x = map_range(n, -1.0, 1.0, win_rect.left(), win_rect.right());
    model.noise_x[0] += 0.01;
    model.noise_x[1] += 0.01;
    let n = model.noise.get(model.noise_y);
    model.y = map_range(n, -1.0, 1.0, win_rect.bottom(), win_rect.top());
    model.noise_y[0] += 0.01;
    model.noise_y[1] += 0.01;

    model.x = model.x.clamp(win_rect.left(), win_rect.right());
    model.y = model.y.clamp(win_rect.bottom(), win_rect.top());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(48.0, 48.0)
        .rgba(0.5, 0.5, 0.5, 1.0)
        .stroke_weight(2.0)
        .stroke(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
