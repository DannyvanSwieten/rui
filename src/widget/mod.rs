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

use crate::app::{App, AppState};
use crate::canvas::{Canvas2D, Point, Size};
use crate::constraints::BoxConstraints;
use crate::window::MouseEvent;
pub use child_slot::ChildSlot;
use popup::PopupRequest;
use style::Theme;
use winit::event::KeyboardInput;

pub fn map_range(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    let slope = (d - c) / (b - a);
    c + slope * (x - a)
}

pub enum Action<Model> {
    None,
    Layout {
        nodes: Vec<&'static str>,
    },
    PopupRequest {
        request: PopupRequest<Model>,
        position: Point,
    },
    TriggerPopupMenu {
        menu: usize,
        sub_menu: usize,
    },
}

pub trait AppAction<Model> {
    fn undo(&self, _state: &mut Model);
    fn redo(&self, _state: &mut Model);
}

pub struct Properties {
    pub position: Point,
    pub size: Size,
}

#[allow(unused_variables)]
pub trait Widget<Model: AppState> {
    fn layout(&mut self, constraints: &BoxConstraints, model: &Model) -> Size;
    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, model: &Model);
    fn flex(&self) -> f32 {
        0.0
    }
    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        app: &mut App<Model>,
        model: &mut Model,
    ) {
    }
    fn mouse_up(&mut self, event: &MouseEvent, app: &mut App<Model>, model: &mut Model);
    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, model: &mut Model);
    fn mouse_moved(&mut self, event: &MouseEvent, model: &mut Model);
    fn mouse_entered(&mut self, event: &MouseEvent, model: &mut Model);
    fn mouse_left(&mut self, event: &MouseEvent, model: &mut Model);

    fn keyboard_event(&mut self, event: &KeyboardInput, model: &mut Model) -> bool {
        false
    }
    fn character_received(&mut self, character: char, model: &mut Model) -> bool {
        false
    }
}
