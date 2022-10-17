use crate::{
    app::AppState,
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{style::Theme, ChildSlot, Event, EventCtx, PaintCtx, Widget},
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
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) -> bool {
        for child in &mut self.children {
            if child.event(event, ctx, state) {
                return true;
            }
        }

        false
    }

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        // This is not a scrollable view. It needs constraints
        assert!(constraints.has_max());

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

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, state: &State) {
        for child in &self.children {
            child.paint(theme, ctx, canvas, state)
        }
    }

    fn flex(&self) -> f32 {
        0.0
    }
}

impl<State: AppState> Default for Row<State> {
    fn default() -> Self {
        Self::new()
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
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) -> bool {
        for child in &mut self.children {
            if child.event(event, ctx, state) {
                return true;
            }
        }

        false
    }

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        // It needs constraints
        assert!(constraints.has_max());

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

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, state: &State) {
        for child in &self.children {
            child.paint(theme, ctx, canvas, state)
        }
    }

    fn flex(&self) -> f32 {
        0.0
    }
}

impl<State: AppState> Default for Column<State> {
    fn default() -> Self {
        Self::new()
    }
}
