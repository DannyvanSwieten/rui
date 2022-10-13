use crate::{
    app::AppState,
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Event, EventCtx, Properties, Theme, Widget},
    window::MouseEvent,
};
use winit::event::KeyboardInput;

pub struct Center<State> {
    child: ChildSlot<State>,
    size: Option<Size>,
}

impl<State: AppState> Center<State> {
    pub fn new<W: Widget<State> + 'static>(child: W) -> Self {
        Self {
            child: ChildSlot::new_with_box(Box::new(child)),
            size: None,
        }
    }
}

impl<State: AppState> Widget<State> for Center<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) {
        self.child.event(event, ctx, state)
    }

    // The layout strategy for a center node: return own size if not None, otherwise as big as possible within given constraints.
    // Then center the child.
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        let my_size = if let Some(size) = &self.size {
            *size
        } else {
            // If not given a size then we need to have constraints from parent.
            Size::new(
                constraints.max_width().unwrap(),
                constraints.max_height().unwrap(),
            )
        };

        let child_size = self.child.layout(
            &BoxConstraints::new()
                .with_max_width(my_size.width)
                .with_max_height(my_size.height),
            state,
        );

        self.child.set_size(&child_size);

        let x_offset = (my_size.width - child_size.width) / 2.0;
        let y_offset = (my_size.height - child_size.height) / 2.0;
        self.child.set_position(&Point::new(x_offset, y_offset));

        my_size
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, state: &State) {
        self.child.paint(theme, canvas, rect, state)
    }

    fn keyboard_event(&mut self, event: &KeyboardInput, state: &mut State) -> bool {
        self.child.keyboard_event(event, state)
    }
}
