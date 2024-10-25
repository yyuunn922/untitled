use bevy::prelude::*;
use crate::components::common_component::game_state::LevelState;

pub fn main_menu(app: &mut App) {
    app.add_systems(OnEnter(LevelState::MainMenu), init)
        .add_systems(Update, update_test.run_if(in_state(LevelState::MainMenu)))
        .add_systems(OnExit(LevelState::MainMenu), clear);

}

// 초기화 시스템
fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
        material: materials.add(Color::srgb_u8(255, 0, 0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

// 업데이트 시스템
fn update_test() {
    println!("테스트고고");
}

// 클린 시스템
fn clear() {
    println!("clear")
}
