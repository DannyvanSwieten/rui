use crate::{
    app::{App, AppState},
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Properties, Theme, Widget},
    window::MouseEvent,
};
use winit::event::KeyboardInput;

pub struct FlexBox<State> {
    child: ChildSlot<State>,
    flex: f32,
}

impl<State: AppState> Widget<State> for FlexBox<State> {
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        self.child.layout(constraints, state)
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, state: &State) {
        self.child.paint(theme, canvas, rect, state)
    }

    fn flex(&self) -> f32 {
        self.flex
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        _: &mut App<State>,
        state: &mut State,
    ) {
        todo!()
    }

    fn mouse_up(&mut self, event: &MouseEvent, _: &mut App<State>, state: &mut State) {
        todo!()
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, state: &mut State) {
        todo!()
    }

    fn mouse_moved(&mut self, event: &MouseEvent, state: &mut State) {
        todo!()
    }

    fn mouse_entered(&mut self, event: &MouseEvent, state: &mut State) {
        todo!()
    }

    fn mouse_left(&mut self, event: &MouseEvent, state: &mut State) {
        todo!()
    }

    fn keyboard_event(&mut self, event: &KeyboardInput, state: &mut State) -> bool {
        self.child.keyboard_event(event, state)
    }
}
