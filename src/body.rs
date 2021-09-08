type Vec2 = nalgebra::Vector2<f64>;

pub struct Mass(pub f64);
pub struct Radius(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f64,
    pub radius: f64,
}

impl Body {
    pub fn new(m: Mass, r: Radius) -> Body {
        Body {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0),
            mass: m.0,
            radius: r.0,
        }
    }

    pub fn compute_force(&self, other: &Self, gravitational_constant: f64) -> Vec2 {
        const S_SOFT: f64 = 0.1 * 0.1;
        let dist = ((self.pos - other.pos).dot(&(self.pos - other.pos)) + S_SOFT).sqrt();
        let acc =
            gravitational_constant * other.mass * (other.pos - self.pos) / (dist * dist * dist);
        return acc;
    }

    pub fn pull_by(&mut self, other: &Self, gravitational_constant: f64) {
        self.acc += self.compute_force(other, gravitational_constant);
    }

    pub fn process_forces(&mut self, dt: f64) {
        self.vel += dt * self.acc;
        self.pos += dt * self.vel;
        self.acc = Vec2::new(0.0, 0.0);
    }
}
