use bevy::prelude::*;
use bevy::window::WindowMode;

mod score;
use score::ScorePlugin;

mod bat_lpl;
use bat_lpl::BatPlugin;

mod pillar_lpl;
use pillar_lpl::PillarPlugin;

mod coin_lpl;
use coin_lpl::CoinPlugin;

mod gear_lpl;
use gear_lpl::GearPlugin;

mod heart_lpl;
use heart_lpl::HeartPlugin;

mod background;
use background::BackgroundPlugin;

mod menu;
use menu::MenuPlugin;

mod level_end;
use level_end::LevelEndPlugin;

#[derive(Component)]
struct GameOverText;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum LevelState {
    #[default]
    Level1,
    Level2,
    Level3,
    LevelEnd
}

#[derive(Component)]
pub struct LevelEndUi;

#[derive(Resource, Default)]
pub struct TimeScore {
    pub seconds: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bat Flight".into(),
                resolution: (1280, 720).into(), 
                fit_canvas_to_parent: true,
                mode: WindowMode::Windowed, 
                ..default()
            }),
            ..default()
        }))
        
        .init_state::<GameState>()
        .init_state::<LevelState>()
        .init_gizmo_group::<DefaultGizmoConfigGroup>() 
        .add_plugins(ScorePlugin)
        .add_plugins(BatPlugin)
        .add_plugins(PillarPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(CoinPlugin)         
        .add_plugins(GearPlugin)   
        .add_plugins(HeartPlugin)
        .add_plugins(LevelEndPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, restart_input.run_if(in_state(GameState::GameOver)))
        .add_systems(OnEnter(GameState::GameOver), show_gameover)
        .add_systems(OnExit(GameState::GameOver),  hide_gameover)
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
            volume: bevy::audio::Volume::Linear(0.2),
            ..default()
        },
    ));
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
) {
    let pressed = keyboard.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();

    if pressed {
        next.set(GameState::Playing);
    }
}