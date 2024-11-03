// use bevy::math::Vec3;
// use bevy::prelude::{ParamSet, Query, Transform, With};
// use crate::components::camera::Camera;
// use crate::components::player::Player;
//
// // player에 맞춰 움직이는 카메라
// pub fn camera_movement(
//     mut param_set: ParamSet<(
//         Query<(&mut Transform, &mut Player), With<Player>>,
//         Query<(&mut Transform, &mut Camera), With<Camera>>
//     )>
// ) {
//     // 카메라의 위치를 플레이어 위치에 맞춰 수정합니다.
//     if let Ok((player_transform, _)) = param_set.p0().get_single() {
//         let player_position = player_transform.translation;
//
//         if let Ok((mut camera_transform, camera_follow)) = param_set.p1().get_single_mut() {
//             camera_transform.translation = player_position + camera_follow.offset; // 카메라 위치 업데이트
//             camera_transform.look_at(player_position, Vec3::Y); // 카메라가 플레이어를 바라보도록 설정
//         }
//     }
// }