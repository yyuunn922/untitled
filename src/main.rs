use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new().
        add_plugins(DefaultPlugins).
        add_plugins(RapierPhysicsPlugin::<NoUserData>::default()). // Rapier 물리 엔진 플러그인 추가
        add_plugins(RapierDebugRenderPlugin::default()). // 디버그 렌더 플러그인 추가
        add_systems(Startup, init_object).
        add_systems(Update, (player_movement, camera_movement)).
        add_systems(PostUpdate, landed_player_check).
        run();
}

// 플레이어 컴포넌트 정의
#[derive(Component)]
struct Player {
    is_jumping: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {is_jumping: false}
    }
}

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct CameraFollow {
    offset: Vec3,    // 카메라와 플레이어 사이의 기본 오프셋
}

#[derive(Component)]
struct Ground;


fn init_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 바닥
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::srgb_u8(255, 0, 0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0), // 바닥의 위치 조정
            ..default()
        },
        Collider::cuboid(10.0, 0.0, 10.0), // 바닥의 콜라이더 두께를 0.1로 설정
        Ground,
    ));


    // 플레이어
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
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
        GravityScale(10.0),
    ));

    // 빛
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });


    // 카메라
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Camera,
        CameraFollow {
            offset: Vec3::new(0.0, 10.0, 10.0), // 기본 오프셋
        },
    ));
}


fn player_movement(
    mut query: Query<(&mut Transform, &mut Player), With<Player>>, // 가변 쿼리
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>, // Delta time 리소스
) {
    let speed = 10.0;

    // 쿼리된 엔티티들의 Transform 반복 처리
    if let Ok((mut transform, mut player_state)) = query.get_single_mut() {
        if !player_state.is_jumping {
            let mut direction = Vec3::ZERO;
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction.z -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction.z += 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                direction.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                direction.x += 1.0;
            }
            if keyboard_input.pressed(KeyCode::Space) {
                player_state.is_jumping = true;
                direction.y += 10.0;
            }
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}


fn camera_movement(
    mut param_set: ParamSet<(
        Query<(&mut Transform, &mut Player), With<Player>>,  // 플레이어 쿼리
        Query<(&mut Transform, &mut CameraFollow), With<Camera>>,  // 카메라 쿼리
    )>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut wheel_event: EventReader<MouseWheel>,
) {
    let zoom_speed = 0.5;


    // TODO:: 이해하지 못했음 이해해야함
    // 마우스 중앙 버튼을 누른 경우, 카메라 회전을 처리합니다.
    if mouse_button.pressed(MouseButton::Middle) {
        for mouse_move_event in mouse_motion.read() {
            if let Ok((player_transform, _)) = param_set.p0().get_single() {
                let player_position = player_transform.translation;  // 플레이어 위치를 먼저 가져옴

                if let Ok((mut camera_transform, mut camera_follow)) = param_set.p1().get_single_mut() {
                    // 좌우 회전은 월드 Y축, 상하 회전은 카메라 로컬 X축을 기준으로 처리
                    // 음수를 곱해 마우스 이동 방향을 반전시킴
                    let rotation = Quat::from_rotation_y(-mouse_move_event.delta.x * 0.005)
                        * Quat::from_axis_angle(*camera_transform.local_x(), -mouse_move_event.delta.y * 0.005);

                    // 카메라 오프셋을 회전시켜 새로운 위치 계산
                    camera_follow.offset = rotation * camera_follow.offset;
                    camera_transform.translation = player_position + camera_follow.offset;

                    // 카메라가 항상 플레이어를 바라보도록 설정
                    camera_transform.look_at(player_position, Vec3::Y);
                }
            }
        }
    }

    // TODO:: 이해하지 못했음 이해해야함
    // 이제 카메라의 위치를 수정합니다.
    if let Ok((player_transform, _)) = param_set.p0().get_single() {
        let player_position = player_transform.translation;
        if let Ok((mut camera_transform, camera_follow)) = param_set.p1().get_single_mut() {
            camera_transform.translation = player_position + camera_follow.offset;
            camera_transform.look_at(player_position, Vec3::Y);
        }
    }

    // TODO:: 이해하지 못했음 이해해야함
    // 휠로 줌을 조정할 때, 오프셋을 Z축 방향으로만 조정합니다.
    for v in wheel_event.read() {
        if let Ok((_, mut camera_follow)) = param_set.p1().get_single_mut() {
            // Z축 오프셋만 조정하여 줌을 적용
            let zoom_delta = -v.y * zoom_speed;
            let offset_length = camera_follow.offset.length();

            // 줌 범위를 제한하고 오프셋을 조정
            let new_length = (offset_length + zoom_delta).clamp(5.0, 20.0);
            camera_follow.offset = camera_follow.offset.normalize() * new_length;
        }
    }
}


fn landed_player_check(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    let player_entity = player_query.get_single().ok();
    let ground_entity = ground_query.get_single().ok();

    for event in collision_events.read() {
        println!("Collision event detected: {:?}", event);
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                println!("entity1: {:?}, entity2: {:?}", entity1, entity2);

                if let (Some(player), Some(ground)) = (player_entity, ground_entity) {
                    println!("Player Entity: {:?}, Ground Entity: {:?}", player, ground);

                    if (*entity1 == player && *entity2 == ground) || (*entity1 == ground && *entity2 == player) {
                        println!("플레이어가 바닥에 닿았습니다.");
                    } else {
                        println!("충돌은 발생했지만 플레이어와 바닥이 아닙니다.");
                    }
                }
            }
            _ => {}
        }
    }
}