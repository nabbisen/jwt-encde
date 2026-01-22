use arboard::Clipboard;
use iced::widget::text_editor::{Action, Edit};

use super::{Window, message::Message};
use crate::app::util::jwt::{decode, encode};

impl Window {
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
}
