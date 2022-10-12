mod application_delegate;
mod application_model;

pub use application_delegate::ApplicationDelegate;
pub use application_model::ApplicationModel;

use crate::{
    widget::Widget,
    window::{WindowId, WindowRegistry},
};
use std::collections::VecDeque;
use vk_utils::vulkan::Vulkan;

use std::ffi::{CStr, CString};
use std::path::Path;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

use ash::extensions::{ext::DebugUtils, khr::Surface};

#[cfg(target_os = "macos")]
use ash::extensions::mvk::MacOSSurface;

#[cfg(target_os = "macos")]
use ash::vk::ExtMetalSurfaceFn;

#[cfg(target_os = "macos")]
pub fn surface_extension_name() -> &'static CStr {
    ExtMetalSurfaceFn::name()
}

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;

#[cfg(target_os = "windows")]
pub fn surface_extension_name() -> &'static CStr {
    Win32Surface::name()
}

pub struct WindowRequest<Model: ApplicationModel> {
    pub builder: Box<dyn Fn(&Model) -> Box<dyn Widget<Model>>>,
    pub title: Option<String>,
    pub width: u32,
    pub height: u32,
}

pub struct Application<Model: ApplicationModel> {
    vulkan: Vulkan,
    pending_messages: VecDeque<Model::MessageType>,
    pending_window_requests: VecDeque<WindowRequest<Model>>,
    _state: std::marker::PhantomData<Model>,
}

impl<Model: ApplicationModel + 'static> Application<Model> {
    pub fn new(name: &str) -> Self {
        let layers = [CString::new("VK_LAYER_KHRONOS_validation").expect("String Creation Failed")];
        let instance_extensions = [
            Surface::name(),
            surface_extension_name(),
            DebugUtils::name(),
        ];
        let vulkan = Vulkan::new(name, &layers, &instance_extensions);

        Self {
            vulkan,
            pending_messages: VecDeque::new(),
            pending_window_requests: VecDeque::new(),
            _state: std::marker::PhantomData::<Model>::default(),
        }
    }

    pub fn vulkan(&self) -> &Vulkan {
        &self.vulkan
    }

    pub fn send_message(&mut self, msg: Model::MessageType) {
        self.pending_messages.push_back(msg)
    }

    fn pop_message(&mut self) -> Option<Model::MessageType> {
        self.pending_messages.pop_front()
    }

    pub fn ui_window_request(&mut self, request: WindowRequest<Model>) {
        self.pending_window_requests.push_back(request)
    }

    pub fn run<Delegate>(mut self, delegate: Delegate, state: Model)
    where
        Delegate: ApplicationDelegate<Model> + 'static,
    {
        let mut s = state;
        let event_loop = EventLoop::new();
        let mut d = delegate;

        let mut window_registry = WindowRegistry::new();

        d.application_will_start(&mut self, &mut s, &mut window_registry, &event_loop);
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
                    window_id,
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
                    event:
                        WindowEvent::CursorMoved {
                            device_id,
                            position,
                            ..
                        },
                    window_id,
                } => {
                    if mouse_is_down {
                        let delta = winit::dpi::PhysicalPosition::<f64>::new(
                            position.x - last_mouse_position.x,
                            position.y - last_mouse_position.y,
                        );
                        window_registry.mouse_dragged(&mut s, &window_id, &position, &delta)
                    } else {
                        window_registry.mouse_moved(&mut s, &window_id, &position)
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
                    event:
                        WindowEvent::KeyboardInput {
                            device_id,
                            input,
                            is_synthetic,
                        },
                } => window_registry.keyboard_event(&window_id, &input, &mut s),

                Event::WindowEvent {
                    event:
                        WindowEvent::MouseInput {
                            device_id,
                            state,
                            button,
                            ..
                        },
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
                    d.application_will_update(&self, &mut s, &mut window_registry, event_loop);
                    window_registry.update(&mut s);
                    window_registry.draw(&self, &mut s);
                }
                _ => (),
            }

            if let ControlFlow::Exit = *control_flow {
                d.application_will_quit(&mut self, event_loop)
            }
        });
    }
}
