use crate::{
    app::AppState,
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Event, EventCtx, PaintCtx, Theme, Widget},
};

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
    fn event(&mut self, event: &Event, ctx: &mut EventCtx, state: &State) -> bool {
        self.child.event(event, ctx, state)
    }

    fn layout(&mut self, _: &BoxConstraints, state: &State) -> Size {
        let child_constraints =
            BoxConstraints::new().with_tight_constraints(self.size.width, self.size.height);
        self.child.layout(&child_constraints, state);
        self.child.set_size(&self.size);
        self.size
    }

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, state: &State) {
        self.child.paint(theme, ctx, canvas, state);
    }
}
