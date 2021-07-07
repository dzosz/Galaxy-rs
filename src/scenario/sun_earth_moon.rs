use crate::body::Body;
use crate::screen::Screen;
use crate::scenario::Scenario;

type Vec2 = nalgebra::Vector2<f32>;

pub struct SunEarthMoon {
    Sun: Body,
    Earth: Body,
    Moon: Body,
    G: f32,
}

/* In the restricted three-body problem, a body of negligible mass (the "planetoid") moves under the influence of two massive bodies. Having negligible mass, the force that the planetoid exerts on the two massive bodies may be neglected, and the system can be analysed and can therefore be described in terms of a two-body motion.
 * For simplicity, choose units such that the distance between the two massive bodies, as well as the gravitational constant, are both equal to 1. */
impl SunEarthMoon {
    pub fn new() -> SunEarthMoon {
        let mut obj = SunEarthMoon {
            Sun: Body::new(10000.0, 7.0),
            Earth: Body::new(1000.0, 2.0),
            Moon: Body::new(1.0, 1.2),
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

    fn plot_body(&mut self, renderer : &mut impl Screen, body: Body) {
        // TODO how to get mutable reference to body here?
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

    fn draw(&mut self, renderer : &mut impl Screen) {
        renderer.Clear();
        renderer.Position(self.Sun.pos.x, self.Sun.pos.y);

        self.plot_body(renderer, self.Moon);
        self.plot_body(renderer, self.Earth);
        self.plot_body(renderer, self.Sun);

        renderer.Draw();
    }
}

