pub mod button;
pub mod center;
pub mod container;
pub mod flex;
pub mod flex_box;
pub mod list;
pub mod popup;
pub mod sized_box;
pub mod slider;
pub mod style;
pub mod text_editor;

mod child_slot;
mod event;
mod properties;

pub use child_slot::ChildSlot;
pub use event::{Event, KeyEvent, MouseEvent};
pub use properties::Properties;

use crate::{
    app::{App, AppState, WindowRequest},
    canvas::{Canvas2D, Point, Rect, Size},
    constraints::BoxConstraints,
};
use popup::PopupRequest;
use style::Theme;

pub fn map_range(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    let slope = (d - c) / (b - a);
    c + slope * (x - a)
}

pub enum Action<State> {
    None,
    Layout {
        nodes: Vec<&'static str>,
    },
    PopupRequest {
        request: PopupRequest<State>,
        position: Point,
    },
    TriggerPopupMenu {
        menu: usize,
        sub_menu: usize,
    },
}

pub trait AppAction<State> {
    fn undo(&self, _state: &mut State);
    fn redo(&self, _state: &mut State);
}

#[allow(unused_variables)]
pub trait Widget<State: AppState> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State>, state: &mut State) -> bool;

    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size;

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, state: &State);

    fn flex(&self) -> f32 {
        0.0
    }
}

pub struct EventCtx<'a, State: AppState> {
    app: &'a mut App<State>,
    properties: &'a Properties,
}

impl<'a, State: AppState + 'static> EventCtx<'a, State> {
    pub(crate) fn new(app: &'a mut App<State>, properties: &'a Properties) -> Self {
        Self { app, properties }
    }

    pub fn size(&self) -> &Size {
        &self.properties.size
    }

    pub fn ui_window_request(&mut self, request: WindowRequest<State>) {
        self.app.ui_window_request(request)
    }
}

pub struct PaintCtx<'a> {
    properties: &'a Properties,
}

impl<'a> PaintCtx<'a> {
    pub(crate) fn new(properties: &'a Properties) -> Self {
        Self { properties }
    }

    pub fn size(&self) -> &Size {
        &self.properties.size
    }

    pub fn rect(&self) -> Rect {
        Rect::from_size(self.properties.size)
    }
}
