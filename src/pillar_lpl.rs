use bevy::prelude::*;
use rand::Rng;
use bevy_wind_waker_shader::prelude::*;
use crate::bat_lpl::Bat;
use crate::GameState;
use crate::LevelState;

const PILLAR_SPEED:      f32 = 4.0;
const PILLAR_SPAWN_X:    f32 = 10.0;
const PILLAR_DESPAWN_X:  f32 = -10.0;
const PILLAR_SPAWN_SECS: f32 = 2.0;
const GAP_HALF:          f32 = 1.8;

const BIRD_RADIUS:f32 = 0.4;
const PIPE_HALF_W:f32 = 0.5;
const PIPE_HALF_H:f32 = 2.0;

#[derive(Component)]
pub struct Pillar;

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
            .add_systems( Update,(
                    spawn_pillars,
                    move_pillars,
                    despawn_pillars,
                    check_collision,
                )
                .run_if(in_state(GameState::Playing))
                .run_if(pillar_levels))
            .add_systems(Update, draw_hitbox.run_if(in_state(GameState::GameOver)))
            .add_systems(OnEnter(GameState::GameOver), setup_game_over);
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
    let gap_center: f32 = rng.random_range(-2.0..=2.0);

    commands.spawn((
        SceneRoot(asset_server.load("models/pillarlowpoly.glb#Scene0")),
        Transform::from_xyz(PILLAR_SPAWN_X, gap_center - GAP_HALF - 2.5, 0.0),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
        GlobalTransform::default(),
        Pillar,
    ));
}

fn move_pillars(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Pillar>>,
) {
    for mut transform in &mut query {
        transform.translation.x -= PILLAR_SPEED * time.delta_secs();
    }
}

fn check_collision(
    bat_query: Query<&Transform, With<Bat>>,
    pillar_query: Query<&Transform, With<Pillar>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Ok(bat_t) = bat_query.single() else { return };
    let bp = bat_t.translation;

    for pillar_t in &pillar_query {
        let pp = pillar_t.translation;

        let closest = Vec3::new(
            bp.x.clamp(pp.x - PIPE_HALF_W, pp.x + PIPE_HALF_W),
            bp.y.clamp(pp.y - PIPE_HALF_H, pp.y + PIPE_HALF_H),
            bp.z,
        );

        let dist = (bp - closest).length();
        if dist < BIRD_RADIUS {       
            next.set(GameState::GameOver);
            
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/game_over.ogg"),
            ));
        }
    }
}

fn draw_hitbox(
    mut gizmos: Gizmos,
    bat_query: Query<&Transform, With<Bat>>,
    pillar_query: Query<&Transform, With<Pillar>>,
) {
    if let Ok(bat_t) = bat_query.single() {
        gizmos.sphere(
            Isometry3d::from_translation(bat_t.translation),
            BIRD_RADIUS,
            Color::srgb(0.0, 1.0, 0.0),
        );
    }
    
    for pillar_t in &pillar_query {
        gizmos.cube(
            Transform {
                translation: pillar_t.translation,
                scale: Vec3::new(PIPE_HALF_W * 2.0, PIPE_HALF_H * 2.0, 1.0),
                ..default()
            },
            Color::srgb(1.0, 0.0, 0.0),
        );
    }
}

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

fn pillar_levels(
    level_state: Res<State<LevelState>>,
) -> bool {
    matches!(
        level_state.get(),
        LevelState::Level2 | LevelState::Level3
    )
}
fn setup_game_over(
    mut bat_query: Query<(&mut Bat, &mut Transform), With<Bat>>,
) {
    let Ok((mut bat, mut transform)) = bat_query.single_mut() else { return };

    transform.translation.y = 0.0;
    bat.velocity_y= 2.0;
}