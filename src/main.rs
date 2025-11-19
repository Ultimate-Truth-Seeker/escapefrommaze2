// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod player;
mod caster;
mod sprite;
mod textures;

use raylib::prelude::*;
use std::{f32::consts::PI, thread};
use std::time::Duration;
use framebuffer::Framebuffer;
use line::line;
use maze::{Maze,load_maze};
use player::{Player, process_events};
use caster::{cast_ray};
use sprite::{};

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(Color::RED);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.set_pixel(player.pos.x as u32, player.pos.y as u32);
    //cast_ray(framebuffer, maze, player, block_size);
}

pub fn render_world(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("maze.txt");
    let block_size = 100;
    let num_rays = framebuffer.width;
    let hw = framebuffer.width as f32 /2.0;
    let hh = framebuffer.height as f32 /2.0;
    framebuffer.set_current_color(Color::WHITE);

    //let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, true);

        let mut distance_to_wall = intersect.distance;
        distance_to_wall *= (player.a - a).cos();
        let distance_to_projection_plane = hw / (player.fov / 2.0).tan();
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane * 0.25;

        let stake_top = (hh - (stake_height / 2.0)) as u32;
        let stake_bottom = (hh + (stake_height / 2.0)) as u32;

        for y in stake_top..stake_bottom {
            framebuffer.set_pixel(i, y);
        }
    }
}

fn main() {
    let window_width = 1500;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, Color::BLACK);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    // Load the maze once before the loop
    let maze = load_maze("maze.txt");
    let mut player = Player{
        pos: Vector2::new(200.0, 100.0),
        a: PI/3.0,
        fov: PI/3.0,
    };

    while !window.window_should_close() {
        // 1. clear framebuffer
        framebuffer.clear();
        
        // 2. draw the maze, passing the maze and block size
        process_events(&window, &mut player, &maze, block_size as f32);
        //render_maze(&mut framebuffer, &maze, block_size, &player);
        render_world(&mut framebuffer, &player);

        // 3. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        //thread::sleep(Duration::from_millis(16));
    }
}