mod app;
mod camera;
mod core;
pub mod egui_context;
mod event;
mod model;
mod renderer;
mod resources;
mod shortcut;
mod texture;
mod ui;
pub mod window;

fn main() {
    env_logger::init();

    let app = app::App::new().unwrap();

    app.run();
}
