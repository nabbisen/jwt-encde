use iced::widget::text_editor;
use serde_json::Value;

use crate::app::components::footer::Footer;
use crate::app::components::timestamp::{self, Timestamp};

mod message;
mod update;
mod view;

#[derive(Default)]
pub struct Window {
    jwt_str: String,
    jwt_header_json_str: text_editor::Content,
    jwt_header: Option<Value>,
    jwt_payload_json_str: text_editor::Content,
    jwt_payload: Option<Value>,
    ui_message: Option<String>,
    timestamp: Timestamp,
    footer: Footer,
}

impl Window {
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
