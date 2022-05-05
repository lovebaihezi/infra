use iced::{executor, Application, Column, Command, Row, Text};
use tracing::instrument;

#[derive(Debug, Default)]
pub struct App {}

#[derive(Debug, Clone)]
pub enum RootMessage {}

impl Application for App {
    type Executor = executor::Default;

    type Message = RootMessage;

    type Flags = ();

    #[instrument]
    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = Default::default();
        (app, Command::none())
    }

    #[instrument]
    fn title(&self) -> String {
        "liners".into()
    }

    #[instrument]
    fn update(
        &mut self,
        message: Self::Message,
    ) -> iced::Command<Self::Message> {
        Command::none()
    }

    #[instrument]
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .push(
                Row::new()
                    .push(Text::new("center")),
            )
            .into()
    }
}
