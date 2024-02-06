use std::{fs::File, io::Write};

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::{CHARACTERS_PATH, dnd_logic::prelude::*};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: i32,
    pub class: Class,

    pub experience: i32,
    pub base_armor_class: i32,
    pub initiative_bonus: i32,
    pub speed: i32,

    pub maximum_hit_points: i32,
    pub current_hit_points: i32,
    pub temporary_hit_points: i32,

    pub hit_dice_total: Dice,
    pub hit_dice_used: Dice,
    pub death_saves: DeathSaves,
    pub inspiration: bool,

    pub proficiency_bonus: i32,
    pub stats: Stats,
    pub skills: Skills,

    pub money: Money,
    pub features_and_traits: String,
    pub proficiencies_and_languages: String,

    // DONT SERIALIZE / DESERIALIZE THIS FIELD
    #[serde(skip)]
    pub spell_list: SpellList,

    pub spell_slots_max: Vec<i32>,
    pub spell_slots_used: Vec<i32>,

    spell_serialization_data: Vec<(String, bool)>,

    pub equipment: Vec<Item>,
}

impl Default for Character {
    fn default() -> Self {

        let class = Class::Barbarian;

        Self {
            name: String::from("character"),
            level: 1,
            class: class,
            experience: 0,
            base_armor_class: 10,
            initiative_bonus: 0,
            speed: 30,
            maximum_hit_points: 1,
            current_hit_points: 1,
            temporary_hit_points: 0,
            hit_dice_total: class.get_hit_dice(),
            hit_dice_used: Dice::new(class.get_hit_dice().sides, 0),
            death_saves: DeathSaves::default(),
            inspiration: false,
            proficiency_bonus: 2,

            stats: Stats::default(),
            features_and_traits: String::new(),
            proficiencies_and_languages: String::new(),
            skills: Skills::default(),
            money: Money::default(),
            spell_list: SpellList::default(),
            spell_slots_max: Vec::with_capacity(9),
            spell_slots_used: Vec::with_capacity(9),
            spell_serialization_data: Vec::new(),
            equipment: Vec::new(),
        }
    }
}

impl Character {
    pub fn test_character() -> Self {
        let mut character = Self::default();
        character.maximum_hit_points = 55;
        character.current_hit_points = 55;
        character.temporary_hit_points = 15;
        character.name = String::from("test character");
        character.set_level(5);
        character.class = Class::Barbarian;
        character.stats = Stats::test_stats();
        character.skills = Skills::test_skills();
        character.proficiency_bonus = proficiency_bonus(character.level);
        character.hit_dice_total.count = 5;
        character
    }

    pub fn get_class(&self) -> &Class {
        &self.class
    }

    pub fn get_class_mut(&mut self) -> &mut Class {
        &mut self.class
    }

    pub fn get_ui_proficiency_bonus(&self) -> String {
        format!("+{}", self.proficiency_bonus)
    }

    pub fn get_total_armor_class(&self) -> i32 {
        self.base_armor_class + self.stats.get_stat_modifier(StatType::Dexterity)
    }

    pub fn get_ui_total_initiative(&self) -> String {
        let inititative_bonus = self.initiative_bonus + self.stats.get_stat_modifier(StatType::Dexterity);
        let sign = if inititative_bonus > 0 {
            "+"
        } else {
            ""
        };

        format!("{}{}", sign, inititative_bonus)
    }

    pub fn get_ui_initiative_no_dex(&self) -> String {
        let sign = if self.initiative_bonus > 0 {
            "+"
        } else {
            ""
        };

        format!("{}{}", sign, self.initiative_bonus)
    }

    pub fn get_hit_points_max(&self) -> i32 {
        self.maximum_hit_points
    }

    pub fn get_hit_points_current(&self) -> i32 {
        self.current_hit_points
    }

    pub fn get_hit_points_temp(&self) -> i32 {
        self.temporary_hit_points
    }

    pub fn get_spell_list(&mut self) -> &mut SpellList {
        &mut self.spell_list
    }

    pub fn get_spell_slots_max(&self, lvl: i32) -> i32 {
        if lvl < 1 || lvl > 9 {
            return 0;
        }
        self.spell_slots_max[lvl as usize - 1]
    }

    pub fn get_spell_slots_used(&self, lvl: i32) -> i32 {
        if lvl < 1 || lvl > 9 {
            return 0;
        }
        self.spell_slots_used[lvl as usize - 1]
    }

    pub fn get_ui_hit_dice_total(&self) -> String {
        format!("{}d{}", self.hit_dice_total.count, self.hit_dice_total.sides)
    }

    pub fn get_ui_hit_dice_left(&self) -> String {
        format!("{}d{}", self.hit_dice_total.count - self.hit_dice_used.count, self.hit_dice_total.sides)
    }

    pub fn get_ui_spell_slots(&self) -> Vec<slint::SharedString> {
        let mut slots = Vec::new();
        for i in 0..9 {
            let max = self.spell_slots_max[i];
            let used = self.spell_slots_used[i];
            slots.push(format!("{}/{}", used, max).into());
        }
        slots
    }

    pub fn get_ui_spell_save_dc(&self) -> String {
        let spellcasting_ability = self.class.get_spellcasting_ability();
        if spellcasting_ability.is_none() {
            return String::from("None");
        }
        let dc = 8 + self.proficiency_bonus + self.stats.get_stat_modifier(spellcasting_ability.unwrap());

        format!("{}", dc)
    }

    pub fn get_ui_spell_attack_bonus(&self) -> String {
        let spellcasting_ability = self.class.get_spellcasting_ability();
        if spellcasting_ability.is_none() {
            return String::from("None");
        }
        let bonus = self.proficiency_bonus + self.stats.get_stat_modifier(spellcasting_ability.unwrap());
        format!("{}", bonus)
    }

    pub fn set_experience(&mut self, exp: i32) {
        if exp < 0 {
            self.experience = 0;
            return;
        }
        self.experience = exp;
        self.level = exp_to_lvl(exp);
        self.proficiency_bonus = proficiency_bonus(self.level);
    }

    pub fn set_level(&mut self, lvl: i32) {
        if lvl < 1 {
            self.level = 1;
            return;
        }
        self.level = lvl;
        self.experience = exp_needed_to_lvl(lvl);
        self.proficiency_bonus = proficiency_bonus(self.level);
    }

    pub fn set_class(&mut self, class: Class) {
        self.class = class;
        self.hit_dice_total = class.get_hit_dice();
        self.hit_dice_total.count = self.level;
        self.hit_dice_used = Dice::new(class.get_hit_dice().sides, 0);
    }

    pub fn set_maximum_hit_points(&mut self, max: i32) {
        if max < 0 {
            self.maximum_hit_points = 0;
            return;
        }
        self.maximum_hit_points = max;
    }

    pub fn set_current_hit_points(&mut self, current: i32) {
        if current < 0 {
            self.current_hit_points = 0;
            return;
        }
        if current > self.maximum_hit_points {
            self.current_hit_points = self.maximum_hit_points;
            return;
        }
        self.current_hit_points = current;
    }

    pub fn set_temporary_hit_points(&mut self, temp: i32) {
        if temp < 0 {
            self.temporary_hit_points = 0;
            return;
        }
        self.temporary_hit_points = temp;
    }

    pub fn add_level(&mut self) {
        if self.level >= 20 {
            return;
        }
        self.level += 1;
        self.proficiency_bonus = proficiency_bonus(self.level);
        self.experience = exp_needed_to_lvl(self.level);
        self.hit_dice_total.count = self.level;
    }

    pub fn subtract_level(&mut self) {
        if self.level <= 1 {
            return;
        }
        self.level -= 1;
        self.proficiency_bonus = proficiency_bonus(self.level);
        self.experience = exp_needed_to_lvl(self.level);
        self.hit_dice_total.count = self.level;
    }
  
    pub fn add_one_ac(&mut self) {
        self.base_armor_class += 1;
    }

    pub fn subtract_one_ac(&mut self) {
        if self.base_armor_class <= 0 {
            return;
        }
        self.base_armor_class -= 1;
    }

    pub fn add_one_initiative(&mut self) {
        self.initiative_bonus += 1;
    }

    pub fn subtract_one_initiative(&mut self) {
        self.initiative_bonus -= 1;
    }

    pub fn add_5_speed(&mut self) {
        self.speed += 5;
    }

    pub fn subtract_5_speed(&mut self) {
        if self.speed <= 5 {
            self.speed = 0;
            return;
        }
        self.speed -= 5;
    }

    pub fn take_damage(&mut self, damage: i32) {
        if damage < 0 {
            return;
        }
        if self.temporary_hit_points > 0 {
            self.temporary_hit_points -= damage;
            if self.temporary_hit_points < 0 {
                self.current_hit_points += self.temporary_hit_points;
                self.temporary_hit_points = 0;
            }
            return;
        }
        let mut exceed = 0;
        if self.current_hit_points >= damage {
            self.current_hit_points -= damage;
        } else {
            self.current_hit_points = 0;
            exceed = damage - self.current_hit_points;
        }
        if self.current_hit_points < 0 {
            self.current_hit_points = 0;
        }
        if self.current_hit_points == 0 && exceed > self.maximum_hit_points {
            self.current_hit_points = -9999;
        }
    }

    pub fn heal_damage(&mut self, heal: i32) {
        if heal < 0 {
            return;
        }
        self.current_hit_points += heal;

        if self.current_hit_points > self.maximum_hit_points {
            self.current_hit_points = self.maximum_hit_points;
        }
    }

    pub fn add_temporary_hit_points(&mut self, temp: i32) {
        if temp < 0 {
            return;
        }
        self.temporary_hit_points += temp;
    }

    pub fn subtract_temporary_hit_points(&mut self, temp: i32) {
        if temp < 0 {
            return;
        }
        self.temporary_hit_points -= temp;
        if self.temporary_hit_points < 0 {
            self.temporary_hit_points = 0;
        }
    }

    pub fn add_success_death_save(&mut self) {
        self.death_saves.successes += 1;
        if self.death_saves.successes >= 3 {
            self.death_saves.successes = 3;
        }
    }

    pub fn add_fail_death_save(&mut self) {
        self.death_saves.failures += 1;
        if self.death_saves.failures >= 3 {
            self.death_saves.failures = 3;
        }
    }

    pub fn reset_death_saves(&mut self) {
        self.death_saves = DeathSaves::default();
    }

    

    pub fn add_spell(&mut self, spell:&Spell) {
        self.spell_list.add_spell(&spell, false);
    }

    pub fn save_to_file(&mut self) {
        // copy all spell names and prepared value to spell_serialization_data
        self.spell_serialization_data.clear();
        for cantrip in &self.spell_list.cantrips {
            self.spell_serialization_data.push((cantrip.0.name.clone(), cantrip.1));
        }
        for spells in &self.spell_list.spells {
            for spell in spells {
                self.spell_serialization_data.push((spell.0.name.clone(), spell.1));
            }
        }
        
        let file_name = format!("{}.{}", self.name,"json");
        let mut path = std::path::Path::new(CHARACTERS_PATH);
        let mut file = File::create(path.join(file_name)).unwrap();
        let json = serde_json::to_string_pretty(self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn load_spells(&mut self, spell_database: &SpellList) {
        for (spell_name, prepared) in self.spell_serialization_data.iter() {
            let spell = spell_database.get_spell_by_name(spell_name);
            if spell.is_none() {
                continue;
            }
            let spell = spell.unwrap();
            self.spell_list.add_spell(&spell, *prepared);
        }
    }
    pub fn add_one_spell_slot_max(&mut self, level: i32) {
        if level < 1 || level > 9 {
            return;
        }
        self.spell_slots_max[level as usize - 1] += 1;
    }

    pub fn subtract_one_spell_slot_max(&mut self, level: i32) {
        if level < 1 || level > 9 {
            return;
        }
        if self.spell_slots_max[level as usize - 1] <= 0 {
            return;
        }
        self.spell_slots_max[level as usize - 1] -= 1;
        // if spell slots used is equal to or greater than spell slots max, subtract one from spell slots used
        if self.spell_slots_used[level as usize - 1] >= self.spell_slots_max[level as usize - 1] {
            self.spell_slots_used[level as usize - 1] -= 1;
        }
    }

    pub fn add_one_spell_slot_used(&mut self, level: i32) {
        if level < 1 || level > 9 {
            return;
        }
        if self.spell_slots_used[level as usize - 1] >= self.spell_slots_max[level as usize - 1] {
            return;
        }
        self.spell_slots_used[level as usize - 1] += 1;
    }

    pub fn subtract_one_spell_slot_used(&mut self, level: i32) {
        if level < 1 || level > 9 {
            return;
        }
        if self.spell_slots_used[level as usize - 1] <= 0 {
            return;
        }
        self.spell_slots_used[level as usize - 1] -= 1;
    }

    

    pub fn add_money(&mut self, name: &str, value: i32) {
        match name {
            "cp" => self.money.copper += value,
            "sp" => self.money.silver += value,
            "ep" => self.money.electrum += value,
            "gp" => self.money.gold += value,
            "pp" => self.money.platinum += value,
            _ => {}
        }
    }

    pub fn subtract_money(&mut self, name: &str, value: i32) {
        // check if subtracting value is greater than current value
        match name {
            "cp" => {
                if self.money.copper < value {
                    self.money.copper = 0;
                    return;
                }
                self.money.copper -= value;
            }
            "sp" => {
                if self.money.silver < value {
                    self.money.silver = 0;
                    return;
                }
                self.money.silver -= value;
            }
            "ep" => {
                if self.money.electrum < value {
                    self.money.electrum = 0;
                    return;
                }
                self.money.electrum -= value;
            }
            "gp" => {
                if self.money.gold < value {
                    self.money.gold = 0;
                    return;
                }
                self.money.gold -= value;
            }
            "pp" => {
                if self.money.platinum < value {
                    self.money.platinum = 0;
                    return;
                }
                self.money.platinum -= value;
            }
            _ => {}
        }
    }

    pub fn use_hit_dice(&mut self) {
        if self.hit_dice_used.count >= self.level {
            return;
        }
        self.hit_dice_used.count += 1;
    }

    pub fn restore_one_hit_dice(&mut self) {
        if self.hit_dice_used.count <= 0 {
            return;
        }
        self.hit_dice_used.count -= 1;
    }

    pub fn convert_money(&mut self, from: &String, to: &String, value: i32) {
        let from = Currency::from_string(from);
        let to = Currency::from_string(to);
        if from.is_none() || to.is_none() {
            return;
        }

        let from = from.unwrap();
        let to = to.unwrap();
        let new_amount = self.money.convert_money(value, &from, &to);
        if new_amount.is_none() {
            return;
        }
        let new_amount = new_amount.unwrap();
        self.money.add_money(&to, new_amount);
        self.money.subtract_money(&from, value);
    }

    pub fn add_max_hp(&mut self, value: i32) {
        if self.maximum_hit_points + value < 0 {
            self.maximum_hit_points = 0;
            return;
        }
        self.maximum_hit_points += value;
        if self.current_hit_points > self.maximum_hit_points {
            self.current_hit_points = self.maximum_hit_points;
        }
    }

    pub fn add_hit_dice(&mut self) {
        self.hit_dice_total.count += 1;
    }

    pub fn sub_hit_dice(&mut self) {
        if self.hit_dice_total.count <= 0 {
            return;
        }
        self.hit_dice_total.count -= 1;
    }

    pub fn add_item(&mut self, name: &str, description: &str, amount: i32) {
        let item = Item::new(name, description, amount);
        self.equipment.push(item);
    }

    
}
