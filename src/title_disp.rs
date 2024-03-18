use bevy::{prelude::*, text::Text2dBounds};
use super::GameState;

pub struct TitlePlugin;

const CURSOR_NUM : i32 = 4;

impl Plugin for TitlePlugin{
    fn build(&self, app : &mut App) {
        app
        .add_systems(OnEnter(GameState::Title),on_enter)
        .add_systems(Update, update.run_if(in_state(GameState::Title)))
        .add_systems(OnExit(GameState::Title), on_exit);
    }
}

#[derive(Component, Default)]
struct Cursor
{
    point : i32
}

#[derive(Component)]
struct TitleDisp;

fn on_enter(mut commands : Commands,
    qe : Query<Entity, Without<Camera2d>>)
{
    commands.spawn((Text2dBundle{
        text : Text::from_section(
            "PUYOPUYO", 
            TextStyle{
                font_size: 160.0,
                ..default()
            }),
            text_2d_bounds : Text2dBounds{
                size : Vec2 { x : 600.0, y : 160.0},
                ..default()
            },
            transform : Transform {
                translation : Vec3 {x:0.0, y:200.0, z : 10.0},
                ..default()
            },
            text_anchor : bevy::sprite::Anchor::Center,
            ..default()
        },
        TitleDisp
    ));

    commands.spawn((Text2dBundle{
        text : Text::from_section(
            "1P Game", 
            TextStyle{
                font_size: 40.0,
                ..default()
            }),
            text_2d_bounds : Text2dBounds{
                size : Vec2 { x : 600.0, y : 160.0},
                ..default()
            },
            transform : Transform {
                translation : Vec3 {x:0.0, y:0.0, z : 10.0},
                ..default()
            },
            text_anchor : bevy::sprite::Anchor::Center,
            ..default()
        },
        TitleDisp
    ));

    commands.spawn((Text2dBundle{
        text : Text::from_section(
            "2P Game", 
            TextStyle{
                font_size: 40.0,
                ..default()
            }),
            text_2d_bounds : Text2dBounds{
                size : Vec2 { x : 200.0, y : 40.0},
                ..default()
            },
            transform : Transform {
                translation : Vec3 {x:0.0, y:-50.0, z : 10.0},
                ..default()
            },
            text_anchor : bevy::sprite::Anchor::Center,
            ..default()
        },
        TitleDisp
    ));

    commands.spawn((Text2dBundle{
        text : Text::from_section(
            "2P Online", 
            TextStyle{
                font_size: 40.0,
                ..default()
        }),
        text_2d_bounds : Text2dBounds{
            size : Vec2 { x : 200.0, y : 40.0},
            ..default()
        },
        transform : Transform {
            translation : Vec3 {x:0.0, y:-100.0, z : 10.0},
            ..default()
        },
        text_anchor : bevy::sprite::Anchor::Center,
        ..default()
        },
        TitleDisp
    ));

    commands.spawn((Text2dBundle{
        text : Text::from_section(
            "Exit Game", 
            TextStyle{
                font_size: 40.0,
                ..default()
            }),
            text_2d_bounds : Text2dBounds{
                size : Vec2 { x : 200.0, y : 40.0},
                ..default()
            },
            transform : Transform {
                translation : Vec3 {x:0.0, y:-150.0, z : 10.0},
                ..default()
            },
            text_anchor : bevy::sprite::Anchor::Center,
            ..default()
        },
        TitleDisp
    ));

    commands.spawn((Text2dBundle{
        text : Text::from_section(
            "|>", 
            TextStyle{
                font_size: 40.0,
                ..default()
            }),
            text_2d_bounds : Text2dBounds{
                size : Vec2 { x : 80.0, y : 40.0},
                ..default()
            },
            transform : Transform {
                translation : Vec3 {x:-200.0, y:0.0, z : 10.0},
                ..default()
            },
            text_anchor : bevy::sprite::Anchor::Center,
            ..default()
        },
        Cursor{
        ..default()
        },
        TitleDisp
    ));
}

fn update(
    key_input : Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut q_cursor : Query<(&mut Transform, &mut Cursor)>)
{

    for(mut transform, mut cursor) in &mut q_cursor
    {
        if key_input.just_pressed(KeyCode::Down)
        {
            cursor.point += 1;
            cursor.point %= CURSOR_NUM;
        }
        if key_input.just_pressed(KeyCode::Up)
        {
            cursor.point += 3;
            cursor.point %= CURSOR_NUM;
        }

        if key_input.just_pressed(KeyCode::Return)
        {
            match cursor.point{
                0=>{
                    game_state.set(GameState::Game1P);
                }
                1=>{
                    game_state.set(GameState::Game2PLocal);
                }
                2=>{
                    game_state.set(GameState::Connecting);
                }
                3=>{
                    std::process::exit(0);
                }
                _=>{}
            }
        }

        transform.translation.y = cursor.point as f32 * -50.0;
    }
}

fn on_exit(
    mut commands : Commands,
    query : Query<Entity, With<TitleDisp>>
){
    for entity in &query{
        commands.entity(entity).despawn();
    }
}
