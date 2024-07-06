use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Clone, Debug)]
pub struct Toast {
    pub content: Option<String>,
    pub timestamp: u128,
    pub visible: bool,
}

impl Toast {
    pub fn new() -> Self {
        let timestamp = Self::get_timestamp();

        Toast {
            content: None,
            visible: false,
            timestamp,
        }
    }

    pub fn show(&mut self, message: String) {
        self.content = Some(message);
        self.visible = true;
        self.timestamp = Self::get_timestamp();
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.timestamp = Self::get_timestamp();
    }

    fn get_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}
