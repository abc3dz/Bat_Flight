use bevy::prelude::*;

use crate::GameState;
use crate::LevelState;

#[derive(Resource)]
pub struct Score {
    pub coin: u32,
    pub heart: u32,
    pub pillar: u32,
    pub gear: u32,
    pub owl: u32,
    pub game_over: u32,
}
impl Default for Score {
    fn default() -> Self {
        Self {
            coin: 0,
            heart: 3,
            pillar: 0,
            gear: 0,
            owl: 0,
            game_over: 0
        }
    }
}

#[derive(Component)]
pub struct CoinUi;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HelpText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score::default())
            .add_systems(OnEnter(GameState::Playing), setup_score_ui)
            .add_systems(OnEnter(GameState::Playing), setup_help_text)
            .add_systems(Update,check_level_progress.run_if(in_state(GameState::Playing)),)
            .add_systems(Update, update_score_ui.chain().run_if(in_state(GameState::Playing)));
    }
}

fn setup_score_ui(mut commands: Commands, score: Res<Score>, asset_server: Res<AssetServer>) {
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
                CoinUi,
                ImageNode::new(asset_server.load("images/coin.png")),
                Node {
                    width: px(50.0),
                    height: px(50.0),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new(score.coin.to_string()),
                ScoreText,
            ));
        });
    });
}

fn setup_help_text(
    mut commands: Commands,
) {
    commands.spawn((
        HelpText,
        Text::new("Press S to shoot projectile"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(20.0),
            left: px(20.0),
            ..default()
        },
    ));
}

fn update_score_ui(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if !score.is_changed() { return; }
    for mut text in &mut query {
        **text = score.coin.to_string();
    }
}

fn check_level_progress(
    score: Res<Score>,
    level_state: Res<State<LevelState>>,
    mut next_level: ResMut<NextState<LevelState>>,
) {
    if *level_state.get() == LevelState::Level1
        && score.coin >= 10
    {
        next_level.set(LevelState::Level2);
    }
    if *level_state.get() == LevelState::Level2
        && score.coin >= 20
    {
        next_level.set(LevelState::Level3);
    }
    if *level_state.get() == LevelState::Level3
        && score.coin >= 30
    {
        next_level.set(LevelState::Level4);
    }
    if *level_state.get() == LevelState::Level4
        && score.coin >= 40
    {
        next_level.set(LevelState::Level5);
    }
    if *level_state.get() == LevelState::Level5
        && score.coin >= 50
    {
        next_level.set(LevelState::LevelEnd);
    }
}