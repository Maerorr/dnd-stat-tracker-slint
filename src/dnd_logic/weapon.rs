use std::fmt::format;

use serde::{Deserialize, Serialize};
use crate::SlintWeaponInfo;
use crate::SlintWeaponType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    Weapon,
    Ammo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub damage: String,
    pub weapon_type: WeaponType,
    pub ammo: i32,
    pub properties: String,
    pub description: String,
} 

impl Weapon {
    pub fn to_slint_weapon(&self) -> SlintWeaponInfo {
        let weapon_type = match self.weapon_type {
            WeaponType::Weapon => SlintWeaponType::Weapon,
            WeaponType::Ammo => SlintWeaponType::Ammo,
        };
        SlintWeaponInfo {
            name: self.name.clone().into(),
            damage: self.damage.clone().into(),
            weapon_type: weapon_type.clone(),
            ammo: format!("{}", self.ammo).into(),
            properties: self.properties.clone().into(),
            description: self.description.clone().into(),
            equipped: false,
        }
    }
}
