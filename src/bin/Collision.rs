use galaxyy::scenario::{Scenario, Collision};
use galaxyy::screen::{TextRender, Zoom};

fn main() {
    let mut scenario = Collision::new(20000);
    let mut renderer = TextRender::new(Zoom(5.0));
    let dt = 1.0 / 40.0;

    loop {
        scenario.process(dt);
        scenario.draw(&mut renderer);
    }
}
