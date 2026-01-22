use iced::highlighter;
use iced::widget::text::Wrapping;
use iced::{
    Alignment::{self, Center},
    Element,
    Length::{Fill, FillPortion},
    widget::{Space, button, column, container, row, stack, text, text_editor, text_input},
};

use super::{Window, message::Message};

impl Window {
    pub fn view(&self) -> Element<'_, Message> {
        let encoded = container(column![
            row![
                text("JWT"),
                button("Copy").style(button::secondary).on_press_maybe(
                    if !self.jwt_str.is_empty() {
                        Some(Message::CopyEncoded)
                    } else {
                        None
                    }
                ),
                Space::new().width(Fill),
                button("Clear")
                    .style(button::secondary)
                    .on_press(Message::Clear),
            ]
            .padding(5)
            .spacing(20)
            .align_y(Center),
            text_input("JWT here...", &self.jwt_str)
                .on_input(Message::EncodedChanged)
                .padding(10)
                .size(20),
        ]);

        let buttons = container(
            row![
                button("↓ Decode")
                    .padding(5)
                    .on_press_maybe(if !self.jwt_str.is_empty() {
                        Some(Message::Decode)
                    } else {
                        None
                    }),
                button("Encode ↑").padding(5).on_press_maybe(
                    if !self.jwt_header_json_str.is_empty() || !self.jwt_payload_json_str.is_empty()
                    {
                        Some(Message::Encode)
                    } else {
                        None
                    }
                ),
            ]
            .spacing(40)
            .align_y(Center),
        )
        .center_x(Fill)
        .padding(8);

        let decoded = container(row![
            column![
                row![
                    text("Header"),
                    button("Copy").style(button::secondary).on_press_maybe(
                        if !self.jwt_header_json_str.is_empty() {
                            Some(Message::CopyJwtHeaderJsonStr)
                        } else {
                            None
                        }
                    )
                ]
                .padding(5)
                .spacing(20)
                .align_y(Center),
                text_editor(&self.jwt_header_json_str)
                    .placeholder("JSON str of JWT header here...")
                    .on_action(Message::JwtHeaderChanged)
                    .highlight("json", highlighter::Theme::Base16Ocean)
                    .height(Fill)
                    .padding(5)
                    .size(20)
                    .wrapping(Wrapping::WordOrGlyph),
            ]
            .width(FillPortion(3))
            .height(Fill),
            column![
                row![
                    text("Payload"),
                    button("Copy").style(button::secondary).on_press_maybe(
                        if !self.jwt_payload_json_str.is_empty() {
                            Some(Message::CopyJwtPayloadJsonStr)
                        } else {
                            None
                        }
                    )
                ]
                .padding(5)
                .spacing(20)
                .align_y(Center),
                text_editor(&self.jwt_payload_json_str)
                    .placeholder("JSON str of JWT payload here...")
                    .on_action(Message::JwtPayloadChanged)
                    .highlight("json", highlighter::Theme::Base16Ocean)
                    .height(Fill)
                    .padding(10)
                    .size(20)
                    .wrapping(Wrapping::WordOrGlyph),
            ]
            .width(FillPortion(7))
            .height(Fill),
        ]);

        let timestamp_helper = container(row![
            self.timestamp
                .view()
                .map(move |msg| Message::TimestampMessage(msg))
        ]);

        let footer = container(
            self.footer
                .view()
                .map(move |msg| Message::FooterMessage(msg)),
        );

        let content = column![encoded, buttons, decoded, timestamp_helper, footer]
            .spacing(15)
            .padding(20)
            .align_x(Alignment::Start);

        let mut stack = stack![
            container(content)
                .width(Fill)
                .height(Fill)
                .center_x(Fill)
                .center_y(Fill)
        ];
        if let Some(ui_message) = self.ui_message.clone() {
            let ui_message = container(text(ui_message))
                .padding(5)
                .width(Fill)
                .align_x(Center);
            stack = stack.push(ui_message);
        }

        stack.into()
    }
}
