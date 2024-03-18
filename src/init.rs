use bevy::prelude::*;
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

fn update(mut game_state: ResMut<NextState<GameState>>)
{
    // 初期化処理が終わったらタイトルへ
    game_state.set(GameState::Title);
}

fn on_exit()
{

}
