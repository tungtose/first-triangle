use cgmath::{Point3, Vector2, Vector3};
use egui::{Button, Context, FontDefinitions, FullOutput, RawInput, TopBottomPanel, Vec2};

use crate::{
    camera::Camera,
    event::{AppStatus, EventProxy, UserEvent},
    mouse::Mouse,
    shortcut::Shortcut,
};

pub struct UI {
    context: Context,
    shortcut: Shortcut,
}

impl UI {
    pub fn new() -> Self {
        let context = Context::default();

        let sc = Shortcut::new();

        Self {
            context,
            shortcut: sc,
        }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn prepare(
        &mut self,
        raw_input: RawInput,
        event_proxy: &impl EventProxy<UserEvent>,
        state: &mut UiState,
        camera: &mut Camera,
        mouse: &mut Mouse,
    ) -> FullOutput {
        self.context.run(raw_input, |ctx| {
            self.ui(ctx, event_proxy, state, camera, mouse);
        })
    }

    fn ui(
        &self,
        ctx: &Context,
        event_proxy: &impl EventProxy<UserEvent>,
        state: &mut UiState,
        camera: &mut Camera,
        mouse: &mut Mouse,
    ) {
        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.app_quit)) {
            event_proxy.send_event(UserEvent::Quit);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_new)) {
            event_proxy.send_event(UserEvent::NewFile);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_open)) {
            event_proxy.send_event(UserEvent::OpenFile);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_save)) {
            event_proxy.send_event(UserEvent::SaveFile);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_save_as)) {
            event_proxy.send_event(UserEvent::SaveFileAs);
        }

        egui::containers::Window::new("Debuger")
            .default_open(true)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Camera").show(ui, |ui| {
                    egui::Grid::new("debug_camera_grid")
                        .num_columns(2)
                        .spacing([10.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Eye:");
                            camera.eye.render_xyz(ui, "eye");
                            ui.end_row();

                            ui.label("Target:");
                            camera.target.render_xyz(ui, "target");
                            ui.end_row();

                            ui.label("Up:");
                            camera.up.render_xyz(ui, "up");
                            ui.end_row();

                            ui.label("Aspect:");
                            ui.add(egui::TextEdit::singleline(&mut camera.aspect.to_string()));
                            ui.end_row();

                            ui.label("Fovy:");
                            ui.add(egui::TextEdit::singleline(&mut camera.fovy.to_string()));
                            ui.end_row();

                            ui.label("Z Near:");
                            ui.add(egui::TextEdit::singleline(&mut camera.znear.to_string()));
                            ui.end_row();

                            ui.label("Z Far:");
                            ui.add(egui::TextEdit::singleline(&mut camera.zfar.to_string()));
                            ui.end_row();
                        })
                });
                egui::CollapsingHeader::new("Mouse").show(ui, |ui| {
                    egui::Grid::new("debug_mouse_grid")
                        .num_columns(2)
                        .spacing([10.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("viewport:");
                            mouse.pos_viewport().render_xy(ui, "mouse_viewport");
                            ui.end_row();

                            ui.label("Ndc:");
                            mouse.pos_ndc().render_xy(ui, "mouse_ndc");
                            ui.end_row();

                            ui.label("Pressed:");
                            ui.checkbox(&mut mouse.pressed(), "");
                            ui.end_row();

                            ui.label("Released:");
                            ui.checkbox(&mut mouse.released(), "");
                            ui.end_row();
                        })
                });
            });
    }
}

struct Xyz<T> {
    x: T,
    y: T,
    z: T,
}

struct Xy<T> {
    x: T,
    y: T,
}

trait XYContent {
    const MIN_RECT: Vec2 = Vec2::new(40., 15.);
    fn xy(&self) -> Xy<f32>;
    fn render_xy(&self, ui: &mut egui::Ui, label: &str) {
        egui::Grid::new(label)
            .num_columns(3)
            .min_col_width(5.)
            .spacing([5., 10.])
            .show(ui, |ui| {
                ui.label("x");
                ui.add(
                    egui::TextEdit::singleline(&mut self.xy().x.to_string())
                        .min_size(Self::MIN_RECT),
                );
                ui.label("y");
                ui.add(
                    egui::TextEdit::singleline(&mut self.xy().y.to_string())
                        .min_size(Self::MIN_RECT),
                );
            });
    }
}

trait XYZContent {
    const MIN_RECT: Vec2 = Vec2::new(40., 15.);
    fn xyz(&self) -> Xyz<f32>;
    fn render_xyz(&self, ui: &mut egui::Ui, label: &str) {
        egui::Grid::new(label)
            .num_columns(3)
            .min_col_width(3.)
            .spacing([10., 10.])
            .show(ui, |ui| {
                ui.label("x");
                ui.add(
                    egui::TextEdit::singleline(&mut self.xyz().x.to_string())
                        .min_size(Self::MIN_RECT),
                );
                ui.label("y");
                ui.add(
                    egui::TextEdit::singleline(&mut self.xyz().y.to_string())
                        .min_size(Self::MIN_RECT),
                );
                ui.label("z");
                ui.add(
                    egui::TextEdit::singleline(&mut self.xyz().z.to_string())
                        .min_size(Self::MIN_RECT),
                );
            });
    }
}

impl XYContent for Vector2<f32> {
    fn xy(&self) -> Xy<f32> {
        Xy {
            x: self.x,
            y: self.y,
        }
    }
}

impl XYZContent for Point3<f32> {
    fn xyz(&self) -> Xyz<f32> {
        Xyz {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl XYZContent for Vector3<f32> {
    fn xyz(&self) -> Xyz<f32> {
        Xyz {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

pub struct UiState {
    pub is_paused: bool,
    pub status: AppStatus,
}

fn setup_fonts(ctx: &mut Context) {
    let mut fonts = FontDefinitions::default();

    // const FONT_MATERIAL_ICON: &'static str = "MaterialIcons-Regular";
    //
    // fonts.font_data.insert(
    //     FONT_MATERIAL_ICON.to_owned(),
    //     FontData::from_static(material_icons::FONT),
    // );
    //
    // if let Some(vec) = fonts.families.get_mut(&FontFamily::Proportional) {
    //     vec.push(FONT_MATERIAL_ICON.to_owned());
    // }
    //
    // if let Some(vec) = fonts.families.get_mut(&FontFamily::Monospace) {
    //     vec.push(FONT_MATERIAL_ICON.to_owned());
    // }

    ctx.set_fonts(fonts);
}
