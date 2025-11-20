use raylib::prelude::*;
use std::f32::consts::PI;

use crate::maze::{Maze};

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

pub fn process_events(window: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: f32) {
    let move_speed: f32 = block_size / 10.0;
    const ROTATION_SPEED: f32 = PI / 50.0;

    // Rotate player
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED;
    }

    // Direction vector from angle
    let dir_x = player.a.cos();
    let dir_y = player.a.sin();

    // Helper closure to test if a world position is inside a wall
    let can_walk_to = |x: f32, y: f32| -> bool {
        if x < 0.0 || y < 0.0 {
            return false;
        }

        let i = (x / block_size) as usize;
        let j = (y / block_size) as usize;

        if j >= maze.len() || i >= maze[0].len() {
            return false;
        }

        maze[j][i] == ' '
    };

    // Move forward
    if window.is_key_down(KeyboardKey::KEY_UP) {
        // Try moving on X axis
        let next_x = player.pos.x + dir_x * move_speed;
        if can_walk_to(next_x, player.pos.y) {
            player.pos.x = next_x;
        }

        // Try moving on Y axis
        let next_y = player.pos.y + dir_y * move_speed;
        if can_walk_to(player.pos.x, next_y) {
            player.pos.y = next_y;
        }
    }

    // Move backward
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        // Try moving on X axis
        let next_x = player.pos.x - dir_x * move_speed;
        if can_walk_to(next_x, player.pos.y) {
            player.pos.x = next_x;
        }

        // Try moving on Y axis
        let next_y = player.pos.y - dir_y * move_speed;
        if can_walk_to(player.pos.x, next_y) {
            player.pos.y = next_y;
        }
    }
}