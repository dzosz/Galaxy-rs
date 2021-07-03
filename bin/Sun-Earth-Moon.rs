mod screen;

use screen::Screen;

type Vec2 = nalgebra::Vector2<f32>;

struct Body
{
	r : f32,
	m : f32,
	pos : Vec2,
	vel : Vec2,
	acc : Vec2,
}

impl Body {
    /*
	fn new() -> Body {
        Body {
            r:0.2,
            m:1.0,
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0)
        }
    }
	
    
    fn new(m : f32) -> Body {
        Body {
            r:0.2 * m.cbrt(),
            m:m,
            pos:Vec2::new(0.0, 0.0),
            vel:Vec2::new(0.0, 0.0),
            acc:Vec2::new(0.0, 0.0)
        }
    }
    */

	fn new(m : f32, r : f32) -> Body {
        Body {
            r:r,
            m:m,
            pos:Vec2::new(0.0, 0.0),
            vel:Vec2::new(0.0, 0.0),
            acc:Vec2::new(0.0, 0.0)
        }
	}
	
	fn setPos(&mut self, x : f32,y : f32) {
		self.pos.x=x;
		self.pos.y=y;
	}
	
    fn PulledBy(&mut self, other : &Self) {
		let G : f32=1.0;
		let dist = (self.pos-other.pos).dot(&(self.pos-other.pos)).sqrt();
		self.acc += G*other.m*(other.pos-self.pos) / dist/dist/dist;
	}
	
	fn Update(&mut self, dt : f32) {
		self.vel+=dt*self.acc;
		self.pos+=dt*self.vel;
		self.acc=Vec2::new(0.0, 0.0);
	}
}

fn Plot(body : &Body, scr : &mut Screen)
{
	scr.PlotCircle(body.pos.x,body.pos.y,body.r);
}

fn main() {
	let mut scr = Screen::new(0.0,0.0,10.0);
	
	let dt=1.0/100.0;
	let r=5.5;
    let R=30.0;
	
	let mut Sun = Body::new(10000.0,7.0);
	let mut Earth = Body::new(1000.0,2.0);
	let mut Moon = Body::new(1.0,1.2);
	
	Sun.pos=Vec2::new(0.0,0.0);
	Sun.vel=Vec2::new(0.0,0.0);
	
	Earth.pos=Vec2::new(R,0.0);
	Earth.vel=Vec2::new(0.0,(Sun.m/R).sqrt());
	
	Moon.pos=Vec2::new(R,r);
	Moon.vel=Vec2::new((Earth.m/r).sqrt(),Earth.vel.y);
	
	loop {
		scr.Clear();
		
		Moon.PulledBy(&Earth);
		Moon.PulledBy(&Sun);
		Earth.PulledBy(&Moon);
		Earth.PulledBy(&Sun);
		Sun.PulledBy(&Moon);
		Sun.PulledBy(&Earth);
		
		Moon.Update(dt);
		Earth.Update(dt);
		Sun.Update(dt);
		
		scr.Position(Sun.pos.x,Sun.pos.y);
		
		Plot(&Moon, &mut scr);
		Plot(&Earth,&mut scr);
		Plot(&Sun,  &mut scr);
		
		scr.Draw();
        //use std::{thread, time};
        //thread::sleep(time::Duration::from_millis(1000));
	}
}
