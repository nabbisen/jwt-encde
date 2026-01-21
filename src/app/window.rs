use arboard::Clipboard;
use iced::{
    Alignment, Element,
    Length::Fill,
    widget::{Text, button, column, container, row, text_editor, text_input},
};
use serde_json::Value;

use crate::app::jwt::{decode, encode};

#[derive(Default)]
pub struct Window {
    encoded_str: String,
    decoded_str: text_editor::Content,
    payload: Value,
    ui_message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    EncodedChanged(String),
    CopyEncoded,
    DecodedChanged(text_editor::Action),
    CopyDecoded,
    Decode,
    Encode,
    Clear,
}

impl Window {
    pub fn view(&self) -> Element<'_, Message> {
        let encoded = row![
            text_input("JWT here...", &self.encoded_str)
                .on_input(Message::EncodedChanged)
                .padding(10)
                .size(20),
            button("Copy").on_press(Message::CopyEncoded)
        ];

        let buttons = row![
            button("Decode").on_press(Message::Decode).padding(10),
            button("Encode").on_press(Message::Encode).padding(10),
            button("Clear").on_press(Message::Clear).padding(10),
        ]
        .spacing(20);

        let decoded = row![
            text_editor(&self.decoded_str)
                .placeholder("JSON here...")
                .on_action(Message::DecodedChanged)
                .height(Fill)
                .padding(10)
                .size(20),
            button("Copy").on_press(Message::CopyDecoded)
        ];

        let mut content = column![];
        if let Some(ui_message) = self.ui_message.clone() {
            content = content.push(Text::new(ui_message));
        }
        for child in [encoded, buttons, decoded] {
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
            Message::EncodedChanged(s) => self.encoded_str = s,
            Message::CopyEncoded => {
                if !self.encoded_str.is_empty()
                    && let Ok(mut clipboard) = Clipboard::new()
                {
                    // todo: error handling
                    let _ = clipboard.set_text(self.encoded_str.clone());

                    self.ui_message = Some("Copied".to_owned());
                }
            }
            Message::DecodedChanged(action) => self.decoded_str.perform(action),
            Message::CopyDecoded => {
                if !self.decoded_str.is_empty()
                    && let Ok(mut clipboard) = Clipboard::new()
                {
                    // todo: error handling
                    let _ = clipboard.set_text(self.decoded_str.text());

                    self.ui_message = Some("Copied".to_owned());
                }
            }
            Message::Decode => {
                // todo: button disabled
                if self.encoded_str.is_empty() {
                    return;
                }

                match decode(self.encoded_str.as_str()) {
                    Ok(x) => {
                        self.payload = x;
                        let s = serde_json::to_string_pretty(&self.payload)
                            .expect("failed to get str from json value");
                        self.decoded_str = text_editor::Content::with_text(s.as_str());
                    }
                    Err(_) => self.decoded_str = text_editor::Content::new(),
                }
            }
            Message::Encode => {
                // todo: button disabled
                if self.decoded_str.text().is_empty() {
                    return;
                }

                let s = self.decoded_str.text();
                let v: Value = match json5::from_str(&s) {
                    Ok(x) => x,
                    Err(err) => {
                        self.ui_message =
                            Some(format!("failed to get json from str: {}", err.to_string()));
                        return;
                    }
                };
                self.payload = v;
                self.encoded_str = match encode(&self.payload) {
                    Ok(x) => x,
                    Err(_) => String::default(),
                };
            }
            Message::Clear => {
                self.encoded_str = String::default();
                self.payload = Value::default();
                self.decoded_str = text_editor::Content::default();
                self.ui_message = None;
            }
        }
    }
}
