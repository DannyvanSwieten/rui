use winit::window::WindowId;

use crate::app::{App, AppState};
use std::path::Path;

pub trait WindowDelegate<State: AppState> {
    fn close_button_pressed(&mut self, state: &mut State, window_id: WindowId) -> bool;

    fn file_hovered(&mut self, state: &mut State, window_id: WindowId, path: &Path, x: f32, y: f32);

    fn file_dropped(&mut self, state: &mut State, window_id: WindowId, path: &Path, x: f32, y: f32);

    fn mouse_moved(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        window_id: WindowId,
        x: f32,
        y: f32,
    );

    fn mouse_dragged(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        window_id: WindowId,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    );

    fn mouse_down(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        window_id: WindowId,
        x: f32,
        y: f32,
    );
    fn mouse_up(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        window_id: WindowId,
        x: f32,
        y: f32,
    );

    fn resized(
        &mut self,
        window: &winit::window::Window,
        app: &App<State>,
        state: &mut State,
        window_id: WindowId,
        width: u32,
        height: u32,
    );

    fn keyboard_event(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        window_id: WindowId,
        event: &winit::event::KeyboardInput,
    );

    fn character_received(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        window_id: WindowId,
        character: char,
    );

    fn draw(&mut self, app: &App<State>, state: &State);

    fn update(&mut self, state: &mut State);
}
