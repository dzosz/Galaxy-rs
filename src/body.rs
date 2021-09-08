type Vec2 = nalgebra::Vector2<f64>;

pub struct Mass(pub f64);
pub struct Radius(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Body {
    pub r: f64,
    pub m: f64,
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

impl Body {
    pub fn new(m: Mass, r: Radius) -> Body {
        Body {
            r: r.0,
            m: m.0,
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0),
        }
    }

    pub fn setPos(&mut self, x: f64, y: f64) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn computeForce(&self, other: &Self, G: f64) -> Vec2 {
        let S_SOFT = 0.1 * 0.1;
        let dist = ((self.pos - other.pos)
            .dot(&(self.pos - other.pos)) + S_SOFT)
            .sqrt();
        let acc = G * other.m * (other.pos - self.pos) / (dist * dist * dist);
        return acc;
    }

    pub fn PulledBy(&mut self, other: &Self, G: f64) {
        self.acc += self.computeForce(other, G);
    }

    pub fn Update(&mut self, dt: f64) {
        self.vel += dt * self.acc;
        self.pos += dt * self.vel;
        self.acc = Vec2::new(0.0, 0.0);
    }
}
