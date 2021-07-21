use crate::screen::Screen;

mod collision;
mod sun_earth_moon;
mod three_body;

pub use collision::Collision;
pub use sun_earth_moon::SunEarthMoon;
pub use three_body::ThreeBody;

pub trait Scenario
{
    fn process(&mut self, dt : f32);
    fn draw(&self, renderer : &mut dyn Screen);
}
