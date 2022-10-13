use crate::app::{App, AppState};
use crate::canvas::{Canvas2D, Point, Size};
use crate::constraints::BoxConstraints;
use crate::widget::{style::StyleContext, EventCtx, *};
use crate::window::MouseEvent;
use std::path::Path;
use winit::{event::KeyboardInput, window::WindowId};

pub struct WindowContext {
    origin: Point,
    size: Size,
    id: WindowId,
}

pub struct DragContext<State> {
    dragged_widgets: Vec<Box<dyn Widget<State>>>,
}

pub struct UserInterface<State: AppState> {
    pub root: ChildSlot<State>,
    pub style_ctx: StyleContext,
    actions: Vec<Action<State>>,
    theme: String,
}

impl<State: AppState + 'static> UserInterface<State> {
    pub fn new(root: Box<dyn Widget<State>>, theme: &str) -> Self {
        UserInterface {
            root: ChildSlot::new_with_box(root),
            style_ctx: StyleContext::new(),
            actions: Vec::new(),
            theme: theme.to_string(),
        }
    }

    pub fn file_dropped(&mut self, state: &mut State, path: &Path, position: &Point) {
        // self.actions
        //     .push(self.root.file_dropped(state, path, position))
    }

    pub fn file_hovered(&mut self, state: &mut State, path: &Path, position: &Point) {}

    fn build_popup(&mut self, request: popup::PopupRequest<State>, position: &Point) {}

    pub fn resize(&mut self, state: &State, width: u32, height: u32) {
        let constraints = BoxConstraints::new().with_tight_constraints(width as f32, height as f32);
        self.layout(&constraints, state);
    }

    pub fn resized(&mut self, state: &mut State) {}

    pub fn mouse_down(&mut self, app: &mut App<State>, state: &mut State, event: &MouseEvent) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root.event(&Event::MouseDown(*event), &mut ctx, state)
    }

    pub fn mouse_up(&mut self, app: &mut App<State>, state: &mut State, event: &MouseEvent) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root.event(&Event::MouseUp(*event), &mut ctx, state)
    }

    pub fn double_click(&mut self, state: &mut State, event: &MouseEvent) {}

    pub fn mouse_drag(&mut self, app: &mut App<State>, state: &mut State, event: &MouseEvent) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root.event(&Event::MouseDrag(*event), &mut ctx, state)
    }

    pub fn mouse_moved(&mut self, app: &mut App<State>, state: &mut State, event: &MouseEvent) {
        let mut ctx = EventCtx::new(app, *self.root.size());
        self.root.event(&Event::MouseMove(*event), &mut ctx, state)
    }

    pub fn mouse_leave(&mut self, state: &mut State, event: &MouseEvent) {}
    pub fn keyboard_event(&mut self, state: &mut State, event: &KeyboardInput) {
        self.root.keyboard_event(event, state);
    }
    pub fn character_received(&mut self, state: &mut State, character: char) {
        self.root.character_received(character, state);
    }
    pub fn layout(&mut self, constraints: &BoxConstraints, state: &State) {
        let size = self.root.layout(constraints, state);
        self.root.set_size(&size);
    }

    pub fn layout_child_with_name(&mut self, child_name: &str, state: &State) {
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
