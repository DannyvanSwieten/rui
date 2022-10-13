use crate::app::{App, AppState};
use crate::canvas::{Canvas2D, Point, Size};
use crate::constraints::BoxConstraints;
use crate::widget::{
    style::StyleContext, Action, ChildSlot, Event, EventCtx, KeyEvent, MouseEvent, Widget,
};
use crate::window;
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
}

impl<State: AppState + 'static> UserInterface<State> {
    pub fn new(root: Box<dyn Widget<State>>, theme: &str) -> Self {
        UserInterface {
            root: ChildSlot::new_with_box(root),
            style_ctx: StyleContext::new(),
            _actions: Vec::new(),
            theme: theme.to_string(),
        }
    }

    pub fn file_dropped(&mut self, _: &mut State, _: &Path, _: &Point) {}
    pub fn file_hovered(&mut self, _: &mut State, _: &Path, _: &Point) {}

    pub fn resize(&mut self, state: &State, width: u32, height: u32) {
        let constraints = BoxConstraints::new().with_tight_constraints(width as f32, height as f32);
        self.layout(&constraints, state);
    }

    pub fn resized(&mut self, _: &mut State) {}

    pub fn mouse_down(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        event: &window::MouseEvent,
    ) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root.event(
            &Event::Mouse(MouseEvent::MouseDown(*event)),
            &mut ctx,
            state,
        );
    }

    pub fn mouse_up(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        event: &window::MouseEvent,
    ) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root
            .event(&Event::Mouse(MouseEvent::MouseUp(*event)), &mut ctx, state);
    }

    pub fn double_click(&mut self, _: &mut State, _: &MouseEvent) {}

    pub fn mouse_drag(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        event: &window::MouseEvent,
    ) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root.event(
            &Event::Mouse(MouseEvent::MouseDrag(*event)),
            &mut ctx,
            state,
        );
    }

    pub fn mouse_moved(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        event: &window::MouseEvent,
    ) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root.event(
            &Event::Mouse(MouseEvent::MouseMove(*event)),
            &mut ctx,
            state,
        );
    }

    pub fn mouse_leave(&mut self, _: &mut State, _: &window::MouseEvent) {}

    pub fn keyboard_event(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        event: &KeyboardInput,
    ) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root
            .event(&Event::Key(KeyEvent::Input(*event)), &mut ctx, state);
    }

    pub fn character_received(&mut self, app: &mut App<State>, state: &mut State, character: char) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root
            .event(&Event::Key(KeyEvent::Char(character)), &mut ctx, state);
    }

    pub fn layout(&mut self, constraints: &BoxConstraints, state: &State) {
        let size = self.root.layout(constraints, state);
        self.root.set_size(&size);
    }

    pub fn layout_child_with_name(&mut self, _: &str, _: &State) {
        // self.root.layout_child_with_name(child_name, state)
    }

    pub fn paint(&mut self, state: &State, canvas: &mut dyn Canvas2D) {
        canvas.clear(&self.style_ctx.theme(&self.theme).unwrap().background);
        self.root.paint(
            self.style_ctx.theme(&self.theme).unwrap(),
            canvas,
            self.root.size(),
            state,
        );
    }
}
