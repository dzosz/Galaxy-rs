use galaxyy::scenario::{Scenario, SunEarthMoon};
use galaxyy::screen::{TextRender, Zoom};

fn main() {
    let mut scenario = SunEarthMoon::new();
    let mut renderer = TextRender::new(Zoom(5.0));

    let dt = 1.0 / 100.0;
    loop {
        scenario.process(dt);
        scenario.draw(&mut renderer);
    }
}
