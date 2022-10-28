use crate::{
    app::{App, AppRequest, AppState, CursorIconRequest},
    canvas::{Canvas2D, Point, Size},
    constraints::BoxConstraints,
    widget::{
        style::StyleContext, Action, ChildSlot, Event, EventCtx, KeyEvent, LayoutCtx, MouseEvent,
        PaintCtx, Properties, Widget,
    },
    window,
};
use std::path::Path;
use winit::{event::KeyboardInput, window::WindowId};

pub struct WindowContext {
    _origin: Point,
    _size: Size,
    _id: WindowId,
}

pub struct DragContext<State> {
    _dragged_widgets: Vec<Box<dyn Widget<State>>>,
}

pub struct UserInterface<State: AppState> {
    pub root: ChildSlot<State>,
    pub style_ctx: StyleContext,
    _actions: Vec<Action<State>>,
    theme: String,
    mouse_move_consumer: Option<usize>,
}

impl<State: AppState + 'static> UserInterface<State> {
    pub fn new(root: Box<dyn Widget<State>>, theme: &str) -> Self {
        UserInterface {
            root: ChildSlot::new_with_box(root),
            style_ctx: StyleContext::new(),
            _actions: Vec::new(),
            theme: theme.to_string(),
            mouse_move_consumer: None,
        }
    }

    pub fn file_dropped(&self, _: &State, _: WindowId, _: &Path, _: &Point) {}
    pub fn file_hovered(&self, _: &State, _: WindowId, _: &Path, _: &Point) {}

    pub fn resize(&mut self, state: &State, width: u32, height: u32) {
        let constraints = BoxConstraints::new().with_tight_constraints(width as f32, height as f32);
        self.layout(&constraints, state);
    }

    pub fn resized(&self, _: &State, _: WindowId) {}

    pub fn mouse_down(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        event: &window::MouseEvent,
    ) {
        let properties = Properties {
            size: *self.root.size(),
            ..Properties::default()
        };
        let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
        self.root.event(
            &Event::Mouse(MouseEvent::MouseDown(*event)),
            &mut ctx,
            state,
        );
    }

    pub fn mouse_up(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        event: &window::MouseEvent,
    ) {
        let properties = Properties {
            size: *self.root.size(),
            ..Properties::default()
        };
        let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
        self.root
            .event(&Event::Mouse(MouseEvent::MouseUp(*event)), &mut ctx, state);
    }

    pub fn double_click(&self, _: &State, _: &MouseEvent) {}

    pub fn mouse_drag(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        event: &window::MouseEvent,
    ) {
        let properties = Properties {
            size: *self.root.size(),
            ..Properties::default()
        };
        let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
        self.root.event(
            &Event::Mouse(MouseEvent::MouseDrag(*event)),
            &mut ctx,
            state,
        );
    }

    pub fn mouse_moved(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        event: &window::MouseEvent,
    ) {
        let properties = Properties {
            size: *self.root.size(),
            ..Properties::default()
        };
        let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
        self.root.event(
            &Event::Mouse(MouseEvent::MouseMove(*event)),
            &mut ctx,
            state,
        );

        if let Some(consumer) = ctx.consumer() {
            if self.mouse_move_consumer.is_none() {
                let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
                ctx.set_target(consumer);
                self.root.event(
                    &Event::Mouse(MouseEvent::MouseEnter(*event)),
                    &mut ctx,
                    state,
                );
            } else {
                let previous_consumer = self.mouse_move_consumer.unwrap();
                if consumer != previous_consumer {
                    let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
                    ctx.set_target(consumer);
                    self.root.event(
                        &Event::Mouse(MouseEvent::MouseEnter(*event)),
                        &mut ctx,
                        state,
                    );

                    let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
                    ctx.set_target(previous_consumer);
                    self.root.event(
                        &Event::Mouse(MouseEvent::MouseLeave(*event)),
                        &mut ctx,
                        state,
                    );
                }
            }
        } else if let Some(previous_consumer) = self.mouse_move_consumer {
            let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
            ctx.set_target(previous_consumer);
            self.root.event(
                &Event::Mouse(MouseEvent::MouseLeave(*event)),
                &mut ctx,
                state,
            );
        }

        self.mouse_move_consumer = ctx.consumer();

        app.request(AppRequest::ChangeCursorRequest(CursorIconRequest::new(
            window_id,
            ctx.cursor(),
        )))
    }

    pub fn mouse_leave(&self, _: &State, _: &window::MouseEvent) {}

    pub fn keyboard_event(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        event: &KeyboardInput,
    ) {
        let properties = Properties {
            size: *self.root.size(),
            ..Properties::default()
        };
        let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
        self.root
            .event(&Event::Key(KeyEvent::Input(*event)), &mut ctx, state);
    }

    pub fn character_received(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        character: char,
    ) {
        let properties = Properties {
            size: *self.root.size(),
            ..Properties::default()
        };
        let mut ctx = EventCtx::new(&properties, window_id, app.message_tx.clone());
        self.root
            .event(&Event::Key(KeyEvent::Char(character)), &mut ctx, state);
    }

    pub fn layout(&mut self, constraints: &BoxConstraints, state: &State) {
        let mut ctx = LayoutCtx::new();
        let size = self.root.layout(constraints, &mut ctx, state);
        self.root.set_size(&size);
    }

    pub fn paint(&self, state: &State, canvas: &mut dyn Canvas2D) {
        canvas.clear(&self.style_ctx.theme(&self.theme).unwrap().background);

        let properties = Properties {
            size: *self.root.size(),
            ..Properties::default()
        };

        self.root.paint(
            self.style_ctx.theme(&self.theme).unwrap(),
            &PaintCtx::new(&properties),
            canvas,
            state,
        );
    }
}
