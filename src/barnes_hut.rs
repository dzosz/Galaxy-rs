use crate::body::*;

type Vec2 = nalgebra::Vector2<f64>;

const MASS_SUN: f64 = 1.988435e30;
const PC_IN_M: f64 = 3.08567758129e16; //???
const GAMMA_SI: f64 = 6.67428e-11;
pub const GAMMA: f64 =
    GAMMA_SI / (PC_IN_M * PC_IN_M * PC_IN_M) * MASS_SUN * (365.25 * 86400.0) * (365.25 * 86400.0);

mod quadrant {
    pub const NORTH_WEST: usize = 0;
    pub const NORTH_EAST: usize = 1;
    pub const SOUTH_WEST: usize = 2;
    pub const SOUTH_EAST: usize = 3;
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

    fn calculate_force(&self, body: Body) -> Vec2 {
        let acc = self.calculate_force_on_tree(body);
        // calculate the force from particles not in the barnes hut tree on particle p
        /*
        for (std::size_t i=0; i<s_renegades.size(); ++i)
        {
        Vec2D buf = CalcAcc(p1, s_renegades[i]);
        acc.x += buf.x;
        acc.y += buf.y;
        }*/
        acc
    }
    // Compute the force acting from this node and it's child to a particle p
    fn calculate_force_on_tree(&self, body: Body) -> Vec2 {
        match self.nested.as_ref().unwrap() {
            NestedBody::Single(body2) => self.calculate_acceleration(body, *body2),
            NestedBody::Multiple(data) => {
                let r = (body.pos - self.mass_center)
                    .dot(&(body.pos - self.mass_center))
                    .sqrt();
                let d = self.pos_upper_bound.x - self.pos_lower_bound.x; // TODO FIXME why only x?
                const S_THETA: f64 = 0.9;
                if d / r <= S_THETA {
                    // THE HEART OF THE ALGORITHM
                    // self.too_close = false;
                    let k = GAMMA * self.mass / (r * r * r);
                    let acc = k * (self.mass_center - body.pos);
                    return acc;
                } else {
                    // self.too_close = true;
                    let mut acc = Vec2::new(0.0, 0.0);
                    for i in 0..quadrant::MAX {
                        match data[i].as_ref() {
                            Some(node) => {
                                acc += node.calculate_force_on_tree(body);
                            }
                            None => {}
                        }
                    }
                    return acc;
                }
            }
        }
    }

    fn calculate_acceleration(&self, body1: Body, body2: Body) -> Vec2 {
        if body1.pos == body2.pos {
            // same body
            return Vec2::new(0.0, 0.0);
        }

        return body1.compute_force(&body2, GAMMA);
    }

    fn get_quadrant(&self, x: f64, y: f64) -> usize {
        if x <= self.center.x && y <= self.center.y {
            return quadrant::SOUTH_WEST;
        } else if x <= self.center.x && y >= self.center.y {
            return quadrant::NORTH_WEST;
        } else if x >= self.center.x && y >= self.center.y {
            return quadrant::NORTH_EAST;
        } else if x >= self.center.x && y <= self.center.y {
            return quadrant::SOUTH_EAST;
        }
        unreachable!();
        //quadrant::INVALID
    }

    fn compute_mass_distribution(&mut self) -> Vec2 {
        match &mut self.nested {
            None => {
                unreachable!();
            }
            Some(NestedBody::Multiple(data)) => {
                assert!(self.mass == 0.0);
                assert!(self.mass_center == Vec2::new(0.0, 0.0));
                for i in 0..quadrant::MAX {
                    match data[i].as_mut() {
                        Some(node) => {
                            node.compute_mass_distribution();
                            self.mass += node.mass;
                            self.mass_center += node.mass_center * node.mass;
                        }
                        None => {}
                    }
                }
                self.mass_center /= self.mass;
            }
            Some(NestedBody::Single(data)) => {
                self.mass = data.mass;
                self.mass_center = data.pos;
            }
        }
        self.mass_center
    }

    fn insert_particle(&mut self, body: Body) {
        let quad = self.get_quadrant(body.pos.x, body.pos.y);
        if body.pos.x < self.pos_lower_bound.x
            || body.pos.x > self.pos_upper_bound.x
            || body.pos.y < self.pos_lower_bound.y
            || body.pos.y > self.pos_upper_bound.y
        {
            return;
        }

        let get_new_quadrant_bounds =
            |quad, lower: Vec2, center: Vec2, upper: Vec2| -> (Vec2, Vec2) {
                match quad {
                    quadrant::SOUTH_WEST => (lower, center),
                    quadrant::NORTH_WEST => {
                        (Vec2::new(lower.x, center.y), Vec2::new(center.x, upper.y))
                    }
                    quadrant::NORTH_EAST => (center, upper),
                    quadrant::SOUTH_EAST => {
                        (Vec2::new(center.x, lower.y), Vec2::new(upper.x, center.y))
                    }
                    _ => unreachable!(),
                }
            };

        match &mut self.nested {
            None => {
                self.nested = Some(NestedBody::Single(body));
            }
            Some(NestedBody::Multiple(data)) => match &mut data[quad] {
                Some(node) => node.insert_particle(body),
                None => {
                    let (lower, upper) = get_new_quadrant_bounds(
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
                let prev_quad = self.get_quadrant(prev_body.pos.x, prev_body.pos.y);
                let (lower, upper) = get_new_quadrant_bounds(
                    prev_quad,
                    self.pos_lower_bound,
                    self.center,
                    self.pos_upper_bound,
                );
                quads[prev_quad] = Some(Box::new(Node::new(prev_body, lower, upper)));

                // add new body
                if prev_quad != quad {
                    let (lower, upper) = get_new_quadrant_bounds(
                        quad,
                        self.pos_lower_bound,
                        self.center,
                        self.pos_upper_bound,
                    );
                    quads[quad] = Some(Box::new(Node::new(body, lower, upper)));
                } else {
                    match &mut quads[quad] {
                        Some(node) => {
                            node.insert_particle(body);
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
pub struct BarnesHutSimulation {
    pos_upper_bound: Vec2,
    pos_lower_bound: Vec2,
    pub center: Vec2,
    tree: Node,
    roi: f64,
    pub bodies: Vec<Body>,
    particle_num: usize,
}

impl BarnesHutSimulation {
    pub fn new(bodies: Vec<Body>) -> Self {
        let mut obj: Self = Default::default();
        obj.particle_num = bodies.len();
        obj.pos_upper_bound = Vec2::new(std::f64::MIN, std::f64::MIN);
        obj.pos_lower_bound = Vec2::new(std::f64::MAX, std::f64::MAX);
        obj.bodies = bodies;
        obj.init();
        obj
    }

    fn build_quadrant_tree(&mut self) {
        self.tree = Node::default();
        self.tree.pos_upper_bound = self.center.add_scalar(self.roi);
        self.tree.pos_lower_bound = self.center.add_scalar(-self.roi);
        self.tree.center = (self.tree.pos_upper_bound + self.tree.pos_lower_bound) / 2.0;

        for body in &self.bodies {
            self.tree.insert_particle(*body);
        }

        self.center = self.tree.compute_mass_distribution();
    }

    pub fn eval(&mut self) {
        self.build_quadrant_tree();

        for i in 0..self.bodies.len() {
            self.bodies[i].acc = self.tree.calculate_force(self.bodies[i]);
        }
    }

    fn init(&mut self) {
        for body in &self.bodies {
            self.pos_upper_bound.x = self.pos_upper_bound.x.max(body.pos.x);
            self.pos_upper_bound.y = self.pos_upper_bound.y.max(body.pos.y);

            self.pos_lower_bound.x = self.pos_lower_bound.x.min(body.pos.x);
            self.pos_lower_bound.y = self.pos_lower_bound.y.min(body.pos.y);
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

// taken from paper 'MATHEMATICAL MODEL FOR THE 0.5 BILLION YEARS AGED SUN'
const APPROXIMATIONS_ADB6: [f64; 6] = [
    4277.0 / 1440.0,
    -7923.0 / 1440.0,
    9982.0 / 1440.0,
    -7298.0 / 1440.0,
    2877.0 / 1440.0,
    -475.0 / 1440.0,
];

// six step adams-bashforth integration method
// https://web.mit.edu/10.001/Web/Course_Notes/Differential_Equations_Notes/node6.html
pub struct IntegratorADB6 {
    dimensions: usize,
    timestep: f64,
    data: [Vec<Body>; 6],
}

impl IntegratorADB6 {
    pub fn new(dimensions: usize, timestep: f64) -> Self {
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

    // TODO make integrator independent of barnes hut simulation? in original implementation we
    // were just processing raw numeric data in doubles
    pub fn integrate(&mut self, dt: f64, model: &mut BarnesHutSimulation) {
        for i in 0..self.dimensions {
            model.bodies[i].pos += dt
                * (APPROXIMATIONS_ADB6[0] * self.data[5][i].pos
                    + APPROXIMATIONS_ADB6[1] * self.data[4][i].pos
                    + APPROXIMATIONS_ADB6[2] * self.data[3][i].pos
                    + APPROXIMATIONS_ADB6[3] * self.data[2][i].pos
                    + APPROXIMATIONS_ADB6[4] * self.data[1][i].pos
                    + APPROXIMATIONS_ADB6[5] * self.data[0][i].pos);
            model.bodies[i].vel += dt
                * (APPROXIMATIONS_ADB6[0] * self.data[5][i].vel
                    + APPROXIMATIONS_ADB6[1] * self.data[4][i].vel
                    + APPROXIMATIONS_ADB6[2] * self.data[3][i].vel
                    + APPROXIMATIONS_ADB6[3] * self.data[2][i].vel
                    + APPROXIMATIONS_ADB6[4] * self.data[1][i].vel
                    + APPROXIMATIONS_ADB6[5] * self.data[0][i].vel);
            for j in 0..5 {
                self.data[j][i] = self.data[j + 1][i];
            }
        }
        for i in 0..self.dimensions {
            self.data[5][i].pos = model.bodies[i].vel;
            self.data[5][i].vel = model.bodies[i].acc;
        }
    }

    pub fn set_initial_state(&mut self, model: &mut BarnesHutSimulation) {
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

