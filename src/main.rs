mod components;
mod systems;
mod scenes;
mod ui;
mod util;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use components::common_component::game_state::LevelState;
use components::common_component::camera;
use scenes::village::village;
use scenes::field::field;
use scenes::main_menu::main_menu;
use ui::main_menu::main_menu as ui_main_menu;
use components::common_component::game_state::UiState;
use crate::util::i18n::{I18n, Language};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default()) // Rapier 물리 엔진 플러그인 추가
        .add_plugins(RapierDebugRenderPlugin::default()) // 디버그 렌더 플러그인 추가
        .insert_resource(I18n::new())
        .add_systems(Startup, init_setup) // 초기 기본 카메라등록
        .insert_state(LevelState::MainMenu)// 초기 레벨 상태
        .insert_state(UiState::MainMenu) // 초기 UI 상태
        .insert_state(Language::Korean) // 초기 언어 상태
        .add_level_extends() // 레벨 변경 시스템
        .add_ui_extends() // UI 변경 시스템
        .run();
}

// 맵별 시스템을 호출하여 등록하는 함수
trait ExtendSystem {
    fn add_level_extends(&mut self) -> &mut Self; // 레벨 변경 시스템
    fn add_ui_extends(&mut self) -> &mut Self; // UI 변경 시스템
}

// 시스템 구현
impl ExtendSystem for App {
    // 레벨 등록
    fn add_level_extends(&mut self) -> &mut Self {
        main_menu(self);
        village(self);
        field(self);
        self
    }

    // UI
    fn add_ui_extends(&mut self) -> &mut Self {
        ui_main_menu(self);
        self
    }
}

// 기본 카메라 및 빛 추가
fn init_setup(mut commands: Commands) {
    // 빛 생성
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 카메라 생성
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        camera::Camera::default(),
    ));
}