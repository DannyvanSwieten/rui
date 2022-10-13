pub mod skia_cpu_canvas;

pub use skia_safe::{
    font, textlayout, textlayout::Paragraph, Color, Color4f, Font, FontStyle, Paint, Point, Rect,
    Size, TextBlob, Typeface,
};

pub trait Canvas2D {
    fn clear(&mut self, color: &Color);

    fn save(&mut self);
    fn restore(&mut self);
    fn translate(&mut self, point: &Point);

    fn draw_rect(&mut self, rect: &Rect, paint: &Paint);
    fn draw_rounded_rect(&mut self, rect: &Rect, rx: f32, ry: f32, paint: &Paint);

    fn draw_circle(&mut self, center: &Point, radius: f32, paint: &Paint);

    fn draw_string(&mut self, rect: &Rect, text: &str, font: &Font, paint: &Paint);
    fn draw_text_blob(&mut self, pos: &Point, blob: &TextBlob, paint: &Paint);
    fn draw_paragraph(&mut self, pos: &Point, paragraph: &Paragraph);
}
