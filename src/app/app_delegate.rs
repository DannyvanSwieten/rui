use super::{App, AppState, WindowRequest};
use crate::window::{WindowId, WindowRegistry};
use std::path::Path;
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};

pub trait AppDelegate<Model: AppState> {
    fn app_will_start(
        &mut self,
        app: &mut App<Model>,
        model: &mut Model,
        window_registry: &mut WindowRegistry<Model>,
        event_loop: &EventLoopWindowTarget<()>,
    ) {
        let _ = app;
        let _ = model;
        let _ = window_registry;
        let _ = event_loop;
    }

    fn app_will_quit(&mut self, app: &mut App<Model>, event_loop: &EventLoopWindowTarget<()>) {
        let _ = app;
        let _ = event_loop;
    }

    fn app_will_update(
        &mut self,
        app: &App<Model>,
        model: &mut Model,
        window_registry: &mut WindowRegistry<Model>,
        event_loop: &EventLoopWindowTarget<()>,
    ) {
        let _ = app;
        let _ = model;
        let _ = window_registry;
        let _ = event_loop;
    }

    fn window_requested(
        &mut self,
        app: &App<Model>,
        model: &mut Model,
        window_registry: &mut WindowRegistry<Model>,
        event_loop: &EventLoopWindowTarget<()>,
        request: WindowRequest<Model>,
    ) {
        let _ = app;
        let _ = model;
        let _ = window_registry;
        let _ = event_loop;
        let _ = request;
    }

    fn window_moved(
        &mut self,
        window_id: &WindowId,
        position: &winit::dpi::PhysicalPosition<i32>,
    ) -> ControlFlow {
        let _ = window_id;
        let _ = position;

        ControlFlow::Wait
    }

    fn window_got_focus(&mut self, window_id: &WindowId) -> ControlFlow {
        let _ = window_id;

        ControlFlow::Wait
    }
    fn window_lost_focus(&mut self, window_id: &WindowId) -> ControlFlow {
        let _ = window_id;

        ControlFlow::Wait
    }

    fn window_requested_redraw(
        &mut self,
        app: &App<Model>,
        model: &Model,
        window_id: &WindowId,
    ) -> ControlFlow {
        let _ = app;
        let _ = model;
        let _ = window_id;

        ControlFlow::Wait
    }

    fn file_dropped(&mut self, window_id: &WindowId, path: &Path) -> ControlFlow {
        let _ = window_id;
        let _ = path;

        ControlFlow::Wait
    }
}
