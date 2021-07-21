pub trait Screen
{
    fn Clear(&mut self);
    fn PlotPoint(&mut self, x : f32, y :f32);
    fn PlotLine(&mut self, x1 : f32, y1 :f32, x2 : f32, y2 : f32);
    fn PlotCircle(&mut self, x : f32, y: f32, r: f32);
    fn PlotRectangle(&mut self, x1 : f32, y1 :f32, x2 : f32, y2 : f32);
    fn Position(&mut self, x : f32, y : f32);
    fn Zoom(&mut self, zoom : f32);
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

