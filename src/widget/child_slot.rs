use std::{sync::atomic::AtomicUsize, sync::atomic::Ordering};

use crate::{
    app::AppState,
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{Event, EventCtx, MouseEvent, PaintCtx, Properties, Theme, Widget},
};

use super::LayoutCtx;

fn next_uid() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub struct ChildSlot<State> {
    uid: usize,
    widget: Box<dyn Widget<State>>,
    properties: Properties,
}

impl<State: AppState> ChildSlot<State> {
    pub fn new(widget: impl Widget<State> + 'static) -> Self {
        Self {
            uid: next_uid(),
            widget: Box::new(widget),
            properties: Properties::default(),
        }
    }

    pub fn new_with_box(widget: Box<dyn Widget<State>>) -> Self {
        Self {
            uid: next_uid(),
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
        ctx: &mut EventCtx<State::Message>,
        state: &State,
    ) {
        let inner_event = event.to_local(self.position());
        let mut inner_ctx = EventCtx {
            properties: &self.properties,
            window_id: ctx.window_id,
            message_tx: ctx.message_tx.clone(),
            cursor: ctx.cursor,
            consumer: ctx.consumer,
            target: ctx.target,
        };

        if let Some(target) = ctx.target() {
            if target == self.uid() || !self.properties.children.is_empty() {
                self.widget
                    .event(&Event::Mouse(inner_event), &mut inner_ctx, state);
            }
        }

        if self.hit_test(event.local_position()) {
            let inner_event = event.to_local(self.position());
            if self
                .widget
                .event(&Event::Mouse(inner_event), &mut inner_ctx, state)
            {
                ctx.set_consumer(self.uid())
            }

            ctx.change_cursor(inner_ctx.cursor());
            if let Some(uid) = inner_ctx.consumer() {
                ctx.set_consumer(uid)
            }
        }
    }
}

impl<State: AppState> Widget<State> for ChildSlot<State> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State::Message>, state: &State) -> bool {
        match event {
            Event::Mouse(event) => self.propagate_mouse_event(event, ctx, state),
            Event::Key(_) => {
                if self.widget.event(event, ctx, state) {
                    ctx.set_consumer(self.uid())
                }
            }
        }

        false
    }

    fn layout(
        &mut self,
        constraints: &BoxConstraints,
        _ctx: &mut LayoutCtx,
        state: &State,
    ) -> Size {
        let mut inner_ctx = LayoutCtx::new();
        let size = self.widget.layout(constraints, &mut inner_ctx, state);
        for child in &inner_ctx.children {
            self.properties.children.push(*child)
        }

        size
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

    fn uid(&self) -> usize {
        self.uid
    }
}
