use iced::{Sandbox, *};
use iced_aw::*;

pub mod style;

#[derive(Default)]
pub struct App {
    split_pane: split::State,
    launch_button: button::State,
    launched_status: LaunchPhase,
    style: style::Theme,
}

#[derive(Debug, Clone)]
pub enum LaunchPhase {
    NotLaunched,
    Launching,
    Launched,
}

#[derive(Debug, Clone)]
pub enum Message {
    OnResize(u16),
    LaunchButton,
}

impl Default for LaunchPhase {
    fn default() -> Self {
        Self::NotLaunched
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Ruten v2")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OnResize(pos) => self.split_pane.set_divider_position(pos),
            Message::LaunchButton => {
                self.launched_status = LaunchPhase::Launching;
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .push(
                Split::new(
                    &mut self.split_pane,
                    Column::new()
                        .push(
                            Text::new("Ruten v2")
                                .width(Length::Fill)
                                .size(20)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .color(self.style.text_colour()),
                        )
                        .padding(15)
                        .width(Length::Fill)
                        .height(Length::Fill),
                    Column::new()
                        .push(
                            Text::new("Settings")
                                .width(Length::Fill)
                                .size(20)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .color(self.style.text_colour()),
                        )
                        .padding(15)
                        .width(Length::Fill)
                        .height(Length::Fill),
                    Self::Message::OnResize,
                )
                .style(self.style),
            )
            .push(
                Container::new(
                    Button::new(
                        &mut self.launch_button,
                        Text::new(match self.launched_status {
                            LaunchPhase::NotLaunched => "Launch",
                            LaunchPhase::Launching => "Launching...",
                            LaunchPhase::Launched => "Launched...",
                        })
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(22)
                        .width(Length::Fill),
                    )
                    .on_press(Self::Message::LaunchButton)
                    .width(Length::Fill)
                    .style(self.style),
                )
                .style(self.style)
                .width(Length::Fill),
            )
            .into()
    }
}
