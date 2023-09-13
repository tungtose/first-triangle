use egui::{Button, Context, FontDefinitions, FullOutput, RawInput, TopBottomPanel};

use crate::{
    event::{AppStatus, EventProxy, UserEvent},
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
        state: UiState,
    ) -> FullOutput {
        self.context.run(raw_input, |ctx| {
            self.ui(ctx, event_proxy, state);
        })
    }

    fn ui(&self, ctx: &Context, event_proxy: &impl EventProxy<UserEvent>, state: UiState) {
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

        egui::containers::Window::new("Window")
            .default_open(true)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Loader").show(ui, |ui| {
                    if ui.button("Load glTF").clicked() {
                        println!("Hello!!!!!");
                    }
                });
                ui.add(egui::Hyperlink::from_label_and_url(
                    "Link...",
                    "https://github.com/tungtose",
                ));
            });
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
