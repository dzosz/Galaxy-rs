use galaxyy::scenario::{Scenario, ThreeBody};
use galaxyy::screen::TerminalScreen;

fn main() {
    let mut scenario = ThreeBody::new();
    let dt = 1.0 / 100.0;
    let mut renderer = TerminalScreen::new(0.0, 0.0, 200.0);

    loop {
        scenario.process(dt);
        scenario.draw(&mut renderer);
        //solarSystem.iter().for_each(|body| {
        //    print!("{:>16.8} {:>16.8}\n", body.vel.x, body.vel.y);
        //});
        //use std::{thread, time};
        //thread::sleep(time::Duration::from_millis(1000));
    }
}
