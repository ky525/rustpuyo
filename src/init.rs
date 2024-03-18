use bevy::prelude::*;
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap, InputManagerBundle};
use crate::game_statics;

use super::GameState;

pub struct InitPlugin;

impl Plugin for InitPlugin{
    fn build(&self, app : &mut App) {
        app
        .add_systems(OnEnter(GameState::Init),on_enter)
        .add_systems(Update, update.run_if(in_state(GameState::Init)))
        .add_systems(OnExit(GameState::Init), on_exit);
    }
}

fn on_enter()
{

}

fn update(mut game_state: ResMut<NextState<GameState>>,
    mut commands : Commands,
    gamepads : Res<Gamepads>,
)
{
    let mut input_map_p1 = InputMap::new([
        (game_statics::Action::Up, KeyCode::Up),
        (game_statics::Action::Down, KeyCode::Down),
        (game_statics::Action::Left, KeyCode::Left),
        (game_statics::Action::Right, KeyCode::Right),
        (game_statics::Action::RotR, KeyCode::X),
        (game_statics::Action::RotL, KeyCode::Z),
    ]);

    let mut input_map_p2 = InputMap::new([
        (game_statics::Action::Up, KeyCode::K),
        (game_statics::Action::Down, KeyCode::J),
        (game_statics::Action::Left, KeyCode::H),
        (game_statics::Action::Right, KeyCode::L),
        (game_statics::Action::RotR, KeyCode::G),
        (game_statics::Action::RotL, KeyCode::F),
    ]);
    
    let mut i = 0;
    for gamepad in gamepads.iter()
    {
        if i == 0
        {
            input_map_p1
                .set_gamepad(gamepad)
                .insert(game_statics::Action::Up, GamepadButtonType::DPadUp)
                .insert(game_statics::Action::Down, GamepadButtonType::DPadDown)
                .insert(game_statics::Action::Left, GamepadButtonType::DPadLeft)
                .insert(game_statics::Action::Right, GamepadButtonType::DPadRight)
                .insert(game_statics::Action::RotR, GamepadButtonType::East)
                .insert(game_statics::Action::RotL, GamepadButtonType::South);
        }
        if i == 1
        {
            input_map_p2
                .set_gamepad(gamepad)
                .insert(game_statics::Action::Up, GamepadButtonType::DPadUp)
                .insert(game_statics::Action::Down, GamepadButtonType::DPadDown)
                .insert(game_statics::Action::Left, GamepadButtonType::DPadLeft)
                .insert(game_statics::Action::Right, GamepadButtonType::DPadRight)
                .insert(game_statics::Action::RotR, GamepadButtonType::East)
                .insert(game_statics::Action::RotL, GamepadButtonType::South);
        }
        i = i + 1;
    }
    
    commands.spawn(
        InputManagerBundle::<game_statics::Action> {
        action_state : ActionState::default(),   
        input_map : input_map_p1
    })
    .insert(game_statics::Player1)
    .insert(game_statics::Permanent);

    commands.spawn(
        InputManagerBundle::<game_statics::Action> {
        action_state : ActionState::default(),   
        input_map : input_map_p2
    })
    .insert(game_statics::Player2)
    .insert(game_statics::Permanent);

    // 初期化処理が終わったらタイトルへ
    game_state.set(GameState::Title);
}

fn on_exit()
{

}
