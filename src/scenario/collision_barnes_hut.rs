use crate::barnes_hut::NBodyWnd;
use crate::scenario::Scenario;
use crate::screen::Screen;

impl Scenario for NBodyWnd {
    fn process(&mut self, dt: f64) {
        self.render(dt);
    }

    fn draw(&self, renderer: &mut dyn Screen) {
        renderer.clear();
        renderer.position(self.model.center.x, self.model.center.y);

        for i in 0..self.model.bodies.len() {
            renderer.plot_circle(
                self.model.bodies[i].pos.x,
                self.model.bodies[i].pos.y,
                self.model.bodies[i].radius,
            );
        }

        /*
        // drawing
        if (self.model.bodies[0].pos - self.model.bodies[4000].pos).dot(&(self.model.bodies[0].pos - self.model.bodies[4000].pos))
            < 90.0 * 90.0
        {
            renderer.set_zoom(5.0);
        } else if (self.model.bodies[0].pos - self.model.bodies[4000].pos).dot(&(self.model.bodies[0].pos - self.model.bodies[4000].pos))
            > 110.0 * 110.0
        {
            renderer.set_zoom(2.0);
        }*/
        renderer.draw();
    }
}
