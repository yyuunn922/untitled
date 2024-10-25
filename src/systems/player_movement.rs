// use bevy::input::ButtonInput;
// use bevy::math::Vec3;
// use bevy::prelude::{Entity, EventReader, KeyCode, ParamSet, Query, Res, Time, Transform, With};
// use bevy_rapier3d::pipeline::CollisionEvent;
// use crate::{Camera, Ground, Obstacle};
// use crate::components::player::Player;
//
// pub fn player_movement(
//     mut param_set: ParamSet<(
//         Query<(&mut Transform, &mut Player), With<Player>>,
//         Query<&Transform, With<Camera>>,
//     )>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     if let Ok(camera_transform) = param_set.p1().get_single() {
//         let forward: Vec3 = camera_transform.forward().into();
//         let right: Vec3 = camera_transform.right().into();
//
//         let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize();
//         let right_flat = Vec3::new(right.x, 0.0, right.z).normalize();
//
//         if let Ok((mut transform, mut player_state)) = param_set.p0().get_single_mut() {
//             // 중력 적용
//             if player_state.is_jumping {
//                 player_state.velocity.y += player_state.gravity * time.delta_seconds(); // 중력 적용
//                 transform.translation.y += player_state.velocity.y * time.delta_seconds(); // Y 위치 업데이트
//             }
//
//             // 점프 처리
//             if !player_state.is_jumping && keyboard_input.pressed(KeyCode::Space) {
//                 player_state.is_jumping = true;
//                 player_state.velocity.y = player_state.jump_strength; // 점프 시작
//             }
//
//             // 수평 이동
//             let mut direction = Vec3::ZERO;
//             if keyboard_input.pressed(KeyCode::KeyW) {
//                 direction += forward_flat;
//             }
//             if keyboard_input.pressed(KeyCode::KeyS) {
//                 direction -= forward_flat;
//             }
//             if keyboard_input.pressed(KeyCode::KeyA) {
//                 direction -= right_flat;
//             }
//             if keyboard_input.pressed(KeyCode::KeyD) {
//                 direction += right_flat;
//             }
//
//             // 수평 속도 업데이트
//             player_state.velocity.x = direction.x * player_state.speed; // X 방향 속도
//             player_state.velocity.z = direction.z * player_state.speed; // Z 방향 속도
//
//             // 위치 업데이트
//             transform.translation.x += player_state.velocity.x * time.delta_seconds(); // X 위치 업데이트
//             transform.translation.z += player_state.velocity.z * time.delta_seconds(); // Z 위치 업데이트
//         }
//     }
// }
//
//
//
//
// pub fn landed_player_check(
//     mut collision_events: EventReader<CollisionEvent>,
//     mut player_query: Query<(Entity, &mut Player), With<Player>>,
//     obstacle_query: Query<Entity, With<Obstacle>>,
//     ground_query: Query<Entity, With<Ground>>,
// ) {
//     let obstacle_entity = obstacle_query.get_single().ok();
//     let ground_entity = ground_query.get_single().ok();
//
//     // 플레이어의 엔티티와 상태를 가변 참조로 가져옵니다.
//     if let Ok((player, mut player_state)) = player_query.get_single_mut() {
//         for event in collision_events.read() {
//             match event {
//                 CollisionEvent::Started(entity1, entity2, _) => {
//                     // 장애물 충돌
//                     if let Some(obstacle) = obstacle_entity {
//                         if (*entity1 == player && *entity2 == obstacle)
//                             || (*entity1 == obstacle && *entity2 == player)
//                         {
//                             println!("플레이어가 장애물과 충돌했습니다!");
//                         }
//                     }
//
//                     // 바닥 충돌
//                     if let Some(ground) = ground_entity {
//                         if (*entity1 == player && *entity2 == ground)
//                             || (*entity1 == ground && *entity2 == player)
//                         {
//                             player_state.is_jumping = false;
//                             println!("플레이어가 바닥에 닿았습니다. 점프 상태를 해제합니다.");
//                         }
//                     }
//                 }
//                 CollisionEvent::Stopped(entity1, entity2, _) => {
//                     // 장애물 충돌 해제
//                     if let Some(obstacle) = obstacle_entity {
//                         if (*entity1 == player && *entity2 == obstacle)
//                             || (*entity1 == obstacle && *entity2 == player)
//                         {
//                             println!("플레이어가 장애물과의 충돌에서 해제되었습니다.");
//                         }
//                     }
//
//                     // 바닥 충돌 해제
//                     if let Some(ground) = ground_entity {
//                         if (*entity1 == player && *entity2 == ground)
//                             || (*entity1 == ground && *entity2 == player)
//                         {
//                             player_state.is_jumping = true;
//                             println!("플레이어가 바닥과의 접촉을 멈췄습니다. 점프 상태입니다.");
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
//
//
