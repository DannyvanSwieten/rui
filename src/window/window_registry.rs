use super::{WindowDelegate, WindowId};
use crate::application::{Application, ApplicationModel};
use std::{collections::HashMap, path::Path};
use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};

pub struct WindowRegistry<Model: 'static> {
    windows: HashMap<WindowId, Window>,
    window_delegates: HashMap<WindowId, Box<dyn WindowDelegate<Model>>>,
}

impl<Model: ApplicationModel> WindowRegistry<Model> {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            window_delegates: HashMap::new(),
        }
    }

    pub fn create_window(
        &self,
        target: &EventLoopWindowTarget<()>,
        title: &str,
        width: u32,
        height: u32,
    ) -> Window {
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize { width, height })
            .build(target)
            .unwrap()
    }

    pub fn register_with_delegate(
        &mut self,
        window: Window,
        delegate: Box<dyn WindowDelegate<Model>>,
    ) {
        self.window_delegates.insert(window.id(), delegate);
        self.windows.insert(window.id(), window);
    }

    pub fn register(&mut self, window: Window) {
        self.windows.insert(window.id(), window);
    }

    pub fn active_window_count(&self) -> usize {
        self.windows.len()
    }

    pub(crate) fn update(&mut self, state: &mut Model) {
        for (_, delegate) in self.window_delegates.iter_mut() {
            delegate.update(state)
        }
    }

    pub(crate) fn window_resized(
        &mut self,
        app: &Application<Model>,
        state: &mut Model,
        id: &winit::window::WindowId,
        size: &winit::dpi::PhysicalSize<u32>,
    ) {
        if let Some(window) = self.window_delegates.get_mut(id) {
            window.resized(
                self.windows.get(id).unwrap(),
                app,
                state,
                size.width,
                size.height,
            )
        }
    }

    pub(crate) fn character_received(&mut self, id: &WindowId, character: char, state: &mut Model) {
        if let Some(delegate) = self.window_delegates.get_mut(id) {
            delegate.character_received(state, character)
        }
    }

    pub(crate) fn keyboard_event(
        &mut self,
        id: &WindowId,
        event: &winit::event::KeyboardInput,
        state: &mut Model,
    ) {
        if let Some(delegate) = self.window_delegates.get_mut(id) {
            delegate.keyboard_event(state, event)
        }
    }

    pub(crate) fn close_button_pressed(&mut self, id: &WindowId, state: &mut Model) {
        if let Some(delegate) = self.window_delegates.get_mut(id) {
            if delegate.close_button_pressed(state) {
                self.windows.remove(id);
            }
        }
    }

    pub(crate) fn mouse_moved(
        &mut self,
        state: &mut Model,
        id: &WindowId,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(delegate) = self.window_delegates.get_mut(id) {
            delegate.mouse_moved(state, position.x as f32, position.y as f32);
        }
    }

    pub(crate) fn mouse_dragged(
        &mut self,
        state: &mut Model,
        id: &winit::window::WindowId,
        position: &winit::dpi::PhysicalPosition<f64>,
        delta: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(delegate) = self.window_delegates.get_mut(id) {
            delegate.mouse_dragged(
                state,
                position.x as f32,
                position.y as f32,
                delta.x as f32,
                delta.y as f32,
            );
        }
    }

    pub(crate) fn mouse_down(
        &mut self,
        app: &mut Application<Model>,
        state: &mut Model,
        id: &winit::window::WindowId,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(delegate) = self.window_delegates.get_mut(id) {
            delegate.mouse_down(app, state, position.x as f32, position.y as f32);
        }
    }

    pub(crate) fn mouse_up(
        &mut self,
        app: &mut Application<Model>,
        state: &mut Model,
        id: &winit::window::WindowId,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(delegate) = self.window_delegates.get_mut(id) {
            delegate.mouse_up(app, state, position.x as f32, position.y as f32);
        }
    }

    pub(crate) fn window_moved(
        &mut self,
        _: &winit::window::WindowId,
        _: &winit::dpi::PhysicalPosition<i32>,
    ) {
    }

    pub(crate) fn draw(&mut self, app: &Application<Model>, state: &mut Model) {
        for (_, delegate) in self.window_delegates.iter_mut() {
            delegate.draw(app, state)
        }
    }

    pub(crate) fn window_destroyed(&mut self, id: &WindowId) {
        self.window_delegates.remove(id);
    }

    pub(crate) fn file_dropped(
        &mut self,
        id: &WindowId,
        state: &mut Model,
        file: &Path,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(delegate) = self.window_delegates.get_mut(id) {
            delegate.file_dropped(state, file, position.x as f32, position.y as f32)
        }
    }

    pub(crate) fn file_hovered(
        &mut self,
        id: &WindowId,
        state: &mut Model,
        file: &Path,
        position: &winit::dpi::PhysicalPosition<f64>,
    ) {
        if let Some(delegate) = self.window_delegates.get_mut(id) {
            delegate.file_hovered(state, file, position.x as f32, position.y as f32)
        }
    }
}
