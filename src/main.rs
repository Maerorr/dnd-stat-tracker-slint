#![windows_subsystem = "windows"]

use std::{
    cell::RefCell, env::{self, current_exe}, path::Path, rc::Rc, thread::current
};

mod dnd_logic;
mod utils;
use crate::dnd_logic::prelude::*;
use dnd_logic::{character, skill};
use slint::{Color, Image, Model, ModelRc};
use utils::*;

slint::include_modules!();

pub const STRENGTH_COLOR: Color = Color::from_rgb_u8(193, 96, 77);
pub const DEXTERITY_COLOR: Color = Color::from_rgb_u8(84, 222, 178);
pub const CONSTITUTION_COLOR: Color = Color::from_rgb_u8(236, 207, 73);
pub const INTELLIGENCE_COLOR: Color = Color::from_rgb_u8(140, 196, 123);
pub const WISDOM_COLOR: Color = Color::from_rgb_u8(171, 98, 156);
pub const CHARISMA_COLOR: Color = Color::from_rgb_u8(233, 219, 204);

pub const SPELLS_PATH: &str = "res/spells";
pub const CHARACTERS_PATH: &str = "res/characters";
pub const DEFAULT_CHARACTER_IMAGE_PATH: &str = "res/images/characters/default-character-image.png";
pub const CHARACTER_IMAGE_PATH: &str = "res/images/characters/";

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    CharacterSelect,
    CharacterCreate,
    StatTracker,
    StatTrackerEdit,
}

pub struct StatTracker {
    pub state: AppState,
    pub previous_state: AppState,
    pub characters: Vec<Character>,
    pub character_images: Vec<(String, Image)>,
    pub current_character: usize, // index of current character in characters
    pub first_frame: bool,
    pub spell_database: SpellList,
}

impl StatTracker {
    pub fn new() -> Self {
        let spell_database = load_spells_from_files();
        let characters = load_characters(&spell_database);
        let images = load_character_images(characters.clone());
        Self {
            state: AppState::CharacterSelect,
            previous_state: AppState::CharacterSelect,
            characters,
            character_images: images,
            current_character: 0,
            first_frame: true,
            spell_database,
        }
    }

    pub fn get_current_character(&mut self) -> &mut Character {
        &mut self.characters[self.current_character]
    }
}

pub fn set_stat_ui_data(c: &Character, stats: &mut Vec<SlintStat>) {
    stats[0] = c.stats.strength.get_slint_stat(c);
    stats[1] = c.stats.dexterity.get_slint_stat(c);
    stats[2] = c.stats.constitution.get_slint_stat(c);
    stats[3] = c.stats.intelligence.get_slint_stat(c);
    stats[4] = c.stats.wisdom.get_slint_stat(c);
    stats[5] = c.stats.charisma.get_slint_stat(c);
}

pub fn set_skills_ui_data(c: &Character, skills: &mut Vec<SlintSkill>) {
    *skills = c.skills.get_slint_skills(c);
}

pub fn set_all_characters_data(characters: &Vec<Character>, images: &Vec<(String, Image)>, ui: &AppWindow) {
    let mut all_characters: Vec<SlintCharacterShortInfo> = Vec::with_capacity(characters.len());
    
    for character in characters.iter() {
        let mut current_character: SlintCharacterShortInfo = SlintCharacterShortInfo::default();
        current_character.name = character.name.clone().into();
        current_character.level = character.level;
        current_character.class = character.class.get_name().into();
        current_character.exp = character.experience;
        current_character.image = {
            let image = images.iter().find(|(name, _)| 
                name == &character.name.replace(" ", "_").to_lowercase()
            );
            if image.is_none() {
                Image::load_from_path(Path::new(DEFAULT_CHARACTER_IMAGE_PATH)).unwrap()
            } else {
                image.unwrap().1.clone()
            }
        };

        all_characters.push(current_character);
    }

    ui.set_all_characters(all_characters.as_slice().into());
}

pub fn set_ui_character_data(c: &Character, ui: &AppWindow) {
    let mut current_character: SlintCharacter = ui.get_current_character();
    current_character.name = c.name.clone().into();
    current_character.level = c.level;
    current_character.class = c.class.get_name().into();
    current_character.exp = c.experience;
    current_character.proficiency_bonus = c.get_ui_proficiency_bonus().into();
    let mut stats: Vec<SlintStat> = Vec::with_capacity(6);
    stats.resize(6, SlintStat::default());
    set_stat_ui_data(&c, &mut stats);

    let mut skills: Vec<SlintSkill> = Vec::with_capacity(18);
    skills.resize(18, SlintSkill::default());
    set_skills_ui_data(&c, &mut skills);

    current_character.stats = stats.as_slice().into();//ModelRc::from(stats.as_slice());
    current_character.skills = skills.as_slice().into();//ModelRc::from(skills.as_slice());

    current_character.armor_class = c.get_total_armor_class();
    current_character.initiative = c.get_ui_total_initiative().into();
    current_character.initiative_no_dex =c.get_ui_initiative_no_dex().into();
    current_character.speed = c.speed;

    current_character.current_hp = c.get_hit_points_current();
    current_character.max_hp = c.get_hit_points_max();
    current_character.temp_hp = c.get_hit_points_temp();

    current_character.hit_dice_total = c.get_ui_hit_dice_total().into();
    current_character.hit_dice_left = c.get_ui_hit_dice_left().into();

    current_character.money = c.money.to_slint_money();

    current_character.death_saves_successes = c.death_saves.successes;
    current_character.death_saves_failures = c.death_saves.failures;

    current_character.features_traits = c.features_and_traits.clone().into();

    let spells = c.spell_list.get_ui_spell_entries();
    current_character.cantrips = spells[0].as_slice().into();
    current_character.spells1 = spells[1].as_slice().into();
    current_character.spells2 = spells[2].as_slice().into();
    current_character.spells3 = spells[3].as_slice().into();
    current_character.spells4 = spells[4].as_slice().into();
    current_character.spells5 = spells[5].as_slice().into();
    current_character.spells6 = spells[6].as_slice().into();
    current_character.spells7 = spells[7].as_slice().into();
    current_character.spells8 = spells[8].as_slice().into();
    current_character.spells9 = spells[9].as_slice().into();

    current_character.spell_slots = c.get_ui_spell_slots().as_slice().into();

    current_character.is_spellcaster = if c.class.get_spellcasting_ability().is_none() {
        false
    } else {
        true
    };  
    current_character.spellcasting_ability = if c.class.get_spellcasting_ability().is_none() {
        String::from("None").into()
    } else {
        c.class.get_spellcasting_ability().unwrap().get_short_name().into()
    };
    current_character.spell_save_dc = c.get_ui_spell_save_dc().into();
    current_character.spell_attack_bonus = c.get_ui_spell_attack_bonus().into();

    current_character.languages_proficiencies = c.proficiencies_and_languages.clone().into();

    current_character.equipment = c.equipment.iter().map(|item| item.to_slint_item()).collect::<Vec<SlintItem>>().as_slice().into();

    let mut weapons: Vec<SlintWeaponInfo> = Vec::new();
    for weapon in c.weapons.iter() {
        let mut slint_weapon = weapon.0.to_slint_weapon();
        slint_weapon.equipped = weapon.1;
        weapons.push(slint_weapon);
    }

    current_character.weapons = weapons.as_slice().into();

    ui.set_current_character(current_character.into());
}

fn set_ui_spell_database_data(spell_database: &SpellList, ui: &AppWindow) {
    // convert each spell into SlintSpellData containing name and level, then collect them into SlintSpellDatabase that has lists of spells for each level
    let mut spell_database_ui: SlintSpellDatabase = SlintSpellDatabase::default();
    spell_database_ui.cantrips = spell_database.get_spells_of_level(0).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();
    spell_database_ui.spells1 = spell_database.get_spells_of_level(1).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();
    spell_database_ui.spells2 = spell_database.get_spells_of_level(2).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();
    spell_database_ui.spells3 = spell_database.get_spells_of_level(3).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();
    spell_database_ui.spells4 = spell_database.get_spells_of_level(4).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();
    spell_database_ui.spells5 = spell_database.get_spells_of_level(5).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();
    spell_database_ui.spells6 = spell_database.get_spells_of_level(6).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();
    spell_database_ui.spells7 = spell_database.get_spells_of_level(7).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();
    spell_database_ui.spells8 = spell_database.get_spells_of_level(8).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();
    spell_database_ui.spells9 = spell_database.get_spells_of_level(9).iter().map(|spell| spell.0.name.clone().into()).collect::<Vec<slint::SharedString>>().as_slice().into();

    ui.set_spell_database(spell_database_ui.into());
}

fn main() {
    let app_data = Rc::new(RefCell::new(StatTracker::new()));
    env::set_var("SLINT_BACKEND", "skia");

    let ui = AppWindow::new().unwrap();

    let mut c = app_data.clone();
    
    let mut state: SlintState = ui.get_state();
    state = SlintState::CharacterSelect;
    ui.set_state(state);

    set_all_characters_data(&c.borrow().characters, &c.borrow().character_images, &ui);
    set_ui_character_data(&c.borrow_mut().get_current_character(), &ui);
    set_ui_spell_database_data(&c.borrow().spell_database, &ui);

    ui.on_add_money({
        let ui_handle = ui.as_weak();
        move |name, value| {
            let ui = ui_handle.unwrap();
            print!("value: {}", value);
            let value = value.trim().parse::<i32>();
            if value.is_err() {
                return;
            }
            let value = value.unwrap();
            let mut c = c.borrow_mut();
            c.get_current_character().add_money(&name, value);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_subtract_money({
        let ui_handle = ui.as_weak();
        move |name, value| {
            let ui = ui_handle.unwrap();
            let value = value.trim().parse::<i32>();
            if value.is_err() {
                return;
            }
            let value = value.unwrap();

            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().subtract_money(&name, value);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_take_damage({
        let ui_handle = ui.as_weak();
        move |damage| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let val = damage.trim().parse::<i32>();
            if val.is_err() {
                return;
            }
            let val = val.unwrap();
            c.get_current_character().take_damage(val);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_heal({
        let ui_handle = ui.as_weak();
        move |heal| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let val = heal.trim().parse::<i32>();
            if val.is_err() {
                return;
            }
            let val = val.unwrap();
            c.get_current_character().heal_damage(val);
            
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_temp_hp({
        let ui_handle = ui.as_weak();
        move |temp_hp| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let val = temp_hp.parse::<i32>().unwrap();
            c.get_current_character().add_temporary_hit_points(val);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_subtract_temp_hp({
        let ui_handle = ui.as_weak();
        move |temp_hp| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let val = temp_hp.parse::<i32>().unwrap();
            c.get_current_character().subtract_temporary_hit_points(val);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_use_hit_dice({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().use_hit_dice();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_death_save_success({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().add_success_death_save();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_death_save_failure({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().add_fail_death_save();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_reset_death_saves({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().reset_death_saves();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_convert_money({
        let ui_handle = ui.as_weak();
        move |from, to, amount| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let from = String::from(from);
            let to = String::from(to);
            let amount = amount.parse::<i32>();
            if amount.is_err() {
                return;
            }
            let amount = amount.unwrap();
            c.get_current_character().convert_money(&from, &to, amount);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_use_spell_slot({
        let ui_handle = ui.as_weak();
        move |level| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let level = level.trim().parse::<i32>().unwrap();
            c.get_current_character().add_one_spell_slot_used(level);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_spell_slot({
        let ui_handle = ui.as_weak();
        move |level| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let level = level.trim().parse::<i32>().unwrap();
            c.get_current_character().subtract_one_spell_slot_used(level);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    ui.on_add_max_spell_slot({
            let app_data_handle = app_data.clone();
            let ui_handle = ui.as_weak();
            move |level| {
                let ui = ui_handle.unwrap();
                let mut c = app_data_handle.borrow_mut();
                let level = level.trim().parse::<i32>().unwrap();
                c.get_current_character().add_one_spell_slot_max(level);
                set_ui_character_data(&c.get_current_character(), &ui);
            }
        }
    );

    ui.on_subtract_max_spell_slot({
            let app_data_handle = app_data.clone();
            let ui_handle = ui.as_weak();
            move |level| {
                let ui = ui_handle.unwrap();
                let mut c = app_data_handle.borrow_mut();
                let level = level.trim().parse::<i32>().unwrap();
                c.get_current_character().subtract_one_spell_slot_max(level);
                set_ui_character_data(&c.get_current_character(), &ui);
            }
        }
    );

    let app_data_handle = app_data.clone();
    ui.on_edit_stat({
        let ui_handle = ui.as_weak();
        move |stat, value| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            if value == "+" {
                c.get_current_character().stats.get_stat_mut(StatType::from_string(&stat).unwrap()).add_one();
            } else {
                c.get_current_character().stats.get_stat_mut(StatType::from_string(&stat).unwrap()).subtract_one();
            }
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_save({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let character = c.get_current_character();
            character.save_to_file();
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_save_character_select({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let character = c.get_current_character();
            character.save_to_file();
            let mut state: SlintState = ui.get_state();
            state = SlintState::CharacterSelect;
            ui.set_state(state);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_set_skill_proficiency({
        let ui_handle = ui.as_weak();
        move |skill, prof| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let skill = SkillType::from_string(&skill).unwrap();
            c.get_current_character().skills.set_skill_proficiency(skill, prof);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_set_skill_expertise({
        let ui_handle = ui.as_weak();
        move |skill, exp| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let skill = SkillType::from_string(&skill).unwrap();
            c.get_current_character().skills.set_skill_expertise(skill, exp);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_set_save_proficiency({
        let ui_handle = ui.as_weak();
        move |stat, prof| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let stat = StatType::from_string(&stat).unwrap();
            c.get_current_character().stats.set_save_proficiency(stat, prof);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_set_languages_proficiencies({
        let ui_handle = ui.as_weak();
        move |lang_prof| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().proficiencies_and_languages = lang_prof.into();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_ac({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().add_one_ac();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_subtract_ac({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().subtract_one_ac();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_initiative({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().add_one_initiative();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_subtract_initiative({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().subtract_one_initiative();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_speed({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().add_5_speed();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_subtract_speed({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().subtract_5_speed();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_max_hp({
        let ui_handle = ui.as_weak();
        move |hp| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let hp = hp.trim().parse::<i32>().unwrap();
            c.get_current_character().add_max_hp(hp);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });
    
    let app_data_handle = app_data.clone();
    ui.on_add_hit_dice({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().add_hit_dice();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_subtract_hit_dice({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().sub_hit_dice();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_hit_dice_left({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().restore_one_hit_dice();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_subtract_hit_dice_left({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().use_hit_dice();
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    let app_data_handle = app_data.clone();
    ui.on_add_spell({
        let ui_handle = ui.as_weak();
        move |name| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let spell = c.spell_database.get_spell_by_name(&name);
            if spell.is_none() {
                return;
            }
            c.get_current_character().spell_list.add_spell(&spell.unwrap(), false);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    ui.on_remove_spell({
        let app_data_handle = app_data.clone();
        let ui_handle = ui.as_weak();
        move |name| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let spell = c.get_current_character().spell_list.get_spell_by_name(&name);
            if spell.is_none() {
                return;
            }
            c.get_current_character().spell_list.remove_spell_by_name(&name);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    ui.on_add_item({
        let app_data_handle = app_data.clone();
        let ui_handle = ui.as_weak();
        move |name, description, value| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let value = value.trim().parse::<i32>();
            
            if value.is_err() {
                return;
            }
            
            let value = value.unwrap();
            c.get_current_character().add_item(name.as_str(), description.as_str(), value);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    ui.on_item_amount_changed({
        let app_data_handle = app_data.clone();
        let ui_handle = ui.as_weak();
        move |name, amount| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let amount = amount.trim().parse::<i32>();
            if amount.is_err() {
                return;
            }
            let amount = amount.unwrap();
            c.get_current_character().change_item_amount(name.as_str(), amount);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    ui.on_remove_item({
        let app_data_handle = app_data.clone();
        let ui_handle = ui.as_weak();
        move |name| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().remove_item(name.as_str());
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    ui.on_character_select({
        let app_data_handle = app_data.clone();
        let ui_handle = ui.as_weak();
        move |name| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let name = name.as_str();
            let index = c.characters.iter().position(|character| character.name == name);
            if index.is_none() {
                return;
            }
            c.current_character = index.unwrap();
            set_ui_character_data(&c.get_current_character(), &ui);
            let mut state: SlintState = ui.get_state();
            state = SlintState::StatTracker;
            ui.set_state(state);
        }
    });

    ui.on_set_prepared_spell({
        let app_data_handle = app_data.clone();
        let ui_handle = ui.as_weak();
        move |name, prepared| {
            let ui = ui_handle.unwrap();
            let mut c = app_data_handle.borrow_mut();
            let spell = c.get_current_character().spell_list.get_spell_by_name(&name);
            if spell.is_none() {
                return;
            }
            c.get_current_character().spell_list.set_prepared(name.to_string(), prepared);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    ui.run();

    // dont close console after program
    // let _ = std::process::Command::new("cmd")
    //     .arg("/C")
    //     .arg("pause")
    //     .status();
}