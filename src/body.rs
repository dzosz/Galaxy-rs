type Vec2 = nalgebra::Vector2<f32>;

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

