type Vec2 = nalgebra::Vector2<f32>;

pub trait Scenario
{
    fn process(&mut self, dt : f32);
    fn draw(&mut self);
}

#[derive(Debug, Copy, Clone)]
pub struct Body
{
	pub r : f32,
	pub m : f32,
	pub pos : Vec2,
	pub vel : Vec2,
	pub acc : Vec2,
}

impl Body {
	pub fn new(m : f32, r : f32) -> Body {
        Body {
            r:r,
            m:m,
            pos:Vec2::new(0.0, 0.0),
            vel:Vec2::new(0.0, 0.0),
            acc:Vec2::new(0.0, 0.0)
        }
	}
	
	pub fn setPos(&mut self, x : f32,y : f32) {
		self.pos.x=x;
		self.pos.y=y;
	}
	
    /* In the restricted three-body problem, a body of negligible mass (the "planetoid") moves under the influence of two massive bodies. Having negligible mass, the force that the planetoid exerts on the two massive bodies may be neglected, and the system can be analysed and can therefore be described in terms of a two-body motion.
     * For simplicity, choose units such that the distance between the two massive bodies, as well as the gravitational constant, are both equal to 1. */
    pub fn PulledBy(&mut self, other : &Self, G : f32) {
		let dist = (self.pos-other.pos).dot(&(self.pos-other.pos)).sqrt();
		self.acc += G*other.m*(other.pos-self.pos) / dist/dist/dist;
	}
	
	pub fn Update(&mut self, dt : f32) {
		self.vel+=dt*self.acc;
		self.pos+=dt*self.vel;
		self.acc=Vec2::new(0.0, 0.0);
	}
    
}

