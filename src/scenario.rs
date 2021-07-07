use crate::screen::Screen;

pub mod collision;
pub mod sun_earth_moon;
pub mod three_body;

pub trait Scenario
{
    fn process(&mut self, dt : f32);
    fn draw(&mut self, renderer : &mut impl Screen);
}
