use crate::{
    app::{App, AppState},
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{Properties, Theme, Widget},
    window::MouseEvent,
};
use winit::event::KeyboardInput;

pub struct ChildSlot<State> {
    position: Point,
    size: Size,
    widget: Box<dyn Widget<State>>,
    has_mouse: bool,
}

impl<State: AppState> ChildSlot<State> {
    pub fn new(widget: impl Widget<State> + 'static) -> Self {
        Self {
            position: Point::default(),
            size: Size::default(),
            widget: Box::new(widget),
            has_mouse: false,
        }
    }

    pub fn new_with_box(widget: Box<dyn Widget<State>>) -> Self {
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

impl<State: AppState> Widget<State> for ChildSlot<State> {
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        self.widget.layout(constraints, state)
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, _: &Size, state: &State) {
        canvas.save();
        canvas.translate(self.position());
        self.widget.paint(theme, canvas, self.size(), state);
        canvas.restore();
    }

    fn flex(&self) -> f32 {
        self.widget.flex()
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        _: &Properties,
        app: &mut App<State>,
        state: &mut State,
    ) {
        if self.hit_test(event.local_position()) {
            let properties = Properties {
                position: *self.position(),
                size: *self.size(),
            };
            let new_event = event.to_local(self.position());
            self.widget.mouse_down(&new_event, &properties, app, state);
        }
    }

    fn mouse_up(&mut self, event: &MouseEvent, app: &mut App<State>, state: &mut State) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());
            self.widget.mouse_up(&new_event, app, state);
        } else if self.has_mouse {
            self.has_mouse = false;
            let new_event = event.to_local(self.position());
            self.widget.mouse_left(&new_event, state);
        }
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, state: &mut State) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());
            self.widget.mouse_dragged(&new_event, properties, state);
        }
    }

    fn mouse_moved(&mut self, event: &MouseEvent, state: &mut State) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());

            if !self.has_mouse {
                self.has_mouse = true;
                self.mouse_entered(event, state);
            }

            self.widget.mouse_moved(&new_event, state);
        } else {
            let new_event = event.to_local(self.position());
            if self.has_mouse {
                self.has_mouse = false;
                self.widget.mouse_left(event, state);
            }

            self.widget.mouse_moved(&new_event, state);
        }
    }

    fn mouse_entered(&mut self, event: &MouseEvent, state: &mut State) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());
            self.widget.mouse_entered(&new_event, state)
        }
    }

    fn mouse_left(&mut self, event: &MouseEvent, state: &mut State) {
        if self.hit_test(event.local_position()) {
            let new_event = event.to_local(self.position());
            self.widget.mouse_left(&new_event, state)
        }
    }

    fn keyboard_event(&mut self, event: &KeyboardInput, state: &mut State) -> bool {
        self.widget.keyboard_event(event, state)
    }

    fn character_received(&mut self, character: char, state: &mut State) -> bool {
        self.widget.character_received(character, state)
    }
}
