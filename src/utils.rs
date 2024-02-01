use std::fs::File;

use serde_json::from_reader;
use strum::IntoEnumIterator;

use crate::{dnd_logic::prelude::*, CHARACTERS_PATH, SPELLS_PATH};

pub fn load_spells_from_files() -> SpellList {
    let path = std::path::Path::new(SPELLS_PATH);
    //let cantrips_json = File::open(path.join("cantrips.json")).unwrap();
    let cantrips_json = File::open(path.join("spell_level_0.json")).unwrap();
    
    let mut spell_database = SpellList::default();
    spell_database.cantrips = {
        let spells: Vec<Spell> = serde_json::from_reader(cantrips_json).unwrap();
        let spells = spells.into_iter()
        .map(|x| (x, false))
        .collect::<Vec<(Spell, bool)>>();
        spells
    };
    for i in 0..9 {
        //let name = format!("{}{}", i+1, "_lvl_spells.json");
        let name = format!("{}{}.json", "spell_level_", i+1);
        let file = File::open(path.join(name)).unwrap();
        spell_database.spells[i] = {
            let spells: Vec<Spell> = serde_json::from_reader(file).unwrap();
            let spells = spells.into_iter()
            .map(|x| (x, false))
            .collect::<Vec<(Spell, bool)>>();
            spells
        };
    }    spell_database
}

pub fn load_characters(spell_database: &SpellList) -> Vec<Character> {
    let path = std::path::Path::new(CHARACTERS_PATH);
    let mut characters = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file = File::open(entry.path()).unwrap();
        let mut character: Character = serde_json::from_reader(file).unwrap();
        character.load_spells(spell_database);
        characters.push(character);
    }
    characters
}