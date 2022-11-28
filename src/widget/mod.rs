pub mod alignment;
pub mod button;
pub mod center;
pub mod container;
pub mod expanded;
pub mod flex;
pub mod flex_box;
pub mod justification;
pub mod label;
pub mod list;
pub mod popup;
pub mod sized_box;
pub mod slider;
pub mod style;
pub mod switch;
pub mod text_editor;

mod child_slot;
mod event;
mod properties;

pub use child_slot::ChildSlot;
pub use event::{Event, KeyEvent, MouseEvent};
pub use properties::Properties;

use crate::{
    app::AppState,
    canvas::{Canvas2D, Point, Rect, Size},
    constraints::BoxConstraints,
};
use popup::PopupRequest;
use std::sync::mpsc;
use style::Theme;
use winit::window::{CursorIcon, WindowId};

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
    fn undo(&self, _state: &State);
    fn redo(&self, _state: &State);
}

#[allow(unused_variables)]
pub trait Widget<State: AppState> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State::Message>, state: &State) -> bool;

    fn layout(&mut self, constraints: &BoxConstraints, ctx: &mut LayoutCtx, state: &State) -> Size;

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, state: &State);

    fn uid(&self) -> usize {
        std::usize::MAX
    }

    fn flex(&self) -> f32 {
        0.0
    }
}

pub struct LayoutCtx {
    children: Vec<usize>,
}

impl LayoutCtx {
    pub(crate) fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn register_child(&mut self, child: usize) {
        self.children.push(child)
    }
}

pub struct EventCtx<'a, Message> {
    properties: &'a Properties,
    window_id: WindowId,
    message_tx: mpsc::Sender<Message>,
    cursor: CursorIcon,
    consumer: Option<usize>,
    target: Option<usize>,
}

impl<'a, Message> EventCtx<'a, Message> {
    pub(crate) fn new(
        properties: &'a Properties,
        window_id: WindowId,
        message_tx: mpsc::Sender<Message>,
    ) -> Self {
        Self {
            properties,
            window_id,
            message_tx,
            cursor: CursorIcon::Default,
            consumer: None,
            target: None,
        }
    }

    pub fn size(&self) -> &Size {
        &self.properties.size
    }

    pub fn publish(&self, message: Message) {
        self.message_tx.send(message).unwrap()
    }

    pub fn change_cursor(&mut self, icon: CursorIcon) {
        self.cursor = icon
    }

    pub fn cursor(&self) -> CursorIcon {
        self.cursor
    }

    pub fn set_consumer(&mut self, uid: usize) {
        self.consumer = Some(uid)
    }

    pub fn consumer(&self) -> Option<usize> {
        self.consumer
    }

    pub fn set_target(&mut self, uid: usize) {
        self.target = Some(uid)
    }

    pub fn target(&self) -> Option<usize> {
        self.target
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
