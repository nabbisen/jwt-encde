use iced::{
    Element,
    Length::Fill,
    alignment::Horizontal::Right,
    widget::{button, container, row, svg, text, tooltip},
};

const REPOSITORY_URL: &str = "https://github.com/nabbisen/jwt-encde";

#[derive(Default)]
pub struct Footer {}

#[derive(Debug, Clone)]
pub enum Message {
    RepositoryLink,
}

impl Footer {
    pub fn view(&self) -> Element<'_, Message> {
        let github_svg_bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/assets/github.svg"
        ));
        let github_icon = svg(svg::Handle::from_memory(github_svg_bytes))
            .width(12)
            .height(12);
        let github_icon_with_tooltip = tooltip(
            github_icon,
            text(format!("Open-source project\n{}", REPOSITORY_URL)).size(14),
            tooltip::Position::Top,
        );
        let repository_link = button(github_icon_with_tooltip)
            .style(button::secondary)
            .on_press(Message::RepositoryLink);

        let footer = container(
            row![
                text("jwt-encde is GUI JWT encoder / decoder - Local, private, easy.")
                    .color(iced::color!(0x878787)),
                repository_link
            ]
            .spacing(8),
        )
        .width(Fill)
        .align_x(Right);

        footer.into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::RepositoryLink => {
                let _ = webbrowser::open(REPOSITORY_URL);
            }
        }
    }
}
