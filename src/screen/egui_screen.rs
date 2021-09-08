use crate::scenario::*;
use crate::screen::*;

use std::cell::RefCell;
use std::rc::Rc;

const HEIGHT: usize = 45;
const WIDTH: usize = 130;

type Vec2 = nalgebra::Vector2<f64>;
struct Point(i32, i32);

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
    renderer: EguiRenderer,
    activeScenario: Box<dyn Scenario>,
    frame: SharedFrame,
    dt : f64,
}

impl Default for EguiScreen {
    fn default() -> EguiScreen {
        let mut obj = EguiScreen {
            //renderer: TextRender::new(Zoom(5.0)),
            renderer: EguiRenderer::default(),
            activeScenario: Box::new(SunEarthMoon::new()), // TODO avoid duplication?
            frame: Rc::new(RefCell::new(String::with_capacity(WIDTH * HEIGHT))),
            dt: 1.0/100.0,
        };

        //obj.renderer.TextOutputter(Box::new(EguiTextOutput::new( HEIGHT, WIDTH, Rc::clone(&obj.frame),)));
        obj
    }
}

impl epi::App for EguiScreen {
    fn name(&self) -> &str {
        "egui text render"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _: &mut epi::Frame<'_>) {
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
                if ui.button("NbodyWnd").clicked() {
                    self.renderer.Zoom(20.0);
                    self.activeScenario = Box::new(NBodyWnd::new()); // TODO self.scenarios[1].clone();
                    self.dt = 100.0;
                }
                if ui.button("ThreeBody").clicked() {
                    self.renderer.Zoom(200.0);
                    self.activeScenario = Box::new(ThreeBody::new());
                    self.dt = 1.0/100.0;
                }
            });
        });

        self.activeScenario.process(self.dt);

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );

            ui.expand_to_include_rect(painter.clip_rect());
            self.renderer.painter = Some(painter);
            //ui.monospace(self.frame.borrow().clone());
            //egui::warn_if_debug_build(ui);
            //self.renderer.Draw();
            ui.ctx().request_repaint(); // MAX FPS, NEVER SLEEP!
            self.renderer.width = ui.max_rect().width() as i64;
            self.renderer.height = ui.max_rect().height() as i64;
        });
        self.activeScenario.draw(&mut self.renderer); // fills self.frame
    }
}

//#[derive(Default)]
struct EguiRenderer {
    width: i64,
    height: i64,
    zoom: f64,
    center: Vec2,
    //output: Box<dyn TextOutputter>,
    shapes: Vec<egui::Shape>,
    default_color: egui::Color32,
    painter: Option<egui::Painter>
}

impl Default for EguiRenderer {
    fn default() -> EguiRenderer {
        EguiRenderer {
            width: 800,
            height: 600,
            zoom: 5.0, // TODO
            center: Vec2::new(0.0, 0.0),
            shapes: Default::default(),
            default_color: egui::Color32::GREEN,
            painter: Default::default(),
        }
    }
}

impl EguiRenderer {
    fn transform(&self, x : f64, y: f64) -> Point {
        // from world to screen coordinates
        let xx = ((x - self.center.x) * self.zoom) as i32 + (self.width as i32 / 2);
        let yy = ((y - self.center.y) * self.zoom) as i32 + (self.height as i32 / 2);

        let h = self.height as i32 - 1 - yy as i32;
        let w = xx as i32;
        Point(h, w)
    }

    fn drawPoint(&mut self, point: Point) {
        let width = 1.0;
        let p2 = egui::Pos2::new(point.0 as f32, point.1 as f32);
        let line = [p2, p2];
        self.shapes.push(egui::Shape::line_segment(line, (width, self.default_color)));
    }
    fn drawLine(&mut self, a: Point, b: Point) {
        let width = 1.0;
        let p1 = egui::Pos2::new(a.0 as f32, a.1 as f32);
        let p2 = egui::Pos2::new(b.0 as f32, b.1 as f32);
        let line = [p1, p2];
        //println!("drawLine: {:?} {:?}", p1, p2);
        self.shapes.push(egui::Shape::line_segment(line, (width, self.default_color)));
    }
}

impl Screen for EguiRenderer {
    fn Clear(&mut self) {
        // handled by egui
    }

    fn PlotPoint(&mut self, x: f64, y: f64) {
        let point = self.transform(x, y);
        self.drawPoint(point);
    }

    fn PlotLine(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let p1 = self.transform(x1, y1);
        let p2 = self.transform(x2, y2);
        self.drawLine(p1, p2);
    }

    fn PlotCircle(&mut self, x: f64, y: f64, r: f64) {
        let pcenter = self.transform(x, y);
        let center = egui::Pos2::new(pcenter.0 as f32, pcenter.1 as f32);
        let radius = r * self.zoom;

        let stroke_width = 1.0;
        let circ = egui::Shape::circle_stroke(center, radius as f32, (stroke_width, self.default_color));
        // let circ = egui::Shape::circle_filled(center, radius as f32, self.default_color);
        self.shapes.push(circ);
    }

    fn PlotRectangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let p1 = self.transform(x1, y1);
        let p2 = self.transform(x2, y2);
        //self.drawRectangle(p1, p2); // FIXME
    }

    fn Position(&mut self, x: f64, y: f64) {
        self.center = Vec2::new(x, y);
    }

    fn Zoom(&mut self, zoom: f64) {
        self.zoom = zoom;
    }

    fn TextOutputter(&mut self, output : Box<dyn TextOutputter>) {
        // self.output = output;
    }

    fn Draw(&mut self) {
        if let Some(ref mut painter) = self.painter.as_mut() {
            let drained : Vec<egui::Shape>= self.shapes.drain(..).collect();
            painter.extend(drained);
        } else {
            panic!("painter is empty");
        }
    }

    fn set_palette(&mut self, palette: i32) {
        // TODO remove
    }
}
