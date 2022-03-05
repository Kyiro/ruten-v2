use iced::{window, Sandbox, Settings};

pub mod app;
pub mod launch;

pub use app::App;

fn main() -> iced::Result {
    let mut settings = Settings::default();

    settings.antialiasing = true;
    settings.window.size = (800, 550);
    settings.window.icon = if let Ok(icon) = window::Icon::from_rgba(
        include_bytes!("./../resources/Ruten64.rgba").to_vec(),
        64,
        64,
    ) {
        Some(icon)
    } else {
        None
    };

    App::run(settings)
}
