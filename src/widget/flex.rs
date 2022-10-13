use crate::{
    app::{App, AppState},
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{style::Theme, ChildSlot, Event, EventCtx, Properties, Widget},
    window::MouseEvent,
};

pub struct Row<State> {
    children: Vec<ChildSlot<State>>,
    spacing: f32,
}

impl<State: AppState> Row<State> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
        }
    }

    pub fn push<W>(mut self, child: W) -> Self
    where
        W: Widget<State> + 'static,
    {
        self.children.push(ChildSlot::new(child));
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<State: AppState> Widget<State> for Row<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) {
        for child in &mut self.children {
            child.event(event, ctx, state)
        }
    }

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        // This is not a scrollable view. It needs constraints
        assert!(constraints.max_width().is_some() && constraints.max_height().is_some());
        // Start without child constraints
        let child_constraints = BoxConstraints::new();
        let constrained_sizes: Vec<Size> = self
            .children
            .iter_mut()
            .flat_map(|child| {
                if child.flex() == 0.0 {
                    let child_size = child.layout(&child_constraints, state);
                    child.set_size(&child_size);
                    Some(child_size)
                } else {
                    None
                }
            })
            .collect();

        let constrained_size =
            constrained_sizes
                .iter()
                .fold(Size::new(0.0, 0.0), |mut acc, child_size| {
                    acc.width += child_size.width + self.spacing;
                    acc.height = acc.height.max(child_size.height);
                    acc
                });

        let total_flex = self
            .children
            .iter()
            .fold(0.0, |acc, child| acc + child.flex());

        if total_flex > 0.0 {
            let width = constraints.max_width().unwrap();
            let unconstraint_width = width - constrained_size.width;
            let flex_factor = unconstraint_width / total_flex;
            for child in &mut self.children {
                if child.flex() != 0.0 {
                    let child_constraints = BoxConstraints::new()
                        .with_max_width(flex_factor * child.flex())
                        .with_max_height(constraints.max_height().unwrap());
                    let child_size = child.layout(&child_constraints, state);
                    child.set_size(&child_size);
                }
            }
        }

        let mut position = Point::new(0.0, 0.0);
        for child in &mut self.children {
            child.set_position(&position);
            position.x += child.size().width + self.spacing;
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

    fn flex(&self) -> f32 {
        0.0
    }

    fn keyboard_event(&mut self, event: &winit::event::KeyboardInput, state: &mut State) -> bool {
        for child in &mut self.children {
            if child.keyboard_event(event, state) {
                return true;
            }
        }

        false
    }

    fn character_received(&mut self, character: char, state: &mut State) -> bool {
        for child in &mut self.children {
            if child.character_received(character, state) {
                return true;
            }
        }

        false
    }
}

pub struct Column<State> {
    children: Vec<ChildSlot<State>>,
    spacing: f32,
}

impl<State: AppState> Column<State> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
        }
    }

    pub fn push<W>(mut self, child: W) -> Self
    where
        W: Widget<State> + 'static,
    {
        self.children.push(ChildSlot::new(child));
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<State: AppState> Widget<State> for Column<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) {
        for child in &mut self.children {
            child.event(event, ctx, state)
        }
    }

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        // It needs constraints
        assert!(constraints.max_width().is_some() && constraints.max_height().is_some());
        let total_spacing = (self.children.len() as f32 - 1.0) * self.spacing;
        // Start with no constraints
        let child_constraints = BoxConstraints::new();
        let constrained_sizes: Vec<Size> = self
            .children
            .iter_mut()
            .flat_map(|child| {
                if child.flex() == 0.0 {
                    let child_size = child.layout(&child_constraints, state);
                    child.set_size(&child_size);
                    Some(child_size)
                } else {
                    None
                }
            })
            .collect();

        let constrained_size =
            constrained_sizes
                .iter()
                .fold(Size::new(0.0, 0.0), |mut acc, child_size| {
                    acc.height += child_size.height + self.spacing;
                    acc.width = acc.width.max(child_size.width);
                    acc
                });

        let total_flex = self
            .children
            .iter()
            .fold(0.0, |acc, child| acc + child.flex());

        if total_flex > 0.0 {
            let height = constraints.max_height().unwrap();
            let unconstraint_height = height - total_spacing - constrained_size.height;
            let flex_factor = unconstraint_height / total_flex;
            for child in &mut self.children {
                if child.flex() != 0.0 {
                    let child_constraints = BoxConstraints::new()
                        .with_max_height(flex_factor * child.flex())
                        .with_max_width(constraints.max_width().unwrap());
                    let child_size = child.layout(&child_constraints, state);
                    child.set_size(&child_size);
                }
            }
        }

        let mut position = Point::new(0.0, 0.0);
        for child in &mut self.children {
            child.set_position(&position);
            position.y += child.size().height + self.spacing;
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

    fn flex(&self) -> f32 {
        0.0
    }

    fn keyboard_event(&mut self, event: &winit::event::KeyboardInput, state: &mut State) -> bool {
        for child in &mut self.children {
            if child.keyboard_event(event, state) {
                return true;
            }
        }

        false
    }

    fn character_received(&mut self, character: char, state: &mut State) -> bool {
        for child in &mut self.children {
            if child.character_received(character, state) {
                return true;
            }
        }

        false
    }
}

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
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) {
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

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, state: &State) {
        self.child.paint(theme, canvas, rect, state)
    }

    fn flex(&self) -> f32 {
        self.flex
    }

    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, state: &mut State) {
        self.child.mouse_dragged(event, properties, state)
    }

    fn mouse_moved(&mut self, event: &MouseEvent, state: &mut State) {
        self.child.mouse_moved(event, state)
    }

    fn mouse_entered(&mut self, _event: &MouseEvent, _state: &mut State) {}

    fn mouse_left(&mut self, _event: &MouseEvent, _state: &mut State) {}

    fn keyboard_event(&mut self, event: &winit::event::KeyboardInput, state: &mut State) -> bool {
        self.child.keyboard_event(event, state)
    }

    fn character_received(&mut self, character: char, state: &mut State) -> bool {
        self.child.character_received(character, state)
    }
}
