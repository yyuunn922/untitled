use bevy::prelude::*;
use crate::client::components::common_component::game_state::{LevelState, UiState};
use crate::client::utils::i18n::I18n;

// 버튼 식별을 위한 컴포넌트
#[derive(Component)]
struct MainMenuClearItem;

#[derive(Component)]
struct MainMenuButton;

pub fn main_menu(app: &mut App) {
    app.add_systems(OnEnter(UiState::MainMenu), init)
        .add_systems(Update, button_system)
        .add_systems(OnExit(UiState::MainMenu), clear);
}

// 초기화 시스템
fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    i18n: Res<I18n>
) {
    let font = asset_server.load("fonts/nanum/NanumGothic-Regular.ttf");
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        }, MainMenuClearItem))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        ..default()
                    },
                    background_color: Color::srgb_u8(255, 255, 255).into(),
                    ..default()
                })
                .insert(MainMenuButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        i18n.get("게임시작"),
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::srgb_u8(0, 0, 0),
                        },
                    ));
                });
        });
}

fn clear(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuClearItem>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// 버튼 클릭 이벤트를 감지하여 상태를 변경하는 시스템
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut level_state: ResMut<NextState<LevelState>>,
    mut ui_state: ResMut<NextState<UiState>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // 상태 변경
                level_state.set(LevelState::MainVillage);
                ui_state.set(UiState::None);
                // 클릭 시 버튼 색상 변경
                *color = Color::rgb(0.7, 0.7, 0.7).into();
            }
            Interaction::Hovered => {
                // 마우스 오버 시 색상 변경
                *color = Color::rgb(0.9, 0.9, 0.9).into();
            }
            Interaction::None => {
                // 기본 색상
                *color = Color::rgb(1.0, 1.0, 1.0).into();
            }
        }
    }
}
