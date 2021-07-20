use galaxyy::scenario::{Scenario, Collision};
use galaxyy::screen::TerminalScreen;

fn main() {
    let mut scenario = Collision::new(20000);
    let mut renderer = TerminalScreen::new(0.0, 0.0, 200.0);
    let dt = 1.0 / 40.0;

    loop {
        scenario.process(dt);
        scenario.draw(&mut renderer);
    }
}
