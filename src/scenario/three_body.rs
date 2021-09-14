use crate::body::{Body, Mass, Radius};
use crate::screen::Screen;
use crate::scenario::Scenario;

type Vec2 = nalgebra::Vector2<f64>;

pub struct ThreeBody {
    solarSystem: [Body; 3],
    G: f64,
}

impl ThreeBody {
    pub fn new() -> ThreeBody {
        let mut obj = ThreeBody {
            solarSystem: [
                Body::new(Mass(1.0), Radius(0.1)),
                Body::new(Mass(1.0), Radius(0.1)),
                Body::new(Mass(1.0), Radius(0.1)),
            ],
            G: 1.0,
        };

        obj.solarSystem[0].pos = Vec2::new(-0.9700436, 0.24308753);
        obj.solarSystem[0].vel = Vec2::new(0.4662036850, 0.4323657300);

        obj.solarSystem[1].pos = Vec2::new(0.0, 0.0);
        obj.solarSystem[1].vel = Vec2::new(-0.93240737, -0.86473146);

        obj.solarSystem[2].pos = Vec2::new(0.9700436, -0.24308753);
        obj.solarSystem[2].vel = Vec2::new(0.4662036850, 0.4323657300);

        obj
    }

    fn plot_body(&self, renderer : &mut dyn Screen, body: Body) {
        let O = body.pos;
        let X = body.pos + 0.5 * body.vel;

        renderer.plot_circle(body.pos.x, body.pos.y, body.radius);
        renderer.plot_line(O.x, O.y, X.x, X.y);

        let mut a = O - X;
        a.normalize_mut();
        a *= 0.1;
        let b = Vec2::new(a.y, -a.x);

        renderer
            .plot_line(X.x, X.y, X.x + a.x + b.x, X.y + a.y + b.y);
        renderer
            .plot_line(X.x, X.y, X.x + a.x - b.x, X.y + a.y - b.y);
    }
}

impl Scenario for ThreeBody {
    fn process(&mut self, dt : f64) {
        for i in 0..(self.solarSystem.len()-1) {
            for j in i+1..self.solarSystem.len() {
                let (left, right) = self.solarSystem.split_at_mut(i+1);
                let idx2 = j - i - 1;
                left[i].pull_by(&right[idx2], self.G);
                right[idx2].pull_by(&left[i], self.G);
            }
        }

        for i in 0..self.solarSystem.len() {
            self.solarSystem[i].process_forces(dt);
        }
    }

    fn draw(&self, renderer : &mut dyn Screen) {
        renderer.clear();
        for i in 0..self.solarSystem.len() {
            self.plot_body(renderer, self.solarSystem[i]);
        }
        renderer.draw();
    }
}

