use rand::Rng;

use crate::body::*;

type Vec2 = nalgebra::Vector2<f64>;

const S_SOFT: f64 = 0.01; // approx. 3 light year
const mass_sun: f64 = 1.988435e30;
const pc_in_m: f64 = 3.08567758129e16;
const gamma_si: f64 = 6.67428e-11;
const gamma: f64 =
    gamma_si / (pc_in_m * pc_in_m * pc_in_m) * mass_sun * (365.25 * 86400.0) * (365.25 * 86400.0);
const s_theta: f64 = 0.9;

mod Quadrant {
    pub const NorthWest: usize = 0;
    pub const NorthEast: usize = 1;
    pub const SouthWest: usize = 2;
    pub const SouthEast: usize = 3;
    pub const MAX: usize = 4;
    pub const INVALID: usize = 5;
}

/* https://beltoforion.de/en/barnes-hut-galaxy-simulator/
 * The Barnes-Hut Algorithm describes an effective method for solving n-body problems. It was originally published in 1986 by Josh Barnes and Piet Hut. Instead of directly summing up all forces, it is using a tree based approximation scheme which reduces the computational complexity of the problem from O(N2) to O(N log N).
 */

struct Node {
    mass: f64,
    mass_center: Vec2,
    pos_upper_bound: Vec2,
    pos_lower_bound: Vec2,
    center: Vec2,
    too_close: bool,
    nested: Option<NestedBody>,
}

type NestedQuadrants = [Option<Box<Node>>; 4];

enum NestedBody {
    Single(Body),
    Multiple(NestedQuadrants),
}

impl Default for Node {
    fn default() -> Node {
        Node {
            mass: 0.0,
            mass_center: Vec2::new(0.0, 0.0),
            pos_upper_bound: Vec2::new(0.0, 0.0),
            pos_lower_bound: Vec2::new(0.0, 0.0),
            center: Vec2::new(0.0, 0.0),
            too_close: false,
            nested: Default::default(), // [None; 4]
        }
    }
}

impl Node {
    fn new(body: Body, lower: Vec2, upper: Vec2) -> Self {
        let mut obj: Self = Default::default();
        obj.nested = Some(NestedBody::Single(body));
        obj.center = (lower + upper) / 2.0;
        obj.pos_lower_bound = lower;
        obj.pos_upper_bound = upper;
        obj
    }

    fn calcForce(&self, body: Body) -> Vec2 {
        let acc = self.calcTreeForce(body);
        /* TODO
        // calculate the force from particles not in the barnes hut tree on particle p
        if (s_renegades.size())
        {
        for (std::size_t i=0; i<s_renegades.size(); ++i)
        {
        Vec2D buf = CalcAcc(p1, s_renegades[i]);
        acc.x += buf.x;
        acc.y += buf.y;
        }
        }*/
        acc
    }
    // Compute the force acting from this node and it's child to a particle p
    fn calcTreeForce(&self, body: Body) -> Vec2 {
        match self.nested.as_ref().unwrap() {
            NestedBody::Single(body2) => self.calcAcc(body, *body2),
            NestedBody::Multiple(data) => {
                let r = (body.pos - self.mass_center)
                    .dot(&(body.pos - self.mass_center))
                    .sqrt();
                let d = self.pos_upper_bound.x - self.pos_lower_bound.x; // TODO FIXME why only x?
                if d / r <= s_theta {
                    // THE HEART OF THE ALGORITHM
                    // self.too_close = false;
                    let k = gamma * self.mass / (r * r * r);
                    let acc = k * (self.mass_center - body.pos);
                    return acc;
                } else {
                    // self.too_close = true;
                    let mut acc = Vec2::new(0.0, 0.0);
                    for i in 0..Quadrant::MAX {
                        match data[i].as_ref() {
                            Some(node) => {
                                acc += node.calcTreeForce(body);
                            }
                            None => {}
                        }
                    }
                    return acc;
                }
            }
        }
    }

    // Calculate the accelleration caused by gravitaion of p2 on p1.
    fn calcAcc(&self, body1: Body, body2: Body) -> Vec2 {
        let zeroAcc = Vec2::new(0.0, 0.0);
        if body1.pos == body2.pos {
            // same body
            return zeroAcc;
        }

        let result = body1.computeForce(&body2, gamma);
        result // valid
    }

    fn getQuadrant(&self, x: f64, y: f64) -> usize {
        if x <= self.center.x && y <= self.center.y {
            return Quadrant::SouthWest;
        } else if x <= self.center.x && y >= self.center.y {
            return Quadrant::NorthWest;
        } else if x >= self.center.x && y >= self.center.y {
            return Quadrant::NorthEast;
        } else if x >= self.center.x && y <= self.center.y {
            return Quadrant::SouthEast;
        }
        unreachable!();
        //Quadrant::INVALID
    }

    fn computeMassDistribution(&mut self) -> Vec2 {
        match &mut self.nested {
            None => {
                unreachable!();
            }
            Some(NestedBody::Multiple(data)) => {
                assert!(self.mass == 0.0);
                assert!(self.mass_center == Vec2::new(0.0, 0.0));
                for i in 0..Quadrant::MAX {
                    match data[i].as_mut() {
                        Some(node) => {
                            node.computeMassDistribution();
                            self.mass += node.mass;
                            self.mass_center += (node.mass_center * node.mass);
                        }
                        None => {}
                    }
                }
                self.mass_center /= self.mass;
            }
            Some(NestedBody::Single(data)) => {
                self.mass = data.m;
                self.mass_center = data.pos;
            }
        }
        self.mass_center
    }

    fn insertParticle(&mut self, body: Body, level: u32) {
        let quad = self.getQuadrant(body.pos.x, body.pos.y);
        if body.pos.x < self.pos_lower_bound.x
            || body.pos.x > self.pos_upper_bound.x
            || body.pos.y < self.pos_lower_bound.y
            || body.pos.y > self.pos_upper_bound.y
        {
            return;
        }

        let getNewBounds = |quad, lower: Vec2, center: Vec2, upper: Vec2| -> (Vec2, Vec2) {
            match quad {
                Quadrant::SouthWest => (lower, center),
                Quadrant::NorthWest => (Vec2::new(lower.x, center.y), Vec2::new(center.x, upper.y)),
                Quadrant::NorthEast => (center, upper),
                Quadrant::SouthEast => (Vec2::new(center.x, lower.y), Vec2::new(upper.x, center.y)),
                _ => unreachable!(),
            }
        };

        match &mut self.nested {
            None => {
                self.nested = Some(NestedBody::Single(body));
            }
            Some(NestedBody::Multiple(data)) => match &mut data[quad] {
                Some(node) => node.insertParticle(body, level + 1),
                None => {
                    let (lower, upper) = getNewBounds(
                        quad,
                        self.pos_lower_bound,
                        self.center,
                        self.pos_upper_bound,
                    );
                    data[quad] = Some(Box::new(Node::new(body, lower, upper)));
                }
            },
            Some(NestedBody::Single(data)) => {
                let prev_body = *data; // steal
                let mut quads: NestedQuadrants = Default::default();

                if prev_body.pos == body.pos {
                    panic!("two same positions"); // TODO add to renegades?
                }
                // add previosuly existing body
                let prev_quad = self.getQuadrant(prev_body.pos.x, prev_body.pos.y);
                let (lower, upper) = getNewBounds(
                    prev_quad,
                    self.pos_lower_bound,
                    self.center,
                    self.pos_upper_bound,
                );
                quads[prev_quad] = Some(Box::new(Node::new(prev_body, lower, upper)));

                // add new body
                if prev_quad != quad {
                    let (lower, upper) = getNewBounds(
                        quad,
                        self.pos_lower_bound,
                        self.center,
                        self.pos_upper_bound,
                    );
                    quads[quad] = Some(Box::new(Node::new(body, lower, upper)));
                } else {
                    match &mut quads[quad] {
                        Some(node) => {
                            node.insertParticle(body, level + 1);
                        }
                        _ => unreachable!("we allocated node with single body few lines above"),
                    }
                }

                self.nested = Some(NestedBody::Multiple(quads));
            }
        }
    }
}

#[derive(Default)]
pub struct ModelNBody {
    pos_upper_bound: Vec2,
    pos_lower_bound: Vec2,
    pub center: Vec2,
    tree: Node,
    roi: f64,
    pub bodies: Vec<Body>,
    particle_num: usize,
}

impl ModelNBody {
    fn new(particle_num: usize) -> Self {
        let mut obj: Self = Default::default();
        obj.particle_num = particle_num;
        obj.pos_upper_bound = Vec2::new(std::f64::MIN, std::f64::MIN);
        obj.pos_lower_bound = Vec2::new(std::f64::MAX, std::f64::MAX);
        obj
    }

    fn buildTree(&mut self) {
        // Reset the quadtree, make sure only particles inside the roi
        // are handled. The renegade ones may live long and prosper
        // outside my simulation
        self.tree = Node::default();
        self.tree.pos_upper_bound = self.center.add_scalar(self.roi);
        self.tree.pos_lower_bound = self.center.add_scalar(-self.roi);
        self.tree.center = (self.tree.pos_upper_bound + self.tree.pos_lower_bound) / 2.0;

        let mut u = 0;
        for body in &self.bodies {
            self.tree.insertParticle(*body, 0);
            u += 1;
        }

        self.center = self.tree.computeMassDistribution();
    }

    fn eval(&mut self) {
        self.buildTree();

        for i in 0..self.bodies.len() {
            self.bodies[i].acc = self.tree.calcForce(self.bodies[i]);
        }
    }

    fn initCollision(&mut self) {
        self.bodies.clear();
        self.bodies.reserve(self.particle_num);

        let add_body = |mass, position, velocity, radius, bodies: &mut Vec<Body>| {
            let mut new_body = Body::new(mass, radius);
            new_body.pos = position;
            new_body.vel = velocity;
            (*bodies).push(new_body);
        };

        let get_orbital_velocity = |pos1: Vec2, pos2: Vec2, m1: f64| {
            let dist: f64 = (pos1 - pos2).dot(&(pos1 - pos2)).sqrt();
            let v = (gamma * m1 / dist).sqrt();
            return Vec2::new((pos1.y - pos2.y) / dist * v, -(pos1.x - pos2.x) / dist * v);
        };

        // add black hole
        add_body(
            Mass(1000000.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
            Radius(0.5),
            &mut self.bodies,
        );

        // second black hole
        {
            let pos = Vec2::new(10.0, 10.0);
            let vel = get_orbital_velocity(self.bodies[0].pos, pos, self.bodies[0].m) * 0.9;
            add_body(
                Mass(self.bodies[0].m / 10.0),
                pos,
                vel,
                Radius(0.5),
                &mut self.bodies,
            );
        }

        // add first galaxy
        let mut rng = rand::thread_rng();
        for _ in 0..3999.min(self.particle_num - 2) {
            let rad = 10.0;
            let r = 0.1 + 0.8 * (rad * rng.gen_range(0.0..1.0));
            let a = 2.0 * std::f64::consts::PI * rng.gen_range(0.0..1.0);
            let mass = Mass(0.03 + 20.0 * rng.gen_range(0.0..1.0));
            let pos = Vec2::new(r * a.sin(), r * a.cos());
            let vel = get_orbital_velocity(self.bodies[0].pos, pos, self.bodies[0].m);

            add_body(mass, pos, vel, Radius(0.05), &mut self.bodies);

            self.pos_upper_bound.x = self.pos_upper_bound.x.max(pos.x);
            self.pos_upper_bound.y = self.pos_upper_bound.y.max(pos.y);

            self.pos_lower_bound.x = self.pos_lower_bound.x.min(pos.x);
            self.pos_lower_bound.y = self.pos_lower_bound.y.min(pos.y);
        }

        // add second galaxy
        for i in 4001..self.particle_num {
            let rad = 3.0;
            let r = 0.1 + 0.8 * (rad * rng.gen_range(0.0..1.0));
            let a = 2.0 * std::f64::consts::PI * rng.gen_range(0.0..1.0);
            let mass = Mass(0.03 + 20.0 * rng.gen_range(0.0..1.0));
            let pos = Vec2::new(
                self.bodies[1].pos.x + r * a.sin(),
                self.bodies[1].pos.y + r * a.cos(),
            );
            let vel = get_orbital_velocity(self.bodies[1].pos, pos, self.bodies[1].m)
                + self.bodies[1].vel;
            add_body(mass, pos, vel, Radius(0.05), &mut self.bodies);

            self.pos_upper_bound.x = self.pos_upper_bound.x.max(pos.x);
            self.pos_upper_bound.y = self.pos_upper_bound.y.max(pos.y);

            self.pos_lower_bound.x = self.pos_lower_bound.x.min(pos.x);
            self.pos_lower_bound.y = self.pos_lower_bound.y.min(pos.y);
        }

        // The Barnes Hut algorithm needs square shaped quadrants.
        // calculate the height of the square including all particles (and a bit more space)
        let l = 1.05
            * (self.pos_upper_bound.x - self.pos_lower_bound.x)
                .max(self.pos_upper_bound.y - self.pos_lower_bound.y);
        self.roi = l * 1.5;

        // compute the center of the region including all particles
        let c = Vec2::new(
            (self.pos_lower_bound.x + self.pos_upper_bound.x) / 2.0,
            (self.pos_lower_bound.y + self.pos_upper_bound.y) / 2.0,
        );

        self.pos_upper_bound.x = c.x + l / 2.0;
        self.pos_lower_bound.x = c.x - l / 2.0;
        self.pos_upper_bound.y = c.y + l / 2.0;
        self.pos_lower_bound.y = c.y - l / 2.0;

        //println!("initial particle distribution");
        //println!("min={:?} max={:?}", self.pos_lower_bound, self.pos_upper_bound);
        //println!("l={:?} roi={:?}", l, self.roi);
    }
}

// taken from paper MATHEMATICAL MODEL FOR THE 0.5 BILLION YEARS AGED SUN
const ApproximationsADB6: [f64; 6] = [
    4277.0 / 1440.0,
    -7923.0 / 1440.0,
    9982.0 / 1440.0,
    -7298.0 / 1440.0,
    2877.0 / 1440.0,
    -475.0 / 1440.0,
];

struct IntegratorADB6 {
    dimensions: usize,
    timestep: f64,
    data: [Vec<Body>; 6],
}

impl IntegratorADB6 {
    fn new(dimensions: usize, timestep: f64) -> Self {
        Self {
            dimensions: dimensions,
            timestep: timestep,
            data: [
                vec![Body::new(Mass(0.0), Radius(0.0)); dimensions],
                vec![Body::new(Mass(0.0), Radius(0.0)); dimensions],
                vec![Body::new(Mass(0.0), Radius(0.0)); dimensions],
                vec![Body::new(Mass(0.0), Radius(0.0)); dimensions],
                vec![Body::new(Mass(0.0), Radius(0.0)); dimensions],
                vec![Body::new(Mass(0.0), Radius(0.0)); dimensions],
            ],
        }
    }
    fn singleStep(&mut self, model: &mut ModelNBody) {
        for i in 0..self.dimensions {
            model.bodies[i].pos += self.timestep
                * (ApproximationsADB6[0] * self.data[5][i].pos
                    + ApproximationsADB6[1] * self.data[4][i].pos
                    + ApproximationsADB6[2] * self.data[3][i].pos
                    + ApproximationsADB6[3] * self.data[2][i].pos
                    + ApproximationsADB6[4] * self.data[1][i].pos
                    + ApproximationsADB6[5] * self.data[0][i].pos);
            model.bodies[i].vel += self.timestep
                * (ApproximationsADB6[0] * self.data[5][i].vel
                    + ApproximationsADB6[1] * self.data[4][i].vel
                    + ApproximationsADB6[2] * self.data[3][i].vel
                    + ApproximationsADB6[3] * self.data[2][i].vel
                    + ApproximationsADB6[4] * self.data[1][i].vel
                    + ApproximationsADB6[5] * self.data[0][i].vel);
            for j in 0..5 {
                self.data[j][i] = self.data[j + 1][i];
            }
        }
    }

    fn setInitialState(&mut self, model: &mut ModelNBody) {
        assert!(model.bodies.len() == self.dimensions);
        let initial = model.bodies.clone();

        let mut k1 = model.bodies.clone();
        let mut k2 = model.bodies.clone();
        let mut k3 = model.bodies.clone();
        let mut k4 = model.bodies.clone();

        for n in 0..5 {
            // k1
            model.eval();

            for i in 0..self.dimensions {
                k1[i].pos = model.bodies[i].vel;
                k1[i].vel = model.bodies[i].acc;
                model.bodies[i].pos = initial[i].pos + self.timestep * 0.5 * model.bodies[i].vel;
                model.bodies[i].vel = initial[i].vel + self.timestep * 0.5 * model.bodies[i].acc;
            }

            // k2
            model.eval();
            for i in 0..self.dimensions {
                k2[i].pos = model.bodies[i].vel;
                k2[i].vel = model.bodies[i].acc;
                model.bodies[i].pos = initial[i].pos + self.timestep * 0.5 * model.bodies[i].vel;
                model.bodies[i].vel = initial[i].vel + self.timestep * 0.5 * model.bodies[i].acc;
            }

            // k3
            model.eval();
            for i in 0..self.dimensions {
                k3[i].pos = model.bodies[i].vel;
                k3[i].vel = model.bodies[i].acc;
                model.bodies[i].pos = initial[i].pos + self.timestep * 0.5 * model.bodies[i].vel;
                model.bodies[i].vel = initial[i].vel + self.timestep * 0.5 * model.bodies[i].acc;
            }

            // k4
            model.eval();

            for i in 0..self.dimensions {
                k4[i].pos = model.bodies[i].vel;
                k4[i].vel = model.bodies[i].acc;
                model.bodies[i].pos = initial[i].pos
                    + self.timestep / 6.0 * (k1[i].pos + 2.0 * (k2[i].pos + k3[i].pos) + k4[i].pos);
                model.bodies[i].vel = initial[i].vel
                    + self.timestep / 6.0 * (k1[i].vel + 2.0 * (k2[i].vel + k3[i].vel) + k4[i].vel);

                self.data[n][i] = k1[i];
            }
        }

        model.eval();

        for i in 0..self.dimensions {
            self.data[5][i].pos = model.bodies[i].vel;
            self.data[5][i].vel = model.bodies[i].acc;
        }
    }
}

pub struct NBodyWnd {
    integrator: IntegratorADB6,
    pub model: ModelNBody,
}

impl NBodyWnd {
    pub fn new() -> Self {
        let mut obj = Self {
            integrator: IntegratorADB6::new(5000, 100.0),
            model: ModelNBody::new(5000),
        };
        obj.model.initCollision();
        obj.integrator.setInitialState(&mut obj.model);
        obj
    }

    pub fn render(&mut self) {
        self.integrator.singleStep(&mut self.model);
        self.model.eval();
        // TODO FIXME cover this
        for i in 0..self.integrator.dimensions {
            self.integrator.data[5][i].pos = self.model.bodies[i].vel;
            self.integrator.data[5][i].vel = self.model.bodies[i].acc;
        }
    }
}
