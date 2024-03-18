use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

pub const GAME_READY_COUNT : i32 = 120;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState{
    #[default]None,
    Init,
    Title,
    Connecting,
    Game1P,
    Game2PLocal,
    Game2POnline,
}

#[derive(Eq, PartialEq, Default)]
pub enum Transition
{
    #[default]
    None,
    Start,
    Finish,
}

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Action{
    Up,
    Down,
    Left,
    Right,
    RotR,
    RotL,
}

#[derive(Resource, Default)]
pub struct GameMode1P;

#[derive(Resource, Default)]
pub struct GameMode2PLocal;

#[derive(Resource, Default)]
pub struct GameMode2POnline;


#[derive(Component, Default)]
pub struct Player1;

#[derive(Component, Default)]
pub struct Player2;

#[derive(Component, Default)]
pub struct Permanent;

#[derive(Event)]
pub struct TransitionEvent
{
    pub player_id : i32,
    pub transition : Transition,
}

#[derive(Event)]
pub struct Death(pub i32);

#[derive(Event)]
pub struct Ojama
{
    pub player_id : i32,
    pub num : i32,
    pub frame : i32,
}

#[derive(Event)]
pub struct Reset(pub u64);