// use bevy::asset::Assets;
// use bevy::color::Color;
// use bevy::math::Vec3;
// use bevy::pbr::{DirectionalLightBundle, PbrBundle, StandardMaterial};
// use bevy::prelude::{default, Camera3dBundle, Commands, Component, Cuboid, Mesh, Meshable, Plane3d, ResMut, Transform};
// use bevy_rapier3d::dynamics::{GravityScale, RigidBody};
// use bevy_rapier3d::geometry::{ActiveEvents, Collider};
// use bevy_rapier3d::geometry::ColliderMassProperties::Density;
// use crate::components::camera::Camera;
// use crate::components::player::Player;
//
// #[derive(Component)]
// struct Ground;
//
// // 장애물
// #[derive(Component)]
// struct Obstacle;
//
//
// fn init_object(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // 바닥 생성
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)), // 유지된 mesh 설정
//             material: materials.add(Color::srgb_u8(255, 0, 0)), // 유지된 material 설정
//             transform: Transform::from_xyz(0.0, 0.0, 0.0), // 바닥의 위치 조정
//             ..default()
//         },
//         RigidBody::Fixed,
//         Collider::cuboid(10.0, 0.0, 10.0), // 바닥의 콜라이더 두께를 0.1로 설정
//         Ground,
//         ActiveEvents::COLLISION_EVENTS
//     ));
//
//     // 플레이어 생성
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)), // 유지된 mesh 설정
//             transform: Transform::from_xyz(0.0, 1.0, 0.0), // 플레이어를 바닥보다 위에 위치시킴
//             material: materials.add(StandardMaterial {
//                 base_color: Color::WHITE,
//                 ..default()
//             }),
//             ..default()
//         },
//         Player::default(),
//         RigidBody::Dynamic,
//         Collider::cuboid(0.5, 0.5, 0.5), // 플레이어의 콜라이더 크기
//         Density(10.0),
//         GravityScale(10.0),
//     )).insert(ActiveEvents::COLLISION_EVENTS);
//
//     // 장애물 생성
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)), // 유지된 mesh 설정
//             transform: Transform::from_xyz(3.0, 1.0, 0.0),
//             material: materials.add(StandardMaterial {
//                 base_color: Color::BLACK,
//                 ..default()
//             }),
//             ..default()
//         },
//         Obstacle,
//         RigidBody::Dynamic, // 고정된 장애물로 설정
//         GravityScale(0.0),
//         Density(0.0),
//         Collider::cuboid(0.5, 0.5, 0.5),
//     )).insert(ActiveEvents::COLLISION_EVENTS);
//
//     // 빛 생성
//     commands.spawn(DirectionalLightBundle {
//         transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
//
//     // 카메라 생성
//     commands.spawn((
//         Camera3dBundle {
//             transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
//             ..default()
//         },
//         Camera::default(),
//     ));
//
// }
//
//
//
