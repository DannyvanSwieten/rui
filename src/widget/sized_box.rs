use crate::{
    app::{App, AppState},
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Properties, Theme, Widget},
    window::MouseEvent,
};
use winit::event::KeyboardInput;

pub struct SizedBox<State> {
    size: Size,
    child: ChildSlot<State>,
}

impl<State: AppState> SizedBox<State> {
    pub fn new(size: Size, child: impl Widget<State> + 'static) -> Self {
        Self {
            size,
            child: ChildSlot::new(child),
        }
    }
}

impl<State: AppState> Widget<State> for SizedBox<State> {
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        let child_constraints =
            BoxConstraints::new().with_tight_constraints(self.size.width, self.size.height);
        self.child.layout(&child_constraints, state);
        self.child.set_size(&self.size);
        self.size
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, state: &State) {
        self.child.paint(theme, canvas, rect, state);
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        app: &mut App<State>,
        state: &mut State,
    ) {
        self.child.mouse_down(event, properties, app, state)
    }

    fn mouse_up(&mut self, event: &MouseEvent, app: &mut App<State>, state: &mut State) {
        self.child.mouse_up(event, app, state)
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, state: &mut State) {
        self.child.mouse_dragged(event, properties, state)
    }

    fn mouse_moved(&mut self, event: &MouseEvent, state: &mut State) {
        self.child.mouse_moved(event, state)
    }

    fn mouse_entered(&mut self, event: &MouseEvent, state: &mut State) {
        self.child.mouse_entered(event, state)
    }

    fn mouse_left(&mut self, event: &MouseEvent, state: &mut State) {
        self.child.mouse_left(event, state)
    }

    fn keyboard_event(&mut self, event: &KeyboardInput, state: &mut State) -> bool {
        self.child.keyboard_event(event, state)
    }
}
