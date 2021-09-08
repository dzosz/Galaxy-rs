use crate::scenario::Scenario;
use crate::body::{Body, Mass, Radius};
use crate::screen::Screen;
use crate::barnes_hut::*;

type Vec2 = nalgebra::Vector2<f64>;

impl Scenario for NBodyWnd {
    fn process(&mut self, dt: f64) {
        self.render();
    }

    fn draw(&self, renderer : &mut dyn Screen) {
        renderer.Clear();
        renderer.Position(self.model.center.x, self.model.center.y);

        for i in 0..self.model.bodies.len() {
            renderer.PlotCircle(self.model.bodies[i].pos.x, self.model.bodies[i].pos.y, self.model.bodies[i].r);
        }

        /*
        // drawing
        if (self.model.bodies[0].pos - self.model.bodies[4000].pos).dot(&(self.model.bodies[0].pos - self.model.bodies[4000].pos))
            < 90.0 * 90.0
        {
            renderer.Zoom(5.0);
        } else if (self.model.bodies[0].pos - self.model.bodies[4000].pos).dot(&(self.model.bodies[0].pos - self.model.bodies[4000].pos))
            > 110.0 * 110.0
        {
            renderer.Zoom(2.0);
        }*/
        renderer.Draw();
    }
}
