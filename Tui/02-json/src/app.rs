use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Exiting,
}

#[derive(Debug, Default)]
pub enum CurrentlyEditing {
    #[default]
    Key,
    Value,
}

#[derive(Debug, Default)]
pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            pairs: HashMap::from([
                ("k1".to_string(), "v1".to_string()),
                ("k2".to_string(), "v2".to_string()),
            ]),
            ..Default::default()
        }
    }
    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }
    pub fn toggle_editing(&mut self) {
        self.currently_editing = match &self.currently_editing {
            Some(CurrentlyEditing::Key) => Some(CurrentlyEditing::Value),
            _ => Some(CurrentlyEditing::Key),
        };
    }
    pub fn json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.pairs)?)
    }
}
