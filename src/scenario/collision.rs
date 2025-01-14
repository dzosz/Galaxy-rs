use crate::body::{Body, Mass, Radius};
use crate::screen::Screen;
use crate::scenario::Scenario;

type Vec2 = nalgebra::Vector2<f64>;

pub struct Collision {
    Centre1: Body,
    Bodies1: Vec<Body>,
    Centre2: Body,
    Bodies2: Vec<Body>,
    G: f64,
}

/*
 * The Barnes-Hut Algorithm describes an effective method for solving n-body problems.
 * It works by reducing the number of force calculations by grouping particles. The basic idea behind the algorithm is that the force which a particle group excerts on a single particle can be approximated by the force of a pseudo particle located at the groups center of mass. For instance, the force which the Andromeda galaxy excerts on the milky way can be approximated by a point mass located at the centre of the Andromeda galaxy. There is no need to integrate over all stars in the Andromeda galaxy provided the distance between the two galaxies is large enough. This approximation is valid as long as the distance from a point group to a particle is large and the radius of the group is small in relation to the distance between the group and the particle.
 */
impl Collision {
    pub fn new(subobjects: usize) -> Collision {
        let mut obj = Collision {
            Centre1: Body::new(Mass(2000.0), Radius(2.5)),
            Bodies1: Vec::with_capacity(subobjects),
            Centre2: Body::new(Mass(2000.0), Radius(2.5)),
            Bodies2: Vec::with_capacity(subobjects),
            G: 3.0,
        };

        // Initializing first galaxy
        obj.Centre1.pos = Vec2::new(150.0, 20.0);
        obj.Centre1.vel = Vec2::new(-5.0, 0.0);

        for _ in 0..subobjects {
            let maxRadius = 30.0;
            let theta = random(0.0, 2.0 * std::f64::consts::PI);
            let mut r = random(1.0, maxRadius);
            r = r * r / maxRadius;

            let mut body = Body::new(Mass(1.0), Radius(0.2));
            body.pos = Vec2::new(r * theta.cos(), r * theta.sin());
            body.pos += obj.Centre1.pos;

            let v = (obj.G * obj.Centre1.mass / r).sqrt();
            body.vel = Vec2::new(v * theta.sin(), -v * theta.cos());

            let offset = 0.6;
            body.vel += Vec2::new(random(-offset, offset), random(-offset, offset));
            body.vel += obj.Centre1.vel;

            obj.Bodies1.push(body);
        }

        obj.Centre2.pos = -obj.Centre1.pos;
        obj.Centre2.vel = -obj.Centre1.vel;

        for _ in 0..subobjects {
            let maxRadius = 30.0;
            let theta = random(0.0, 2.0 * std::f64::consts::PI);

            let mut r = random(1.0, maxRadius);
            r = r * r / maxRadius;
            r += 0.2 * obj.Centre2.radius;

            let mut body = Body::new(Mass(1.0), Radius(0.2));
            body.pos = Vec2::new(r * theta.cos(), r * theta.sin());
            body.pos += obj.Centre2.pos;

            let v = (obj.G * obj.Centre2.mass / r).sqrt();
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

    fn plot_body(&self, renderer : &mut dyn Screen, body: Body) {
        // TODO how to get mutable reference to body here?
        renderer.plot_circle(body.pos.x, body.pos.y, body.radius);
    }
}

impl Scenario for Collision {
    fn process(&mut self, dt: f64) {
        // centres attract each other
        self.Centre1.pull_by(&self.Centre2, self.G);
        self.Centre2.pull_by(&self.Centre1, self.G);

        // particles are attracted to centres
        for i in 0..self.Bodies1.len() {
            self.Bodies1[i].pull_by(&self.Centre1, self.G);
            self.Bodies2[i].pull_by(&self.Centre1, self.G);
            self.Bodies1[i].pull_by(&self.Centre2, self.G);
            self.Bodies2[i].pull_by(&self.Centre2, self.G);
        }

        self.Centre1.process_forces(dt);
        self.Centre2.process_forces(dt);

        for i in 0..self.Bodies1.len() {
            self.Bodies1[i].process_forces(dt);
        }
        for i in 0..self.Bodies2.len() {
            self.Bodies2[i].process_forces(dt);
        }
    }

    fn draw(&self, renderer : &mut dyn Screen) {
        renderer.clear();
        self.plot_body(renderer, self.Centre1);
        self.plot_body(renderer, self.Centre2);

        for i in 0..self.Bodies1.len() {
            self.plot_body(renderer, self.Bodies1[i]);
        }
        for i in 0..self.Bodies2.len() {
            self.plot_body(renderer, self.Bodies2[i]);
        }

        // drawing
        if (self.Centre1.pos - self.Centre2.pos).dot(&(self.Centre1.pos - self.Centre2.pos))
            < 90.0 * 90.0
        {
            renderer.set_zoom(5.0);
        } else if (self.Centre1.pos - self.Centre2.pos).dot(&(self.Centre1.pos - self.Centre2.pos))
            > 110.0 * 110.0
        {
            renderer.set_zoom(2.0);
        }
        renderer.draw();
    }
}

fn random(low: f64, high: f64) -> f64 {
    let rand_max = 1.0;
    low + rand::random::<f64>() / (rand_max / (high - low))
}

