use crate::window::{UiWindowDelegate, WindowDelegate};
use crate::{
    app::{App, AppDelegate, AppState, WindowRequest},
    window::WindowRegistry,
};
use winit::event_loop::EventLoopWindowTarget;

pub struct UIAppDelegate<State: AppState> {
    on_start: Option<Box<dyn FnMut(&mut App<State>, &mut State)>>,
    on_update: Option<Box<dyn FnMut(&App<State>, &mut State)>>,
    _state: std::marker::PhantomData<State>,
}

impl<State: AppState> UIAppDelegate<State> {
    pub fn new() -> Self {
        Self {
            on_start: None,
            on_update: None,
            _state: std::marker::PhantomData::default(),
        }
    }

    pub fn on_start<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut App<State>, &mut State) + 'static,
    {
        self.on_start = Some(Box::new(f));
        self
    }

    pub fn on_update<F>(mut self, f: F) -> Self
    where
        F: FnMut(&App<State>, &mut State) + 'static,
    {
        self.on_update = Some(Box::new(f));
        self
    }
}

impl<State: AppState> AppDelegate<State> for UIAppDelegate<State> {
    fn app_will_start(
        &mut self,
        app: &mut App<State>,
        state: &mut State,
        _: &mut WindowRegistry<State>,
        _: &EventLoopWindowTarget<()>,
    ) {
        // self.device = Some(device);
        if let Some(cb) = self.on_start.as_mut() {
            cb(app, state)
        }
    }

    fn app_will_update(
        &mut self,
        app: &App<State>,
        state: &mut State,
        _: &mut WindowRegistry<State>,
        _: &EventLoopWindowTarget<()>,
    ) {
        if let Some(cb) = self.on_update.as_mut() {
            cb(app, state)
        }
    }

    fn window_requested(
        &mut self,
        app: &App<State>,
        state: &mut State,
        window_registry: &mut WindowRegistry<State>,
        target: &EventLoopWindowTarget<()>,
        request: WindowRequest<State>,
    ) {
        let window = window_registry
            .create_window(
                target,
                &request.title.unwrap_or_else(|| "Untitled".to_string()),
                request.width,
                request.height,
            )
            .expect("Window creation failed");

        let mut window_delegate = UiWindowDelegate::new(
            app.gpu_api().device.clone(),
            app.gpu_api().queue.clone(),
            request.builder,
        );
        window_delegate.resized(
            &window,
            app,
            state,
            window.inner_size().width,
            window.inner_size().height,
        );
        window_registry.register_with_delegate(window, Box::new(window_delegate));
    }
}

impl<State: AppState> Default for UIAppDelegate<State> {
    fn default() -> Self {
        Self::new()
    }
}
