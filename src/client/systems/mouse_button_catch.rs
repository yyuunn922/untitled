// use bevy::input::mouse::{MouseMotion, MouseWheel};
// use bevy::math::{Quat, Vec3};
// use bevy::prelude::{ButtonInput, EventReader, MouseButton, ParamSet, Query, Res, Transform, With};
// use crate::components::camera::Camera;
// use crate::components::player::Player;
//
// pub fn mouse_button_catch(
//     mut param_set: ParamSet<(
//         Query<(&mut Transform, &mut Player), With<Player>>,
//         Query<(&mut Transform, &mut Camera), With<Camera>>
//     )>,
//     mouse_button_event: Res<ButtonInput<MouseButton>>,
//     mut wheel_event: EventReader<MouseWheel>,
//     mut mouse_motion_event: EventReader<MouseMotion>,
// ) {
//     // 마우스 중앙 버튼을 누른 경우, 카메라 회전을 처리합니다.
//     if mouse_button_event.pressed(MouseButton::Middle) {
//         for mouse_move_event in mouse_motion_event.read() {
//             if let Ok((player_transform, _)) = param_set.p0().get_single() {
//                 let player_position = player_transform.translation;
//
//                 if let Ok((mut camera_transform, mut camera_follow)) = param_set.p1().get_single_mut() {
//                     let rotation = Quat::from_rotation_y(-mouse_move_event.delta.x * 0.005)
//                         * Quat::from_axis_angle(*camera_transform.local_x(), -mouse_move_event.delta.y * 0.005);
//
//                     camera_follow.offset = rotation * camera_follow.offset;
//                     camera_transform.translation = player_position + camera_follow.offset;
//                     camera_transform.look_at(player_position, Vec3::Y);
//                 }
//             }
//         }
//     }
//
//     // 휠로 줌을 조정할 때, 오프셋을 Z축 방향으로만 조정합니다.
//     for v in wheel_event.read() {
//         if let Ok((_, mut camera_follow)) = param_set.p1().get_single_mut() {
//             let zoom_delta = -v.y * camera_follow.zoom_speed;
//             let offset_length = camera_follow.offset.length();
//
//             let new_length = (offset_length + zoom_delta).clamp(5.0, 20.0);
//             camera_follow.offset = camera_follow.offset.normalize() * new_length;
//         }
//     }
// }