pub trait Screen
{
    fn Clear(&mut self);
    fn PlotPoint(&mut self, x : f64, y :f64);
    fn PlotLine(&mut self, x1 : f64, y1 :f64, x2 : f64, y2 : f64);
    fn PlotCircle(&mut self, x : f64, y: f64, r: f64);
    fn PlotRectangle(&mut self, x1 : f64, y1 :f64, x2 : f64, y2 : f64);
    fn Position(&mut self, x : f64, y : f64);
    fn Zoom(&mut self, zoom : f64);
    fn Draw(&mut self);
    fn set_palette(&mut self, palette : i32);
    fn TextOutputter(&mut self, output : Box<dyn TextOutputter>);
}
pub trait TextOutputter {
    fn setup(&mut self);
    fn write(&mut self, buf : &[u8]);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

struct Point(i32, i32);

mod text_render;
pub use text_render::TextRender;
pub use text_render::Zoom;
mod egui_screen;
pub use egui_screen::EguiScreen;

