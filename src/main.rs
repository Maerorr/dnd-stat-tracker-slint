use std::env::{self, current_exe};

mod utils;
mod dnd_logic;
use crate::dnd_logic::prelude::*;
use dnd_logic::skill;
use slint::{Color, Model, ModelRc};
use utils::*;

slint::include_modules!();

// out property <color> dark: #141414;
// out property <color> background_color: #2a2b31;
// out property <color> button_idle_color: #2a2b31;
// out property <color> button_hover_color: #6b3d3d;
// out property <color> button_pressed_color: #851a1a;
// out property <color> main_color: #969696;
// out property <color> text_color: #b3b3b6;

// out property <color> copper_color: #b87333;
// out property <color> silver_color: #C0C0C0;
// out property <color> electrum_color: #8FA8B3;
// out property <color> gold_color: #FFD700;
// out property <color> platinum_color: #E5E4E2;

// out property <color> current_hp_color: #DE7879;
// out property <color> temp_hp_color: #A8DDF0;

// out property <color> strength_color: #c1604d;
// out property <color> dexterity_color: #54DEB2;
// out property <color> constitution_color: #ECCF49;
// out property <color> intelligence_color: #8CC47B;
// out property <color> wisdom_color: #AB629C;
// out property <color> charisma_color: #E9DBCC;
// rewrite these as const values

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

    pub fn set_stat_ui_data(&self, c: &Character, stats: &mut Vec<SlintStat>) {
        stats[0] = c.stats.strength.get_slint_stat(c);
        stats[1] = c.stats.dexterity.get_slint_stat(c);
        stats[2] = c.stats.constitution.get_slint_stat(c);
        stats[3] = c.stats.intelligence.get_slint_stat(c);
        stats[4] = c.stats.wisdom.get_slint_stat(c);
        stats[5] = c.stats.charisma.get_slint_stat(c);
    }

    pub fn set_skills_ui_data(&self, c: &Character, skills: &mut Vec<SlintSkill>) {
        *skills = c.skills.get_slint_skills(c);
    }

    pub fn set_ui_character_data(&self, ui: &AppWindow) {
        let c = self.characters[self.current_character].clone();
        let mut current_character: SlintCharacter = ui.get_character();
        current_character.name = c.name.clone().into();
        current_character.level = c.level;
        current_character.class = c.class.get_name().into();
        current_character.exp = c.experience;
        current_character.proficiency_bonus = c.get_ui_proficiency_bonus().into();
        let mut stats: Vec<SlintStat> = Vec::with_capacity(6);
        stats.resize(6, SlintStat::default());
        self.set_stat_ui_data(&c, &mut stats);

        let mut skills: Vec<SlintSkill> = Vec::with_capacity(18);
        skills.resize(18, SlintSkill::default());
        self.set_skills_ui_data(&c, &mut skills);

        current_character.stats = ModelRc::from(stats.as_slice());
        current_character.skills = ModelRc::from(skills.as_slice());
        ui.set_character(current_character.into());
    }
}

fn main() -> Result<(), slint::PlatformError> {
    
    let mut app_data = StatTracker::new();
    env::set_var("SLINT_BACKEND", "skia");

    let ui = AppWindow::new()?;

    app_data.set_ui_character_data(&ui);
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run()
}
