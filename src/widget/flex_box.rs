use crate::{
    app::AppState,
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Event, EventCtx, PaintCtx, Theme, Widget},
};

use super::LayoutCtx;

pub struct FlexBox<State> {
    child: ChildSlot<State>,
    flex: f32,
}

impl<State: AppState> Widget<State> for FlexBox<State> {
    fn event(&mut self, _: &Event, _: &mut EventCtx<State::Message>, _: &State) -> bool {
        false
    }

    fn layout(&mut self, constraints: &BoxConstraints, ctx: &mut LayoutCtx, state: &State) -> Size {
        ctx.register_child(self.child.uid());
        self.child.layout(constraints, ctx, state)
    }

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, state: &State) {
        self.child.paint(theme, ctx, canvas, state)
    }

    fn flex(&self) -> f32 {
        self.flex
    }
}
