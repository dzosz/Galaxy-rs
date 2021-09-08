use crate::screen::Screen;

mod collision;
mod sun_earth_moon;
mod three_body;
mod collision_barnes_hut;
//mod barnes_hut;

pub use collision::Collision;
pub use collision_barnes_hut::*;
pub use crate::barnes_hut::NBodyWnd;
pub use sun_earth_moon::SunEarthMoon;
pub use three_body::ThreeBody;

pub trait Scenario
{
    fn process(&mut self, dt : f64);
    fn draw(&self, renderer : &mut dyn Screen);
}
