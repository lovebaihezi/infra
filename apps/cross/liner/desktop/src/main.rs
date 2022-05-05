use iced::{Application, Settings};
use liner::App;

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::WARN)
        .pretty()
        .init();
    App::run(Settings::default())
}
