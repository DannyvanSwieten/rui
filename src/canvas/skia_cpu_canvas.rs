use skia_safe::{Color, Font, ISize, Paint, Point, Rect, Surface};

use super::Canvas2D;

pub struct SkiaCanvas {
    surface: Surface,
    pub size: ISize,
}

impl SkiaCanvas {
    pub fn new(w: i32, h: i32) -> Self {
        let surface = Surface::new_raster_n32_premul(skia_safe::ISize::new(w, h));
        if let Some(surface) = surface {
            Self {
                surface,
                size: skia_safe::ISize::new(w, h),
            }
        } else {
            panic!()
        }
    }

    pub fn pixels(&mut self) -> Option<Vec<u8>> {
        let w = self.surface.width();
        let h = self.surface.height();
        let bbp = 4;
        let size = w * h * bbp;
        let mut data = Vec::<u8>::new();
        data.resize(size as _, 0);
        let info = self.surface.image_info();
        if self.surface.read_pixels(
            &info,
            &mut data,
            w as usize * bbp as usize,
            skia_safe::IPoint::new(0, 0),
        ) {
            Some(data)
        } else {
            None
        }
    }

    pub fn flush(&mut self) {
        self.surface.flush_submit_and_sync_cpu();
    }
}

impl Canvas2D for SkiaCanvas {
    fn clear(&mut self, color: &Color) {
        self.surface.canvas().clear(*color);
    }

    fn save(&mut self) {
        self.surface.canvas().save();
    }

    fn restore(&mut self) {
        self.surface.canvas().restore();
    }

    fn translate(&mut self, point: &Point) {
        self.surface.canvas().translate(*point);
    }
    fn draw_rect(&mut self, rect: &Rect, paint: &Paint) {
        self.surface.canvas().draw_rect(rect, paint);
    }

    fn draw_rounded_rect(&mut self, rect: &Rect, rx: f32, ry: f32, paint: &Paint) {
        self.surface.canvas().draw_round_rect(rect, rx, ry, paint);
    }

    fn draw_circle(&mut self, center: &Point, radius: f32, paint: &Paint) {
        self.surface.canvas().draw_circle(*center, radius, paint);
    }

    fn draw_string(&mut self, rect: &Rect, text: &str, font: &Font, paint: &Paint) {
        let blob = skia_safe::TextBlob::from_str(text, font);
        if let Some(b) = blob {
            let text_bounds = b.bounds();
            let p = rect.center() - text_bounds.center();
            self.surface.canvas().draw_str(text, p, font, paint);
        }
    }

    fn draw_text_blob(&mut self, pos: &Point, blob: &skia_safe::TextBlob, paint: &Paint) {
        self.surface.canvas().draw_text_blob(blob, *pos, paint);
    }

    fn draw_paragraph(&mut self, pos: &Point, paragraph: &skia_safe::textlayout::Paragraph) {
        paragraph.paint(self.surface.canvas(), *pos);
    }
}
