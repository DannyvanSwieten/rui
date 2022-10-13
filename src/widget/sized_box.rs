use crate::{
    app::AppState,
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Event, EventCtx, Properties, Theme, Widget},
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
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) {
        self.child.event(event, ctx, state)
    }

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

    fn keyboard_event(&mut self, event: &KeyboardInput, state: &mut State) -> bool {
        self.child.keyboard_event(event, state)
    }
}
