use crate::{
    app::AppState,
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Event, EventCtx, PaintCtx, Theme, Widget},
};

pub struct FlexBox<State> {
    child: ChildSlot<State>,
    flex: f32,
}

impl<State: AppState> Widget<State> for FlexBox<State> {
    fn event(&mut self, _: &Event, _: &mut EventCtx, _: &State) -> bool {
        false
    }

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        self.child.layout(constraints, state)
    }

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, state: &State) {
        self.child.paint(theme, ctx, canvas, state)
    }

    fn flex(&self) -> f32 {
        self.flex
    }
}
