use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::stat_type::StatType;

#[derive(Debug, EnumIter, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum SkillType {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl SkillType {
    pub fn get_name(&self) -> String {
        match self {
            SkillType::Acrobatics => String::from("Acrobatics"),
            SkillType::AnimalHandling => String::from("Animal Handling"),
            SkillType::Arcana => String::from("Arcana"),
            SkillType::Athletics => String::from("Athletics"),
            SkillType::Deception => String::from("Deception"),
            SkillType::History => String::from("History"),
            SkillType::Insight => String::from("Insight"),
            SkillType::Intimidation => String::from("Intimidation"),
            SkillType::Investigation => String::from("Investigation"),
            SkillType::Medicine => String::from("Medicine"),
            SkillType::Nature => String::from("Nature"),
            SkillType::Perception => String::from("Perception"),
            SkillType::Performance => String::from("Performance"),
            SkillType::Persuasion => String::from("Persuasion"),
            SkillType::Religion => String::from("Religion"),
            SkillType::SleightOfHand => String::from("Sleight of Hand"),
            SkillType::Stealth => String::from("Stealth"),
            SkillType::Survival => String::from("Survival"),
        }
    }

    pub fn get_base_stat(&self) -> StatType {
        match self {
            SkillType::Acrobatics => StatType::Dexterity,
            SkillType::AnimalHandling => StatType::Wisdom,
            SkillType::Arcana => StatType::Intelligence,
            SkillType::Athletics => StatType::Strength,
            SkillType::Deception => StatType::Charisma,
            SkillType::History => StatType::Intelligence,
            SkillType::Insight => StatType::Wisdom,
            SkillType::Intimidation => StatType::Charisma,
            SkillType::Investigation => StatType::Intelligence,
            SkillType::Medicine => StatType::Wisdom,
            SkillType::Nature => StatType::Intelligence,
            SkillType::Perception => StatType::Wisdom,
            SkillType::Performance => StatType::Charisma,
            SkillType::Persuasion => StatType::Charisma,
            SkillType::Religion => StatType::Intelligence,
            SkillType::SleightOfHand => StatType::Dexterity,
            SkillType::Stealth => StatType::Dexterity,
            SkillType::Survival => StatType::Wisdom,
        }
    }

    pub fn from_string(skill: &str) -> Option<SkillType> {
        let skill = skill.to_lowercase();
        match skill.as_str() {
            "acrobatics" => Some(SkillType::Acrobatics),
            "animal handling" => Some(SkillType::AnimalHandling),
            "arcana" => Some(SkillType::Arcana),
            "athletics" => Some(SkillType::Athletics),
            "deception" => Some(SkillType::Deception),
            "history" => Some(SkillType::History),
            "insight" => Some(SkillType::Insight),
            "intimidation" => Some(SkillType::Intimidation),
            "investigation" => Some(SkillType::Investigation),
            "medicine" => Some(SkillType::Medicine),
            "nature" => Some(SkillType::Nature),
            "perception" => Some(SkillType::Perception),
            "performance" => Some(SkillType::Performance),
            "persuasion" => Some(SkillType::Persuasion),
            "religion" => Some(SkillType::Religion),
            "sleight of hand" => Some(SkillType::SleightOfHand),
            "stealth" => Some(SkillType::Stealth),
            "survival" => Some(SkillType::Survival),
            _ => None,
        }
    }
}