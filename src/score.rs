use bevy::prelude::*;
use crate::pillar_lpl::Pillar;
use crate::bat_lpl::Bat;
use crate::GameState;

#[derive(Resource, Default)]
pub struct Score(pub f32);

#[derive(Component)]
pub struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .add_systems(OnEnter(GameState::Playing), setup_score_ui)
            .add_systems(OnExit(GameState::Playing), cleanup_score_ui)
            .add_systems(Update, (
                check_score, update_score_ui
            ).chain().run_if(in_state(GameState::Playing)));
    }
}

fn setup_score_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("0"),
        TextFont { font_size: 64.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top:  Val::Px(20.0),
            left: Val::Percent(48.0),
            ..default()
        },
        ScoreText,
    ));
}

fn check_score(
    bat_query: Query<&Transform, With<Bat>>,
    mut pillar_query: Query<(&Transform, &mut Pillar)>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok(bat_t) = bat_query.single() else { return };
    for (pillar_t, mut pillar) in &mut pillar_query {
        if !pillar.scored && pillar_t.translation.x < bat_t.translation.x {
            pillar.scored = true;
            score.0 += 0.5;
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/score.ogg"),
            ));
        }
    }
}

fn update_score_ui(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if !score.is_changed() { return; }
    for mut text in &mut query {
        **text = score.0.to_string();
    }
}

fn cleanup_score_ui(mut commands: Commands, query: Query<Entity, With<ScoreText>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}