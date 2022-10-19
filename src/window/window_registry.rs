use super::{WindowDelegate, WindowId};
use crate::app::{App, AppState};
use std::{collections::HashMap, path::Path};
use winit::{
    error::OsError,
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};

pub struct WindowRegistry<State: 'static> {
    entries: HashMap<WindowId, Entry<State>>,
}

pub struct Entry<State> {
    pub window: Window,
    delegate: Box<dyn WindowDelegate<State>>,
}

impl<State: AppState> WindowRegistry<State> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut Entry<State>> {
        self.entries.get_mut(&id)
    }

    pub fn create_window(
        &self,
        target: &EventLoopWindowTarget<()>,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Window, OsError> {
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize { width, height })
            .build(target)
    }

    pub fn register_with_delegate(
        &mut self,
        window: Window,
        delegate: Box<dyn WindowDelegate<State>>,
    ) {
        self.entries.insert(window.id(), Entry { window, delegate });
    }

    pub fn active_window_count(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn update(&mut self, state: &State) {
        for entry in self.entries.values_mut() {
            entry.delegate.update(state)
        }
    }

    pub(crate) fn window_resized(
        &mut self,
        app: &App<State>,
        state: &State,
        id: &winit::window::WindowId,
        size: &winit::dpi::PhysicalSize<u32>,
    ) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry
                .delegate
                .resized(&entry.window, app, state, *id, size.width, size.height)
        }
    }

    pub(crate) fn character_received(
        &mut self,
        id: &WindowId,
        app: &mut App<State>,
        character: char,
        state: &State,
    ) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry
                .delegate
                .character_received(app, state, *id, character)
        }
    }

    pub(crate) fn keyboard_event(
        &mut self,
        id: &WindowId,
        app: &mut App<State>,
        event: &winit::event::KeyboardInput,
        state: &State,
    ) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.delegate.keyboard_event(app, state, *id, event)
        }
    }

    pub(crate) fn close_button_pressed(&mut self, id: &WindowId, state: &State) {
        if let Some(entry) = self.entries.get_mut(id) {
            if entry.delegate.close_button_pressed(state, *id) {
                self.entries.remove(id);
            }
        }
    }

    pub(crate) fn mouse_moved(
        &mut self,
        app: &mut App<State>,
        state: &State,
        id: &WindowId,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry
                .delegate
                .mouse_moved(app, state, *id, position.x as f32, position.y as f32);
        }
    }

    pub(crate) fn mouse_dragged(
        &mut self,
        app: &mut App<State>,
        state: &State,
        id: &winit::window::WindowId,
        position: &winit::dpi::PhysicalPosition<f64>,
        delta: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.delegate.mouse_dragged(
                app,
                state,
                *id,
                position.x as f32,
                position.y as f32,
                delta.x as f32,
                delta.y as f32,
            );
        }
    }

    pub(crate) fn mouse_down(
        &mut self,
        app: &mut App<State>,
        state: &State,
        id: &winit::window::WindowId,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry
                .delegate
                .mouse_down(app, state, *id, position.x as f32, position.y as f32);
        }
    }

    pub(crate) fn mouse_up(
        &mut self,
        app: &mut App<State>,
        state: &State,
        id: &winit::window::WindowId,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry
                .delegate
                .mouse_up(app, state, *id, position.x as f32, position.y as f32);
        }
    }

    pub(crate) fn window_moved(
        &self,
        _: &winit::window::WindowId,
        _: &winit::dpi::PhysicalPosition<i32>,
    ) {
    }

    pub(crate) fn draw(&mut self, app: &App<State>, state: &State) {
        for entry in self.entries.values_mut() {
            entry.delegate.draw(app, state)
        }
    }

    pub(crate) fn window_destroyed(&mut self, id: &WindowId) {
        self.entries.remove(id);
    }

    pub(crate) fn file_dropped(
        &mut self,
        id: &WindowId,
        state: &State,
        file: &Path,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry
                .delegate
                .file_dropped(state, *id, file, position.x as f32, position.y as f32)
        }
    }

    pub(crate) fn file_hovered(
        &mut self,
        id: &WindowId,
        state: &State,
        file: &Path,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry
                .delegate
                .file_hovered(state, *id, file, position.x as f32, position.y as f32)
        }
    }
}

impl<State: AppState> Default for WindowRegistry<State> {
    fn default() -> Self {
        Self::new()
    }
}
