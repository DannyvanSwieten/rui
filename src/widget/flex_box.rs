use crate::{
    app::{App, AppState},
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{ChildSlot, Properties, Theme, Widget},
    window::MouseEvent,
};
use winit::event::KeyboardInput;

pub struct FlexBox<Model> {
    child: ChildSlot<Model>,
    flex: f32,
}

impl<Model: AppState> Widget<Model> for FlexBox<Model> {
    fn layout(&mut self, constraints: &BoxConstraints, model: &Model) -> Size {
        self.child.layout(constraints, model)
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, model: &Model) {
        self.child.paint(theme, canvas, rect, model)
    }

    fn flex(&self) -> f32 {
        self.flex
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        _: &mut App<Model>,
        model: &mut Model,
    ) {
        todo!()
    }

    fn mouse_up(&mut self, event: &MouseEvent, _: &mut App<Model>, model: &mut Model) {
        todo!()
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, model: &mut Model) {
        todo!()
    }

    fn mouse_moved(&mut self, event: &MouseEvent, model: &mut Model) {
        todo!()
    }

    fn mouse_entered(&mut self, event: &MouseEvent, model: &mut Model) {
        todo!()
    }

    fn mouse_left(&mut self, event: &MouseEvent, model: &mut Model) {
        todo!()
    }

    fn keyboard_event(&mut self, event: &KeyboardInput, model: &mut Model) -> bool {
        self.child.keyboard_event(event, model)
    }
}
