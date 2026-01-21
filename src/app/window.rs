use arboard::Clipboard;
use iced::highlighter;
use iced::widget::text_editor::{Action, Edit};
use iced::{
    Alignment::{self, Center},
    Element,
    Length::{Fill, FillPortion},
    widget::{Space, button, column, container, row, stack, text, text_editor, text_input},
};
use jsonwebtoken::Header;
use serde_json::Value;

use crate::app::components::footer::{self, Footer};
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
    footer: Footer,
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
    FooterMessage(footer::Message),
}

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
                button("⬇ Decode")
                    .padding(5)
                    .on_press_maybe(if !self.jwt_str.is_empty() {
                        Some(Message::Decode)
                    } else {
                        None
                    }),
                button("⬆ Encode").padding(5).on_press_maybe(
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
                    .size(20),
            ]
            .width(FillPortion(4))
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

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EncodedChanged(s) => self.jwt_str = s,
            Message::CopyEncoded => {
                self.ui_message = Some(match Clipboard::new() {
                    Ok(mut clipboard) => match clipboard.set_text(self.jwt_str.clone()) {
                        Ok(_) => "JWT Copied".to_owned(),
                        Err(err) => format!("Failed to copy to clipboard: {}", err.to_string()),
                    },
                    Err(err) => format!("Failed to load clipboard: {}", err.to_string()),
                });
            }
            Message::JwtHeaderChanged(action) => self.jwt_header_json_str.perform(action),
            Message::JwtPayloadChanged(action) => self.jwt_payload_json_str.perform(action),
            Message::CopyJwtHeaderJsonStr => {
                self.ui_message = Some(match Clipboard::new() {
                    Ok(mut clipboard) => {
                        match clipboard.set_text(self.jwt_header_json_str.text()) {
                            Ok(_) => "JSON Header Copied".to_owned(),
                            Err(err) => format!("Failed to copy to clipboard: {}", err.to_string()),
                        }
                    }
                    Err(err) => format!("Failed to load clipboard: {}", err.to_string()),
                });
            }
            Message::CopyJwtPayloadJsonStr => {
                self.ui_message = Some(match Clipboard::new() {
                    Ok(mut clipboard) => match clipboard.set_text(self.jwt_payload_json_str.text())
                    {
                        Ok(_) => "JSON Payload Copied".to_owned(),
                        Err(err) => format!("Failed to copy to clipboard: {}", err.to_string()),
                    },
                    Err(err) => format!("Failed to load clipboard: {}", err.to_string()),
                });
            }
            Message::Decode => match decode(self.jwt_str.as_str()) {
                Ok((header, payload)) => {
                    self.jwt_header = header;
                    let jwt_header_json_str = if let Some(header) = self.jwt_header.as_ref() {
                        serde_json::to_string_pretty(header)
                            .expect("Failed to get str from jwt header json")
                    } else {
                        String::default()
                    };
                    self.jwt_header_json_str.perform(Action::SelectAll);
                    self.jwt_header_json_str
                        .perform(Action::Edit(Edit::Paste(jwt_header_json_str.into())));

                    self.jwt_payload = payload;
                    let jwt_payload_json_str = if let Some(payload) = self.jwt_payload.as_ref() {
                        if !payload.is_null() {
                            serde_json::to_string_pretty(payload)
                                .expect("Failed to get str from jwt payload json")
                        } else {
                            String::default()
                        }
                    } else {
                        String::default()
                    };
                    self.jwt_payload_json_str.perform(Action::SelectAll);
                    self.jwt_payload_json_str
                        .perform(Action::Edit(Edit::Paste(jwt_payload_json_str.into())));
                }
                Err(_) => self.clear_decoded(),
            },
            Message::Encode => {
                if !self.jwt_header_json_str.text().is_empty() {
                    match json5::from_str(self.jwt_header_json_str.text().as_str()) {
                        Ok(x) => self.jwt_header = x,
                        Err(err) => {
                            self.ui_message = Some(format!(
                                "Failed to convert header to json: {}",
                                err.to_string()
                            ));
                            return;
                        }
                    }
                } else {
                    self.jwt_header = None;
                }

                if !self.jwt_payload_json_str.text().is_empty() {
                    match json5::from_str(self.jwt_payload_json_str.text().as_str()) {
                        Ok(x) => self.jwt_payload = x,
                        Err(err) => {
                            self.ui_message = Some(format!(
                                "Failed to convert payload to json: {}",
                                err.to_string()
                            ));
                            return;
                        }
                    }
                } else {
                    self.jwt_payload = None;
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
            Message::FooterMessage(msg) => {
                self.footer.update(msg);
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
