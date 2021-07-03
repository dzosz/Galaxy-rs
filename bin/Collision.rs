mod screen;

use screen::Screen;

type Vec2 = nalgebra::Vector2<f32>;

static G : f32 = 3.0;

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

fn random(low : f32, high : f32) -> f32 {
    let rand_max = 1.0;
    low + rand::random::<f32>() / (rand_max / (high - low))
}

fn main() {
	let mut scr = Screen::new(0.0,0.0,5.0);

    let n = 20000;
	let dt=1.0/40.0;


    // Initializing first galaxy
    let mut Centre1 = Body::new(2000.0, 2.5);
    Centre1.pos = Vec2::new(150.0,20.0);
    Centre1.vel = Vec2::new(-5.0,0.0);

    let mut Bodies1 = Vec::new();

    for i in 0..n {
        let maxRadius = 30.0;
        let theta = random(0.0, 2.0*std::f32::consts::PI);
        let mut r = random(1.0, maxRadius);
        r = r*r/maxRadius;

        let mut body = Body::new(1.0, 0.2);
        body.pos = Vec2::new(r * theta.cos(), r* theta.sin());
        body.pos += Centre1.pos;

        let v = (G*Centre1.m/r).sqrt();
        body.vel = Vec2::new(v * theta.sin(), -v*theta.cos());

        let offset = 0.6;
        body.vel += Vec2::new(random(-offset, offset), random(-offset, offset));
        body.vel += Centre1.vel;

        Bodies1.push(body);
    }

    let mut Centre2 = Body::new(2000.0, 2.5);
    Centre2.pos = -Centre1.pos;
    Centre2.vel = -Centre1.vel;


    let mut Bodies2 = Vec::new();
    for i in 0..n {
        let maxRadius = 30.0;
        let theta = random(0.0, 2.0 * std::f32::consts::PI);

        let mut r = random(1.0, maxRadius);
        r = r*r/maxRadius;
        r += 0.2 * Centre2.r;

        let mut body = Body::new(1.0, 0.2);
        body.pos = Vec2::new(r*theta.cos(), r*theta.sin());
        body.pos += Centre2.pos;

        let v = (G*Centre2.m/r).sqrt();
        body.vel = Vec2::new(v*theta.sin(), -v*theta.cos());
        // uncomment for opposite direction of rotation
        // body.vel = -body.vel;

        let offset = 0.6;
        body.vel += Vec2::new(random(-offset, offset), random(-offset, offset));
        body.vel += Centre2.vel;

        Bodies2.push(body);
    }
	
	loop {
		scr.Clear();
		
        // centres attract each other
	    Centre1.PulledBy(&Centre2);
		Centre2.PulledBy(&Centre1);

        // particles are attracted to centres
        for i in 0..n {
            Bodies1[i].PulledBy(&Centre1);
            Bodies2[i].PulledBy(&Centre1);
            Bodies1[i].PulledBy(&Centre2);
            Bodies2[i].PulledBy(&Centre2);
        }

		Centre1.Update(dt);
		Centre2.Update(dt);

        for i in 0..n {
            Bodies1[i].Update(dt);
        }
        for i in 0..n {
            Bodies2[i].Update(dt);
        }
		
        Plot(&Centre1, &mut scr);
        Plot(&Centre2, &mut scr);

        for i in 0..n {
            Plot(&Bodies1[i], &mut scr);
        }
        for i in 0..n {
            Plot(&Bodies2[i], &mut scr);
        }

        // drawing
        if (Centre1.pos-Centre2.pos).dot(&(Centre1.pos-Centre2.pos))< 90.0*90.0 {
            scr.Zoom(9.0);
        } else if (Centre1.pos-Centre2.pos).dot(&(Centre1.pos-Centre2.pos))>110.0*110.0 {
            scr.Zoom(5.0);
        }
        scr.Draw();
	}
}
