use crate::{
    app::{App, AppState},
    canvas::{Canvas2D, Paint, Point, Rect, Size},
    constraints::BoxConstraints,
    widget::{map_range, style::Theme, Properties, Widget},
    window::MouseEvent,
};
enum SliderState {
    Active,
    Inactive,
}

pub struct Slider<State> {
    min: f32,
    max: f32,
    discrete: bool,
    current_normalized: f32,
    current_value: f32,
    last_position: f32,
    value_changed: Option<Box<dyn FnMut(f32, &mut State)>>,
    state: SliderState,
}

impl<State: AppState + 'static> Slider<State> {
    pub fn new() -> Self {
        Slider::new_with_min_max_and_value(0., 1., 0., false)
    }

    pub fn new_with_min_max_and_value(min: f32, max: f32, value: f32, discrete: bool) -> Self {
        Slider {
            min,
            max,
            discrete,
            current_normalized: value / (max - min),
            current_value: value,
            last_position: 0.,
            value_changed: None,
            state: SliderState::Inactive,
        }
    }

    pub fn with_handler<F>(mut self, handler: F) -> Self
    where
        F: FnMut(f32, &mut State) + 'static,
    {
        self.value_changed = Some(Box::new(handler));
        self
    }

    pub fn set_value(&mut self, value: f32) {
        self.current_value = value.max(self.min).min(self.max);
        self.current_normalized = map_range(self.current_value, self.min, self.max, 0., 1.)
    }
}

impl<State: AppState> Widget<State> for Slider<State> {
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        // Boldly unwrapping here. If you have not given constraints to a slider then we don't know how big it should be.
        Size::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        )
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, _: &State) {
        let mut fill_paint = Paint::default();
        fill_paint.set_anti_alias(true);

        let rounding = 4.0;
        let gutter_height = rect.height / 8.0;
        let thumb_size = gutter_height * 1.5;
        let active_thumb_size = thumb_size * 2.5;

        let rect = Rect::from_size(*rect);
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
        let mut fill_rect = Rect::from_wh(self.last_position, gutter_height);
        fill_rect.offset(Point::new(0.0, rect.center_y() - fill_rect.center_y()));
        canvas.draw_rounded_rect(&fill_rect, rounding, rounding, &fill_paint);
        canvas.draw_circle(
            &Point::new(self.last_position, rect.center_y()),
            thumb_size,
            &fill_paint,
        );

        if let SliderState::Active = self.state {
            fill_paint.set_alpha_f(0.25);
            canvas.draw_circle(
                &Point::new(self.last_position, rect.center_y()),
                active_thumb_size,
                &fill_paint,
            );
        }

        // self.thumb.paint(theme, canvas, &rect.size(), state)
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        _: &mut App<State>,
        state: &mut State,
    ) {
        self.last_position = event.local_position().x;
        self.current_normalized = (1. / properties.size.width) * self.last_position;

        self.current_value = map_range(self.current_normalized, 0., 1., self.min, self.max);
        if self.discrete {
            self.current_value = self.current_value.round();
        }
        if let Some(l) = &mut self.value_changed {
            (l)(self.current_value, state);
        }
    }

    fn mouse_up(&mut self, _: &MouseEvent, _: &mut App<State>, _: &mut State) {
        self.state = SliderState::Inactive
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, state: &mut State) {
        self.last_position = event.local_position().x;
        self.current_normalized =
            (1. / properties.size.width) * self.last_position.min(properties.size.width).max(0.);

        self.current_value = map_range(self.current_normalized, 0., 1., self.min, self.max);

        if self.discrete {
            self.current_value = self.current_value.round();
        }
        if let Some(l) = &mut self.value_changed {
            (l)(self.current_value, state);
        }
    }

    fn mouse_moved(&mut self, _: &MouseEvent, _: &mut State) {}

    fn mouse_entered(&mut self, _: &MouseEvent, _: &mut State) {
        self.state = SliderState::Active
    }

    fn mouse_left(&mut self, _: &MouseEvent, _: &mut State) {
        self.state = SliderState::Inactive
    }
}

pub struct Switch<State> {
    value_changed: Option<Box<dyn FnMut(bool, &mut State)>>,
    active: bool,
    state: SliderState,
}

impl<State: AppState + 'static> Switch<State> {
    pub fn new() -> Self {
        Self {
            value_changed: None,
            active: false,
            state: SliderState::Inactive,
        }
    }
}

impl<State: AppState> Widget<State> for Switch<State> {
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        // Boldly unwrapping here. If you have not given constraints to a switch then we don't know how big it should be.
        Size::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        )
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, state: &State) {
        let mut fill_paint = Paint::default();
        fill_paint.set_anti_alias(true);

        let rounding = 4.0;
        let gutter_height = rect.height / 4.0;
        let thumb_size = gutter_height * 1.25;
        let active_thumb_size = thumb_size * 1.5;

        let rect = Rect::from_size(*rect);
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

        if let SliderState::Active = self.state {
            fill_paint.set_alpha_f(0.25);
            canvas.draw_circle(
                &Point::new(thumb_position, rect.center_y()),
                active_thumb_size,
                &fill_paint,
            );
        }
    }

    fn mouse_down(
        &mut self,
        _: &MouseEvent,
        _: &Properties,
        _: &mut App<State>,
        state: &mut State,
    ) {
        self.active = !self.active;
        if let Some(l) = &mut self.value_changed {
            (l)(self.active, state);
        }
    }

    fn mouse_up(&mut self, event: &MouseEvent, app: &mut App<State>, state: &mut State) {
        self.state = SliderState::Inactive
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, state: &mut State) {}

    fn mouse_moved(&mut self, event: &MouseEvent, state: &mut State) {}

    fn mouse_entered(&mut self, event: &MouseEvent, state: &mut State) {
        self.state = SliderState::Active
    }

    fn mouse_left(&mut self, event: &MouseEvent, state: &mut State) {
        self.state = SliderState::Inactive
    }
}
