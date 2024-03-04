use std::{fs::{self, File}, path::{Path, PathBuf}};

use serde_json::from_reader;
use slint::Image;
use strum::IntoEnumIterator;

use crate::{dnd_logic::prelude::*, CHARACTERS_PATH, CHARACTER_IMAGE_PATH, SPELLS_PATH};

pub fn load_spells_from_files() -> SpellList {
    let path = std::path::Path::new(SPELLS_PATH);
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

pub fn load_character_images(characters: Vec<Character>) -> Vec<(String, Image)> {
    let mut images = Vec::new();

    let mut file_names: Vec<String> = Vec::new();

    let dir = fs::read_dir(CHARACTER_IMAGE_PATH).unwrap();

    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        file_names.push(file_name);
    }

    for character in characters {
        let name_no_spaces = character.name.replace(" ", "_").to_lowercase();

        file_names.iter().for_each(|file_name| {
            let name_jpg = format!("{}.jpg", name_no_spaces.clone());
            let name_png = format!("{}.png", name_no_spaces.clone());
            if file_name == &name_jpg {
                let path = format!("{}/{}", CHARACTER_IMAGE_PATH, name_jpg);
                let image = Image::load_from_path(Path::new(&path)).unwrap();
                images.push((name_no_spaces.clone(), image));
            } else if file_name == &name_png {
                let path = format!("{}/{}", CHARACTER_IMAGE_PATH, name_png);
                let image = Image::load_from_path(Path::new(&path)).unwrap();
                images.push((name_no_spaces.clone(), image));
            }
        });
    }
    images
}