use crate::scenario::*;
use crate::screen::*;

use std::cell::RefCell;
use std::rc::Rc;

const HEIGHT: usize = 45;
const WIDTH: usize = 130;

type SharedFrame = Rc<RefCell<String>>;

use eframe::{egui, epi};
struct EguiTextOutput {
    height: usize,
    width: usize,
    frame: SharedFrame,
}

impl EguiTextOutput {
    fn new(height: usize, width: usize, frame: SharedFrame) -> Self {
        EguiTextOutput {
            height: height,
            width: width,
            frame: frame,
        }
    }
}

impl TextOutputter for EguiTextOutput {
    fn setup(&mut self) {
        self.frame.borrow_mut().reserve(self.width * self.height);
    }
    fn write(&mut self, buf: &[u8]) {
        let mut tmp = self.frame.borrow_mut();
        tmp.clear();
        *tmp = String::from_utf8(buf.to_vec()).unwrap(); // TODO remove allocation
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

pub struct EguiScreen {
    renderer: TextRender,
    //scenarios: Vec<Box<dyn Scenario>>,
    activeScenario: Box<dyn Scenario>,
    frame: SharedFrame,
    dt : f32,
}

impl Default for EguiScreen {
    fn default() -> EguiScreen {
        let mut obj = EguiScreen {
            renderer: TextRender::new(Zoom(5.0)),
            /*
            scenarios: vec![
                Box::new(SunEarthMoon::new()),
                Box::new(Collision::new(20000)), // TODO remove hardcoded value
                Box::new(ThreeBody::new()),
            ],
            */
            activeScenario: Box::new(SunEarthMoon::new()), // TODO avoid duplication?
            frame: Rc::new(RefCell::new(String::with_capacity(WIDTH * HEIGHT))),
            dt: 1.0/100.0,
        };
        obj.renderer.TextOutputter(Box::new(EguiTextOutput::new(
            HEIGHT,
            WIDTH,
            Rc::clone(&obj.frame),
        )));
        obj
    }
}

impl epi::App for EguiScreen {
    fn name(&self) -> &str {
        "egui text render"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let dt = 1.0 / 100.0;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.heading("Scenarios:");
            ui.horizontal(|ui| {
                if ui.button("SunEarthMoon").clicked() {
                    self.renderer.Zoom(5.0);
                    self.activeScenario = Box::new(SunEarthMoon::new());
                    self.dt = 1.0/100.0;
                }
                if ui.button("Collision").clicked() {
                    self.renderer.Zoom(5.0);
                    self.activeScenario = Box::new(Collision::new(20000)); // TODO self.scenarios[1].clone();
                    self.dt = 1.0/40.0;
                }
                if ui.button("ThreeBody").clicked() {
                    self.renderer.Zoom(200.0);
                    self.activeScenario = Box::new(ThreeBody::new());
                    self.dt = 1.0/100.0;
                }
            });
        });
        self.activeScenario.process(self.dt);
        self.activeScenario.draw(&mut self.renderer); // fills self.frame
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.monospace(self.frame.borrow().clone());
            egui::warn_if_debug_build(ui);
            ui.ctx().request_repaint(); // MAX FPS, NEVER SLEEP!
        });
    }
}
