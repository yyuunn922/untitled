use crate::client::components::common_component::game_state::LevelState;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;

pub fn village(app: &mut App) {
    app.add_systems(OnEnter(LevelState::MainVillage), init)
        .add_systems(Update, update_test.run_if(in_state(LevelState::MainVillage)))
        .add_systems(OnExit(LevelState::MainVillage), clear);
}

fn init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    // Perlin 노이즈 생성기

    commands.spawn(SceneBundle {
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/untitled.gltf")),
        ..default()
    });
    //
    // commands.spawn(PbrBundle{
    //     mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)), // 유지된 mesh 설정
    //     material: materials.add(Color::srgb_u8(0, 0, 0)), // 유지된 material 설정
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0), // 바닥의 위치 조정
    //     ..default()
    // });
}

fn update_test() {
    // println!("updating test");
}

fn clear() {
    println!("clear");
}
