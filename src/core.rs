use crate::{
    event::{AppResponse, AppStatus, EventProxyWinit, UserEvent},
    renderer::Renderer,
    ui::{UiState, UI},
};

use egui::ClippedPrimitive;
use egui_wgpu::renderer::ScreenDescriptor;
use egui_winit::State;

use anyhow::{bail, Ok, Result};
use winit::{
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Core {
    cursor: [f32; 2],
    event_proxy: EventProxyWinit<UserEvent>,
    state: State,
    status: AppStatus,
    width: f32,
    height: f32,
    // egui_renderer: egui_wgpu::renderer::Renderer,
    ui: UI,
    pub renderer: Renderer,
}

impl Core {
    pub fn new(
        event_loop: &EventLoop<UserEvent>,
        window: &Window,
        width: f32,
        height: f32,
        scale_factor: f32,
    ) -> Result<Self> {
        let renderer = pollster::block_on(Renderer::new(window));

        let ui = UI::new();

        let initial_status = AppStatus::Info("Init Done!".to_owned());

        let mut state = State::new(&event_loop);
        state.set_pixels_per_point(scale_factor);

        let event_proxy = event_loop.create_proxy();
        let event_proxy = EventProxyWinit::from_proxy(event_proxy);

        Ok(Self {
            cursor: Default::default(),
            renderer,
            state,
            width,
            height,
            event_proxy,
            status: initial_status,
            ui,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, scale_factor: f32) {
        // self.size = (width, height);
        self.renderer.resize(new_size);
        self.state.set_pixels_per_point(scale_factor);
    }

    pub fn render(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        let ui_state = UiState {
            is_paused: false,
            status: self.status.clone(),
        };

        let raw_input = self.state.take_egui_input(window);

        let full_output = self.ui.prepare(raw_input, &self.event_proxy, ui_state);

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
