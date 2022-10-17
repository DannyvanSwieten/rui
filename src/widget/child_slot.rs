use crate::{
    app::AppState,
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{Event, EventCtx, MouseEvent, PaintCtx, Properties, Theme, Widget},
};

pub struct ChildSlot<State> {
    widget: Box<dyn Widget<State>>,
    properties: Properties,
}

impl<State: AppState> ChildSlot<State> {
    pub fn new(widget: impl Widget<State> + 'static) -> Self {
        Self {
            widget: Box::new(widget),
            properties: Properties::default(),
        }
    }

    pub fn new_with_box(widget: Box<dyn Widget<State>>) -> Self {
        Self {
            widget,
            properties: Properties::default(),
        }
    }

    pub fn set_position(&mut self, position: &Point) {
        self.properties.position = *position
    }

    pub fn position(&self) -> &Point {
        &self.properties.position
    }

    pub fn set_size(&mut self, size: &Size) {
        self.properties.size = *size
    }

    pub fn size(&self) -> &Size {
        &self.properties.size
    }

    pub fn hit_test(&self, point: &Point) -> bool {
        let pos = self.properties.position;
        let size = self.properties.size;

        let x = point.x >= pos.x && point.x < pos.x + size.width;
        let y = point.y >= pos.y && point.y < pos.y + size.height;

        x && y
    }

    fn propagate_mouse_event(
        &mut self,
        event: &MouseEvent,
        ctx: &mut EventCtx,
        state: &State,
    ) -> bool {
        if self.hit_test(event.local_position()) {
            if !self.properties.has_mouse {
                if let MouseEvent::MouseMove(event) = event {
                    self.properties.has_mouse = true;
                    self.widget
                        .event(&Event::Mouse(MouseEvent::MouseEnter(*event)), ctx, state);
                }
            }

            let inner_event = event.to_local(self.position());
            let mut inner_ctx = EventCtx {
                properties: &self.properties,
                window_id: ctx.window_id,
            };
            self.widget
                .event(&Event::Mouse(inner_event), &mut inner_ctx, state)
        } else if self.properties.has_mouse {
            match event {
                MouseEvent::MouseMove(event) | MouseEvent::MouseUp(event) => {
                    self.properties.has_mouse = false;
                    self.widget
                        .event(&Event::Mouse(MouseEvent::MouseLeave(*event)), ctx, state);
                    false
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

impl<State: AppState> Widget<State> for ChildSlot<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx, state: &State) -> bool {
        match event {
            Event::Mouse(event) => self.propagate_mouse_event(event, ctx, state),
            Event::Key(_) => self.widget.event(event, ctx, state),
        }
    }

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        self.widget.layout(constraints, state)
    }

    fn paint(&self, theme: &Theme, _: &PaintCtx, canvas: &mut dyn Canvas2D, state: &State) {
        let inner_ctx = PaintCtx::new(&self.properties);

        canvas.save();
        canvas.translate(self.position());
        self.widget.paint(theme, &inner_ctx, canvas, state);
        canvas.restore();
    }

    fn flex(&self) -> f32 {
        self.widget.flex()
    }
}
