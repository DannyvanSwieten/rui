use crate::{
    app::{App, AppState},
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{Properties, Theme, Widget},
    window::MouseEvent,
};
use winit::event::KeyboardInput;

pub struct ChildSlot<Model> {
    position: Point,
    size: Size,
    widget: Box<dyn Widget<Model>>,
    has_mouse: bool,
}

impl<Model: AppState> ChildSlot<Model> {
    pub fn new(widget: impl Widget<Model> + 'static) -> Self {
        Self {
            position: Point::default(),
            size: Size::default(),
            widget: Box::new(widget),
            has_mouse: false,
        }
    }

    pub fn new_with_box(widget: Box<dyn Widget<Model>>) -> Self {
        Self {
            position: Point::default(),
            size: Size::default(),
            widget,
            has_mouse: false,
        }
    }

    pub fn set_size(&mut self, size: &Size) {
        self.size = *size
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn set_position(&mut self, position: &Point) {
        self.position = *position
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn hit_test(&mut self, point: &Point) -> bool {
        let x = point.x >= self.position.x && point.x < self.position.x + self.size.width;
        let y = point.y >= self.position.y && point.y < self.position.y + self.size.height;

        x && y
    }
}

impl<Model: AppState> Widget<Model> for ChildSlot<Model> {
    fn layout(&mut self, constraints: &BoxConstraints, model: &Model) -> Size {
        self.widget.layout(constraints, model)
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, _: &Size, model: &Model) {
        canvas.save();
        canvas.translate(self.position());
        self.widget.paint(theme, canvas, self.size(), model);
        canvas.restore();
    }

    fn flex(&self) -> f32 {
        self.widget.flex()
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        _: &Properties,
        app: &mut App<Model>,
        model: &mut Model,
    ) {
        if self.hit_test(event.local_position()) {
            let properties = Properties {
                position: *self.position(),
                size: *self.size(),
            };
            let new_event = event.to_local(self.position());
            self.widget.mouse_down(&new_event, &properties, app, model);
        }
    }

    fn mouse_up(&mut self, event: &MouseEvent, app: &mut App<Model>, model: &mut Model) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());
            self.widget.mouse_up(&new_event, app, model);
        } else if self.has_mouse {
            self.has_mouse = false;
            let new_event = event.to_local(self.position());
            self.widget.mouse_left(&new_event, model);
        }
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, model: &mut Model) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());
            self.widget.mouse_dragged(&new_event, properties, model);
        }
    }

    fn mouse_moved(&mut self, event: &MouseEvent, model: &mut Model) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());

            if !self.has_mouse {
                self.has_mouse = true;
                self.mouse_entered(event, model);
            }

            self.widget.mouse_moved(&new_event, model);
        } else {
            let new_event = event.to_local(self.position());
            if self.has_mouse {
                self.has_mouse = false;
                self.widget.mouse_left(event, model);
            }

            self.widget.mouse_moved(&new_event, model);
        }
    }

    fn mouse_entered(&mut self, event: &MouseEvent, model: &mut Model) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());
            self.widget.mouse_entered(&new_event, model)
        }
    }

    fn mouse_left(&mut self, event: &MouseEvent, model: &mut Model) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());
            self.widget.mouse_left(&new_event, model)
        }
    }

    fn keyboard_event(&mut self, event: &KeyboardInput, model: &mut Model) -> bool {
        self.widget.keyboard_event(event, model)
    }

    fn character_received(&mut self, character: char, model: &mut Model) -> bool {
        self.widget.character_received(character, model)
    }
}
