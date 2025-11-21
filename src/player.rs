use raylib::prelude::*;
use std::f32::consts::PI;

use crate::maze::{Maze};

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
    pub health: i32,
}

impl Player {
    
    pub fn process_events(&mut self, window: &mut RaylibHandle, maze: &Maze, block_size: f32) {
        let move_speed: f32 = block_size / 25.0;
        const ROTATION_SPEED: f32 = PI / 50.0;
    
        // Rotate player
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            self.a -= ROTATION_SPEED;
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.a += ROTATION_SPEED;
        }
    
        let screen_w = window.get_screen_width();
        let screen_h = window.get_screen_height();
    
        let center = Vector2::new(
            (screen_w / 2) as f32,
            (screen_h / 2) as f32,
        );
        let mouse_pos = window.get_mouse_position(); 
        let sensitivity: f32 = 0.003;              // tune to taste
    
            // Only use horizontal movement to rotate
        self.a += (mouse_pos.x - center.x) * sensitivity;
        window.set_mouse_position(center);
    
        // Direction vector from angle
        let dir_x = self.a.cos();
        let dir_y = self.a.sin();
    
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
    
            maze[j][i] == ' ' || maze[j][i] == 'g' || maze[j][i] == 's'
        };
    
        // Move forward
        if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) {
            // Try moving on X axis
            let next_x = self.pos.x + dir_x * move_speed;
            if can_walk_to(next_x, self.pos.y) {
                self.pos.x = next_x;
            }
    
            // Try moving on Y axis
            let next_y = self.pos.y + dir_y * move_speed;
            if can_walk_to(self.pos.x, next_y) {
                self.pos.y = next_y;
            }
        }
    
        // Move backward
        if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) {
            // Try moving on X axis
            let next_x = self.pos.x - dir_x * move_speed;
            if can_walk_to(next_x, self.pos.y) {
                self.pos.x = next_x;
            }
    
            // Try moving on Y axis
            let next_y = self.pos.y - dir_y * move_speed;
            if can_walk_to(self.pos.x, next_y) {
                self.pos.y = next_y;
            }
        }
    
        // Move Rightward
        if window.is_key_down(KeyboardKey::KEY_D) {
            // Try moving on X axis
            let next_x = self.pos.x - dir_y * move_speed;
            if can_walk_to(next_x, self.pos.y) {
                self.pos.x = next_x;
            }
    
            // Try moving on Y axis
            let next_y = self.pos.y + dir_x * move_speed;
            if can_walk_to(self.pos.x, next_y) {
                self.pos.y = next_y;
            }
        }
    
        // Move leftward
        if window.is_key_down(KeyboardKey::KEY_A) {
            // Try moving on X axis
            let next_x = self.pos.x + dir_y * move_speed;
            if can_walk_to(next_x, self.pos.y) {
                self.pos.x = next_x;
            }
    
            // Try moving on Y axis
            let next_y = self.pos.y - dir_x * move_speed;
            if can_walk_to(self.pos.x, next_y) {
                self.pos.y = next_y;
            }
        }
    }
}