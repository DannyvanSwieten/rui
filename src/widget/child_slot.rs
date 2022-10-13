use crate::{
    app::AppState,
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{Event, EventCtx, MouseEvent, Theme, Widget},
};
use winit::event::KeyboardInput;

pub struct ChildSlot<State> {
    position: Point,
    size: Size,
    widget: Box<dyn Widget<State>>,
    has_mouse: bool,
}

impl<State: AppState> ChildSlot<State> {
    pub fn new(widget: impl Widget<State> + 'static) -> Self {
        Self {
            position: Point::default(),
            size: Size::default(),
            widget: Box::new(widget),
            has_mouse: false,
        }
    }

    pub fn new_with_box(widget: Box<dyn Widget<State>>) -> Self {
        Self {
            position: Point::default(),
            size: Size::default(),
            widget,
            has_mouse: false,
        }
    }

    pub fn set_size(&mut self, size: &Size) {
        self.size = *size
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn set_position(&mut self, position: &Point) {
        self.position = *position
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn hit_test(&mut self, point: &Point) -> bool {
        let x = point.x >= self.position.x && point.x < self.position.x + self.size.width;
        let y = point.y >= self.position.y && point.y < self.position.y + self.size.height;

        x && y
    }

    fn propagate_mouse_event(
        &mut self,
        event: &MouseEvent,
        ctx: &mut EventCtx<State>,
        state: &mut State,
    ) {
        if self.hit_test(event.local_position()) {
            let inner_event = event.to_local(self.position());
            let mut inner_ctx = EventCtx {
                app: ctx.app(),
                size: self.size,
            };

            if !self.has_mouse {
                if let MouseEvent::MouseMove(event) = inner_event {
                    self.has_mouse = true;
                    self.widget.event(
                        &Event::Mouse(MouseEvent::MouseEnter(event)),
                        &mut inner_ctx,
                        state,
                    );
                }
            }

            self.widget
                .event(&Event::Mouse(inner_event), &mut inner_ctx, state);
        } else if self.has_mouse {
            match event {
                MouseEvent::MouseMove(event) => {
                    self.has_mouse = false;
                    let mut inner_ctx = EventCtx {
                        app: ctx.app(),
                        size: self.size,
                    };

                    self.widget.event(
                        &Event::Mouse(MouseEvent::MouseLeave(event.to_local(self.position()))),
                        &mut inner_ctx,
                        state,
                    );
                }
                MouseEvent::MouseUp(event) => {
                    self.has_mouse = false; // Is this redundant? Don't we come across MouseMove first?

                    let inner_event = event.to_local(self.position());
                    let mut inner_ctx = EventCtx {
                        app: ctx.app(),
                        size: self.size,
                    };

                    self.widget.event(
                        &Event::Mouse(MouseEvent::MouseLeave(inner_event)),
                        &mut inner_ctx,
                        state,
                    );
                }
                _ => (),
            }
        }
    }
}

impl<State: AppState> Widget<State> for ChildSlot<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) {
        match event {
            Event::Mouse(event) => self.propagate_mouse_event(event, ctx, state),
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

    fn keyboard_event(&mut self, event: &KeyboardInput, state: &mut State) -> bool {
        self.widget.keyboard_event(event, state)
    }

    fn character_received(&mut self, character: char, state: &mut State) -> bool {
        self.widget.character_received(character, state)
    }
}
