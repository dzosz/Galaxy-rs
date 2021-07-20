use galaxyy::scenario::{Scenario, SunEarthMoon};
use galaxyy::screen::TerminalScreen;

fn main() {
    let mut scenario = SunEarthMoon::new();
    let mut renderer = TerminalScreen::new(0.0, 0.0, 5.0);

    let dt = 1.0 / 100.0;
    loop {
        scenario.process(dt);
        scenario.draw(&mut renderer);
    }
}
