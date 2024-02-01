use std::{
    cell::RefCell, env::{self, current_exe}, rc::Rc, thread::current
};

mod dnd_logic;
mod utils;
use crate::dnd_logic::prelude::*;
use dnd_logic::{character, skill};
use slint::{Color, Model, ModelRc};
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
    pub current_character: usize, // index of current character in characters
    pub first_frame: bool,
    pub spell_database: SpellList,
}

impl StatTracker {
    pub fn new() -> Self {
        let spell_database = load_spells_from_files();
        let characters = load_characters(&spell_database);
        Self {
            state: AppState::CharacterSelect,
            previous_state: AppState::CharacterSelect,
            characters,
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

pub fn set_ui_character_data(c: &Character, ui: &AppWindow) {
    let mut current_character: SlintCharacter = ui.get_character();
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

    current_character.stats = ModelRc::from(stats.as_slice());
    current_character.skills = ModelRc::from(skills.as_slice());

    current_character.armor_class = c.get_total_armor_class();
    current_character.initiative = c.get_ui_total_initiative().into();
    current_character.speed = c.speed;

    current_character.current_hp = c.get_hit_points_current();
    current_character.max_hp = c.get_hit_points_max();
    current_character.temp_hp = c.get_hit_points_temp();

    current_character.hit_dice_total = c.get_ui_hit_dice_total().into();
    current_character.hit_dice_left = c.get_ui_hit_dice_left().into();

    current_character.money = c.money.to_slint_money();

    current_character.death_saves_successes = c.death_saves.successes;
    current_character.death_saves_failures = c.death_saves.failures;

    ui.set_character(current_character.into());
}

fn main() -> Result<(), slint::PlatformError> {
    let app_data = Rc::new(RefCell::new(StatTracker::new()));
    env::set_var("SLINT_BACKEND", "skia");

    let ui = AppWindow::new()?;

    let mut c = app_data.clone();
    set_ui_character_data(&c.borrow_mut().get_current_character(), &ui);

    ui.on_add_money({
        let ui_handle = ui.as_weak();
        move |name, value| {
            let ui = ui_handle.unwrap();
            let value = value.parse::<i32>().unwrap();
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
            let value = value.parse::<i32>().unwrap();
            let mut c = app_data_handle.borrow_mut();
            c.get_current_character().subtract_money(&name, value);
            set_ui_character_data(&c.get_current_character(), &ui);
        }
    });

    ui.run()
}
