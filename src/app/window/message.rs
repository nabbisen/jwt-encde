use iced::widget::text_editor;

use crate::app::components::footer;
use crate::app::components::timestamp;

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
