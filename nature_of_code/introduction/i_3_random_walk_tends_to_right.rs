// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// I.3: Walker that tends to move to the right
use nannou::prelude::*;

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
    let r = random_range(0.0, 1.0);
    if r < 0.4 {
        model.x += 1.0;
    } else if r < 0.6 {
        model.x -= 1.0;
    } else if r < 0.8 {
        model.y -= 1.0;
    } else {
        model.y += 1.0;
    }
    let win_rect = app.window_rect();
    model.x = model.x.clamp(win_rect.left(), win_rect.right());
    model.y = model.y.clamp(win_rect.bottom(), win_rect.top());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(WHITE);
    }

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(2.0, 2.0)
        .rgba(0.0, 0.0, 0.0, 1.0)
        .stroke(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
