use galaxyy::scenario::{Scenario, sun_earth_moon::SunEarthMoon};
use galaxyy::screen::{Screen, TerminalScreen};

fn main() {
    let mut scenario = SunEarthMoon::new();
    let mut renderer = TerminalScreen::new(0.0, 0.0, 5.0);

    let dt = 1.0 / 100.0;
    loop {
        scenario.process(dt);
        scenario.draw(&mut renderer);
        //use std::{thread, time};
        //thread::sleep(time::Duration::from_millis(1000));
    }
}
