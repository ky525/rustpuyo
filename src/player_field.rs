use bevy::{prelude::*, render::render_resource::encase::rts_array::Length, text::Text2dBounds};
use rand::prelude::*;
use std::{f32::consts::PI, marker::PhantomData};

use crate::game_statics;

pub const FIELD_WIDTH : usize = 8;
const FIELD_HEIGHT : usize = 22;
const FIELD_DISP_HEIGHT : usize = 12;

pub const BLOCK_SIZE : f32 = 40.0;
pub const BLOCK_INNER_SIZE : f32 = 36.0;

const INIT_FALL_COUNT : i32 = 20;
const FALL_DOWN_Y : f32 = 20.0;

const MOVING_FIRST_COUNT : i32 = 12;
const MOVING_CONT_COUNT : i32 = 2;

const ROTATE_COUNT : i32 = 3;

const FIX_COUNT : i32 = 20;
const SHAKE_COUNT : i32 = 20;

const DELETE_BLINK : i32 = 30;
const DELETE_END : i32 = 40;
const DELETE_BLINK_INTERVAL : i32 = 2;
const DELETE_GROUP : i32 = 5;

const OJAMA_COUNT : i32 = 30;

const OJAMA_SMALL_SIZE : f32 = 16.0;
const OJAMA_SMALL_INNER_SIZE : f32 = 14.0;

const OJAMA_BIG_SIZE : f32 = 34.0;
const OJAMA_BIG_INNER_SIZE : f32 = 32.0;

const OJAMA_ROCK_SIZE : f32 = 40.0;
const OJAMA_ROCK_INNER_SIZE : f32 = 32.0;

const MAX_OJAMA_DISP_X : f32 = 240.0;

const SCORE_PER_OJAMA : i32 = 70; 

const COLORS : [Color;8]=[
    Color::Rgba{red : 0.0, green : 0.0, blue : 0.0, alpha : 0.0},
    Color::Rgba{red : 0.9, green : 0.0, blue : 0.0, alpha : 1.0},
    Color::Rgba{red : 0.0, green : 0.6, blue : 0.0, alpha : 1.0},
    Color::Rgba{red : 0.9, green : 0.9, blue : 0.0, alpha : 1.0},
    Color::Rgba{red : 0.7, green : 0.0, blue : 0.7, alpha : 1.0},
    Color::Rgba{red : 0.0, green : 0.0, blue : 0.9, alpha : 1.0},
    Color::Rgba{red : 1.0, green : 1.0, blue : 1.0, alpha : 0.2},
    Color::Rgba{red : 0.5, green : 0.5, blue : 0.5, alpha : 1.0},
    ];

const RENSA_BONUS : [i32;20]=[0,0,8,16,32,64,96,128,160,192,224,256,288,320,352,384,416,448,480,512];
const RENKETSU_BONUS : [i32;12]=[0,0,0,0,0,2,3,4,5,6,7,10];
const COLOR_BONUS : [i32;6] = [0,0,3,6,12,24];

enum ColorIndices
{
    //None = 0,
    //Red = 1,
    //Green = 2,
    //Yellow = 3,
    //Purple = 4,
    //Blue = 5,
    Ojama = 6,
    Wall = 7,
}

#[derive(PartialEq, Default)]
enum FieldState
{
    #[default]
    Init,
    Wait,
    Next,
    Move,
    FallCheck,
    Fall,
    DeleteCheck,
    Delete,
    OjamaFallCheck,
    OjamaFall,
    Win,
    Lose,
    LoseFall,
}

#[derive(Default)]
enum OjamaType
{
    #[default]
    Small,
    Big,
    Rock,
}

#[derive(Component, Default)]
struct Block
{
    color_idx : i32,
    is_move : bool,
    is_primary : bool,

    expect_field : (usize, usize),
    fall_y : f32,
    shake_count : i32,

    is_delete : bool,
    delete_group : i32,

    is_next : bool,
}

#[derive(Component, Default)]
struct OjamaDisp
{
    ojama_type : OjamaType,
    disp_x : f32,
}

#[derive(Component, Default)]
struct Field
{
    arrange : [[i32;FIELD_HEIGHT];FIELD_WIDTH] ,
    line_height : [f32;FIELD_WIDTH], 
    line_dan : [i32;FIELD_WIDTH],
    base_pos : Vec2,
    field_state : FieldState,

    moving_pos : Vec2,
    moving_line : usize,
    moving_dir_idx : i32,
    moving_color_idx : (i32, i32),
    fall_count : i32,
    fall_next_count : i32,
    fall_down_count : i32,

    moving_lr_count : i32,

    moving_prev_dir_idx : i32,
    moving_rot_count : i32,

    on_ground : bool,
    fix_count : i32,

    delete_count : i32,
    score : i32,
    ojama_sent_score : i32,

    base_score : i32,
    bonus_multiply : i32,
    rensa_num : i32,

    elapsed_frame : i32,

    kakutei_ojama : i32,
    kari_ojama : i32,
    //kakutei_frame : i32,
    ojama_count : i32,
}

impl Field
{
    fn update_line_height(&mut self)
    {
        for i in 0..FIELD_WIDTH
        {
            let line = &self.arrange[i];
            let mut dan_count = 0;
            for col in line{
                if *col == 0 {
                    break;
                }
                dan_count += 1;
            }
            self.line_height[i] = self.base_pos.y + dan_count as f32 * BLOCK_SIZE;
            self.line_dan[i] = dan_count;
        }
    }

    fn randing(&mut self, down_diff : f32)
    {
        // 下に物があるか
        match self.moving_dir_idx
        {
            0=>{
                if self.line_height[self.moving_line] > self.moving_pos.y - down_diff
                {
                    self.moving_pos.y = self.line_height[self.moving_line];
                    return;
                }
            }
            1=>{
                if  self.line_height[self.moving_line] > self.moving_pos.y - down_diff ||
                self.line_height[self.moving_line + 1] > self.moving_pos.y - down_diff
                {
                    self.moving_pos.y = f32::max(self.line_height[self.moving_line], self.line_height[self.moving_line + 1]);
                    return;
                }
            }
            2=>{
                if self.line_height[self.moving_line] > self.moving_pos.y - BLOCK_SIZE - down_diff
                {
                    self.moving_pos.y = self.line_height[self.moving_line] + BLOCK_SIZE;
                    return;
                }
            }
            3=>
            {
                if  self.line_height[self.moving_line] > self.moving_pos.y - 1.0 ||
                self.line_height[self.moving_line - 1] > self.moving_pos.y - 1.0
                {
                    self.moving_pos.y = f32::max(self.line_height[self.moving_line], self.line_height[self.moving_line - 1]);
                    return;
                }
            }
            _=>{}
        }
        self.moving_pos.y -= down_diff;
    }

    fn can_down(&self, down_diff : f32) -> bool
    {
        // 下に物があるか
        match self.moving_dir_idx
        {
            0=>{
                if self.line_height[self.moving_line] > self.moving_pos.y - down_diff
                {
                    return false;
                }
            }
            1=>{
                if  self.line_height[self.moving_line] > self.moving_pos.y - down_diff ||
                self.line_height[self.moving_line + 1] > self.moving_pos.y - down_diff
                {
                    return false;
                }
            }
            2=>{
                if self.line_height[self.moving_line] > self.moving_pos.y - BLOCK_SIZE - down_diff
                {
                    return false;
                }
            }
            3=>
            {
                if  self.line_height[self.moving_line] > self.moving_pos.y - 1.0 ||
                self.line_height[self.moving_line - 1] > self.moving_pos.y - 1.0
                {
                    return false;
                }
            }
            _=>{}
        }
        return true;
    }

    fn can_left(&self) -> bool
    {
        match self.moving_dir_idx
        {
            0=>
            {
                if self.line_height[self.moving_line - 1] > self.moving_pos.y
                {
                    return false;
                }
            }
            1=>
            {
                if self.line_height[self.moving_line - 1] > self.moving_pos.y
                {
                    return false;
                }
            }
            2=>
            {
                if self.line_height[self.moving_line - 1] > self.moving_pos.y - BLOCK_SIZE
                {
                    return false;
                }
            }
            3=>
            {
                if self.line_height[self.moving_line - 2] > self.moving_pos.y
                {
                    return false;
                }
            }
            _=>{}
        }
        return true;
    }
    fn can_right(&self) -> bool
    {
        match self.moving_dir_idx
        {
            0=>
            {
                if self.line_height[self.moving_line + 1] > self.moving_pos.y
                {
                    return false;
                }
            }
            1=>
            {
                if self.line_height[self.moving_line + 2] > self.moving_pos.y
                {
                    return false;
                }
            }
            2=>
            {
                if self.line_height[self.moving_line + 1] > self.moving_pos.y - BLOCK_SIZE
                {
                    return false;
                }
            }
            3=>
            {
                if self.line_height[self.moving_line + 1] > self.moving_pos.y
                {
                    return false;
                }
            }
            _=>{}
        }
        return true;
    }

    fn delete(&mut self) -> Vec<(usize, usize)>
    {
        let mut ch_ar = self.arrange.clone();
        let mut v_ret = Vec::new();

        self.rensa_num += 1;
        let mut bonus_multiply = RENSA_BONUS[self.rensa_num as usize];
        let mut ap_col : [bool;6] = [false;6];

        for i in 1..(FIELD_WIDTH-1)
        {
            for j in 1..=FIELD_DISP_HEIGHT
            {
                let col = ch_ar[i][j];
                if col <= 0 || col >= 6
                {
                    continue;
                }
                let mut v = delete_sub(&mut ch_ar, col, i, j);
                if v.length() >= 4
                {
                    for e in &v
                    {
                        self.arrange[e.0][e.1] = 0;
                    }
                    
                    bonus_multiply += RENKETSU_BONUS[v.length().min(11) as usize];
                    ap_col[col as usize] = true;
                    v_ret.append(&mut v);
                }
            }
        }

        let mut col_num = 0;
        for ap in ap_col
        {
            if ap
            {
                col_num += 1;
            }
        }
        bonus_multiply += COLOR_BONUS[col_num as usize];

        self.base_score = v_ret.length() as i32 * 10;
        self.bonus_multiply = bonus_multiply.max(1);

        // お邪魔ぷよを消す
        let mut v_ojm : Vec<(usize, usize)> = vec![];

        for e in &v_ret
        {
            if self.arrange[e.0 - 1][e.1] == 6 
            {
                self.arrange[e.0 - 1][e.1] = 0;
                v_ojm.push((e.0 - 1, e.1));
            }
            if self.arrange[e.0][e.1 - 1] == 6 
            {
                self.arrange[e.0][e.1 - 1] = 0;
                v_ojm.push((e.0, e.1 - 1));
            }
            if self.arrange[e.0 + 1][e.1] == 6 
            {
                self.arrange[e.0 + 1][e.1] = 0;
                v_ojm.push((e.0 + 1, e.1));
            }
            if self.arrange[e.0][e.1 + 1] == 6 && e.1 < FIELD_DISP_HEIGHT 
            {
                self.arrange[e.0][e.1 + 1] = 0;
                v_ojm.push((e.0, e.1 + 1));
            }
        }

        v_ret.append(&mut v_ojm);
        return v_ret;
    }

}
    
fn delete_sub(ch_ar : &mut [[i32;FIELD_HEIGHT];FIELD_WIDTH] , col : i32, i : usize, j : usize) -> Vec<(usize, usize)>
{
    let mut v = Vec::new();
    if j > FIELD_DISP_HEIGHT
    {
        return v;
    }

    if ch_ar[i][j] != col
    {
        return v;
    }

    ch_ar[i][j] = -1;
    v.push((i,j));
    v.append(&mut delete_sub(ch_ar, col, i+1,j));
    v.append(&mut delete_sub(ch_ar, col, i-1,j));
    v.append(&mut delete_sub(ch_ar, col, i,j+1));
    v.append(&mut delete_sub(ch_ar, col, i,j-1));
    return v;
}

fn setup<T : Component + Default>(
    commands : &mut Commands,
    fvr : Res<FieldValueResource<T>>,
    mut e_reset : EventWriter<game_statics::Reset>,
)
{
    let values = fvr.values;

    commands.spawn(Field{
        field_state : FieldState::Wait,
        fall_count : INIT_FALL_COUNT,
        base_pos : Vec2{x : values.field_base_x, y : values.field_base_y},
        ..default()
    })
    .insert(T::default());

    commands.spawn(Text2dBundle{
        text : Text::from_section(
            "", 
            TextStyle{
                font_size: BLOCK_INNER_SIZE,
                ..default()
            }),
            text_2d_bounds : Text2dBounds{
                size : Vec2 { x : BLOCK_SIZE * FIELD_WIDTH as f32, y : BLOCK_SIZE},
                ..default()
            },
            transform : Transform {
                translation : Vec3 {x:values.field_base_x + BLOCK_SIZE * 6.0, y:values.field_base_y, z : 10.0},
                ..default()
            },
            text_anchor : bevy::sprite::Anchor::CenterRight,
            ..default()
        })
        .insert(T::default());

    // 1P側からのみ送信
    if fvr.values.player_id == 0
    {
        e_reset.send(game_statics::Reset(thread_rng().gen::<u64>()));
    }
}

fn create_next<T : Component + Default>(
    commands : &mut Commands,
    values : &FieldValues,
    rng : &mut StdRng,
)
{
    let col0 = (rng.gen::<u32>() % 4) as i32 + 1;
    let col1 = (rng.gen::<u32>() % 4) as i32 + 1;

    commands.spawn((
        SpriteBundle{
            transform : Transform { 
                translation : Vec3{x:values.next_x, y:values.next_y, z : 0.0},
                scale : Vec3::new(BLOCK_INNER_SIZE, BLOCK_INNER_SIZE,1.0),
                ..default()
            },
            sprite: Sprite{
                color: COLORS[col0 as usize],
                ..default()
            },
            ..default()
        },
        Block{
            is_next : true,
            is_primary : true,
            color_idx : col0,
            ..default()
        },
    ))
    .insert(T::default());

    commands.spawn((
        SpriteBundle{
            transform : Transform { 
                translation : Vec3{x:values.next_x, y:values.next_y + BLOCK_SIZE, z : 0.0},
                scale : Vec3::new(BLOCK_INNER_SIZE, BLOCK_INNER_SIZE,1.0),
                ..default()
            },
            sprite: Sprite{
                color: COLORS[col1 as usize],
                ..default()
            },
            ..default()
        },
        Block{
            is_next : true,
            is_primary : false,
            color_idx : col1,
            ..default()
        },
    ))
    .insert(T::default());
}

fn update_keys<T : Component + Default>(
    key_input : Res<Input<KeyCode>>,
    //axes : Res<Input<GamepadAxis>>,
    //buttons : Res<Input<GamepadButton>>,
    mut key_resource : ResMut<KeyResource<T>>,
    fvr : Res<FieldValueResource<T>>,
)
{
    // reset
    key_resource.reset();

    // key_input
    key_resource.down |= key_input.pressed(u32_to_keycode(fvr.values.key_down));
    key_resource.left |= key_input.pressed(u32_to_keycode(fvr.values.key_left));
    key_resource.right |= key_input.pressed(u32_to_keycode(fvr.values.key_right));
    key_resource.just_left |= key_input.just_pressed(u32_to_keycode(fvr.values.key_left));
    key_resource.just_right |= key_input.just_pressed(u32_to_keycode(fvr.values.key_right));
    key_resource.just_rot_r |= key_input.just_pressed(u32_to_keycode(fvr.values.key_rot_r));
    key_resource.just_rot_l |= key_input.just_pressed(u32_to_keycode(fvr.values.key_rot_l));

}

fn update<T : Component + Default>(
    mut commands : Commands,
    key_resource : ResMut<KeyResource<T>>,
    mut query : Query<(Entity, &mut Transform, &mut Block, &mut Sprite), With<T>>,
    mut q_field : Query<&mut Field, With<T>>,
    fvr : Res<FieldValueResource<T>>,
    mut rng : ResMut<NextRngResource<T>>,
    mut e_gc : EventReader<game_statics::TransitionEvent>,
    mut e_death : EventWriter<game_statics::Death>,
    mut e_ojama : EventWriter<game_statics::Ojama>,
    mut e_reset : EventWriter<game_statics::Reset>,
) {
    let values = fvr.values;
    let mut field;
    if let Ok(c) = q_field.get_single_mut()
    {
        field = q_field.single_mut();
    }
    else {
        self::setup::<T>(&mut commands, fvr, e_reset);
        return;
    }

    match field.field_state
    {
        FieldState::Init => {
            for (entity,_,_,_) in query.iter()
            {
                commands.entity(entity).despawn();
            }

            for i in 0..FIELD_WIDTH
            {
                for j in 0..FIELD_HEIGHT
                {
                    if i == 0 || i == FIELD_WIDTH -1 || j == 0
                    {
                        field.arrange[i][j] = ColorIndices::Wall as i32;
                        let pos = Vec3{x : field.base_pos.x + i as f32 * BLOCK_SIZE , y : field.base_pos.y + j as f32 * BLOCK_SIZE , z : 1.0};

                        commands.spawn((
                            SpriteBundle{
                                transform : Transform { 
                                    translation : pos,
                                    scale : Vec3::new(BLOCK_SIZE, BLOCK_SIZE,1.0),
                                    ..default()
                                },
                                sprite: Sprite{
                                    color: COLORS[ColorIndices::Wall as usize],
                                    ..default()
                                },
                                ..default()
                            },
                            Block{
                                is_move : false,

                                expect_field : (i, j),
                                ..default()
                            },
                        ))
                        .insert(T::default());
                    }
                    else {
                        field.arrange[i][j] = 0;
                    }
                }
            }

            // フィールドの上端と見えない上側を隠すやつ
            commands.spawn((
                SpriteBundle{
                    transform : Transform { 
                        translation : Vec3::new(field.base_pos.x + BLOCK_SIZE * 3.5, field.base_pos.y + BLOCK_SIZE * 13.0, 10.0),
                        scale : Vec3::new(BLOCK_SIZE * 8.0, BLOCK_SIZE,1.0),
                        ..default()
                    },
                    sprite: Sprite{
                        color: COLORS[ColorIndices::Wall as usize],
                        ..default()
                    },
                    ..default()
                },
            ))
            .insert(T::default());
            commands.spawn((
                SpriteBundle{
                    transform : Transform { 
                        translation : Vec3::new(field.base_pos.x + BLOCK_SIZE * 3.5, field.base_pos.y + BLOCK_SIZE * 18.5, 10.0),
                        scale : Vec3::new(BLOCK_SIZE * 8.0, BLOCK_SIZE * 10.0 ,1.0),
                        ..default()
                    },
                    sprite: Sprite{
                        color: Color::BLACK,
                        ..default()
                    },
                    ..default()
                },
            ))
            .insert(T::default());

            create_next::<T>(&mut commands, &values, &mut rng.rng);
            field.score = 0;
            field.kakutei_ojama = 0;
            field.kari_ojama = 0;
            field.ojama_sent_score = 0;
            field.ojama_count = OJAMA_COUNT;
            field.field_state = FieldState::Wait;
        }

        FieldState::Wait =>
        {
            // NOP
        }

        FieldState::Next => {
            if field.arrange[3][12] != 0
            {
                field.field_state = FieldState::Lose;
                return;
            }
            field.rensa_num = 0;

            field.moving_line = 3;
            field.fall_next_count = 0;
            field.moving_lr_count = 0;

            let movefx = 3;
            let movefy = 13;

            field.moving_dir_idx = 0;
            field.moving_pos = Vec2{x : field.base_pos.x + movefx as f32 * BLOCK_SIZE , y : field.base_pos.y + movefy as f32 * BLOCK_SIZE};


            for (_,_, mut block,_) in &mut query
            {
                if !block.is_next
                {
                    continue;
                }
                block.is_next = false;
                block.is_move = true;
                if block.is_primary {
                    field.moving_color_idx.0 = block.color_idx;
                }
                else {
                    field.moving_color_idx.1 = block.color_idx;
                }
            }

            create_next::<T>(&mut commands, &values, &mut rng.rng);

            field.update_line_height();
            field.field_state = FieldState::Move;
        }

        FieldState::Move => {
            // 落下
            if !field.on_ground
            {
                field.fall_next_count += 1;
            }
            if key_resource.down
            {
                field.fall_next_count += field.fall_count;
                if field.on_ground
                {
                    field.fix_count += FIX_COUNT;
                }
            }
            if field.fall_next_count >= field.fall_count
            {
                if key_resource.down
                {
                    if field.fall_down_count % 2 == 0
                    {
                        field.score += 1;
                    }
                    field.fall_down_count += 1;
                }
                else {
                    field.fall_down_count = 0;
                }

                field.fall_next_count = 0;
                if !field.can_down(FALL_DOWN_Y * 0.5)
                {
                    field.on_ground = true;
                    field.fall_next_count = 0;
                }
                field.randing(FALL_DOWN_Y);
            }

            // 左
            let mut move_left = false;
            if key_resource.just_left 
            {
                move_left = true;
                field.moving_lr_count = MOVING_FIRST_COUNT;
            }
            if key_resource.left 
            {
                field.moving_lr_count -= 1;
                if field.moving_lr_count == 0
                {
                    move_left = true;
                    field.moving_lr_count = MOVING_CONT_COUNT;
                }
            }

            if move_left
            {
                if !field.can_left()
                {
                    move_left = false;
                }
            }
            if move_left
            {
                field.moving_line -= 1;
                field.moving_pos.x -= BLOCK_SIZE;
            }

            // 右
            let mut move_right = false;
            if key_resource.just_right
            {
                move_right = true;
                field.moving_lr_count = MOVING_FIRST_COUNT;
            }
            if key_resource.right
            {
                field.moving_lr_count -= 1;
                if field.moving_lr_count == 0
                {
                    move_right = true;
                    field.moving_lr_count = MOVING_CONT_COUNT;
                }
            }

            if move_right
            {
                if !field.can_right()
                {
                    move_right = false;
                }
            }
            if move_right
            {
                field.moving_line += 1;
                field.moving_pos.x += BLOCK_SIZE;
            }

            // 右回転
            if key_resource.just_rot_r
            {
                field.moving_prev_dir_idx = field.moving_dir_idx;
                field.moving_rot_count = ROTATE_COUNT;
                match field.moving_dir_idx
                {
                    0=>
                    {
                        field.moving_line -= 1;
                        field.moving_dir_idx = 1;
                        if field.can_right()
                        {
                            field.moving_line += 1;
                        }
                        else
                        {
                            field.moving_line += 1;
                            if field.can_left()
                            {
                                field.moving_line -= 1;
                                field.moving_pos.x -= BLOCK_SIZE;
                            }    
                            else {
                                field.moving_dir_idx = 0;
                                field.moving_rot_count = 0;
                            }
                        }
                    }
                    1=>
                    {
                        field.moving_dir_idx = 2;
                        if !field.can_down(0.0)
                        {
                            field.randing(0.0);
                            field.on_ground = true;
                        }
                    }
                    2=>
                    {
                        field.moving_line += 1;
                        field.moving_dir_idx = 3;
                        if field.can_left()
                        {
                            field.moving_line -= 1;
                        }
                        else
                        {
                            field.moving_line -= 1;
                            if field.can_right()
                            {
                                field.moving_line += 1;
                                field.moving_pos.x += BLOCK_SIZE;
                            }    
                            else {
                                field.moving_dir_idx = 2;
                                field.moving_rot_count = 0;
                            }
                        }
                    }
                    3=>
                    {
                        field.moving_dir_idx = 0;
                    }
                    _=>{}
                }
            }

            // 左回転
            if key_resource.just_rot_l
            {
                field.moving_prev_dir_idx = field.moving_dir_idx;
                field.moving_rot_count = ROTATE_COUNT;
                match field.moving_dir_idx
                {
                    0=>
                    {
                        field.moving_line += 1;
                        field.moving_dir_idx = 3;
                        if field.can_left()
                        {
                            field.moving_line -= 1;
                        }
                        else
                        {
                            field.moving_line -= 1;
                            if field.can_right()
                            {
                                field.moving_line += 1;
                                field.moving_pos.x += BLOCK_SIZE;
                            }    
                            else {
                                field.moving_dir_idx = 0;
                                field.moving_rot_count = 0;
                            }
                        }
                    }
                    1=>
                    {
                        field.moving_dir_idx = 0;
                    }
                    2=>
                    {
                        field.moving_line -= 1;
                        field.moving_dir_idx = 1;
                        if field.can_right()
                        {
                            field.moving_line += 1;
                        }
                        else
                        {
                            field.moving_line += 1;
                            if field.can_left()
                            {
                                field.moving_line -= 1;
                                field.moving_pos.x -= BLOCK_SIZE;
                            }    
                            else {
                                field.moving_dir_idx = 2;
                                field.moving_rot_count = 0;
                            }
                        }
                    }
                    3=>
                    {
                        field.moving_dir_idx = 2;
                        if !field.can_down(0.0)
                        {
                            field.randing(0.0);
                            field.on_ground = true;
                        }
                    }
                    _=>{}
                }
            }

            // 離れ判定
            if field.on_ground
            {
                if field.can_down(FALL_DOWN_Y * 0.5)
                {
                    field.on_ground = false;
                }
            }

            // 固着判定
            let mut is_fix = false;
            if field.on_ground
            {
                field.fix_count += 1;
                if field.fix_count >= FIX_COUNT 
                {
                    field.fix_count = 0;
                    field.moving_rot_count = 0;
                    field.on_ground = false;
                    is_fix = true;

                    // 固着処理
                    let mut block_fpos_p = (0,0);
                    let mut block_fpos_s = (0,0);
                    match field.moving_dir_idx 
                    {
                        0=>{
                            let moving_line = field.moving_line;
                            let line = &field.arrange[moving_line];
                            for i in 0..FIELD_HEIGHT
                            {
                                if line[i] == 0
                                {
                                    field.arrange[moving_line][i] = field.moving_color_idx.0;
                                    field.arrange[moving_line][i + 1] = field.moving_color_idx.1;
                                    block_fpos_p = (moving_line, i);
                                    block_fpos_s = (moving_line, i + 1);
                                    break;
                                }
                            }
                        }
                        1=>{
                            let moving_line = field.moving_line;
                            let line_0 = &field.arrange[moving_line];
                            let line_1 = &field.arrange[moving_line + 1];
                            for i in 0..FIELD_HEIGHT
                            {
                                if line_0[i] == 0 && line_1[i] == 0
                                {
                                    field.arrange[moving_line][i] = field.moving_color_idx.0;
                                    field.arrange[moving_line + 1][i] = field.moving_color_idx.1;
                                    block_fpos_p = (moving_line, i);
                                    block_fpos_s = (moving_line + 1, i);
                                    break;
                                }
                            }
                        }
                        2=>{
                            let moving_line = field.moving_line;
                            let line = &field.arrange[moving_line];
                            for i in 0..FIELD_HEIGHT
                            {
                                if line[i] == 0
                                {
                                    field.arrange[moving_line][i] = field.moving_color_idx.1;
                                    field.arrange[moving_line][i + 1] = field.moving_color_idx.0;
                                    block_fpos_s = (moving_line, i);
                                    block_fpos_p = (moving_line, i + 1);
                                    break;
                                }
                            }
                        }
                        3=>{
                            let moving_line = field.moving_line;
                            let line_0 = &field.arrange[moving_line];
                            let line_1 = &field.arrange[moving_line - 1];
                            for i in 0..FIELD_HEIGHT
                            {
                                if line_0[i] == 0 && line_1[i] == 0
                                {
                                    field.arrange[moving_line][i] = field.moving_color_idx.0;
                                    field.arrange[moving_line - 1][i] = field.moving_color_idx.1;
                                    block_fpos_p = (moving_line, i);
                                    block_fpos_s = (moving_line - 1, i);
                                    break;
                                }
                            }
                        }
                        _=>{}
                    }

                    for (_,_, mut block,_) in &mut query
                    {
                        if block.is_move 
                        {
                            if block.is_primary 
                            {
                                block.expect_field = block_fpos_p;
                            }
                            else
                            {
                                block.expect_field = block_fpos_s;
                            }
                        }
                    }
                }
            }

            // 位置更新
            if field.moving_rot_count > 0
            {
                field.moving_rot_count -= 1;
            }
            for (_,mut transform, block,_) in &mut query
            {
                if block.is_move 
                {
                    if block.is_primary 
                    {
                        transform.translation.x = field.moving_pos.x;
                        transform.translation.y = field.moving_pos.y;
                    }
                    else
                    {
                        let mut sdir = field.moving_prev_dir_idx;
                        let mut edir = field.moving_dir_idx;
                        if sdir == 0 && edir == 3
                        {
                            sdir = 4;
                        }
                        if sdir == 3 && edir == 0
                        {
                            edir = 4;
                        } 
                        let div = field.moving_rot_count as f32 / ROTATE_COUNT as f32;
                        let rot = sdir as f32 * div + edir as f32 * (1.0 - div);
                        let rot_theta = rot / 2.0 * PI;

                        transform.translation.x = field.moving_pos.x + rot_theta.sin() * BLOCK_SIZE;
                        transform.translation.y = field.moving_pos.y + rot_theta.cos() * BLOCK_SIZE;
                    }
                }
            }

            // 最終固着処理
            if is_fix
            {
                field.field_state = FieldState::FallCheck;

                for (_,_, mut block,_) in &mut query
                {
                    if block.is_move 
                    {
                        block.shake_count = SHAKE_COUNT;
                        block.is_move = false;
                    }
                }
            }
        }

        FieldState::FallCheck => {
            for i in 1..(FIELD_WIDTH-1)
            {
                let mut fall_count = 0;
                for j in 1..FIELD_HEIGHT
                {
                    if field.arrange[i][j] == 0
                    {
                        fall_count += 1;
                    }
                    else if fall_count != 0
                    {
                        // ブロックを探す
                        for (_,_, mut block,_)in &mut query
                        {
                            if block.expect_field.0 == i && block.expect_field.1 == j
                            {
                                block.expect_field = (i, j-fall_count);
                                block.fall_y = fall_count as f32 * BLOCK_SIZE;
                                break;
                            }
                        }
                        field.arrange[i][j-fall_count] = field.arrange[i][j];
                        field.arrange[i][j] = 0;
                    }
                }             
            }
            field.field_state = FieldState::Fall;
        }
        FieldState::Fall => {
            let mut fall_end = true;
            for (_,mut transform, mut block,_) in &mut query
            {
                if block.fall_y > 0.0
                {
                    fall_end = false;
                    if block.fall_y > FALL_DOWN_Y 
                    {
                        transform.translation.y -= FALL_DOWN_Y;
                        block.fall_y -= FALL_DOWN_Y;
                    }
                    else {
                        transform.translation.y -= block.fall_y;
                        block.fall_y = 0.0;
                        block.shake_count = SHAKE_COUNT;
                    }
                }
                let bsc = block.shake_count as f32;
                if block.shake_count > 0
                {
                    fall_end = false;
                    transform.scale.x = (block.expect_field.0 % 2 + 2) as f32 * bsc.cos() * (bsc / SHAKE_COUNT as f32) + BLOCK_INNER_SIZE;
                    transform.scale.y = (block.expect_field.1 % 2 + 2) as f32 * bsc.sin() * (bsc / SHAKE_COUNT as f32) + BLOCK_INNER_SIZE;
                    block.shake_count -= 1;
                }
            }
            if fall_end 
            {
                field.field_state = FieldState::DeleteCheck;
            }
        }
        FieldState::DeleteCheck =>{
            let v = field.delete();
            let mut dg = 0;
            for (_,_, mut block,_) in &mut query
            {
                for e in &v
                {
                    if block.expect_field != *e {
                        continue;
                    }
                    block.is_delete = true;
                    block.delete_group = dg;
                    dg = (dg + 1) % DELETE_GROUP;
                }
            }
            if v.length() != 0
            {
                field.delete_count = 0;
                field.field_state = FieldState::Delete;
            }
            else {
                field.field_state = FieldState::OjamaFallCheck;
            }
        }
        FieldState::Delete =>
        {
            if field.delete_count < DELETE_BLINK 
            {
                let mut alpha = 1.0;
                if field.delete_count % (DELETE_BLINK_INTERVAL * 2) < DELETE_BLINK_INTERVAL
                {
                    alpha = 0.0;
                }
                for (_, _, block, mut sprite) in &mut query
                {
                    if !block.is_delete
                    {
                        continue;
                    }
                    if block.color_idx == ColorIndices::Ojama as i32
                    {
                        continue;
                    }
                    sprite.color.set_a(alpha);
                }
            }
            else {
                if field.delete_count == DELETE_BLINK
                {
                    for (_, _, block, mut sprite) in &mut query
                    {
                        if !block.is_delete
                        {
                            continue;
                        }
                        if block.color_idx == ColorIndices::Ojama as i32
                        {
                            continue;
                        }

                        let sp_col = sprite.color;
                        sprite.color.set_r((sp_col.r() + 1.0) * 0.5);
                        sprite.color.set_g((sp_col.g() + 1.0) * 0.5);
                        sprite.color.set_b((sp_col.b() + 1.0) * 0.5);
                        sprite.color.set_a(1.0);
                    }

                    // 点数反映
                    field.score += field.base_score * field.bonus_multiply;
                }
                 
                let ojama_a = ((DELETE_END - field.delete_count) as f32 / (DELETE_END - DELETE_BLINK) as f32) * COLORS[ColorIndices::Ojama as usize].a();
                for (entity, _, block, mut sprite) in &mut query
                {
                    if !block.is_delete
                    {
                        continue;
                    }

                    if block.color_idx == ColorIndices::Ojama as i32
                    {
                        sprite.color.set_a(ojama_a);
                        if field.delete_count == DELETE_END
                        {
                            commands.entity(entity).despawn();
                        }
                    }
                    else
                    {
                        let delete_timing = block.delete_group * (DELETE_END - DELETE_BLINK) / DELETE_GROUP + DELETE_BLINK;
                        if delete_timing <= field.delete_count
                        {
                            commands.entity(entity).despawn();
                        }
                    }
                }
            }

            if field.delete_count > DELETE_END
            {
                // 仮お邪魔送り
                let ojama_num = (field.score - field.ojama_sent_score) / SCORE_PER_OJAMA;
                field.ojama_sent_score += SCORE_PER_OJAMA * ojama_num;
                e_ojama.send(game_statics::Ojama{player_id : fvr.values.player_id, num : ojama_num, frame : 0});

                field.field_state = FieldState::FallCheck;
            }
            field.delete_count += 1;
        }

        FieldState::OjamaFallCheck => {
            // お邪魔確定処理
            e_ojama.send(game_statics::Ojama{player_id : fvr.values.player_id, num : 0, frame : field.elapsed_frame});

            if field.kakutei_ojama == 0
            {
                field.field_state = FieldState::Next;
            }
            else
            {
                // 落とす場所を決める
                let mut ojm_num :[i32; FIELD_WIDTH] = Default::default();
                if field.kakutei_ojama >= 30
                {
                    for i in 1..=6 {
                        ojm_num[i] = 5;
                    }
                }
                else {
                    let dan = field.kakutei_ojama / 6;
                    for i in 1..=6 {
                        ojm_num[i] = dan;
                    }
                    let mut rem = field.kakutei_ojama - dan * 6;
                    while rem > 0{
                        let oc = (thread_rng().gen::<u32>() % 6) as usize + 1;
                        if ojm_num[oc] == dan
                        {
                            ojm_num[oc] += 1;
                            rem -= 1;
                        }
                    }
                }

                // 実際に落とす個数を決める
                field.update_line_height();
                for i in 1..=6 {
                    ojm_num[i] = ojm_num[i].min(13 - field.line_dan[i]);
                }

                let mut ojm_sum = 0;
                for i in 1..=6 {
                    ojm_sum += ojm_num[i];
                }

                if ojm_sum == 0
                {
                    field.field_state = FieldState::Next;
                }
                else {
                    // 落とす場所にお邪魔を生成する
                    for i in 1..=6 {
                        let fall_count = 14 - field.line_dan[i];
                        for j in 0..ojm_num[i]{

                            let block_x = field.base_pos.x + i as f32 * BLOCK_SIZE;
                            let block_y = field.base_pos.y + (14.0 + j as f32) * BLOCK_SIZE;
                            let block_fy = field.line_dan[i] + j;
                            commands.spawn((
                                SpriteBundle{
                                    transform : Transform { 
                                        translation : Vec3{x:block_x, y:block_y, z : 0.0},
                                        scale : Vec3::new(BLOCK_INNER_SIZE, BLOCK_INNER_SIZE,1.0),
                                        ..default()
                                    },
                                    sprite: Sprite{
                                        color: COLORS[ColorIndices::Ojama as usize],
                                        ..default()
                                    },
                                    ..default()
                                },
                                Block{
                                    expect_field : (i, block_fy as usize),
                                    fall_y : fall_count as f32 * BLOCK_SIZE,
                                    is_next : false,
                                    is_primary : false,
                                    color_idx : ColorIndices::Ojama as i32,
                                    ..default()
                                },
                            ))
                            .insert(T::default());

                            field.arrange[i][block_fy as usize] = ColorIndices::Ojama as i32;

                        }
                    }
                    field.field_state = FieldState::OjamaFall;
                }

                // 個数と表示の更新
                field.kakutei_ojama -= ojm_sum;
                field.ojama_count = OJAMA_COUNT;
            }
        }

        FieldState::OjamaFall => {
            let mut fall_end = true;
            for (_,mut transform, mut block,_) in &mut query
            {
                if block.fall_y > 0.0
                {
                    fall_end = false;
                    if block.fall_y > FALL_DOWN_Y 
                    {
                        transform.translation.y -= FALL_DOWN_Y;
                        block.fall_y -= FALL_DOWN_Y;
                    }
                    else {
                        transform.translation.y -= block.fall_y;
                        block.fall_y = 0.0;
                        block.shake_count = SHAKE_COUNT;
                    }
                }
                let bsc = block.shake_count as f32;
                if block.shake_count > 0
                {
                    fall_end = false;
                    transform.scale.x = (block.expect_field.0 % 2 + 2) as f32 * bsc.cos() * (bsc / SHAKE_COUNT as f32) + BLOCK_INNER_SIZE;
                    transform.scale.y = (block.expect_field.1 % 2 + 2) as f32 * bsc.sin() * (bsc / SHAKE_COUNT as f32) + BLOCK_INNER_SIZE;
                    block.shake_count -= 1;
                }
            }
            if fall_end 
            {
                field.field_state = FieldState::Next;
            }
        }

        FieldState::Win => {
            // NOP
        }

        FieldState::Lose => {
            field.field_state = FieldState::LoseFall;
            e_death.send(game_statics::Death(fvr.values.player_id));
        }

        FieldState::LoseFall => {
            // TODO 落ちる処理
        }
    }

    if field.field_state != FieldState::Wait
    {
        field.elapsed_frame += 1;
    }
}

fn update_ui<T : Component + Default>(
    mut q_text : Query<&mut Text, With<T>>,
    q_field : Query<&mut Field, With<T>>)
{
    if let Err(_) = q_field.get_single()
    {
        return;
    }
    let field = q_field.single();
    let mut text = q_text.single_mut();

    if field.field_state == FieldState::Delete
    {
        text.sections[0].value = field.base_score.to_string() + " x " + &field.bonus_multiply.to_string();
    }
    else
    {
        text.sections[0].value = field.score.to_string(); 
    }
}

fn receive_ojama<T : Component + Default>(
    mut q_field : Query<&mut Field, With<T>>,
    fvr : Res<FieldValueResource<T>>,
    mut ev_ojm : EventReader<game_statics::Ojama>)
{
    if let Err(_) = q_field.get_single()
    {
        return;
    }

    let mut field = q_field.single_mut();
    for ojm in ev_ojm.read()
    {
        if fvr.values.player_id == ojm.player_id {
            continue;
        }
        
        if ojm.frame != 0{
            // 確定
            // field.kakutei_frame = ojm.frame;
            field.kakutei_ojama += ojm.num + field.kari_ojama;
            field.kari_ojama = 0;
        }
        else{
            // 仮加算
            field.kari_ojama += ojm.num;
        }

        if ojm.num != 0
        {
            field.ojama_count = OJAMA_COUNT;
        }
    }
}

fn update_ojama<T : Component + Default>(
    mut commands : Commands,
    mut q_field : Query<&mut Field, With<T>>,
    mut query : Query<(Entity, &mut Transform, &mut OjamaDisp, &mut Sprite), With<T>>,
){
    if let Err(_) = q_field.get_single()
    {
        return;
    }

    let mut field = q_field.single_mut();
    let ojama_x = field.base_pos.x + BLOCK_SIZE * 3.5; 
    let ojama_y = field.base_pos.y + BLOCK_SIZE * 13.0; 

    if field.ojama_count == OJAMA_COUNT
    {
        // 初期化
        for (entity,_,_,_) in query.iter()
        {
            commands.entity(entity).despawn();
        }

        let mut pos_x = 0.0;
        let mut remain_ojama = field.kari_ojama + field.kakutei_ojama;
        loop {
            if remain_ojama >= 30
            {
                // 岩
                if pos_x + OJAMA_ROCK_SIZE > MAX_OJAMA_DISP_X {
                    break;
                }

                commands.spawn((
                    SpriteBundle{
                        transform : Transform { 
                            translation : Vec3{x : ojama_x, y : ojama_y, z : 20.0},
                            scale : Vec3::new(OJAMA_ROCK_INNER_SIZE, OJAMA_ROCK_INNER_SIZE,1.0),
                            ..default()
                        },
                        sprite: Sprite{
                            color: COLORS[1],
                            ..default()
                        },
                        ..default()
                    },
                    OjamaDisp{
                        ojama_type : OjamaType::Rock,
                        disp_x : pos_x + field.base_pos.x + BLOCK_SIZE,
                        ..default()
                    },
                ))
                .insert(T::default());

                pos_x += OJAMA_ROCK_SIZE;
                remain_ojama -= 30;
            }
            else if remain_ojama >= 6
            {
                // 大
                if pos_x + OJAMA_BIG_SIZE > MAX_OJAMA_DISP_X {
                    break;
                }

                commands.spawn((
                    SpriteBundle{
                        transform : Transform { 
                            translation : Vec3{x : ojama_x, y : ojama_y, z : 20.0},
                            scale : Vec3::new(OJAMA_BIG_INNER_SIZE, OJAMA_BIG_INNER_SIZE,1.0),
                            ..default()
                        },
                        sprite: Sprite{
                            color: Color::WHITE,
                            ..default()
                        },
                        ..default()
                    },
                    OjamaDisp{
                        ojama_type : OjamaType::Big,
                        disp_x : pos_x + field.base_pos.x + BLOCK_SIZE,
                        ..default()
                    },
                ))
                .insert(T::default());

                pos_x += OJAMA_BIG_SIZE;
                remain_ojama -= 6;
            }
            else if remain_ojama >= 1
            {
                // 小
                if pos_x + OJAMA_SMALL_SIZE > MAX_OJAMA_DISP_X {
                    break;
                }

                commands.spawn((
                    SpriteBundle{
                        transform : Transform { 
                            translation : Vec3{x : ojama_x, y : ojama_y, z : 20.0},
                            scale : Vec3::new(OJAMA_SMALL_INNER_SIZE, OJAMA_SMALL_INNER_SIZE,1.0),
                            ..default()
                        },
                        sprite: Sprite{
                            color: Color::WHITE,
                            ..default()
                        },
                        ..default()
                    },
                    OjamaDisp{
                        ojama_type : OjamaType::Small,
                        disp_x : pos_x+ field.base_pos.x + BLOCK_SIZE,
                        ..default()
                    },
                ))
                .insert(T::default());

                pos_x += OJAMA_SMALL_SIZE;
                remain_ojama -= 1;
            }
            else {
                break;
            }
        }
    }
    else if field.ojama_count >= 0
    {
        let rate = field.ojama_count as f32 / OJAMA_COUNT as f32;
        // 更新
        for (_,mut transform,ojama_disp,_) in &mut query
        {
            transform.translation.x = ojama_x * rate + ojama_disp.disp_x * (1.0 - rate);
        }
    }

    if field.ojama_count >= 0
    {
        field.ojama_count -= 1;
    }
}

fn receive_command<T : Component + Default>(
    fvr : Res<FieldValueResource<T>>,
    mut rng : ResMut<NextRngResource<T>>,
    mut e_gc : EventReader<game_statics::TransitionEvent>,
    mut e_reset : EventReader<game_statics::Reset>,
    mut q_field : Query<&mut Field, With<T>>,
)
{
    if let Err(_) = q_field.get_single()
    {
        return;
    }
    let mut field = q_field.single_mut();

    for cmd in e_gc.read()
    {
        if cmd.transition == game_statics::Transition::Finish
        {
            if fvr.values.player_id != cmd.player_id
            {
                field.field_state = FieldState::Win;
            }
        }

        if cmd.transition == game_statics::Transition::Start
        {
            field.field_state = FieldState::Next;
        }
    }

    for rst in e_reset.read()
    {
        rng.rng = rand::SeedableRng::seed_from_u64(rst.0);
        field.field_state = FieldState::Init;
    }
}

fn nothing<T : Component + Default>(
    mut commands : Commands,
    qe : Query<Entity, With<T>>)
{
    for e in qe.iter()
    {
        commands.entity(e).despawn();
    }
}

#[derive(Component, Default)]
struct Player1;

#[derive(Component, Default)]
struct Player2;

#[derive(Default)]
pub struct PlayerFieldPlugin{
    pub values : FieldValues,
}

#[derive(Default, Component, Clone, Copy)]
pub struct FieldValues
{
    pub field_base_x : f32,
    pub field_base_y : f32,
    pub next_x : f32,
    pub next_y : f32,
    pub player_id : i32,
    pub game_mode : i32,

    pub key_left : u32,
    pub key_right : u32,
    pub key_down : u32,
    pub key_rot_r : u32,
    pub key_rot_l : u32,
}

#[derive(Default, Resource)]
pub struct FieldValueResource<T>
{
    pub values : FieldValues,
    phantom : PhantomData<T>,
}


#[derive(Default, Resource)]
pub struct KeyResource<T>
{
    pub down : bool,
    pub left : bool,
    pub right : bool,
    pub just_left : bool,
    pub just_right : bool,
    pub just_rot_r : bool,
    pub just_rot_l : bool,
    phantom : PhantomData<T>,
}

impl<T> KeyResource<T>
{
    pub fn reset(&mut self)
    {
        self.down = false;
        self.left = false;
        self.right = false;
        self.just_left = false;
        self.just_right = false;
        self.just_rot_r = false;
        self.just_rot_l = false;
    }
}

fn u32_to_keycode(n : u32) -> KeyCode
{
    match n {
        33 => KeyCode::X,
        35 => KeyCode::Z,
        70 => KeyCode::Left,
        71 => KeyCode::Up,
        72 => KeyCode::Right,
        73 => KeyCode::Down,
        _ => KeyCode::F24,
    }
}

#[derive(Resource)]
pub struct NextRngResource<T>
{
    pub rng : StdRng,
    phantom : PhantomData<T>,
}

impl Plugin for PlayerFieldPlugin{

    fn build(&self, app : &mut App) {
        match self.values.game_mode{
            0=>{
                    self.build_sub::<game_statics::GameMode1P, Player1>(app);
                }
            1=>{
                if self.values.player_id == 0
                {
                    self.build_sub::<game_statics::GameMode2PLocal, Player1>(app);
                }
                else
                {
                    self.build_sub::<game_statics::GameMode2PLocal, Player2>(app);
                }
            }
            2=>{
                if self.values.player_id == 0
                {
                    self.build_sub::<game_statics::GameMode2POnline, Player1>(app);
                }
                else
                {
                    self.build_sub::<game_statics::GameMode2POnline, Player2>(app);
                }
            }
            _=>{}
        }

    }

    fn is_unique(&self) -> bool {
        return false;
    }
}

impl PlayerFieldPlugin{
    pub fn set_player<T>(&self) -> &PlayerFieldPlugin
    {
        return self;
    }

    fn build_sub<R : Resource, C : Component + Default> (&self, app : &mut App){
        app
        .add_systems(PreUpdate, update_keys::<C>.run_if(resource_exists::<R>()))
        .add_systems(Update, update::<C>.run_if(resource_exists::<R>()))
        .add_systems(Update, receive_ojama::<C>.run_if(resource_exists::<R>()))
        .add_systems(Update, update_ui::<C>.run_if(resource_exists::<R>()))
        .add_systems(Update, update_ojama::<C>.run_if(resource_exists::<R>()))
        .add_systems(PostUpdate, receive_command::<C>.run_if(resource_exists::<R>()))
        .insert_resource(FieldValueResource::<C>{
            values : self.values, 
            ..default()
        })
        .insert_resource(KeyResource::<C>{
            ..default()
        })
        .insert_resource(NextRngResource::<C>{
            rng : rand::SeedableRng::from_seed([0;32]),
            phantom : PhantomData::<C>::default(),
        })
        ;
    }
}