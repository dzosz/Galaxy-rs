use galaxyy::scenario::{Scenario, GalaxyCollisionBarnesHut};
use galaxyy::screen::{TextRender, Zoom};

fn main() {
    let mut scenario = GalaxyCollisionBarnesHut::new();
    let mut renderer = TextRender::new(Zoom(15.0));
    let dt = 100.0;

    loop {
        scenario.process(dt);
        scenario.draw(&mut renderer);
    }
}
