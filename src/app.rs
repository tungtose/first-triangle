use crate::{core::Core, event::UserEvent};
use anyhow::Result;
use winit::{
    dpi::{LogicalSize, Size},
    event::{ElementState, Event, KeyboardInput, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder, WindowId},
};

const RECOMMAND_HEIGHT: f64 = 720.0;
const RECOMMAND_WIDTH: f64 = 1280.0;

const RECOMMAND_SIZE: Size = Size::Logical(LogicalSize::new(RECOMMAND_WIDTH, RECOMMAND_HEIGHT));

pub struct App {
    core: Core,
    event_loop: EventLoop<UserEvent>,
    // sub_window_map: HashMap<WindowId, Box<dyn WindowExt<UserEvent>>>,
    window: Window,
}

impl App {
    pub fn new() -> Result<Self> {
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

        let window = WindowBuilder::new()
            // .with_min_inner_size(Size::Logical(LogicalSize::new(720.0, 360.0)))
            // .with_inner_size(RECOMMAND_SIZE)
            // .with_title(format_title(&None))
            .build(&event_loop)?;

        try_resize_window(&window);

        let core = Core::new(&event_loop, &window, window.scale_factor() as f32)?;

        Ok(Self {
            core,
            event_loop,
            window,
        })
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, event_loop, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawRequested(window_id) => {
                    if window_id == self.window.id() {
                        self.core.renderer.update();

                        match self.core.render(&self.window) {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                self.core.renderer.resize(self.core.renderer.size)
                            }
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            // We're ignoring timeouts
                            Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                        }
                    }
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    if !self.core.renderer.input(event) {
                        match event {
                            WindowEvent::CloseRequested => {
                                if window_id == self.window.id() {
                                    *control_flow = ControlFlow::Exit;
                                }
                            }
                            WindowEvent::CursorMoved { position, .. } => {
                                if window_id == self.window.id() {
                                    self.core
                                        .update_cursor(position.x as f32, position.y as f32)
                                }
                            }
                            WindowEvent::MouseInput { button, state, .. } => {
                                if window_id == self.window.id() {
                                    if let MouseButton::Left = button {
                                        self.core
                                            .handle_mouse_input(*state == ElementState::Pressed)
                                    }
                                }
                            }
                            WindowEvent::Resized(physical_size) => {
                                if window_id == self.window.id() {
                                    self.core
                                        .resize(*physical_size, self.window.scale_factor() as f32);
                                }
                            }
                            WindowEvent::ScaleFactorChanged {
                                scale_factor,
                                new_inner_size,
                            } => {
                                if window_id == self.window.id() {
                                    try_resize_window(&self.window);

                                    self.core.resize(**new_inner_size, *scale_factor as f32);
                                } else {
                                    // if let Some(window) = self.sub_window_map.get_mut(&window_id) {
                                    //     window.on_scaled(*scale_factor as f32);
                                    // }
                                }
                            }
                            _ => {}
                        }
                    }
                    self.core.handle_window_event(event);
                }
                Event::UserEvent(event) => {
                    todo!();
                }
                _ => {}
            }
        });
    }
}

fn try_resize_window(window: &Window) {
    if let Some(monitor) = window.current_monitor() {
        let monitor_size = monitor.size();
        let outer_size = window.outer_size();

        if monitor_size.width <= outer_size.width || monitor_size.height <= outer_size.height {
            window.set_maximized(true);
        }
    }
}
