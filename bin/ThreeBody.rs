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
    let O = body.pos;
    let X = body.pos + 0.5 * body.vel;

	scr.PlotCircle(body.pos.x,body.pos.y,body.r);
    scr.PlotLine(O.x, O.y, X.x, X.y);

    let mut a = O-X;
    a.normalize();
    a *= 0.1;
    let b = Vec2::new(a.y, -a.x);

    scr.PlotLine(X.x, X.y, X.x + a.x + a.x, X.y + a.y + b.y);
    scr.PlotLine(X.x, X.y, X.x + a.x - b.x, X.y + a.y - b.y);
}

fn main() {
	let mut scr = Screen::new(0.0,0.0,200.0);
	
	let dt=1.0/100.0;

    let mut solarSystem = [Body::new(1.0, 0.1), Body::new(1.0, 0.1), Body::new(1.0, 0.1)];

	solarSystem[0].pos=Vec2::new(-0.9700436, 0.24308753);
	solarSystem[0].vel=Vec2::new(0.4662036850, 0.4323657300);

	solarSystem[1].pos=Vec2::new(0.0, 0.0);
	solarSystem[1].vel=Vec2::new(-0.93240737, -0.86473146);

	solarSystem[2].pos=Vec2::new(0.9700436, -0.24308753);
	solarSystem[2].vel=Vec2::new(0.4662036850, 0.4323657300);
	
	loop {
		scr.Clear();

        for i in 0..solarSystem.len() {
            for j in 0..solarSystem.len() {
                if i == j {
                    continue;
                 } else if i < j {
                     let (left, right) = solarSystem.split_at_mut(j);
                     left[i].PulledBy(&right[0]);
                } else {
                     let (left, right) = solarSystem.split_at_mut(i);
                     right[0].PulledBy(&left[j]);
                 }
            }
        }
		
        for i in 0..solarSystem.len() {
            solarSystem[i].Update(dt);
        }
        for i in 0..solarSystem.len() {
            Plot(&solarSystem[i], &mut scr);
        }
		
		scr.Draw();
        //solarSystem.iter().for_each(|body| {
        //    print!("{:>16.8} {:>16.8}\n", body.vel.x, body.vel.y);
        //});
        //use std::{thread, time};
        //thread::sleep(time::Duration::from_millis(1000));
	}
}
