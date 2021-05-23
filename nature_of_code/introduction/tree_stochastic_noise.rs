// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou::rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    yoff: f32,
    seed: u64,
    noise: Perlin,
}

fn model(app: &App) -> Model {
    let yoff = 0.0;
    let seed = 5;

    let _window = app
        .new_window()
        .size(380, 200)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();
    Model {
        yoff,
        seed,
        noise: Perlin::new(),
    }
}

fn branch(
    h: f32,
    xoff: f32,
    yoff: f32,
    pos: Point2<f32>,
    angle: f32,
    draw: &Draw,
    random: &mut StdRng,
    noise: &Perlin,
) {
    let sw = map_range(h, 2.0, 100.0, 1.0, 5.0);
    let end = pos + pt2(h * angle.sin(), h * angle.cos());
    draw.line()
        .start(pos)
        .end(end)
        .weight(sw)
        .rgba(0.0, 0.5, 0.5, 1.0);

    let h = h * 0.7;
    let xoff = xoff + 0.1;
    if h > 4.0 {
        let n = random.gen_range(0, 5) as u8;
        for i in 0..n {
            let noise_value = noise.get([(xoff + i as f32) as f64, yoff as f64]);
            let mut theta = map_range(noise_value, -1.0, 1.0, 0.0, PI / 9.0);
            if n % 2 == 0 {
                theta *= -1.0;
            }

            let angle = angle + theta;
            branch(h, xoff, yoff, end, angle, &draw, random, noise);
        }
    }
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    model.yoff = random_range(0.0, 1000.0);
    model.seed = (app.time * 1000.0) as u64;
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.yoff += 0.005;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let win_rect = app.window_rect();
    let pos = pt2(0.0, win_rect.bottom());
    let angle = 0.0;

    let mut random = StdRng::seed_from_u64(model.seed);

    branch(
        60.0,
        0.0,
        model.yoff,
        pos,
        angle,
        &draw,
        &mut random,
        &model.noise,
    );

    draw.to_frame(app, &frame).unwrap();
}
