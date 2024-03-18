pub mod game_statics;
pub mod init;
pub mod title_disp;
pub mod game_1p;
pub mod game_2p_local;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use game_statics::*;

const BLOCK_SIZE : f32 = 40.0;
const BLOCK_INNER_SIZE : f32 = 36.0;

fn setup(
    mut commands : Commands,
    mut game_state: ResMut<NextState<GameState>>)
{
    commands.spawn(Camera2dBundle::default());
    game_state.set(GameState::Init);
}

fn none()
{
    // NOP
}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<game_statics::Action>::default())
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, none.run_if(in_state(GameState::None)))
        .add_plugins((
            init::InitPlugin,
            title_disp::TitlePlugin,
            game_1p::Game1PPlugin,
            game_2p_local::Game2PLocalPlugin,
        ))
        .add_event::<game_statics::TransitionEvent>()
        .add_event::<game_statics::Death>()
        .add_event::<game_statics::Ojama>()
        .add_event::<game_statics::Reset>()
        .run();
}
