use crate::{
    app::AppState,
    canvas::{Canvas2D, Paint, Point, Rect, Size},
    constraints::BoxConstraints,
    widget::{style::Theme, Event, EventCtx, MouseEvent, PaintCtx, Widget},
};

pub struct Switch<State> {
    value_changed: Option<Box<dyn FnMut(bool, &mut State)>>,
    active: bool,
}

impl<State: AppState + 'static> Switch<State> {
    pub fn new() -> Self {
        Self {
            value_changed: None,
            active: false,
        }
    }
}

impl<State: AppState> Widget<State> for Switch<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) -> bool {
        match event {
            Event::Mouse(MouseEvent::MouseEnter(_)) => ctx.request_repaint(),
            Event::Mouse(MouseEvent::MouseLeave(_)) => ctx.request_repaint(),
            Event::Mouse(MouseEvent::MouseDown(_)) => {
                self.active = !self.active;
                if let Some(l) = &mut self.value_changed {
                    (l)(self.active, state);
                }
                ctx.request_repaint()
            }
            Event::Mouse(MouseEvent::MouseUp(_)) => ctx.request_repaint(),
            _ => (),
        }

        false
    }

    fn layout(&mut self, constraints: &BoxConstraints, _: &State) -> Size {
        // Boldly unwrapping here. If you have not given constraints to a switch then we don't know how big it should be.
        Size::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        )
    }

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, _: &State) {
        let rect = ctx.rect();

        let mut fill_paint = Paint::default();
        fill_paint.set_anti_alias(true);

        let rounding = 4.0;
        let gutter_height = rect.height() / 4.0;
        let thumb_size = gutter_height * 1.25;
        let active_thumb_size = thumb_size * 1.5;

        let mut fill_rect = Rect::from_wh(rect.width(), gutter_height);
        fill_rect.offset(Point::new(0.0, rect.center_y() - fill_rect.center_y()));
        let mut unfill_rect = Rect::from_wh(rect.width(), gutter_height);
        unfill_rect.offset(Point::new(0.0, rect.center_y() - unfill_rect.center_y()));

        fill_paint.set_color(theme.slider.fill);
        fill_paint.set_stroke(true);
        canvas.draw_rounded_rect(&unfill_rect, rounding, rounding, &fill_paint);

        fill_paint.set_alpha_f(0.25);
        fill_paint.set_stroke(false);
        canvas.draw_rounded_rect(&unfill_rect, rounding, rounding, &fill_paint);

        fill_paint.set_alpha_f(1.0);
        let mut fill_rect = Rect::from_wh(0.0, gutter_height);
        fill_rect.offset(Point::new(0.0, rect.center_y() - fill_rect.center_y()));
        canvas.draw_rounded_rect(&fill_rect, rounding, rounding, &fill_paint);
        let thumb_position = if self.active {
            rect.right() - thumb_size * 0.5
        } else {
            0.0
        };
        canvas.draw_circle(
            &Point::new(thumb_position, rect.center_y()),
            thumb_size,
            &fill_paint,
        );

        if ctx.is_mouse_over() {
            fill_paint.set_alpha_f(0.25);
            canvas.draw_circle(
                &Point::new(thumb_position, rect.center_y()),
                active_thumb_size,
                &fill_paint,
            );
        }
    }
}

impl<State: AppState + 'static> Default for Switch<State> {
    fn default() -> Self {
        Self::new()
    }
}
