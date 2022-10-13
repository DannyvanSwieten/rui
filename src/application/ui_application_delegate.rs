use crate::window::{UIGpuDrawingWindowDelegate, WindowDelegate};
use crate::{
    application::{Application, ApplicationDelegate, ApplicationModel, WindowRequest},
    window::WindowRegistry,
};
use std::rc::Rc;
use winit::event_loop::EventLoopWindowTarget;

pub struct UIApplicationDelegate<Model: ApplicationModel> {
    on_start: Option<Box<dyn FnMut(&mut Application<Model>, &mut Model)>>,
    on_update: Option<Box<dyn FnMut(&Application<Model>, &mut Model)>>,
    _state: std::marker::PhantomData<Model>,
}

impl<Model: ApplicationModel> UIApplicationDelegate<Model> {
    pub fn new() -> Self {
        Self {
            on_start: None,
            on_update: None,
            _state: std::marker::PhantomData::default(),
        }
    }

    pub fn on_start<F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut Application<Model>, &mut Model) + 'static,
    {
        self.on_start = Some(Box::new(f));
        self
    }

    pub fn on_update<F>(mut self, f: F) -> Self
    where
        F: FnMut(&Application<Model>, &mut Model) + 'static,
    {
        self.on_update = Some(Box::new(f));
        self
    }
}

impl<Model: ApplicationModel> ApplicationDelegate<Model> for UIApplicationDelegate<Model> {
    fn application_will_start(
        &mut self,
        app: &mut Application<Model>,
        state: &mut Model,
        _: &mut WindowRegistry<Model>,
        _: &EventLoopWindowTarget<()>,
    ) {
        // self.device = Some(device);
        if let Some(cb) = self.on_start.as_mut() {
            cb(app, state)
        }
    }

    fn application_will_update(
        &mut self,
        app: &Application<Model>,
        state: &mut Model,
        _: &mut WindowRegistry<Model>,
        _: &EventLoopWindowTarget<()>,
    ) {
        if let Some(cb) = self.on_update.as_mut() {
            cb(app, state)
        }
    }

    fn window_requested(
        &mut self,
        app: &Application<Model>,
        state: &mut Model,
        window_registry: &mut WindowRegistry<Model>,
        target: &EventLoopWindowTarget<()>,
        request: WindowRequest<Model>,
    ) {
        let window = window_registry
            .create_window(
                target,
                &request.title.unwrap_or_else(|| "Untitled".to_string()),
                request.width,
                request.height,
            )
            .expect("Window creation failed");

        let mut window_delegate = UIGpuDrawingWindowDelegate::new(
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
