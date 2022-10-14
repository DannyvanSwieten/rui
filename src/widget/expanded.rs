use crate::{
    app::AppState,
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{style::Theme, ChildSlot, Event, EventCtx, PaintCtx, Widget},
};

pub struct Expanded<State> {
    child: ChildSlot<State>,
    width: Option<f32>,
    height: Option<f32>,
    flex: f32,
}

impl<State: AppState> Expanded<State> {
    pub fn new(child: impl Widget<State> + 'static) -> Self {
        Self {
            child: ChildSlot::new(child),
            width: None,
            height: None,
            flex: 1.0,
        }
    }

    pub fn with_flex(mut self, flex: f32) -> Self {
        self.flex = flex;
        self
    }

    pub fn with_width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }
}

impl<State: AppState> Widget<State> for Expanded<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) -> bool {
        self.child.event(event, ctx, state)
    }

    // If given to a flex container it will expand based on it's flex parameter in the dominant layout direction.
    // If for example you add it to a row it will expand in the horizontal direction. Therefor you should provide a height.
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        let size = Size::new(
            self.width
                .unwrap_or_else(|| constraints.max_width().unwrap()),
            self.height
                .unwrap_or_else(|| constraints.max_height().unwrap()),
        );

        let child_size = self.child.layout(
            &BoxConstraints::new().with_tight_constraints(size.width, size.height),
            state,
        );

        self.child.set_size(&child_size);
        size
    }

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, state: &State) {
        self.child.paint(theme, ctx, canvas, state)
    }

    fn flex(&self) -> f32 {
        self.flex
    }
}
