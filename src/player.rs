use raylib::prelude::*;
use std::f32::consts::PI;

use crate::maze::{Maze};

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

pub fn process_events(window: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: f32) {
    let MOVE_SPEED: f32 = block_size / 10.0;
    const ROTATION_SPEED: f32 = PI / 50.0;

    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED; 
    } 
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED; 
    } 
    let mut i = ((player.pos.x + MOVE_SPEED * player.a.cos())/block_size.floor()) as usize;
    let mut j = ((player.pos.y + MOVE_SPEED * player.a.cos())/block_size.floor()) as usize;
    if window.is_key_down(KeyboardKey::KEY_UP) && maze[j][i] == ' ' {
        player.pos.x += MOVE_SPEED * player.a.cos(); 
        player.pos.y += MOVE_SPEED * player.a.sin();
    }
    i = ((player.pos.x - MOVE_SPEED * player.a.cos())/block_size.floor()) as usize;
    j = ((player.pos.y - MOVE_SPEED * player.a.cos())/block_size.floor()) as usize;
    if window.is_key_down(KeyboardKey::KEY_DOWN) && maze[j][i] == ' ' {
        player.pos.x -= MOVE_SPEED * player.a.cos(); 
        player.pos.y -= MOVE_SPEED * player.a.sin();
    } 
}