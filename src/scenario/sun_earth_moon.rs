use crate::body::{Body, Mass, Radius};
use crate::screen::Screen;
use crate::scenario::Scenario;

type Vec2 = nalgebra::Vector2<f64>;

pub struct SunEarthMoon {
    Sun: Body,
    Earth: Body,
    Moon: Body,
    G: f64,
}

impl SunEarthMoon {
    pub fn new() -> SunEarthMoon {
        let mut obj = SunEarthMoon {
            Sun: Body::new(Mass(10000.0), Radius(7.0)),
            Earth: Body::new(Mass(1000.0), Radius(2.0)),
            Moon: Body::new(Mass(1.0), Radius(1.2)),
            G: 1.0,
        };

        let r = 5.5;
        let R = 30.0;

        obj.Sun.pos = Vec2::new(0.0, 0.0);
        obj.Sun.vel = Vec2::new(0.0, 0.0);

        obj.Earth.pos = Vec2::new(R, 0.0);
        obj.Earth.vel = Vec2::new(0.0, (obj.Sun.mass / R).sqrt());

        obj.Moon.pos = Vec2::new(R, r);
        obj.Moon.vel = Vec2::new((obj.Earth.mass / r).sqrt(), obj.Earth.vel.y);

        obj
    }

    fn plot_body(&self, renderer : &mut dyn Screen, body: Body) {
        renderer.plot_circle(body.pos.x, body.pos.y, body.radius);
    }
}
impl Scenario for SunEarthMoon {
    fn process(&mut self, dt: f64) {
        self.Moon.pull_by(&self.Earth, self.G);
        self.Moon.pull_by(&self.Sun, self.G);
        self.Earth.pull_by(&self.Moon, self.G);
        self.Earth.pull_by(&self.Sun, self.G);
        self.Sun.pull_by(&self.Moon, self.G);
        self.Sun.pull_by(&self.Earth, self.G);

        self.Moon.process_forces(dt);
        self.Earth.process_forces(dt);
        self.Sun.process_forces(dt);
    }

    fn draw(&self, renderer : &mut dyn Screen) {
        renderer.clear();
        renderer.position(self.Sun.pos.x, self.Sun.pos.y);

        self.plot_body(renderer, self.Moon);
        self.plot_body(renderer, self.Earth);
        self.plot_body(renderer, self.Sun);

        renderer.draw();
    }
}

