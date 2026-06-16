use bevy::prelude::*;

use crate::bat_lpl::Bat;
use crate::coin_lpl::Coin;
use crate::gear_lpl::Gear;
use crate::heart_lpl::Heart;
use crate::GameState;
use crate::LevelState;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
    pub heart: u32,
}
impl Default for Score {
    fn default() -> Self {
        Self {
            value: 0,
            heart: 3,
        }
    }
}

#[derive(Component)]
struct HeartsContainer;
#[derive(Component)]
struct HeartsUi;

#[derive(Component)]
pub struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score::default())
            .add_systems(OnEnter(GameState::Playing), setup_score_ui)
            //.add_systems(OnExit(GameState::Playing), cleanup_score_ui)
            .add_systems(Update,check_level_progress.run_if(in_state(GameState::Playing)),)
            .add_systems(Update, (check_score, update_score_ui).chain().run_if(in_state(GameState::Playing)));
    }
}

fn setup_score_ui(mut commands: Commands, score: Res<Score>, asset_server: Res<AssetServer>) {
     commands.spawn((
        HeartsContainer,
        Node {
            position_type: PositionType::Absolute,
            top: px(10.0),
            left: px(10.0),
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|parent| {
        for _ in 0..score.heart {
            parent.spawn((
                HeartsUi,
                ImageNode::new(
                    asset_server.load("images/heart.png")
                ),
                Node {
                    width: px(50.0),
                    height: px(50.0),
                    ..default()
                }
            ));
        }
    });
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            padding: UiRect::top(px(10.0)),
            ..default()
        },
    )).with_children(|parent| {

        parent.spawn((
            Node {
                align_items: AlignItems::Center,
                ..default()
            },
        )).with_children(|parent| {

            parent.spawn((
                ImageNode::new(asset_server.load("images/coin.png")),
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
    });
}

fn check_score(
    bat_query: Query<&Transform, With<Bat>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
    gear_query: Query<(Entity, &Transform), With<Gear>>,
    heart_query: Query<Entity, With<HeartsUi>>,
    heart_query_trans: Query<(Entity, &Transform), With<Heart>>,
    container_query: Query<Entity, With<HeartsContainer>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Ok(bat_t) = bat_query.single() else { return };

    for (entity, coin_transform) in &coin_query {
        let distance = bat_t
            .translation
            .distance(coin_transform.translation);

        if distance < 1.0 {
            if coin_transform.translation.y < -1.0 {
                score.value += 5;
            }else{
                score.value += 5;
            }
            commands.entity(entity).despawn();
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/score.ogg"),
            ));
        }
    }

    for (entity, gear_transform) in &gear_query {
        let distance = bat_t
            .translation
            .distance(gear_transform.translation);

        if distance < 1.0 {
            if score.heart <= 0 {
                score.heart = 3;
                next.set(GameState::GameOver);
            }else{
                score.value -= 1;
                score.heart -= 1;
            }
            commands.entity(entity).despawn();
            
            if let Some(heart_entity) = heart_query.iter().next() {
                commands.entity(heart_entity).despawn();
            }
        }
    }

    for (entity, heart_trans) in &heart_query_trans {
        let distance = bat_t
            .translation
            .distance(heart_trans.translation);

        if distance < 1.0 {
            score.heart += 1;
            commands.entity(entity).despawn();

            if let Ok(container) = container_query.single() {
                commands.entity(container).with_children(|parent| {
                    parent.spawn((
                        HeartsUi,
                        ImageNode::new(asset_server.load("images/heart.png")),
                        Node {
                            width: px(50.0),
                            height: px(50.0),
                            ..default()
                        }
                    ));
                });
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