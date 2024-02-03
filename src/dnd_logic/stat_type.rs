use serde::{Deserialize, Serialize};
use slint::Color;
use strum_macros::EnumIter;

use crate::*;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl StatType {
    pub fn get_name(&self) -> String {
        match self {
            StatType::Strength => String::from("Strength"),
            StatType::Dexterity => String::from("Dexterity"),
            StatType::Constitution => String::from("Constitution"),
            StatType::Intelligence => String::from("Intelligence"),
            StatType::Wisdom => String::from("Wisdom"),
            StatType::Charisma => String::from("Charisma"),
        }
    }

    pub fn get_short_name(&self) -> String {
        match self {
            StatType::Strength => String::from("STR"),
            StatType::Dexterity => String::from("DEX"),
            StatType::Constitution => String::from("CON"),
            StatType::Intelligence => String::from("INT"),
            StatType::Wisdom => String::from("WIS"),
            StatType::Charisma => String::from("CHA"),
        }
    }

    pub fn get_stat_color(&self) -> Color {
        match self {
            StatType::Strength => STRENGTH_COLOR,
            StatType::Dexterity => DEXTERITY_COLOR,
            StatType::Constitution => CONSTITUTION_COLOR,
            StatType::Intelligence => INTELLIGENCE_COLOR,
            StatType::Wisdom => WISDOM_COLOR,
            StatType::Charisma => CHARISMA_COLOR,
        }
    }

    pub fn from_string(s: &str) -> Option<StatType> {
        let s = s.to_ascii_lowercase();
        match s.as_str() {
            "strength" => Some(StatType::Strength),
            "dexterity" => Some(StatType::Dexterity),
            "constitution" => Some(StatType::Constitution),
            "intelligence" => Some(StatType::Intelligence),
            "wisdom" => Some(StatType::Wisdom),
            "charisma" => Some(StatType::Charisma),
            _ => None,
        }
    }
}