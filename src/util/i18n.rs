use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;
use bevy::asset::ron::from_str;

// 언어 상태
#[derive(States)]
pub enum Language {
    Korean,
    English,
}

#[derive(Resource)]
pub struct I18n {
    data: HashMap<Language, HashMap<String, String>>,
    language: Language,
}

impl I18n {
    pub fn new() -> Self {
        let mut temp: HashMap<Language, HashMap<String, String>> = HashMap::new();
        for i in ["english", "korean"] {
            let path = format!("assets/i18n/{}.json", i);
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(map) = from_str::<HashMap<String, String>>(&content) {
                    match i {
                        "english" => temp.insert(Language::English, map),
                        _ => temp.insert(Language::Korean, map),
                    };
                } else {
                    println!("Failed to parse JSON: {}", path);
                }
            } else {
                println!("Failed to read file: {}", path);
            }
        }
        I18n{
            data: temp,
            language: Language::Korean,
        }
    }
}