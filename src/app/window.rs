use iced::{
    Alignment, Element,
    Length::Fill,
    widget::{button, column, container, row, text_editor, text_input},
};
use serde_json::Value;

use crate::app::util::{decode, encode};

#[derive(Default)]
pub struct Window {
    encoded_value: String,
    decoded_value: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    EncodedChanged(String),
    DecodedChanged(text_editor::Action),
    Decode,
    Encode,
    Clear,
}

impl Window {
    pub fn new() -> Self {
        Self {
            encoded_value: String::new(),
            decoded_value: text_editor::Content::new(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let encoded = text_input("JWT here...", &self.encoded_value)
            .on_input(Message::EncodedChanged)
            .padding(10)
            .size(20);

        let buttons = row![
            button("Decode").on_press(Message::Decode).padding(10),
            button("Encode").on_press(Message::Encode).padding(10),
            button("Clear").on_press(Message::Clear).padding(10),
        ]
        .spacing(20);

        let decoded = text_editor(&self.decoded_value)
            .placeholder("JSON here...")
            .on_action(Message::DecodedChanged)
            .height(Fill)
            .padding(10)
            .size(20);

        let content = column![encoded, buttons, decoded]
            .spacing(15)
            .padding(20)
            .align_x(Alignment::Start);

        container(content)
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EncodedChanged(s) => self.encoded_value = s,
            Message::DecodedChanged(action) => self.decoded_value.perform(action),
            Message::Decode => {
                if self.encoded_value.is_empty() {
                    return;
                }

                match decode(self.encoded_value.as_str()) {
                    Ok(x) => {
                        let s = x.as_str().expect("failed to get str from json value");
                        self.decoded_value = text_editor::Content::with_text(s);
                    }
                    Err(_) => self.decoded_value = text_editor::Content::new(),
                }
            }
            Message::Encode => {
                if self.decoded_value.text().is_empty() {
                    return;
                }

                let s = self.decoded_value.text();
                // todo: exit unexpectedly when JSON value is abc instead of "abc" - show error message
                let v: Value = json5::from_str(&s).expect("failed to get json from str");
                let p = serde_json::to_string_pretty(&v).expect("failed to prettify");
                if s != p {
                    self.decoded_value = text_editor::Content::with_text(p.as_str());
                }
                self.encoded_value = match encode(p.as_str()) {
                    Ok(x) => x,
                    Err(_) => String::new(),
                };
            }
            Message::Clear => {
                self.encoded_value = String::new();
                self.decoded_value = text_editor::Content::new();
            }
        }
    }
}
