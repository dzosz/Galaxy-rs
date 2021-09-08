use galaxyy::scenario::Scenario;
use galaxyy::screen::{TextRender, Zoom};
use galaxyy::barnes_hut::NBodyWnd;

fn main() {
    let mut scenario = NBodyWnd::new();
    let mut renderer = TextRender::new(Zoom(15.0));
    let dt = 1.0 / 40.0;

    loop {
        scenario.process(dt);
        scenario.draw(&mut renderer);
    }
}
