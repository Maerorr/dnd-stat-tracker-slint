use serde::{Deserialize, Serialize};

use crate::SlintItem;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub amount: i32,
}

impl Item {
    pub fn new(name: &str, description: &str, amount: i32) -> Item {
        Item {
            name: name.to_string(),
            description: description.to_string(),
            amount,
        }
    }

    pub fn to_slint_item(&self) -> SlintItem {
        SlintItem {
            name: self.name.clone().into(),
            description: self.description.clone().into(),
            amount: self.amount.into(),
        }
    }
}