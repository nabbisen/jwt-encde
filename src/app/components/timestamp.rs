use iced::{
    Element,
    Length::{Fill, FillPortion},
    widget::{container, row, text, text_input},
};

#[derive(Default)]
pub struct Timestamp {
    pub unix: String,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Input(String),
    Clear,
}

impl Timestamp {
    pub fn view(&self) -> Element<'_, Message> {
        let content = row![
            text("Timestamp converter"),
            text_input("Unix timestamp here...", &self.unix)
                .on_input(Message::Input)
                .width(FillPortion(1)),
            text(self.timestamp.as_str()).width(FillPortion(1))
        ]
        .width(Fill)
        .spacing(20);

        container(content).width(Fill).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Input(s) => {
                let filtered = filter_i64_string(s.as_str());
                self.unix = if filtered != s.as_str() { filtered } else { s };

                if self.unix.is_empty() {
                    return;
                }

                self.timestamp = if let Some(dt) = chrono::DateTime::<chrono::Utc>::from_timestamp(
                    self.unix.parse::<i64>().unwrap(),
                    0,
                ) {
                    dt.format("%Y-%m-%d %H:%M:%S (UTC)").to_string()
                } else {
                    String::default()
                };
            }
            Message::Clear => self.clear(),
        }
    }

    fn clear(&mut self) {
        self.unix = String::default();
        self.timestamp = String::default();
    }
}

fn filter_i64_string(input: &str) -> String {
    let mut result = String::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            result.push(c);
        } else if c == '-' && i == 0 {
            result.push(c);
        }
    }

    result
}
