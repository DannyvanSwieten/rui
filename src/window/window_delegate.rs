use crate::app::{App, AppState};
use std::path::Path;

pub trait WindowDelegate<State: AppState> {
    fn close_button_pressed(&mut self, state: &mut State) -> bool {
        let _ = state;

        true
    }

    fn file_hovered(&mut self, state: &mut State, path: &Path, x: f32, y: f32) {
        let _ = state;
        let _ = path;
        let _ = x;
        let _ = y;
    }

    fn file_dropped(&mut self, state: &mut State, path: &Path, x: f32, y: f32) {
        let _ = state;
        let _ = path;
        let _ = x;
        let _ = y;
    }

    fn mouse_moved(&mut self, app: &mut App<State>, state: &mut State, x: f32, y: f32) {
        let _ = app;
        let _ = state;
        let _ = x;
        let _ = y;
    }

    fn mouse_dragged(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) {
        let _ = app;
        let _ = state;
        let _ = x;
        let _ = y;
        let _ = dx;
        let _ = dy;
    }

    fn mouse_down(&mut self, app: &mut App<State>, state: &mut State, x: f32, y: f32) {
        let _ = app;
        let _ = state;
        let _ = x;
        let _ = y;
    }

    fn mouse_up(&mut self, app: &mut App<State>, state: &mut State, x: f32, y: f32) {
        let _ = app;
        let _ = state;
        let _ = x;
        let _ = y;
    }

    fn resized(
        &mut self,
        window: &winit::window::Window,
        app: &App<State>,
        state: &mut State,
        width: u32,
        height: u32,
    ) {
        let _ = window;
        let _ = app;
        let _ = state;
        let _ = width;
        let _ = height;
    }

    fn keyboard_event(&mut self, state: &mut State, event: &winit::event::KeyboardInput) {
        let _ = state;
        let _ = event;
    }

    fn character_received(&mut self, state: &mut State, character: char) {
        let _ = state;
        let _ = character;
    }

    fn draw(&mut self, app: &App<State>, state: &State) {
        let _ = app;
        let _ = state;
    }

    fn update(&mut self, state: &mut State) {
        let _ = state;
    }
}
