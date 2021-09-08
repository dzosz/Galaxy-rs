pub trait Screen
{
    fn clear(&mut self);
    fn plot_point(&mut self, x : f64, y :f64);
    fn plot_line(&mut self, x1 : f64, y1 :f64, x2 : f64, y2 : f64);
    fn plot_circle(&mut self, x : f64, y: f64, r: f64);
    fn plot_rectangle(&mut self, x1 : f64, y1 :f64, x2 : f64, y2 : f64);
    fn position(&mut self, x : f64, y : f64);
    fn set_zoom(&mut self, zoom : f64);
    fn draw(&mut self);
    fn set_palette(&mut self, palette : i32);
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

