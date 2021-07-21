use crate::body::{Body, Mass, Radius};
use crate::screen::Screen;
use crate::scenario::Scenario;

type Vec2 = nalgebra::Vector2<f32>;

pub struct SunEarthMoon {
    Sun: Body,
    Earth: Body,
    Moon: Body,
    G: f32,
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
        obj.Earth.vel = Vec2::new(0.0, (obj.Sun.m / R).sqrt());

        obj.Moon.pos = Vec2::new(R, r);
        obj.Moon.vel = Vec2::new((obj.Earth.m / r).sqrt(), obj.Earth.vel.y);

        obj
    }

    fn plot_body(&self, renderer : &mut dyn Screen, body: Body) {
        renderer.PlotCircle(body.pos.x, body.pos.y, body.r);
    }
}
impl Scenario for SunEarthMoon {
    fn process(&mut self, dt: f32) {
        self.Moon.PulledBy(&self.Earth, self.G);
        self.Moon.PulledBy(&self.Sun, self.G);
        self.Earth.PulledBy(&self.Moon, self.G);
        self.Earth.PulledBy(&self.Sun, self.G);
        self.Sun.PulledBy(&self.Moon, self.G);
        self.Sun.PulledBy(&self.Earth, self.G);

        self.Moon.Update(dt);
        self.Earth.Update(dt);
        self.Sun.Update(dt);
    }

    fn draw(&self, renderer : &mut dyn Screen) {
        renderer.Clear();
        renderer.Position(self.Sun.pos.x, self.Sun.pos.y);

        self.plot_body(renderer, self.Moon);
        self.plot_body(renderer, self.Earth);
        self.plot_body(renderer, self.Sun);

        renderer.Draw();
    }
}

