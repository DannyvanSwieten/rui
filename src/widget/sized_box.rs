use crate::{
    app::{App, AppState},
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Properties, Theme, Widget},
    window::MouseEvent,
};
use winit::event::KeyboardInput;

pub struct SizedBox<Model> {
    size: Size,
    child: ChildSlot<Model>,
}

impl<Model: AppState> SizedBox<Model> {
    pub fn new(size: Size, child: impl Widget<Model> + 'static) -> Self {
        Self {
            size,
            child: ChildSlot::new(child),
        }
    }
}

impl<Model: AppState> Widget<Model> for SizedBox<Model> {
    fn layout(&mut self, constraints: &BoxConstraints, model: &Model) -> Size {
        let child_constraints =
            BoxConstraints::new().with_tight_constraints(self.size.width, self.size.height);
        self.child.layout(&child_constraints, model);
        self.child.set_size(&self.size);
        self.size
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, model: &Model) {
        self.child.paint(theme, canvas, rect, model);
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        app: &mut App<Model>,
        model: &mut Model,
    ) {
        self.child.mouse_down(event, properties, app, model)
    }

    fn mouse_up(&mut self, event: &MouseEvent, app: &mut App<Model>, model: &mut Model) {
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
