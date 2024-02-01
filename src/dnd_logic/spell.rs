use serde::{Serialize, Deserialize};

use super::class::Class;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum SchoolOfMagic {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
    Dunamancy,
}

impl ToString for SchoolOfMagic {
    fn to_string(&self) -> String{
        match self {
            SchoolOfMagic::Abjuration => String::from("Abjuration"),
            SchoolOfMagic::Conjuration => String::from("Conjuration"),
            SchoolOfMagic::Divination => String::from("Divination"),
            SchoolOfMagic::Enchantment => String::from("Enchantment"),
            SchoolOfMagic::Evocation => String::from("Evocation"),
            SchoolOfMagic::Illusion => String::from("Illusion"),
            SchoolOfMagic::Necromancy => String::from("Necromancy"),
            SchoolOfMagic::Transmutation => String::from("Transmutation"),
            SchoolOfMagic::Dunamancy => String::from("Dunamancy"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub level: i32,
    pub school: SchoolOfMagic,
    pub casting_time: String,
    pub range: String,
    pub target: String,
    pub components: String,
    pub duration: String,
    pub description: String,
    pub higher_levels: String,
    pub classes: Vec<Class>,
    pub ritual: bool,
}

impl Spell {
    pub fn new(
        name: String,
        level: i32,
        school: SchoolOfMagic,
        casting_time: String,
        range: String,
        target: String,
        components: String,
        duration: String,
        description: String,
        classes: Vec<Class>,
        ritual: bool,
        higher_levels: String,
    ) -> Self {
        Self {
            name,
            level,
            school,
            casting_time,
            target,
            range,
            components,
            duration,
            description,
            higher_levels,
            classes,
            ritual,
        }
    }
}