use crate::{
    application::{Application, ApplicationModel},
    canvas::{Canvas2D, Color4f, Paint, Point, Rect, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Properties, Theme, Widget},
    window::MouseEvent,
};
use winit::event::KeyboardInput;

pub struct Container<Model> {
    padding: f32,
    margin: f32,
    border: f32,
    width: Option<f32>,
    height: Option<f32>,
    child: ChildSlot<Model>,
    paint: Option<Paint>,
}

impl<Model: ApplicationModel> Container<Model> {
    pub fn new(child: impl Widget<Model> + 'static) -> Self {
        Self {
            padding: 0.0,
            margin: 0.0,
            border: 0.0,
            width: None,
            height: None,
            child: ChildSlot::new(child),
            paint: None,
        }
    }

    pub fn with_color(mut self, color: &Color4f) -> Self {
        self.paint = Some(Paint::new(*color, None));
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }
}

impl<Model: ApplicationModel> Widget<Model> for Container<Model> {
    // The container's layout strategy is to be as small as possible.
    // So shrink input constraints by border, padding and margin
    // Then return its child's size as its own size.

    fn layout(&mut self, constraints: &BoxConstraints, model: &Model) -> Size {
        // If the container is not given constraints from the parent check if we've been given a size
        // If not given a size we ask the child to layout without constraints
        // This might panic if the child is a flex container.
        // If given a size we ask the child to layout with that size.
        // This might still panic if the child is for example a horizontal container, but only height is given.

        // If the container is given constraints we'll shrink them by padding/margin and ask the child to layout with those constraints

        let space_around = self.padding + self.margin + self.border;
        let child_size = if constraints.max_width().is_none() || constraints.max_height().is_none()
        {
            if self.width.is_none() || self.height.is_none() {
                self.child.layout(&BoxConstraints::new(), model)
            } else {
                let mut child_constraints = BoxConstraints::new();
                if self.width.is_some() {
                    child_constraints = child_constraints.with_max_width(self.width.unwrap_or(0.0))
                }
                if self.height.is_some() {
                    child_constraints =
                        child_constraints.with_max_height(self.height.unwrap_or(0.0))
                }
                self.child
                    .layout(&child_constraints.shrunk(space_around, space_around), model)
            }
        } else {
            let child_constraints = constraints.shrunk(space_around * 2.0, space_around * 2.0);
            self.child.layout(&child_constraints, model)
        };

        self.child
            .set_position(&Point::new(space_around, space_around));
        self.child.set_size(&child_size);

        Size::new(
            (child_size.width + space_around * 2.0).max(constraints.min_width().unwrap_or(0.0)),
            (child_size.height + space_around * 2.0).max(constraints.min_height().unwrap_or(0.0)),
        )
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, size: &Size, model: &Model) {
        if let Some(paint) = &self.paint {
            canvas.draw_rect(&Rect::from_size(*size), paint);
        }

        self.child.paint(theme, canvas, self.child.size(), model);
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        app: &mut Application<Model>,
        model: &mut Model,
    ) {
        self.child.mouse_down(event, properties, app, model);
    }

    fn mouse_up(&mut self, event: &MouseEvent, app: &mut Application<Model>, model: &mut Model) {
        self.child.mouse_up(event, app, model);
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

    fn flex(&self) -> f32 {
        0.0
    }

    fn character_received(&mut self, character: char, model: &mut Model) -> bool {
        self.child.character_received(character, model)
    }
}
