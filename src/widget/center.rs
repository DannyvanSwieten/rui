use crate::{
    application::{Application, ApplicationModel},
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Properties, Theme, Widget},
    window::MouseEvent,
};
use winit::event::KeyboardInput;

pub struct Center<Model> {
    child: ChildSlot<Model>,
    size: Option<Size>,
}

impl<Model: ApplicationModel> Center<Model> {
    pub fn new<W: Widget<Model> + 'static>(child: W) -> Self {
        Self {
            child: ChildSlot::new_with_box(Box::new(child)),
            size: None,
        }
    }
}

impl<Model: ApplicationModel> Widget<Model> for Center<Model> {
    // The layout strategy for a center node: return own size if not None, otherwise as big as possible within given constraints.
    // Then center the child.
    fn layout(&mut self, constraints: &BoxConstraints, model: &Model) -> Size {
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
            model,
        );

        self.child.set_size(&child_size);

        let x_offset = (my_size.width - child_size.width) / 2.0;
        let y_offset = (my_size.height - child_size.height) / 2.0;
        self.child.set_position(&Point::new(x_offset, y_offset));

        my_size
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, model: &Model) {
        self.child.paint(theme, canvas, rect, model)
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        app: &mut Application<Model>,
        model: &mut Model,
    ) {
        self.child.mouse_down(event, properties, app, model)
    }

    fn mouse_up(&mut self, event: &MouseEvent, app: &mut Application<Model>, model: &mut Model) {
        self.child.mouse_up(event, app, model)
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, model: &mut Model) {
        self.child.mouse_dragged(event, properties, model)
    }

    fn mouse_moved(&mut self, event: &MouseEvent, model: &mut Model) {
        self.child.mouse_moved(event, model)
    }

    fn mouse_entered(&mut self, event: &MouseEvent, model: &mut Model) {
        self.child.mouse_entered(event, model)
    }

    fn mouse_left(&mut self, event: &MouseEvent, model: &mut Model) {
        self.child.mouse_left(event, model)
    }

    fn keyboard_event(&mut self, event: &KeyboardInput, model: &mut Model) -> bool {
        self.child.keyboard_event(event, model)
    }
}
