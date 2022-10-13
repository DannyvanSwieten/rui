use crate::{
    app::AppState,
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Event, EventCtx, Theme, Widget},
};
use winit::event::KeyboardInput;

pub struct FlexBox<State> {
    child: ChildSlot<State>,
    flex: f32,
}

impl<State: AppState> Widget<State> for FlexBox<State> {
    fn event(&mut self, _: &Event, _: &mut EventCtx<State>, _: &mut State) {
        //
    }

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        self.child.layout(constraints, state)
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, state: &State) {
        self.child.paint(theme, canvas, rect, state)
    }

    fn flex(&self) -> f32 {
        self.flex
    }

    fn keyboard_event(&mut self, event: &KeyboardInput, state: &mut State) -> bool {
        self.child.keyboard_event(event, state)
    }
}
