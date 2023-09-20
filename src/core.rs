use crate::{
    event::{AppStatus, EventProxyWinit, UserEvent},
    mouse::Mouse,
    renderer::Renderer,
    ui::{UiState, UI},
};

use egui::ClippedPrimitive;
use egui_wgpu::renderer::ScreenDescriptor;
use egui_winit::State;

use anyhow::{Ok, Result};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

pub struct Core {
    event_proxy: EventProxyWinit<UserEvent>,
    state: State,
    status: AppStatus,
    ui: UI,
    pub renderer: Renderer,
    // mouse: Mouse,
}

impl Core {
    pub fn new(
        event_loop: &EventLoop<UserEvent>,
        window: &Window,
        scale_factor: f32,
    ) -> Result<Self> {
        let renderer = pollster::block_on(Renderer::new(window));

        // let mouse = Mouse::new(
        //     window.inner_size().width as f32,
        //     window.inner_size().height as f32,
        // );

        let ui = UI::new();

        let initial_status = AppStatus::Info("Init Done!".to_owned());

        let mut state = State::new(&event_loop);
        state.set_pixels_per_point(scale_factor);

        let event_proxy = event_loop.create_proxy();
        let event_proxy = EventProxyWinit::from_proxy(event_proxy);

        Ok(Self {
            renderer,
            state,
            event_proxy,
            status: initial_status,
            ui,
            // mouse,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, scale_factor: f32) {
        // self.size = (width, height);
        self.renderer.resize(new_size);
        self.state.set_pixels_per_point(scale_factor);
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
        self.state.on_event(self.ui.context(), event).repaint
    }

    pub fn handle_mouse_input(&mut self, event: &WindowEvent) {
        // self.mouse.process_events(event);
    }

    pub fn render(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        let mut ui_state = UiState {
            is_paused: false,
            status: self.status.clone(),
        };

        let raw_input = self.state.take_egui_input(window);

        let full_output = self.ui.prepare(
            raw_input,
            &self.event_proxy,
            &mut ui_state,
            &mut self.renderer.camera(),
            &mut self.renderer.mouse(),
        );

        self.state
            .handle_platform_output(window, self.ui.context(), full_output.platform_output);

        let clipped_primitives: &[ClippedPrimitive] =
            &self.ui.context().tessellate(full_output.shapes);

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.renderer.size.width, self.renderer.size.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        self.renderer.render(
            &full_output.textures_delta,
            clipped_primitives,
            &screen_descriptor,
        )
    }
}
