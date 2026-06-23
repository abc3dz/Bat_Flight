use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;

use crate::GameState;
use crate::score::Score;

const GRAVITY:    f32 = -12.0;
const FLAP_FORCE: f32 =  6.0;

#[derive(Component)]
pub struct Bat {
    pub velocity_y: f32,
}

#[derive(Component)]
pub struct BatLaser {
    pub speed: f32,
    pub radius: f32,
    pub half_length: f32,
}

#[derive(Resource)]
pub struct BatAnimationToPlay {
    pub graph: Handle<AnimationGraph>,
    pub index: AnimationNodeIndex,
}

pub struct BatPlugin;

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(WindWakerShaderPlugin::default())
        //.add_systems(Startup, spawn_bat)
        .add_systems(Update, (
            play_animation_when_ready,
            bat_input,
            bat_physics,
            move_bat_laser,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::GameOver), cleanup_bat)
        .add_systems(OnEnter(GameState::Playing), spawn_bat);
    }
}

fn spawn_bat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
        let clip = asset_server.load("models/batlowpoly.glb#Animation0");
        let mut graph = AnimationGraph::new();
        let index = graph.add_clip(clip, 1.0, graph.root);
        let graph_handle = graphs.add(graph);
        commands.insert_resource(BatAnimationToPlay {
            graph: graph_handle,
            index,
        });

    commands.spawn((
            SceneRoot(asset_server.load("models/batlowpoly.glb#Scene0")),
            Transform::from_xyz(-3.0, 3.0, 0.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            WindWakerShaderBuilder::default()
                .time_of_day(TimeOfDay::Day)
                .weather(Weather::Sunny)
                .build(),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            Bat { velocity_y: 5.0 }
        ));
}

fn bat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse:    Res<ButtonInput<MouseButton>>,
    touches:  Res<Touches>,
    mut query: Query<&mut Bat>,
    bat_transform: Query<&Transform, With<Bat>>,
    mut commands: Commands,        
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let flapped = keyboard.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();

    if flapped {
        for mut bat in &mut query {
            bat.velocity_y = FLAP_FORCE;
        }
        
        commands.spawn(AudioPlayer::new(
            asset_server.load("sounds/fly.ogg"),
        ));
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        if let Ok(bat_transform) = bat_transform.single() {
            
            // กำหนดขนาดของ Cylinder แนวนอน
            let radius = 0.3;
            let length = 3.0;

            // สร้าง Mesh Cylinder (ปกติจะเป็นแนวตั้ง)
            let cylinder_mesh = Cylinder::new(radius, length);

            // หมุนให้เป็นแนวนอน (หมุนรอบแกน X 90 องศา เพื่อให้ทอดไปตามแกน Z)
            let horizontal_rotation = Quat::from_rotation_x(90.0);

            commands.spawn((
                BatLaser {
                    speed: 15.0, // ความเร็วในการวิ่ง
                    radius,
                    half_length: length / 2.0,
                },
                Mesh3d(meshes.add(cylinder_mesh)),
                MeshMaterial3d(materials.add(Color::from(LinearRgba::BLUE))), // พลังสีน้ำเงิน
                Transform {
                    // ปล่อยออกจากตำแหน่งค้างคาว
                    translation: bat_transform.translation, 
                    // หมุนตัวตัววัตถุให้เป็นแนวนอน
                    rotation: horizontal_rotation, 
                    ..default()
                }
            ));
        }
    }
}

pub fn move_bat_laser(
    mut commands: Commands,
    time: Res<Time>,
    mut laser_query: Query<(Entity, &mut Transform, &BatLaser)>,
) {
    for (entity, mut transform, laser) in &mut laser_query {
        //transform.rotate_y(time.delta_secs());
        transform.translation.x += laser.speed * time.delta_secs();

        if transform.translation.x > 50.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn bat_physics(
    time: Res<Time>,
    mut query: Query<(&mut Bat, &mut Transform)>,
    mut next: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut score: ResMut<Score>
) {
    for (mut bat, mut transform) in &mut query {
        bat.velocity_y += GRAVITY * time.delta_secs();
        transform.translation.y += bat.velocity_y * time.delta_secs();
        if transform.translation.y < -5.0 {
            score.game_over += 1;
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/game_over.ogg"),
            ));
            next.set(GameState::GameOver);
        }
    }
}

pub fn play_animation_when_ready(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
    anim: Res<BatAnimationToPlay>,
) {
    for (entity, mut player) in &mut players {
        commands.entity(entity).insert(AnimationGraphHandle(anim.graph.clone()));
        player.play(anim.index).repeat();
    }
}

fn cleanup_bat(
    mut commands: Commands,
    query: Query<Entity, With<Bat>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}