use galaxyy::scenario::{Scenario, ThreeBody};
use galaxyy::screen::{Screen, EguiScreen};

fn main() {
    let app = EguiScreen::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(950 as f32, 670 as f32));
    eframe::run_native(Box::new(app), native_options);
}
