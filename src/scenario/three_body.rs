use crate::body::Body;
use crate::screen::Screen;
use crate::scenario::Scenario;

type Vec2 = nalgebra::Vector2<f32>;

pub struct ThreeBody {
    solarSystem: [Body; 3],
    G: f32,
}

impl ThreeBody {
    pub fn new() -> ThreeBody {
        let mut obj = ThreeBody {
            solarSystem: [
                Body::new(1.0, 0.1),
                Body::new(1.0, 0.1),
                Body::new(1.0, 0.1),
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

    fn plot_body(&self, renderer : &mut impl Screen, body: Body) { // TODO how to get mutable reference to body here?
        let O = body.pos;
        let X = body.pos + 0.5 * body.vel;

        renderer.PlotCircle(body.pos.x, body.pos.y, body.r);
        renderer.PlotLine(O.x, O.y, X.x, X.y);

        let mut a = O - X;
        a.normalize_mut();
        a *= 0.1;
        let b = Vec2::new(a.y, -a.x);

        renderer
            .PlotLine(X.x, X.y, X.x + a.x + a.x, X.y + a.y + b.y);
        renderer
            .PlotLine(X.x, X.y, X.x + a.x - b.x, X.y + a.y - b.y);
    }
}

impl Scenario for ThreeBody {
    fn process(&mut self, dt : f32) {
        for i in 0..self.solarSystem.len() {
            for j in 0..self.solarSystem.len() {
                if i == j {
                    continue;
                } else if i < j {
                    let (left, right) = self.solarSystem.split_at_mut(j);
                    left[i].PulledBy(&right[0], self.G);
                } else {
                    let (left, right) = self.solarSystem.split_at_mut(i);
                    right[0].PulledBy(&left[j], self.G);
                }
            }
        }

        for i in 0..self.solarSystem.len() {
            self.solarSystem[i].Update(dt);
        }
    }

    fn draw(&self, renderer : &mut impl Screen) {
        renderer.Clear();
        for i in 0..self.solarSystem.len() {
            self.plot_body(renderer, self.solarSystem[i]);
        }
        renderer.Draw();
    }
}

