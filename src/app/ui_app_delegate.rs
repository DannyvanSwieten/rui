use crate::{
    app::{App, AppDelegate, AppRequest, AppState, WindowRequest},
    window::{UiWindowDelegate, WindowDelegate, WindowRegistry},
};
use winit::event_loop::EventLoopWindowTarget;

pub struct UIAppDelegate<State: AppState> {
    initial_window_request: Option<WindowRequest<State>>,
    on_start: Option<Box<dyn FnMut(&mut App<State>)>>,
    on_update: Option<Box<dyn FnMut(&App<State>, &State)>>,
    _state: std::marker::PhantomData<State>,
}

impl<State: AppState> UIAppDelegate<State> {
    pub fn new(window_request: WindowRequest<State>) -> Self {
        Self {
            initial_window_request: Some(window_request),
            on_start: None,
            on_update: None,
            _state: std::marker::PhantomData::default(),
        }
    }

    pub fn on_start<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut App<State>) + 'static,
    {
        self.on_start = Some(Box::new(f));
        self
    }

    pub fn on_update<F>(mut self, f: F) -> Self
    where
        F: FnMut(&App<State>, &State) + 'static,
    {
        self.on_update = Some(Box::new(f));
        self
    }
}

impl<State: AppState> AppDelegate<State> for UIAppDelegate<State> {
    fn app_will_start(
        &mut self,
        app: &mut App<State>,
        _: &State,
        _: &mut WindowRegistry<State>,
        _: &EventLoopWindowTarget<()>,
    ) {
        // self.device = Some(device);
        if let Some(request) = self.initial_window_request.take() {
            app.request(AppRequest::OpenWindow(request));
        }

        if let Some(cb) = self.on_start.as_mut() {
            cb(app)
        }
    }

    fn app_will_update(
        &mut self,
        app: &App<State>,
        state: &State,
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
        state: &State,
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
            window.id(),
            window.inner_size().width,
            window.inner_size().height,
        );
        window_registry.register_with_delegate(window, Box::new(window_delegate));
    }
}
