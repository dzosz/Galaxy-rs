use crate::Screen;

pub trait Scenario
{
    fn process(&mut self, dt : f32);
    fn draw(&mut self, renderer : &mut impl Screen);
}

