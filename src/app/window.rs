use arboard::Clipboard;
use iced::{
    Alignment, Element,
    Length::{Fill, FillPortion},
    alignment::Horizontal::Right,
    widget::{Text, button, column, container, row, text, text_editor, text_input},
};
use jsonwebtoken::Header;
use serde_json::Value;

use crate::app::jwt::{decode, encode};

#[derive(Default)]
pub struct Window {
    jwt_str: String,
    jwt_header_json_str: text_editor::Content,
    jwt_header: Option<Header>,
    jwt_payload_json_str: text_editor::Content,
    jwt_payload: Option<Value>,
    ui_message: Option<String>,
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
}

impl Window {
    pub fn view(&self) -> Element<'_, Message> {
        let encoded = row![column![
            row![text("JWT"), button("Copy").on_press(Message::CopyEncoded)].spacing(20),
            text_input("JWT here...", &self.jwt_str)
                .on_input(Message::EncodedChanged)
                .padding(10)
                .size(20),
        ]];

        let buttons = row![
            button("Decode").on_press(Message::Decode).padding(10),
            button("Encode").on_press(Message::Encode).padding(10),
            button("Clear").on_press(Message::Clear).padding(10),
        ]
        .spacing(20);

        let decoded = row![
            column![
                row![
                    text("Header"),
                    button("Copy").on_press(Message::CopyJwtHeaderJsonStr)
                ]
                .spacing(20),
                text_editor(&self.jwt_header_json_str)
                    .placeholder("JSON str of JWT header here...")
                    .on_action(Message::JwtHeaderChanged)
                    .height(Fill)
                    .padding(10)
                    .size(20),
            ]
            .width(FillPortion(4))
            .height(Fill),
            column![
                row![
                    text("Payload"),
                    button("Copy").on_press(Message::CopyJwtPayloadJsonStr)
                ]
                .spacing(20),
                text_editor(&self.jwt_payload_json_str)
                    .placeholder("JSON str of JWT payload here...")
                    .on_action(Message::JwtPayloadChanged)
                    .height(Fill)
                    .padding(10)
                    .size(20),
            ]
            .width(FillPortion(6))
            .height(Fill),
        ];

        let footer = row![text(
            "jwt-encde is GUI JWT encoder / decoder - Local, private, easy. Thank you for using. Dev as OSS @ https://github.com/nabbisen/jwt-encde"
        ).width(Fill).align_x(Right).color(iced::color!(0x878787))];

        let mut content = column![];
        if let Some(ui_message) = self.ui_message.clone() {
            content = content.push(Text::new(ui_message));
        }
        for child in [encoded, buttons, decoded, footer] {
            content = content.push(child);
        }
        content = content.spacing(15).padding(20).align_x(Alignment::Start);

        container(content)
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .center_y(Fill)
            .into()
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

                    self.ui_message = Some("Copied".to_owned());
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

                    self.ui_message = Some("Copied".to_owned());
                }
            }
            Message::CopyJwtPayloadJsonStr => {
                if !self.jwt_payload_json_str.is_empty()
                    && let Ok(mut clipboard) = Clipboard::new()
                {
                    // todo: error handling
                    let _ = clipboard.set_text(self.jwt_payload_json_str.text());

                    self.ui_message = Some("Copied".to_owned());
                }
            }
            Message::Decode => {
                // todo: button disabled
                if self.jwt_str.is_empty() {
                    return;
                }

                match decode(self.jwt_str.as_str()) {
                    Ok((header, payload)) => {
                        self.jwt_header = header;
                        if let Some(header) = self.jwt_header.as_ref() {
                            let s = serde_json::to_string_pretty(header)
                                .expect("failed to get str from json value");
                            self.jwt_header_json_str = text_editor::Content::with_text(s.as_str());
                        }
                        self.jwt_payload = payload;
                        if let Some(payload) = self.jwt_payload.as_ref() {
                            let s = serde_json::to_string_pretty(payload)
                                .expect("failed to get str from json value");
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
                            "failed to convert header to json: {}",
                            err.to_string()
                        ));
                        return;
                    }
                };
                match json5::from_str(self.jwt_payload_json_str.text().as_str()) {
                    Ok(x) => self.jwt_payload = x,
                    Err(err) => {
                        self.ui_message = Some(format!(
                            "failed to convert payload to json: {}",
                            err.to_string()
                        ));
                        return;
                    }
                };
                match encode(self.jwt_header.as_ref(), self.jwt_payload.as_ref()) {
                    Ok(x) => self.jwt_str = x,
                    Err(_) => self.clear_encoded(),
                };
            }
            Message::Clear => self.clear(),
        }
    }

    fn clear(&mut self) {
        self.clear_encoded();
        self.clear_decoded();
        self.ui_message = None;
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
