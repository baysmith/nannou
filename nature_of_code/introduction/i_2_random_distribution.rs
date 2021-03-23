// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// I-2: Random Distribution
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug)]
struct Model {
    random_counts: [u32; 20],
}

fn model(app: &App) -> Model {
    let random_counts = [0; 20];

    let _window = app.new_window().size(640, 360).view(view).build().unwrap();
    Model { random_counts }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.random_counts[random_range(0, model.random_counts.len())] += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let win_rect = app.window_rect();
    let width = win_rect.w() / model.random_counts.len() as f32;

    for (i, count) in model.random_counts.iter().enumerate() {
        draw.rect()
            .x_y(
                win_rect.left() + width / 2.0 + i as f32 * width,
                win_rect.bottom(),
            )
            .w_h(width, *count as f32)
            .rgba(0.5, 0.5, 0.5, 1.0)
            .stroke_weight(2.0)
            .stroke(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
