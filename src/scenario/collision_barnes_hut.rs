use crate::scenario::Scenario;
use crate::screen::Screen;
use crate::barnes_hut::{GAMMA, BarnesHutSimulation, IntegratorADB6};
use crate::body::*;
use rand::Rng;


type Vec2 = nalgebra::Vector2<f64>;

pub struct GalaxyCollisionBarnesHut {
    integrator: IntegratorADB6,
    model: BarnesHutSimulation,
}

fn get_galaxies() -> Vec<Body> {
    let mut bodies = Vec::new();

    let create_body = |mass, position, velocity, radius| {
        let mut new_body = Body::new(mass, radius);
        new_body.pos = position;
        new_body.vel = velocity;
        return new_body;
    };

    let get_orbital_velocity = |pos1: Vec2, pos2: Vec2, m1: f64| {
        let dist: f64 = (pos1 - pos2).dot(&(pos1 - pos2)).sqrt();
        let v = (GAMMA * m1 / dist).sqrt();
        return Vec2::new((pos1.y - pos2.y) / dist * v, -(pos1.x - pos2.x) / dist * v);
    };

    // add black hole
    let black_hole1 = create_body(
        Mass(1000000.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        Radius(0.5)
    );

    // second black hole
    let black_hole2 = (|| {
        let pos = Vec2::new(10.0, 10.0);
        let vel = get_orbital_velocity(black_hole1.pos, pos, black_hole1.mass) * 0.9;
        return create_body(
            Mass(black_hole1.mass / 10.0),
            pos,
            vel,
            Radius(0.5),
        );
    })();

    // add first galaxy
    let mut rng = rand::thread_rng();
    let galaxy1 = (0..3999).map(|_| {
        let rad = 10.0;
        let r = 0.1 + 0.8 * (rad * rng.gen_range(0.0..1.0));
        let a = 2.0 * std::f64::consts::PI * rng.gen_range(0.0..1.0);
        let mass = Mass(0.03 + 20.0 * rng.gen_range(0.0..1.0));
        let pos = Vec2::new(r * a.sin(), r * a.cos());
        let vel = get_orbital_velocity(black_hole1.pos, pos, black_hole1.mass);
        return create_body(mass, pos, vel, Radius(0.05));
    }).collect::<Vec<Body>>();

    // add second galaxy
    let galaxy2 : Vec<Body> = (4001..5000).map(|_| { 
        let rad = 3.0;
        let r = 0.1 + 0.8 * (rad * rng.gen_range(0.0..1.0));
        let a = 2.0 * std::f64::consts::PI * rng.gen_range(0.0..1.0);
        let mass = Mass(0.03 + 20.0 * rng.gen_range(0.0..1.0));
        let pos = Vec2::new(
            black_hole2.pos.x + r * a.sin(),
            black_hole2.pos.y + r * a.cos(),
        );
        let vel = get_orbital_velocity(black_hole2.pos, pos, black_hole2.mass)
            + black_hole2.vel;
        return create_body(mass, pos, vel, Radius(0.05));
    }).collect();

    bodies.push(black_hole1);
    bodies.push(black_hole2);
    bodies.extend(galaxy1);
    bodies.extend(galaxy2);
    return bodies;
}

impl GalaxyCollisionBarnesHut {
    pub fn new() -> Self {
        let bodies = get_galaxies();
        let mut obj = Self {
            integrator: IntegratorADB6::new(bodies.len(), 100.0),
            model: BarnesHutSimulation::new(bodies),
        };
        obj.integrator.set_initial_state(&mut obj.model);
        obj
    }

}
impl Scenario for GalaxyCollisionBarnesHut {
    fn process(&mut self, dt: f64) {
        self.integrator.integrate(dt, &mut self.model);
        self.model.eval();
    }

    fn draw(&self, renderer: &mut dyn Screen) {
        renderer.clear();
        renderer.position(self.model.center.x, self.model.center.y);

        for i in 0..self.model.bodies.len() {
            renderer.plot_circle(
                self.model.bodies[i].pos.x,
                self.model.bodies[i].pos.y,
                self.model.bodies[i].radius,
            );
        }

        /*
        // drawing
        if (self.model.bodies[0].pos - self.model.bodies[4000].pos).dot(&(self.model.bodies[0].pos - self.model.bodies[4000].pos))
            < 90.0 * 90.0
        {
            renderer.set_zoom(5.0);
        } else if (self.model.bodies[0].pos - self.model.bodies[4000].pos).dot(&(self.model.bodies[0].pos - self.model.bodies[4000].pos))
            > 110.0 * 110.0
        {
            renderer.set_zoom(2.0);
        }*/
        renderer.draw();
    }
}
