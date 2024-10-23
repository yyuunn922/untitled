use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::prelude::ColliderMassProperties::Density;

fn main() {
    App::new().
        add_plugins(DefaultPlugins).
        add_plugins(RapierPhysicsPlugin::<NoUserData>::default()). // Rapier 물리 엔진 플러그인 추가
        add_plugins(RapierDebugRenderPlugin::default()). // 디버그 렌더 플러그인 추가
        add_systems(Startup, init_object).
        add_systems(Update, player_movement).
        add_systems(Update, camera_movement).
        add_systems(Update, landed_player_check).
        run();
}

// 플레이어 컴포넌트 정의
#[derive(Component)]
struct Player {
    is_jumping: bool,
    velocity: Vec3, // Y 방향 속도 추가
}

impl Default for Player {
    fn default() -> Self {
        Self { is_jumping: false, velocity: Vec3::ZERO, }
    }
}

// 카메라
#[derive(Component)]
struct Camera {
    offset: Vec3,
}

// 바닥
#[derive(Component)]
struct Ground;

// 장애물
#[derive(Component)]
struct Obstacle;

fn init_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 바닥 생성
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)), // 유지된 mesh 설정
            material: materials.add(Color::srgb_u8(255, 0, 0)), // 유지된 material 설정
            transform: Transform::from_xyz(0.0, 0.0, 0.0), // 바닥의 위치 조정
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.0, 10.0), // 바닥의 콜라이더 두께를 0.1로 설정
        Ground,
        ActiveEvents::COLLISION_EVENTS
    ));

    // 플레이어 생성
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)), // 유지된 mesh 설정
            transform: Transform::from_xyz(0.0, 1.0, 0.0), // 플레이어를 바닥보다 위에 위치시킴
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            }),
            ..default()
        },
        Player::default(),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5), // 플레이어의 콜라이더 크기
        Density(10.0),
        GravityScale(10.0),
    )).insert(ActiveEvents::COLLISION_EVENTS);

    // 장애물 생성
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)), // 유지된 mesh 설정
            transform: Transform::from_xyz(3.0, 1.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..default()
            }),
            ..default()
        },
        Obstacle,
        RigidBody::Dynamic, // 고정된 장애물로 설정
        GravityScale(10.0),
        Density(10.0),
        Collider::cuboid(0.5, 0.5, 0.5),
    )).insert(ActiveEvents::COLLISION_EVENTS);

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
        Camera {
            offset: Vec3::new(0.0, 10.0, 10.0),
        },
    ));
}

fn player_movement(
    mut param_set: ParamSet<(
        Query<(&mut Transform, &mut Player), With<Player>>,
        Query<&Transform, With<Camera>>, // 카메라의 Transform만 불변으로 가져옵니다.
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 10.0;
    let jump_strength = 20.0; // 점프의 힘
    let gravity = 0.0; // 중력 값

    if let Ok(camera_transform) = param_set.p1().get_single() {
        let forward: Vec3 = camera_transform.forward().into();
        let right: Vec3 = camera_transform.right().into();

        // Y 축 성분을 제거하여 수평 이동만 하도록 수정합니다.
        let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let right_flat = Vec3::new(right.x, 0.0, right.z).normalize();

        if let Ok((mut transform, mut player_state)) = param_set.p0().get_single_mut() {
            // 중력 적용
            if !player_state.is_jumping {
                // 바닥에 있을 때만 점프 가능
                if keyboard_input.pressed(KeyCode::Space) {
                    player_state.is_jumping = true;
                    player_state.velocity.y = jump_strength; // 점프 시작
                }
            }

            // 중점 (Y) 속도 업데이트
            if player_state.is_jumping {
                player_state.velocity.y += gravity * time.delta_seconds(); // 중력 적용

                // Y 위치 업데이트
                transform.translation.y += player_state.velocity.y * time.delta_seconds();

                // 바닥에 닿았는지 체크
                if transform.translation.y <= 0.0 { // 바닥 높이 체크 (0.0을 바닥으로 간주)
                    transform.translation.y = 0.0; // 바닥에 고정
                    player_state.is_jumping = false; // 점프 상태 리셋
                    player_state.velocity.y = 0.0; // Y 속도 리셋
                }
            }

            // 수평 이동
            let mut direction = Vec3::ZERO;
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction += forward_flat;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction -= forward_flat;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                direction -= right_flat;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                direction += right_flat;
            }
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}

fn camera_movement(
    mut param_set: ParamSet<(
        Query<(&mut Transform, &Player), With<Player>>,
        Query<(&mut Transform, &mut Camera), With<Camera>>,
    )>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut wheel_event: EventReader<MouseWheel>,
) {
    let zoom_speed = 0.5;

    // 마우스 중앙 버튼을 누른 경우, 카메라 회전을 처리합니다.
    if mouse_button.pressed(MouseButton::Middle) {
        for mouse_move_event in mouse_motion.read() {
            if let Ok((player_transform, _)) = param_set.p0().get_single() {
                let player_position = player_transform.translation;

                if let Ok((mut camera_transform, mut camera_follow)) = param_set.p1().get_single_mut() {
                    let rotation = Quat::from_rotation_y(-mouse_move_event.delta.x * 0.005)
                        * Quat::from_axis_angle(*camera_transform.local_x(), -mouse_move_event.delta.y * 0.005);

                    camera_follow.offset = rotation * camera_follow.offset;
                    camera_transform.translation = player_position + camera_follow.offset;
                    camera_transform.look_at(player_position, Vec3::Y);
                }
            }
        }
    }

    // 카메라의 위치를 플레이어 위치에 맞춰 수정합니다.
    if let Ok((player_transform, _)) = param_set.p0().get_single() {
        let player_position = player_transform.translation;
        if let Ok((mut camera_transform, camera_follow)) = param_set.p1().get_single_mut() {
            camera_transform.translation = player_position + camera_follow.offset;
            camera_transform.look_at(player_position, Vec3::Y);
        }
    }

    // 휠로 줌을 조정할 때, 오프셋을 Z축 방향으로만 조정합니다.
    for v in wheel_event.read() {
        if let Ok((_, mut camera_follow)) = param_set.p1().get_single_mut() {
            let zoom_delta = -v.y * zoom_speed;
            let offset_length = camera_follow.offset.length();

            let new_length = (offset_length + zoom_delta).clamp(5.0, 20.0);
            camera_follow.offset = camera_follow.offset.normalize() * new_length;
        }
    }
}


fn landed_player_check(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
    obstacle_query: Query<Entity, With<Obstacle>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    let obstacle_entity = obstacle_query.get_single().ok();
    let ground_entity = ground_query.get_single().ok();

    // 플레이어의 엔티티와 상태를 가변 참조로 가져옵니다.
    if let Ok((player, mut player_state)) = player_query.get_single_mut() {
        for event in collision_events.read() {
            match event {
                CollisionEvent::Started(entity1, entity2, _) => {
                    // 장애물 충돌
                    if let Some(obstacle) = obstacle_entity {
                        if (*entity1 == player && *entity2 == obstacle)
                            || (*entity1 == obstacle && *entity2 == player)
                        {
                            println!("플레이어가 장애물과 충돌했습니다!");
                        }
                    }

                    // 바닥 충돌
                    if let Some(ground) = ground_entity {
                        if (*entity1 == player && *entity2 == ground)
                            || (*entity1 == ground && *entity2 == player)
                        {
                            player_state.is_jumping = false;
                            println!("플레이어가 바닥에 닿았습니다. 점프 상태를 해제합니다.");
                        }
                    }
                }
                CollisionEvent::Stopped(entity1, entity2, _) => {
                    // 장애물 충돌 해제
                    if let Some(obstacle) = obstacle_entity {
                        if (*entity1 == player && *entity2 == obstacle)
                            || (*entity1 == obstacle && *entity2 == player)
                        {
                            println!("플레이어가 장애물과의 충돌에서 해제되었습니다.");
                        }
                    }

                    // 바닥 충돌 해제
                    if let Some(ground) = ground_entity {
                        if (*entity1 == player && *entity2 == ground)
                            || (*entity1 == ground && *entity2 == player)
                        {
                            player_state.is_jumping = true;
                            println!("플레이어가 바닥과의 접촉을 멈췄습니다. 점프 상태입니다.");
                        }
                    }
                }
            }
        }
    }
}



