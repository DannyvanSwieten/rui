mod app_delegate;
mod app_state;
mod ui_app_delegate;

pub use app_delegate::AppDelegate;
pub use app_state::AppState;
pub use ui_app_delegate::UIAppDelegate;

use crate::{widget::Widget, window::WindowRegistry};
use std::{collections::VecDeque, rc::Rc};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct WindowRequest<State: AppState> {
    pub builder: Box<dyn Fn(&State) -> Box<dyn Widget<State>>>,
    pub title: Option<String>,
    pub width: u32,
    pub height: u32,
}

impl<State: AppState> WindowRequest<State> {
    pub fn new(
        title: &str,
        width: u32,
        height: u32,
        builder: impl Fn(&State) -> Box<dyn Widget<State>> + 'static,
    ) -> Self {
        Self {
            title: Some(title.to_string()),
            width,
            height,
            builder: Box::new(builder),
        }
    }
}

pub struct GpuApi {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: Rc<wgpu::Device>,
    pub queue: Rc<wgpu::Queue>,
}

impl GpuApi {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::METAL | wgpu::Backends::DX12);
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        Self {
            instance,
            adapter,
            device: Rc::new(device),
            queue: Rc::new(queue),
        }
    }
}

pub struct App<State: AppState> {
    gpu_api: GpuApi,
    pending_messages: VecDeque<State::MessageType>,
    pending_window_requests: VecDeque<WindowRequest<State>>,
    _state: std::marker::PhantomData<State>,
}

impl<State: AppState + 'static> App<State> {
    pub async fn new(name: &str) -> Self {
        let gpu_api = GpuApi::new().await;
        Self {
            pending_messages: VecDeque::new(),
            pending_window_requests: VecDeque::new(),
            _state: std::marker::PhantomData::<State>::default(),
            gpu_api,
        }
    }

    pub fn gpu_api(&self) -> &GpuApi {
        &self.gpu_api
    }

    pub fn send_message(&mut self, msg: State::MessageType) {
        self.pending_messages.push_back(msg)
    }

    fn pop_message(&mut self) -> Option<State::MessageType> {
        self.pending_messages.pop_front()
    }

    pub fn ui_window_request(&mut self, request: WindowRequest<State>) {
        self.pending_window_requests.push_back(request)
    }

    pub fn run<Delegate>(mut self, delegate: Delegate, state: State)
    where
        Delegate: AppDelegate<State> + 'static,
    {
        let mut s = state;
        let event_loop = EventLoop::new();
        let mut d = delegate;

        let mut window_registry = WindowRegistry::new();

        d.app_will_start(&mut self, &mut s, &mut window_registry, &event_loop);
        let mut last_mouse_position = winit::dpi::PhysicalPosition::<f64>::new(0., 0.);
        let mut last_file_drop: Vec<std::path::PathBuf> = Vec::new();
        let mut mouse_is_down = false;
        event_loop.run(move |e, event_loop, control_flow| {
            while let Some(msg) = self.pop_message() {
                s.handle_message(msg)
            }

            while let Some(request) = self.pending_window_requests.pop_front() {
                d.window_requested(&self, &mut s, &mut window_registry, event_loop, request)
            }
            *control_flow = winit::event_loop::ControlFlow::Poll;
            match e {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    window_registry.close_button_pressed(&window_id, &mut s);
                    if window_registry.active_window_count() == 0 {
                        *control_flow = ControlFlow::Exit;
                    }
                }

                Event::WindowEvent {
                    event: WindowEvent::Destroyed,
                    window_id,
                } => window_registry.window_destroyed(&window_id),

                Event::WindowEvent {
                    event: WindowEvent::Moved(physical_position),
                    window_id,
                } => window_registry.window_moved(&window_id, &physical_position),

                Event::WindowEvent {
                    event: WindowEvent::Resized(physical_size),
                    window_id,
                } => window_registry.window_resized(&self, &mut s, &window_id, &physical_size),

                Event::WindowEvent {
                    event: WindowEvent::DroppedFile(path_buffer),
                    ..
                } => last_file_drop.push(path_buffer),
                Event::WindowEvent {
                    event: WindowEvent::HoveredFile(path_buffer),
                    window_id,
                } => window_registry.file_hovered(
                    &window_id,
                    &mut s,
                    &path_buffer,
                    &last_mouse_position,
                ),
                Event::WindowEvent {
                    event: WindowEvent::Focused(f),
                    window_id,
                } => {
                    *control_flow = if f {
                        d.window_got_focus(&window_id)
                    } else {
                        d.window_lost_focus(&window_id)
                    }
                }

                Event::RedrawRequested(_window_id) => {
                    //window_registry.window_requested_redraw(&self, &s, &window_id)
                }

                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    window_id,
                } => {
                    if mouse_is_down {
                        let delta = winit::dpi::PhysicalPosition::<f64>::new(
                            position.x - last_mouse_position.x,
                            position.y - last_mouse_position.y,
                        );
                        window_registry
                            .mouse_dragged(&mut self, &mut s, &window_id, &position, &delta)
                    } else {
                        window_registry.mouse_moved(&mut self, &mut s, &window_id, &position)
                    }

                    if !last_file_drop.is_empty() {
                        window_registry.file_dropped(
                            &window_id,
                            &mut s,
                            &last_file_drop[0],
                            &position,
                        );

                        last_file_drop.clear();
                    }

                    last_mouse_position = position;
                }

                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::ReceivedCharacter(character),
                } => window_registry.character_received(&window_id, character, &mut s),

                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::KeyboardInput { input, .. },
                } => window_registry.keyboard_event(&window_id, &input, &mut s),

                Event::WindowEvent {
                    event: WindowEvent::MouseInput { state, .. },
                    window_id,
                } => match state {
                    winit::event::ElementState::Pressed => {
                        mouse_is_down = true;
                        window_registry.mouse_down(
                            &mut self,
                            &mut s,
                            &window_id,
                            &last_mouse_position,
                        )
                    }
                    winit::event::ElementState::Released => {
                        mouse_is_down = false;
                        window_registry.mouse_up(
                            &mut self,
                            &mut s,
                            &window_id,
                            &last_mouse_position,
                        )
                    }
                },
                Event::MainEventsCleared => {
                    d.app_will_update(&self, &mut s, &mut window_registry, event_loop);
                    window_registry.update(&mut s);
                    window_registry.draw(&self, &mut s);
                }
                _ => (),
            }

            if let ControlFlow::Exit = *control_flow {
                d.app_will_quit(&mut self, event_loop)
            }
        });
    }
}
