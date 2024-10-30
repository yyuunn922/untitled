use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;
use bevy::asset::ron::from_str;

// 언어 상태
#[derive(Debug, Clone, States, Eq, PartialEq, Hash)]
pub enum Language {
    Korean,
    English,
}

#[derive(Resource)]
pub struct I18n {
    data: HashMap<Language, HashMap<String, String>>,
    language: Language,
}

// TODO:: 필요없는 코드 살펴보기
// main.rs에 .insert_resource(I18n::new())로 해당코드 의미없음
// 다시 살펴봐야함
impl Default for I18n {
    fn default() -> Self {
        I18n{data: HashMap::new(), language: Language::Korean}
    }
}

impl I18n {
    pub fn new() -> Self {
        let mut temp: HashMap<Language, HashMap<String, String>> = HashMap::new();
        for i in ["english"] {
            let path = format!("assets/i18n/{}.json", i);
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(map) = from_str::<HashMap<String, String>>(&content) {
                    match i {
                        "english" => temp.insert(Language::English, map),
                        _ => temp.insert(Language::Korean, map)
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

    pub fn get(&self, key: &str) -> String {
        if self.language == Language::Korean {
            return key.to_string();
        }

        self.data
            .get(&self.language)
            .and_then(|map| map.get(key).cloned()).unwrap_or(key.to_string())
    }
}