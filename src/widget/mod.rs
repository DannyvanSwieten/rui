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

pub struct Properties {
    pub position: Point,
    pub size: Size,
}

#[allow(unused_variables)]
pub trait Widget<State: AppState> {
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size;
    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, state: &State);
    fn flex(&self) -> f32 {
        0.0
    }
    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        app: &mut App<State>,
        state: &mut State,
    ) {
    }
    fn mouse_up(&mut self, event: &MouseEvent, app: &mut App<State>, state: &mut State);
    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, state: &mut State);
    fn mouse_moved(&mut self, event: &MouseEvent, state: &mut State);
    fn mouse_entered(&mut self, event: &MouseEvent, state: &mut State);
    fn mouse_left(&mut self, event: &MouseEvent, state: &mut State);

    fn keyboard_event(&mut self, event: &KeyboardInput, state: &mut State) -> bool {
        false
    }
    fn character_received(&mut self, character: char, state: &mut State) -> bool {
        false
    }
}
