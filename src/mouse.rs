use cgmath::Vector2;
use winit::event::{ElementState, MouseButton, WindowEvent};

#[derive(Clone, Copy)]
pub struct Mouse {
    pos_viewport: Vector2<f32>,
    pos_ndc: Vector2<f32>,
    screen: Vector2<f32>,
    pressed: bool,
    released: bool,
}

impl Mouse {
    pub fn new(sw: f32, sh: f32) -> Self {
        Self {
            pos_ndc: Vector2::<f32>::new(0., 0.),
            pos_viewport: Vector2::<f32>::new(0., 0.),
            screen: Vector2::new(sw, sh),
            pressed: false,
            released: false,
        }
    }

    pub fn pos_viewport(&self) -> Vector2<f32> {
        self.pos_viewport
    }

    pub fn pos_ndc(&self) -> Vector2<f32> {
        self.pos_ndc
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    pub fn released(&self) -> bool {
        self.released
    }

    pub fn update_from_viewport(&mut self, x: f32, y: f32) {
        self.pos_viewport = Vector2::new(x, y);

        let ndc_x = (x / self.screen.x) * 2.0 - 1.0;
        let ndc_y = 1.0 - (y / self.screen.y) * 2.0;

        self.pos_ndc = Vector2::new(ndc_x, ndc_y);
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput { button, state, .. } => {
                if let MouseButton::Left = button {
                    self.pressed = *state == ElementState::Pressed;
                    self.released = *state == ElementState::Released;

                    return true;
                }
                false
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.update_from_viewport(position.x as f32, position.y as f32);
                true
            }
            _ => false,
        }
    }
}
