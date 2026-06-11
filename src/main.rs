use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

mod score;
use score::{ScorePlugin, Score};

mod bat_lpl;
use bat_lpl::BatPlugin;
use crate::bat_lpl::Bat;

mod pillar_lpl;
use pillar_lpl::PillarPlugin;
use crate::pillar_lpl::Pillar;

mod background;
use background::BackgroundPlugin;

mod menu;
use menu::MenuPlugin;

const BIRD_RADIUS:  f32 = 0.6;
const PIPE_HALF_W:  f32 = 1.0;
const PIPE_HALF_H:  f32 = 2.0;

#[derive(Component)]
struct GameOverText;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bat Flight".into(),
                fit_canvas_to_parent: true,
                mode: WindowMode::BorderlessFullscreen(
                    MonitorSelection::Primary,
                ),
                ..default()
            }),
            ..default()
        }))
        
        .init_state::<GameState>()
        .init_gizmo_group::<DefaultGizmoConfigGroup>() 
        
        .add_plugins(ScorePlugin)
        .add_plugins(BatPlugin)
        .add_plugins(PillarPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(MenuPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            check_collision, 
        ).chain().run_if(in_state(GameState::Playing)))
        .add_systems(Update, restart_input.run_if(in_state(GameState::GameOver)))
        .add_systems(OnEnter(GameState::GameOver), show_gameover)
        .add_systems(OnExit(GameState::GameOver),  hide_gameover)
        .add_systems(Update, draw_hitbox.run_if(in_state(GameState::GameOver)))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
    
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/bgm.ogg")),
        PlaybackSettings{
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Linear(0.5),
            ..default()
        },
    ));
}

fn check_collision(
    bat_query: Query<&Transform, With<Bat>>,
    pillar_query: Query<&Transform, With<Pillar>>,
    mut next: ResMut<NextState<GameState>>, 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok(bat_t) = bat_query.single() else { return };
    let bp = bat_t.translation;

    for pillar_t in &pillar_query {
        let pp = pillar_t.translation;

        // หา closest point บน AABB ของท่อ ที่ใกล้นกที่สุด
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

pub fn show_gameover(mut commands: Commands) {
    commands.spawn((
        Text::new("Again!"),
        TextFont { font_size: 80.0, ..default() },
        TextColor(Color::srgb(1.0, 0.3, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            top:  Val::Percent(40.0),
            left: Val::Percent(35.0),
            ..default()
        },
        GameOverText,
    ));
}

fn hide_gameover(
    mut commands: Commands,
    query: Query<Entity, With<GameOverText>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
fn restart_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse:    Res<ButtonInput<MouseButton>>,
    touches:  Res<Touches>,
    mut next: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
    pipes: Query<Entity, With<Pillar>>,
    mut commands: Commands,
) {
    let pressed = keyboard.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();

    if pressed {
        // ลบท่อทั้งหมด
        for entity in &pipes {
            commands.entity(entity).despawn();
        }
        // reset คะแนน
        score.0 = 0.0;
        // กลับไป Playing
        next.set(GameState::Playing);
    }
}

fn draw_hitbox(
    mut gizmos: Gizmos,
    bat_query: Query<&Transform, With<Bat>>,
    pillar_query: Query<&Transform, With<Pillar>>,
) {
    // วงกลมรอบนก
    if let Ok(bat_t) = bat_query.single() {
        gizmos.sphere(
            Isometry3d::from_translation(bat_t.translation),
            BIRD_RADIUS,
            Color::srgb(0.0, 1.0, 0.0), // สีเขียว
        );
    }
    // กล่องรอบแต่ละท่อ
    for pillar_t in &pillar_query {
        gizmos.cube(
            Transform {
                translation: pillar_t.translation,
                scale: Vec3::new(PIPE_HALF_W * 2.0, PIPE_HALF_H * 2.0, 1.0),
                ..default()
            },
            Color::srgb(1.0, 0.0, 0.0), // สีแดง
        );
    }
}