mod body;
mod screen;

use body::*;
use screen::Screen;

type Vec2 = nalgebra::Vector2<f32>;

struct Collision {
    scr: Screen,
    Centre1: Body,
    Bodies1: Vec<Body>,
    Centre2: Body,
    Bodies2: Vec<Body>,
    G: f32,
}

impl Collision {
    fn new(subobjects: usize) -> Collision {
        let mut obj = Collision {
            scr: Screen::new(0.0, 0.0, 5.0),
            Centre1: Body::new(2000.0, 2.5),
            Bodies1: Vec::with_capacity(subobjects),
            Centre2: Body::new(2000.0, 2.5),
            Bodies2: Vec::with_capacity(subobjects),
            G: 3.0,
        };

        // Initializing first galaxy
        obj.Centre1.pos = Vec2::new(150.0, 20.0);
        obj.Centre1.vel = Vec2::new(-5.0, 0.0);

        for i in 0..subobjects {
            let maxRadius = 30.0;
            let theta = random(0.0, 2.0 * std::f32::consts::PI);
            let mut r = random(1.0, maxRadius);
            r = r * r / maxRadius;

            let mut body = Body::new(1.0, 0.2);
            body.pos = Vec2::new(r * theta.cos(), r * theta.sin());
            body.pos += obj.Centre1.pos;

            let v = (obj.G * obj.Centre1.m / r).sqrt();
            body.vel = Vec2::new(v * theta.sin(), -v * theta.cos());

            let offset = 0.6;
            body.vel += Vec2::new(random(-offset, offset), random(-offset, offset));
            body.vel += obj.Centre1.vel;

            obj.Bodies1.push(body);
        }

        obj.Centre2.pos = -obj.Centre1.pos;
        obj.Centre2.vel = -obj.Centre1.vel;

        for i in 0..subobjects {
            let maxRadius = 30.0;
            let theta = random(0.0, 2.0 * std::f32::consts::PI);

            let mut r = random(1.0, maxRadius);
            r = r * r / maxRadius;
            r += 0.2 * obj.Centre2.r;

            let mut body = Body::new(1.0, 0.2);
            body.pos = Vec2::new(r * theta.cos(), r * theta.sin());
            body.pos += obj.Centre2.pos;

            let v = (obj.G * obj.Centre2.m / r).sqrt();
            body.vel = Vec2::new(v * theta.sin(), -v * theta.cos());
            // uncomment for opposite direction of rotation
            // body.vel = -body.vel;

            let offset = 0.6;
            body.vel += Vec2::new(random(-offset, offset), random(-offset, offset));
            body.vel += obj.Centre2.vel;

            obj.Bodies2.push(body);
        }

        obj
    }

    fn plot_body(&mut self, body: Body) {
        // TODO how to get mutable reference to body here?
        self.scr.PlotCircle(body.pos.x, body.pos.y, body.r);
    }
}

impl Scenario for Collision {
    fn process(&mut self, dt: f32) {
        // centres attract each other
        self.Centre1.PulledBy(&self.Centre2, self.G);
        self.Centre2.PulledBy(&self.Centre1, self.G);

        // particles are attracted to centres
        for i in 0..self.Bodies1.len() {
            self.Bodies1[i].PulledBy(&self.Centre1, self.G);
            self.Bodies2[i].PulledBy(&self.Centre1, self.G);
            self.Bodies1[i].PulledBy(&self.Centre2, self.G);
            self.Bodies2[i].PulledBy(&self.Centre2, self.G);
        }

        self.Centre1.Update(dt);
        self.Centre2.Update(dt);

        for i in 0..self.Bodies1.len() {
            self.Bodies1[i].Update(dt);
        }
        for i in 0..self.Bodies2.len() {
            self.Bodies2[i].Update(dt);
        }
    }

    fn draw(&mut self) {
        self.scr.Clear();
        self.plot_body(self.Centre1);
        self.plot_body(self.Centre2);

        for i in 0..self.Bodies1.len() {
            self.plot_body(self.Bodies1[i]);
        }
        for i in 0..self.Bodies2.len() {
            self.plot_body(self.Bodies2[i]);
        }

        // drawing
        if (self.Centre1.pos - self.Centre2.pos).dot(&(self.Centre1.pos - self.Centre2.pos))
            < 90.0 * 90.0
        {
            self.scr.Zoom(9.0);
        } else if (self.Centre1.pos - self.Centre2.pos).dot(&(self.Centre1.pos - self.Centre2.pos))
            > 110.0 * 110.0
        {
            self.scr.Zoom(5.0);
        }
        self.scr.Draw();
    }
}

fn random(low: f32, high: f32) -> f32 {
    let rand_max = 1.0;
    low + rand::random::<f32>() / (rand_max / (high - low))
}

fn main() {
    let mut scenario = Collision::new(20000);
    let dt = 1.0 / 40.0;

    loop {
        scenario.process(dt);
        scenario.draw();
    }
}
