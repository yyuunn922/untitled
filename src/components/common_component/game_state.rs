use bevy::prelude::States;

// 레벨 상태
#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum LevelState {
    MainMenu,
    MainVillage,
    Field,
}

// UI 상태
#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum UiState {
    None,
    MainMenu,
}
