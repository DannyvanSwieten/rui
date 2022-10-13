use crate::{
    app::{App, AppState},
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{style::Theme, ChildSlot, Properties, Widget},
    window::MouseEvent,
};

pub struct List<State> {
    spacing: f32,
    // If not None this will force all children to this size in the scroll direction
    item_size: Option<f32>,
    // If not None this will call the builder callback with index 0..item_count
    item_count: Option<usize>,
    builder: Option<Box<dyn Fn(usize, &State) -> Box<dyn Widget<State>>>>,
    children: Vec<ChildSlot<State>>,
    viewport_position: f32,
}

impl<State: AppState> List<State> {
    pub fn new() -> Self {
        Self {
            spacing: 0.0,
            item_size: None,
            item_count: None,
            builder: None,
            children: Vec::new(),
            viewport_position: 0.0,
        }
    }

    pub fn with_builder<F>(mut self, item_count: usize, builder: F) -> Self
    where
        F: Fn(usize, &State) -> Box<dyn Widget<State>> + 'static,
    {
        self.builder = Some(Box::new(builder));
        self.item_count = Some(item_count);
        self
    }

    pub fn new_with_children(children: Vec<Box<dyn Widget<State>>>) -> Self {
        Self {
            spacing: 0.0,
            item_size: None,
            item_count: None,
            builder: None,
            children: children
                .into_iter()
                .map(|child| ChildSlot::new_with_box(child))
                .collect(),
            viewport_position: 0.0,
        }
    }

    pub fn with_item_size(mut self, size: f32) -> Self {
        self.item_size = Some(size);
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn push(mut self, child: impl Widget<State> + 'static) -> Self {
        self.children.push(ChildSlot::new(child));
        self
    }
}

impl<State: AppState> Widget<State> for List<State> {
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        if let Some(builder) = &self.builder {
            self.children.clear();
            for i in 0..self.item_count.unwrap() {
                self.children
                    .push(ChildSlot::new_with_box(builder(i, state)))
            }
        }

        let mut y = 0.0;

        for child in &mut self.children {
            let child_constraints = if let Some(item_size) = self.item_size {
                BoxConstraints::new()
                    .with_max_height(item_size)
                    .with_max_width(constraints.max_width().unwrap())
            } else {
                BoxConstraints::new().with_max_width(constraints.max_width().unwrap())
            };
            let mut child_size = child.layout(&child_constraints, state);
            child_size.height = self.item_size.unwrap_or(child_size.height);
            child.set_size(&child_size);
            child.set_position(&Point::new(0.0, y));
            y += child_size.height + self.spacing
        }

        Size::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        )
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, state: &State) {
        for child in &self.children {
            child.paint(theme, canvas, rect, state)
        }
    }

    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        app: &mut App<State>,
        state: &mut State,
    ) {
        for child in &mut self.children {
            child.mouse_down(event, properties, app, state)
        }
    }

    fn mouse_up(&mut self, event: &MouseEvent, app: &mut App<State>, state: &mut State) {
        for child in &mut self.children {
            child.mouse_up(event, app, state)
        }
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, state: &mut State) {
        for child in &mut self.children {
            child.mouse_dragged(event, properties, state)
        }
    }

    fn mouse_moved(&mut self, event: &MouseEvent, state: &mut State) {
        for child in &mut self.children {
            child.mouse_moved(event, state)
        }
    }

    fn mouse_entered(&mut self, event: &MouseEvent, state: &mut State) {
        for child in &mut self.children {
            child.mouse_entered(event, state)
        }
    }

    fn mouse_left(&mut self, event: &MouseEvent, state: &mut State) {
        for child in &mut self.children {
            child.mouse_left(event, state)
        }
    }

    fn keyboard_event(&mut self, event: &winit::event::KeyboardInput, state: &mut State) -> bool {
        for child in &mut self.children {
            if child.keyboard_event(event, state) {
                return true;
            }
        }

        false
    }
}
