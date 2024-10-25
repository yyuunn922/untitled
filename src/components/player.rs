use bevy::prelude::*; // Bevy의 기본 요소들을 가져옵니다.

#[derive(Component)] // Bevy 컴포넌트로 정의합니다.
pub struct Player {
    pub is_jumping: bool, // 점프 상태
    pub velocity: Vec3,   // 현재 속도 (X, Y, Z 방향)
    pub speed: f32,       // 이동 속도
    pub jump_strength: f32, // 점프의 힘
    pub gravity: f32,     // 중력 값
}

impl Default for Player {
    fn default() -> Self {
        Self {
            is_jumping: false,
            velocity: Vec3::ZERO,
            speed: 10.0,       // 기본 속도
            jump_strength: 20.0, // 기본 점프 힘
            gravity: -5.8,     // 기본 중력 값
        }
    }
}