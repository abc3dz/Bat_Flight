use bevy::prelude::*;
use rand::Rng;
use bevy_wind_waker_shader::prelude::*;

use crate::{GameState, LevelState};

const CLOUD_SPAWN_SECS: f32 = 2.0;
const CLOUD_SPEED:      f32 = 2.0;

#[derive(Component)]
pub struct PlaneBG;

#[derive(Component)] struct Cloud;

#[derive(Resource)] struct CloudSpawnTimer(Timer);

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CloudSpawnTimer(Timer::from_seconds(
            CLOUD_SPAWN_SECS,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup_background)
        .add_systems(OnEnter(LevelState::Level2),set_ground_level,)
        .add_systems(OnEnter(LevelState::Level3),set_ground_level,)
        .add_systems(OnEnter(LevelState::Level4),set_ground_level,)
        .add_systems(OnEnter(LevelState::Level5),set_ground_level,)
        .add_systems(Update, (
            spawn_clouds,
            move_clouds,
            despawn_clouds,
        ).run_if(in_state(GameState::Playing)));

    }
}

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(200.0, 200.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.154, 0.196, 0.513),
            unlit: true,
            ..default()
        })),
        Transform {
            translation: Vec3::new(0.0, 0.0, -2.0),
            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            scale: Vec3::ONE,
        },
        GlobalTransform::default(),
        PlaneBG
    ));
}

fn spawn_clouds(
    time: Res<Time>,
    mut timer: ResMut<CloudSpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() { return; }

    let mut rng = rand::rng();
    let y: f32 = rng.random_range(-4.0..=4.0);
    let radius: f32 = rng.random_range(0.3..=0.9);

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(radius))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(12.0, y, -1.5),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
        GlobalTransform::default(),
        Cloud,
    ));
}

fn move_clouds(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Cloud>>,
) {
    for mut t in &mut query {
        t.translation.x -= CLOUD_SPEED * time.delta_secs();
    }
}

fn despawn_clouds(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Cloud>>,
) {
    for (entity, t) in &query {
        if t.translation.x < -12.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn set_ground_level(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>, With<PlaneBG>>,
    level_state: Res<State<LevelState>>,
) {
    let level_bonus = match level_state.get() {
        LevelState::Level1 => 0.0,
        LevelState::Level2 => 0.05,
        LevelState::Level3 => 0.10,
        LevelState::Level4 => 0.15,
        LevelState::Level5 => 0.20,
        _ => 0.0,
    };

    let Ok(material_handle) = query.single() else {
        return;
    };

    let Some(material) =
        materials.get_mut(&material_handle.0)
    else {
        return;
    };

    material.base_color =
        Color::srgb(0.154-level_bonus, 0.196-level_bonus, 0.513-level_bonus); 
}