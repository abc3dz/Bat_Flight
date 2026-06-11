use bevy::prelude::*;
use rand::Rng;
use bevy_wind_waker_shader::prelude::*;
use crate::GameState;

const PILLAR_SPEED:      f32 = 4.0;   // ความเร็วท่อ
const PILLAR_SPAWN_X:    f32 = 10.0;  // ตำแหน่ง spawn ด้านขวา
const PILLAR_DESPAWN_X:  f32 = -10.0; // ลบท่อเมื่อออกนอกจอซ้าย
const PILLAR_SPAWN_SECS: f32 = 2.0;   // spawn ทุกกี่วินาที
const GAP_HALF:        f32 = 1.8;   // ครึ่งช่องว่างระหว่างท่อบน/ล่าง

#[derive(Component)]
pub struct Pillar{
    pub scored: bool,
}

#[derive(Resource)]
struct PillarSpawnTimer(Timer);

pub struct PillarPlugin;

impl Plugin for PillarPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PillarSpawnTimer(Timer::from_seconds(
                PILLAR_SPAWN_SECS,
                TimerMode::Repeating,
            )))
            .add_systems(Update, (
                spawn_pillars,
                move_pillars,
                despawn_pillars,
            ).chain().run_if(in_state(GameState::Playing)));
    }
}

fn spawn_pillars(
    time: Res<Time>,
    mut timer: ResMut<PillarSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() { return; }

    let mut rng = rand::rng();
    let gap_center: f32 = rng.random_range(-2.0..=2.0); // สุ่มตำแหน่งช่อง

    // ท่อล่าง
    commands.spawn((
        SceneRoot(asset_server.load("models/pillarlowpoly.glb#Scene0")),
        Transform::from_xyz(PILLAR_SPAWN_X, gap_center - GAP_HALF - 2.5, 0.0),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
        Outline::default(),
        GlobalTransform::default(),
        Pillar { scored: false },
    ));

    // ท่อบน (หมุน 180° ให้หันหัวลง)
    commands.spawn((
        SceneRoot(asset_server.load("models/pillarlowpoly.glb#Scene0")),
        Transform::from_xyz(PILLAR_SPAWN_X, gap_center + GAP_HALF + 2.5, 0.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
        GlobalTransform::default(),
        Pillar { scored: false },
    ));
}

// ── เลื่อนท่อเข้าหานกทุก frame ──────────────────────────────────
fn move_pillars(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Pillar>>,
) {
    for mut transform in &mut query {
        transform.translation.x -= PILLAR_SPEED * time.delta_secs();
    }
}

// ── ลบท่อที่ออกนอกจอ ─────────────────────────────────────────────
fn despawn_pillars(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Pillar>>,
) {
    for (entity, transform) in &query {
        if transform.translation.x < PILLAR_DESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}