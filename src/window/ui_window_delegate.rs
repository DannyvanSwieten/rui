use wgpu::{Device, Queue};
use winit::window::WindowId;

use crate::app::{App, AppState};
use crate::canvas::{skia_cpu_canvas::SkiaCanvas, Point};
use crate::user_interface::UserInterface;
use crate::widget::Widget;
use crate::window::{MouseEvent, WindowDelegate};

use std::path::Path;
use std::rc::Rc;

struct UI<State: AppState> {
    canvas: SkiaCanvas,
    user_interface: UserInterface<State>,
}

pub struct UiWindowDelegate<State: AppState> {
    surface: Option<wgpu::Surface>,
    device: Rc<Device>,
    queue: Rc<Queue>,
    ui: Option<UI<State>>,
    builder: Box<dyn Fn(&State) -> Box<dyn Widget<State>>>,
}

impl<State: AppState + 'static> UiWindowDelegate<State> {
    pub fn new<F>(device: Rc<Device>, queue: Rc<Queue>, builder: F) -> Self
    where
        F: Fn(&State) -> Box<dyn Widget<State>> + 'static,
    {
        Self {
            device,
            queue,
            surface: None,
            ui: None,
            builder: Box::new(builder),
        }
    }

    fn render_ui(&mut self, state: &State) {
        let size = self.ui.as_ref().unwrap().canvas.size;
        let pixels = if let Some(ui) = &mut self.ui {
            ui.user_interface.paint(state, &mut ui.canvas);
            ui.canvas.pixels()
        } else {
            None
        };

        if let Some(surface) = &self.surface {
            let output = surface
                .get_current_texture()
                .expect("Get surface image failed");
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            let stride = size.width * 4;
            let texture_size = wgpu::Extent3d {
                width: size.width as _,
                height: size.height as _,
                depth_or_array_layers: 1,
            };

            self.queue.write_texture(
                // Tells wgpu where to copy the pixel data
                wgpu::ImageCopyTexture {
                    texture: &output.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                // The actual pixel data
                pixels.unwrap(),
                // The layout of the texture
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(stride as _),
                    rows_per_image: std::num::NonZeroU32::new(size.height as _),
                },
                texture_size,
            );

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
            }

            // submit will accept anything that implements IntoIter
            self.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }
    }
}

impl<State: AppState + 'static> WindowDelegate<State> for UiWindowDelegate<State> {
    fn mouse_moved(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        x: f32,
        y: f32,
    ) {
        let p = Point::from((x, y));
        if let Some(ui) = self.ui.as_mut() {
            ui.user_interface
                .mouse_moved(app, state, window_id, &MouseEvent::new(0, &p, &p));
        }
    }

    fn mouse_dragged(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) {
        let p = Point::from((x, y));
        let d = Point::from((dx, dy));
        if let Some(ui) = self.ui.as_mut() {
            ui.user_interface.mouse_drag(
                app,
                state,
                window_id,
                &MouseEvent::new_with_delta(0, &p, &p, &d),
            );
        }
    }

    fn mouse_down(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        x: f32,
        y: f32,
    ) {
        let p = Point::from((x, y));
        if let Some(ui) = self.ui.as_mut() {
            ui.user_interface
                .mouse_down(app, state, window_id, &MouseEvent::new(0, &p, &p));
        }
    }

    fn mouse_up(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        x: f32,
        y: f32,
    ) {
        let p = Point::from((x, y));
        if let Some(ui) = self.ui.as_mut() {
            ui.user_interface
                .mouse_up(app, state, window_id, &MouseEvent::new(0, &p, &p));
        }
    }

    fn resized(
        &mut self,
        window: &winit::window::Window,
        app: &App<State>,
        state: &State,
        window_id: WindowId,
        width: u32,
        height: u32,
    ) {
        if let Some(surface) = &self.surface {
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                format: surface.get_supported_formats(&app.gpu_api().adapter)[0],
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            surface.configure(&app.gpu_api().device, &config);
        } else {
            let surface = unsafe { app.gpu_api().instance.create_surface(window) };
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                format: surface.get_supported_formats(&app.gpu_api().adapter)[0],
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            surface.configure(&app.gpu_api().device, &config);
            self.surface = Some(surface);
        }

        let mut user_interface = UserInterface::new((self.builder)(state), "light");
        user_interface.resize(state, width, height);
        user_interface.resized(state, window_id);

        self.ui = Some(UI {
            canvas: SkiaCanvas::new(width as _, height as _),
            user_interface,
        });
    }

    fn file_dropped(&mut self, state: &State, window_id: WindowId, path: &Path, x: f32, y: f32) {
        if let Some(ui) = self.ui.as_mut() {
            ui.user_interface
                .file_dropped(state, window_id, path, &Point::new(x, y))
        }
    }

    fn file_hovered(&mut self, state: &State, window_id: WindowId, path: &Path, x: f32, y: f32) {
        if let Some(ui) = self.ui.as_mut() {
            ui.user_interface
                .file_hovered(state, window_id, path, &Point::new(x, y))
        }
    }

    fn draw(&mut self, _: &App<State>, state: &State) {
        // draw user interface
        self.render_ui(state)
    }

    fn close_button_pressed(&mut self, _state: &State, _: WindowId) -> bool {
        true
    }

    fn keyboard_event(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        event: &winit::event::KeyboardInput,
    ) {
        if let Some(ui) = self.ui.as_mut() {
            ui.user_interface
                .keyboard_event(app, state, window_id, event)
        }
    }
    fn character_received(
        &mut self,
        app: &mut App<State>,
        state: &State,
        window_id: WindowId,
        character: char,
    ) {
        if let Some(ui) = self.ui.as_mut() {
            ui.user_interface
                .character_received(app, state, window_id, character)
        }
    }

    fn update(&mut self, _state: &State) {}
}
