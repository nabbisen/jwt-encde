use arboard::Clipboard;
use iced::highlighter;
use iced::{
    Alignment::{self, Center},
    Element,
    Length::{Fill, FillPortion},
    alignment::Horizontal::Right,
    widget::{Space, button, column, container, row, stack, text, text_editor, text_input},
};
use jsonwebtoken::Header;
use serde_json::Value;

use crate::app::components::timestamp::{self, Timestamp};

use super::util::jwt::{decode, encode};

#[derive(Default)]
pub struct Window {
    jwt_str: String,
    jwt_header_json_str: text_editor::Content,
    jwt_header: Option<Header>,
    jwt_payload_json_str: text_editor::Content,
    jwt_payload: Option<Value>,
    ui_message: Option<String>,
    timestamp: Timestamp,
}

#[derive(Debug, Clone)]
pub enum Message {
    EncodedChanged(String),
    CopyEncoded,
    JwtHeaderChanged(text_editor::Action),
    JwtPayloadChanged(text_editor::Action),
    CopyJwtHeaderJsonStr,
    CopyJwtPayloadJsonStr,
    Decode,
    Encode,
    Clear,
    TimestampMessage(timestamp::Message),
}

impl Window {
    pub fn view(&self) -> Element<'_, Message> {
        let encoded = container(column![
            row![
                text("JWT"),
                button("Copy")
                    .style(button::secondary)
                    .on_press(Message::CopyEncoded),
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
                button("⬇ Decode").on_press(Message::Decode).padding(5),
                button("⬆ Encode").on_press(Message::Encode).padding(5),
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
                    button("Copy")
                        .style(button::secondary)
                        .on_press(Message::CopyJwtHeaderJsonStr)
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
                    .size(20),
            ]
            .width(FillPortion(4))
            .height(Fill),
            column![
                row![
                    text("Payload"),
                    button("Copy")
                        .style(button::secondary)
                        .on_press(Message::CopyJwtPayloadJsonStr)
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
                    .size(20),
            ]
            .width(FillPortion(6))
            .height(Fill),
        ]);

        let timestamp_helper = container(row![
            self.timestamp
                .view()
                .map(move |msg| Message::TimestampMessage(msg))
        ]);

        let footer = container(row![text(
            "jwt-encde is GUI JWT encoder / decoder - Local, private, easy. Thank you for using. Dev as OSS @ https://github.com/nabbisen/jwt-encde"
        ).color(iced::color!(0x878787))]).width(Fill).align_x(Right);

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

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EncodedChanged(s) => self.jwt_str = s,
            Message::CopyEncoded => {
                if !self.jwt_str.is_empty()
                    && let Ok(mut clipboard) = Clipboard::new()
                {
                    // todo: error handling
                    let _ = clipboard.set_text(self.jwt_str.clone());

                    self.ui_message = Some("JWT Copied".to_owned());
                }
            }
            Message::JwtHeaderChanged(action) => self.jwt_header_json_str.perform(action),
            Message::JwtPayloadChanged(action) => self.jwt_payload_json_str.perform(action),
            Message::CopyJwtHeaderJsonStr => {
                if !self.jwt_header_json_str.is_empty()
                    && let Ok(mut clipboard) = Clipboard::new()
                {
                    // todo: error handling
                    let _ = clipboard.set_text(self.jwt_header_json_str.text());

                    self.ui_message = Some("JSON Header Copied".to_owned());
                }
            }
            Message::CopyJwtPayloadJsonStr => {
                if !self.jwt_payload_json_str.is_empty()
                    && let Ok(mut clipboard) = Clipboard::new()
                {
                    // todo: error handling
                    let _ = clipboard.set_text(self.jwt_payload_json_str.text());

                    self.ui_message = Some("JSON Payload Copied".to_owned());
                }
            }
            Message::Decode => {
                if self.jwt_str.is_empty() {
                    self.ui_message = Some("No input.".to_owned());
                    return;
                }

                match decode(self.jwt_str.as_str()) {
                    Ok((header, payload)) => {
                        self.jwt_header = header;
                        if let Some(header) = self.jwt_header.as_ref() {
                            let s = serde_json::to_string_pretty(header)
                                .expect("Failed to get str from json value");
                            self.jwt_header_json_str = text_editor::Content::with_text(s.as_str());
                        }
                        self.jwt_payload = payload;
                        if let Some(payload) = self.jwt_payload.as_ref() {
                            let s = serde_json::to_string_pretty(payload)
                                .expect("Failed to get str from json value");
                            self.jwt_payload_json_str = text_editor::Content::with_text(s.as_str());
                        }
                    }
                    Err(_) => self.clear_decoded(),
                }
            }
            Message::Encode => {
                match json5::from_str(self.jwt_header_json_str.text().as_str()) {
                    Ok(x) => self.jwt_header = x,
                    Err(err) => {
                        self.ui_message = Some(format!(
                            "Failed to convert header to json: {}",
                            err.to_string()
                        ));
                        return;
                    }
                };
                match json5::from_str(self.jwt_payload_json_str.text().as_str()) {
                    Ok(x) => self.jwt_payload = x,
                    Err(err) => {
                        self.ui_message = Some(format!(
                            "Failed to convert payload to json: {}",
                            err.to_string()
                        ));
                        return;
                    }
                };

                if self.jwt_header.is_none() && self.jwt_payload.is_none() {
                    self.ui_message = Some("No input.".to_owned());
                    return;
                }

                match encode(self.jwt_header.as_ref(), self.jwt_payload.as_ref()) {
                    Ok(x) => self.jwt_str = x,
                    Err(_) => self.clear_encoded(),
                };
            }
            Message::Clear => self.clear(),
            Message::TimestampMessage(msg) => {
                self.timestamp.update(msg);
            }
        }
    }

    fn clear(&mut self) {
        self.clear_encoded();
        self.clear_decoded();
        self.ui_message = None;
        self.timestamp.update(timestamp::Message::Clear);
    }

    fn clear_encoded(&mut self) {
        self.jwt_str = String::default();
    }

    fn clear_decoded(&mut self) {
        self.jwt_header = None;
        self.jwt_header_json_str = text_editor::Content::default();
        self.jwt_payload = None;
        self.jwt_payload_json_str = text_editor::Content::default();
    }
}
