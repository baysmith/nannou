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
    mover: Mover,
}

struct Mover {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    top_speed: f32,
    noise_x: [f64; 2],
    noise_y: [f64; 2],
    noise_magnitude: [f64; 2],
    noise: Perlin,
}

impl Mover {
    fn new(_rect: Rect) -> Self {
        let position = pt2(0.0, 0.0);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let top_speed = 6.0;
        let noise_x = [random_range(0.0, 1000.0), random_range(0.0, 1000.0)];
        let noise_y = [random_range(0.0, 1000.0), random_range(0.0, 1000.0)];
        let noise_magnitude = [random_range(0.0, 1000.0), random_range(0.0, 1000.0)];
        Mover {
            position,
            velocity,
            acceleration,
            top_speed,
            noise_x,
            noise_y,
            noise_magnitude,
            noise: Perlin::new(),
        }
    }

    fn update(&mut self) {
        let accel = vec2(self.noise.get(self.noise_x) as f32, self.noise.get(self.noise_y) as f32);
        let accel_mag = self.noise.get(self.noise_magnitude) as f32;
        self.noise_x[0] += 0.01;
        self.noise_x[1] += 0.05;
        self.noise_y[0] += 0.01;
        self.noise_y[1] += 0.05;
        self.noise_magnitude[0] += 0.01;
        self.noise_magnitude[1] += 0.01;

        self.acceleration = accel * accel_mag;
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit_magnitude(self.top_speed);
        self.position += self.velocity;
    }

    fn check_edges(&mut self, rect: Rect) {
        if self.position.x > rect.right() {
            self.position.x = rect.left();
        } else if self.position.x < rect.left() {
            self.position.x = rect.right();
        }
        if self.position.y > rect.top() {
            self.position.y = rect.bottom();
        } else if self.position.y < rect.bottom() {
            self.position.y = rect.top();
        }
    }

    fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.ellipse()
            .xy(self.position)
            .w_h(48.0, 48.0)
            .gray(0.5)
            .stroke(BLACK)
            .stroke_weight(2.0);
    }
}

fn model(app: &App) -> Model {
    app.new_window().size(640, 360).view(view).build().unwrap();
    let mover = Mover::new(app.window_rect());
    Model { mover }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    m.mover.update();
    m.mover.check_edges(app.window_rect());
}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    m.mover.display(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
