use bevy::prelude::*;
use crate::components::common_component::game_state::LevelState;

pub fn village(app: &mut App) {
    app.add_systems(OnEnter(LevelState::MainVillage), init)
        .add_systems(Update, update_test.run_if(in_state(LevelState::MainVillage)))
        .add_systems(OnExit(LevelState::MainVillage), clear);
}

fn init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle{
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)), // 유지된 mesh 설정
        material: materials.add(Color::srgb_u8(0, 0, 0)), // 유지된 material 설정
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // 바닥의 위치 조정
        ..default()
    });
}

fn update_test() {
    println!("updating test");
}

fn clear() {
    println!("clear");
}