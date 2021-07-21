type Vec2 = nalgebra::Vector2<f32>;

pub struct Mass(pub f32);
pub struct Radius(pub f32);

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
	pub fn new(m : Mass, r : Radius) -> Body {
        Body {
            r:r.0,
            m:m.0,
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
		self.acc += G*other.m*(other.pos-self.pos) / dist/dist/dist; // TODO ignoring self.m ?
	}
	
	pub fn Update(&mut self, dt : f32) {
		self.vel+=dt*self.acc;
		self.pos+=dt*(self.vel);// + self.acc*0.5); // TODO why do we ignore acceleration ?
		self.acc=Vec2::new(0.0, 0.0);
	}
    
}

