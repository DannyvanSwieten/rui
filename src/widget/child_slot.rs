use crate::{
    app::AppState,
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{Event, EventCtx, MouseEvent, Properties, Theme, Widget},
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

    pub fn hit_test(&mut self, point: &Point) -> bool {
        let pos = self.properties.position;
        let size = self.properties.size;

        let x = point.x >= pos.x && point.x < pos.x + size.width;
        let y = point.y >= pos.y && point.y < pos.y + size.height;

        x && y
    }

    fn propagate_mouse_event(
        &mut self,
        event: &MouseEvent,
        ctx: &mut EventCtx<State>,
        state: &mut State,
    ) -> bool {
        if self.hit_test(event.local_position()) {
            let inner_event = event.to_local(self.position());
            let mut inner_ctx = EventCtx {
                app: ctx.app(),
                size: self.properties.size,
            };

            if !self.properties.has_mouse {
                if let MouseEvent::MouseMove(event) = inner_event {
                    self.properties.has_mouse = true;
                    self.widget.event(
                        &Event::Mouse(MouseEvent::MouseEnter(event)),
                        &mut inner_ctx,
                        state,
                    );
                }
            }

            self.widget
                .event(&Event::Mouse(inner_event), &mut inner_ctx, state)
        } else if self.properties.has_mouse {
            match event {
                MouseEvent::MouseMove(event) => {
                    self.properties.has_mouse = false;
                    let mut inner_ctx = EventCtx {
                        app: ctx.app(),
                        size: self.properties.size,
                    };

                    self.widget.event(
                        &Event::Mouse(MouseEvent::MouseLeave(event.to_local(self.position()))),
                        &mut inner_ctx,
                        state,
                    )
                }
                MouseEvent::MouseUp(event) => {
                    self.properties.has_mouse = false; // Is this redundant? Don't we come across MouseMove first?

                    let inner_event = event.to_local(self.position());
                    let mut inner_ctx = EventCtx {
                        app: ctx.app(),
                        size: self.properties.size,
                    };

                    self.widget.event(
                        &Event::Mouse(MouseEvent::MouseLeave(inner_event)),
                        &mut inner_ctx,
                        state,
                    )
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

impl<State: AppState> Widget<State> for ChildSlot<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) -> bool {
        match event {
            Event::Mouse(event) => self.propagate_mouse_event(event, ctx, state),
            Event::Key(_) => self.widget.event(event, ctx, state),
        }
    }

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        self.widget.layout(constraints, state)
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, _: &Size, state: &State) {
        canvas.save();
        canvas.translate(self.position());
        self.widget.paint(theme, canvas, self.size(), state);
        canvas.restore();
    }

    fn flex(&self) -> f32 {
        self.widget.flex()
    }
}
