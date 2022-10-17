use crate::{
    app::AppState,
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{style::Theme, ChildSlot, Event, EventCtx, PaintCtx, Widget},
};

enum Direction {
    Horizontal,
    Vertical,
}

pub struct Flex<State> {
    direction: Direction,
    children: Vec<ChildSlot<State>>,
    spacing: f32,
}

impl<State: AppState> Flex<State> {
    pub fn row() -> Self {
        Self::new(Direction::Horizontal)
    }

    pub fn column() -> Self {
        Self::new(Direction::Vertical)
    }

    fn new(direction: Direction) -> Self {
        Self {
            direction,
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

impl<State: AppState> Widget<State> for Flex<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx, state: &State) -> bool {
        for child in &mut self.children {
            if child.event(event, ctx, state) {
                return true;
            }
        }

        false
    }

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        // This is not a scrollable view. It needs constraints
        // assert!(constraints.has_max());

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
                .fold(Size::default(), |mut acc, child_size| {
                    match self.direction {
                        Direction::Horizontal => {
                            acc.width += child_size.width + self.spacing;
                            acc.height = acc.height.max(child_size.height);
                        }
                        Direction::Vertical => {
                            acc.width = acc.width.max(child_size.width);
                            acc.height += child_size.height + self.spacing;
                        }
                    }

                    acc
                });

        let total_flex = self
            .children
            .iter()
            .fold(0.0, |acc, child| acc + child.flex());

        if total_flex > 0.0 {
            let total_spacing = (self.children.len() as f32 - 1.0) * self.spacing;

            let flex_factor = match self.direction {
                Direction::Horizontal => {
                    assert!(constraints.max_width().is_some());
                    let width = constraints.max_width().unwrap();
                    let unconstraint_width = width - total_spacing - constrained_size.width;
                    unconstraint_width / total_flex
                }
                Direction::Vertical => {
                    assert!(constraints.max_height().is_some());
                    let height = constraints.max_height().unwrap();
                    let unconstraint_height = height - total_spacing - constrained_size.height;
                    unconstraint_height / total_flex
                }
            };

            for child in &mut self.children {
                if child.flex() != 0.0 {
                    let child_constraints = match self.direction {
                        Direction::Horizontal => BoxConstraints::new_with_max(
                            flex_factor * child.flex(),
                            constraints.max_height().unwrap(),
                        ),
                        Direction::Vertical => BoxConstraints::new_with_max(
                            constraints.max_width().unwrap(),
                            flex_factor * child.flex(),
                        ),
                    };

                    let child_size = child.layout(&child_constraints, state);
                    child.set_size(&child_size);
                }
            }
        }

        let mut position = Point::default();
        for child in &mut self.children {
            child.set_position(&position);

            match self.direction {
                Direction::Horizontal => position.x += child.size().width + self.spacing,
                Direction::Vertical => position.y += child.size().height + self.spacing,
            }
        }

        if total_flex > 0.0 {
            Size::new(
                constraints.max_width().unwrap(),
                constraints.max_height().unwrap(),
            )
        } else {
            Size::new(constrained_size.width, constrained_size.height)
        }
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

impl<State: AppState> Default for Flex<State> {
    fn default() -> Self {
        Self::row()
    }
}
