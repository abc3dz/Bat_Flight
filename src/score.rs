use bevy::prelude::*;

use crate::bat_lpl::Bat;
use crate::coin_lpl::Coin;
use crate::gear_lpl::Gear;
use crate::GameState;
use crate::LevelState;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Component)]
pub struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .add_systems(OnEnter(GameState::Playing), setup_score_ui)
            //.add_systems(OnExit(GameState::Playing), cleanup_score_ui)
            .add_systems(Update,check_level_progress.run_if(in_state(GameState::Playing)),)
            .add_systems(Update, (check_score, update_score_ui).chain().run_if(in_state(GameState::Playing)));
    }
}

fn setup_score_ui(mut commands: Commands, score: Res<Score>, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(10.0),
            left: px(10.0),
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|parent| {

        parent.spawn((
            ImageNode::new(
                asset_server.load("images/coin.png")
            ),
            Node {
                width: px(50.0),
                height: px(50.0),
                ..default()
            },
        ));

        parent.spawn((
            Text::new(score.value.to_string()),
            ScoreText,
        ));
    });
}

fn check_score(
    bat_query: Query<&Transform, With<Bat>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
    gear_query: Query<(Entity, &Transform), With<Gear>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok(bat_t) = bat_query.single() else { return };

    for (entity, coin_transform) in &coin_query {
        let distance = bat_t
            .translation
            .distance(coin_transform.translation);

        if distance < 1.0 {
            score.value += 5;
            commands.entity(entity).despawn();
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/score.ogg"),
            ));
            //println!("Coin collected!");
        }

    for (entity, coin_transform) in &gear_query {
        let distance = bat_t
            .translation
            .distance(coin_transform.translation);

        if distance < 1.0 {
            score.value -= 1;
            commands.entity(entity).despawn();
            // commands.spawn(AudioPlayer::new(
            //     asset_server.load("sounds/score.ogg"),
            // ));

            }
        }
    }
}

fn update_score_ui(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if !score.is_changed() { return; }
    for mut text in &mut query {
        **text = score.value.to_string();
    }
}

fn check_level_progress(
    score: Res<Score>,
    level_state: Res<State<LevelState>>,
    mut next_level: ResMut<NextState<LevelState>>,
) {
    if *level_state.get() == LevelState::Level1
        && score.value >= 10
    {
        next_level.set(LevelState::Level2);
    }
    if *level_state.get() == LevelState::Level2
        && score.value >= 20
    {
        next_level.set(LevelState::Level3);
    }
}