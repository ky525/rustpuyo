#[path ="./player_field.rs"]
pub mod player_field;

const FIELD_BASE_X : f32 = -160.0;
const FIELD_BASE_Y : f32 = -260.0;

const NEXT_X : f32 = 200.0;
const NEXT_Y : f32 = 200.0;

use bevy::text::Text2dBounds;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::game_statics;

use super::GameState;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
enum PlayState
{
    #[default]
    None,
    Wait,
    Ready,
    Playing,
    Result,
}

#[derive(Component, Default)]
struct GameValue
{
    ready_count : i32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Message;

pub struct Game1PPlugin;

impl Plugin for Game1PPlugin{
    fn build(&self, app : &mut App) {
        app
        .add_systems(OnEnter(GameState::Game1P),on_enter)
        .add_systems(Update, update.run_if(in_state(GameState::Game1P)))
        .add_systems(OnExit(GameState::Game1P), on_exit)
        
        .add_state::<PlayState>()
        .add_systems(OnEnter(PlayState::Wait),on_enter_wait)
        .add_systems(Update, wait.run_if(in_state(PlayState::Wait)))
        .add_systems(OnEnter(PlayState::Ready),on_enter_ready)
        .add_systems(Update, ready.run_if(in_state(PlayState::Ready)))
        .add_systems(OnEnter(PlayState::Playing),on_enter_playing)
        .add_systems(Update, playing.run_if(in_state(PlayState::Playing)))
        .add_systems(OnEnter(PlayState::Result),on_enter_result)
        .add_systems(Update, result.run_if(in_state(PlayState::Result)))

        
        .add_plugins(player_field::PlayerFieldPlugin{
            values : player_field::FieldValues {
                field_base_x : FIELD_BASE_X,
                field_base_y : FIELD_BASE_Y,
                next_x : NEXT_X,
                next_y : NEXT_Y,
                player_id : 0,
                game_mode : 0,
                ..default()
            }
        });
    }
}

fn on_enter(mut commands : Commands, mut play_state: ResMut<NextState<PlayState>>)
{
    commands.init_resource::<game_statics::GameMode1P>();

    commands.spawn(GameValue { ..default() });

    commands.spawn(Text2dBundle{
        text : Text::from_section(
            "", 
            TextStyle{
                font_size: player_field::BLOCK_INNER_SIZE,
                ..default()
            }),
            text_2d_bounds : Text2dBounds{
                size : Vec2 { x : player_field::BLOCK_SIZE * player_field::FIELD_WIDTH as f32, y : player_field::BLOCK_SIZE * 3.0},
                ..default()
            },
            transform : Transform {
                translation : Vec3 {x:FIELD_BASE_X + player_field::BLOCK_SIZE * 6.0, y:FIELD_BASE_Y + player_field::BLOCK_SIZE * 10.0 , z : 10.0},
                ..default()
            },
            text_anchor : bevy::sprite::Anchor::CenterRight,
            ..default()
        })
        .insert(Message);
    play_state.set(PlayState::Wait);
}

fn update(mut game_state: ResMut<NextState<GameState>>)
{
}

fn on_exit(mut commands : Commands,
    mut play_state: ResMut<NextState<PlayState>>,
    q_egv : Query<Entity, With<GameValue>>,
    q_ems : Query<Entity, With<Message>>)
{
    play_state.set(PlayState::None);
    commands.remove_resource::<game_statics::GameMode1P>();
    
    for egv in q_egv.iter()
    {
        commands.entity(egv).despawn();
    }
    for ems in q_ems.iter()
    {
        commands.entity(ems).despawn();
    }
}

fn on_enter_wait(mut q_text : Query<&mut Text, With<Message>>)
{
    let mut text = q_text.single_mut();
    text.sections[0].value = "Press Z To Ready".to_string();
}

fn wait(
    q_key : Query<&ActionState<game_statics::Action>, With<game_statics::Player1>>,
    mut state : ResMut<NextState<PlayState>>,
){
    let action_state = q_key.single();
    if action_state.just_pressed(&game_statics::Action::RotR)
    {
        state.set(PlayState::Ready);
    }
} 

fn on_enter_ready(
    mut q_text : Query<&mut Text, With<Message>>,
    mut q : Query<&mut GameValue>)
{
    let mut text = q_text.single_mut();
    let mut value = q.single_mut();

    value.ready_count = game_statics::GAME_READY_COUNT;
    let s = format!("{:.2}", value.ready_count as f32 / 60.0);
    text.sections[0].value = "Ready..  ".to_string() + &s.to_string(); 
}
fn ready(
    mut q_text : Query<&mut Text, With<Message>>,
    mut q : Query<&mut GameValue>,
    mut state : ResMut<NextState<PlayState>>,)
{
    let mut text = q_text.single_mut();
    let mut value = q.single_mut();

    let s = format!("{:.2}", value.ready_count as f32 / 60.0);
    text.sections[0].value = "Ready..  ".to_string() + &s.to_string(); 

    value.ready_count -= 1;
    if value.ready_count == 0
    {
        state.set(PlayState::Playing);
    }
}

fn on_enter_playing(mut gc : EventWriter<game_statics::TransitionEvent>,
    mut q_text : Query<&mut Text, With<Message>>,
)
{
    gc.send(game_statics::TransitionEvent{
        player_id : -1,
        transition : game_statics::Transition::Start}
    );
    let mut text = q_text.single_mut();
    text.sections[0].value = "".to_string();
}
fn playing(mut state : ResMut<NextState<PlayState>>,
    mut death : EventReader<game_statics::Death>,
    mut gc : EventWriter<game_statics::TransitionEvent>,)
{
    for d in death.read()
    {
        state.set(PlayState::Result);
        gc.send(game_statics::TransitionEvent{
            player_id : d.0,
            transition : game_statics::Transition::Finish}
        );
    }
}

fn on_enter_result(mut q_text : Query<&mut Text, With<Message>>,)
{
    let mut text = q_text.single_mut();
    text.sections[0].value = "GameOver\nX:Restart\nZ:Menu".to_string();
}

fn result(
    q_key : Query<&ActionState<game_statics::Action>, With<game_statics::Player1>>,
    mut state : ResMut<NextState<PlayState>>,
    mut gstate : ResMut<NextState<GameState>>,
    mut rst: EventReader<game_statics::Reset>
){
    let action_state = q_key.single();
    if action_state.just_pressed(&game_statics::Action::RotL)
    {
        state.set(PlayState::None);
        gstate.set(GameState::Title);
    }

    for _ in rst.read()
    {
        state.set(PlayState::Wait);
    }
}