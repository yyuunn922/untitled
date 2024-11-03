use bevy::math::Vec3;
use bevy::prelude::*; // Bevy의 기본 요소들을 가져옵니다.

#[derive(Component)]
pub struct Camera {
    pub offset: Vec3, // pub으로 선언하여 외부에서 접근 가능하도록 설정
    pub zoom_speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0.0, 10.0, 10.0),
            zoom_speed: 1.0
        }
    }
}