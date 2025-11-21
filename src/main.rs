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
mod gui;
mod game;

use raylib::prelude::*;
use std::{f32::consts::PI, thread};
use std::time::Duration;
use framebuffer::Framebuffer;
use line::line;
use maze::{Maze,load_maze};
use player::{Player};
use caster::{cast_ray};
use sprite::{};

use crate::game::{AppState, StateHandler, find_start_cell};
use crate::gui::screens::Screens;
use crate::sprite::draw_sprite;
use crate::textures::TextureManager;

use crate::sprite::Enemy; // or wherever your Enemy is

pub fn render_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
    enemies: &[Enemy],
) {
    let fb_w = framebuffer.width as i32;
    let fb_h = framebuffer.height as i32;

    let rows = maze.len();
    if rows == 0 { return; }
    let cols = maze[0].len();

    // Size in pixels per map cell on the minimap
    let cell_size: i32 = 4; // tweak this if your map is big/small

    let map_pixel_w = (cols as i32) * cell_size;
    let map_pixel_h = (rows as i32) * cell_size;

    let margin = 10;
    let origin_x = fb_w - map_pixel_w - margin; // upper-right corner
    let origin_y = margin;

    // Draw map cells
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let x0 = origin_x + (col_index as i32) * cell_size;
            let y0 = origin_y + (row_index as i32) * cell_size;

            // choose color based on cell type
            let color = match cell {
                ' ' => continue, // skip empty space
                '-' => Color::DARKGRAY, // wall type 1
                '|' => Color::DARKGRAY, // wall type 1
                '+' => Color::BROWN,    // wall type 2
                's' | 'S' => Color::GREEN,  // start
                'g' | 'G' => Color::YELLOW, // goal
                _   => Color::DARKBLUE,     // other walls
            };

            framebuffer.set_current_color(color);

            for dx in 0..cell_size {
                for dy in 0..cell_size {
                    let px = (x0 + dx) as u32;
                    let py = (y0 + dy) as u32;
                    framebuffer.set_pixel(px, py, 0.0);
                }
            }
        }
    }

    // Helper to draw a marker on the minimap for a world position
    let mut draw_marker = |world_x: f32, world_y: f32, color: Color| {
        // convert world coords to grid indices
        let col = (world_x / block_size as f32) as i32;
        let row = (world_y / block_size as f32) as i32;

        if row < 0 || col < 0 || row >= rows as i32 || col >= cols as i32 {
            return;
        }

        let x0 = origin_x + col * cell_size;
        let y0 = origin_y + row * cell_size;

        framebuffer.set_current_color(color);
        let marker_size = cell_size.max(3); // at least 3x3 so it's visible

        for dx in 0..marker_size {
            for dy in 0..marker_size {
                let px = (x0 + dx) as u32;
                let py = (y0 + dy) as u32;
                framebuffer.set_pixel(px, py, 0.0);
            }
        }
    };

    // Draw player marker (cyan)
    draw_marker(player.pos.x, player.pos.y, Color::SKYBLUE);

    // Draw enemies (red)
    for enemy in enemies {
        draw_marker(enemy.pos.x, enemy.pos.y, Color::ORANGE);
    }
}

pub fn render_world(framebuffer: &mut Framebuffer, player: &Player, maze: &Maze, block_size: usize) {
    let num_rays = framebuffer.width;
    let hw = framebuffer.width as f32 /2.0;
    let hh = framebuffer.height as f32 /2.0;
    framebuffer.set_current_color(Color::WHITE);

    //let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let mut distance_to_wall = intersect.distance;
        distance_to_wall *= (player.a - a).cos();
        let distance_to_projection_plane = hw / (player.fov / 2.0).tan();
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane * 0.15;

        let stake_top = (hh - (stake_height / 2.0)) as u32;
        let stake_bottom = (hh + (stake_height / 2.0)) as u32;

        for y in stake_top..stake_bottom {
            framebuffer.set_current_color(match intersect.impact {
                '+' => Color::ORANGERED,
                'g' => Color::GREEN,
                _ => Color::YELLOW,
            });
            framebuffer.set_pixel(i, y, distance_to_wall);
        }
    }
}

fn main() {
    let window_width = 900;
    let window_height = 600;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    window.set_exit_key(None);

    let audio = RaylibAudio::init_audio_device().expect("Failed to load audio device");
    let music = audio.new_music("assets/video0.MP3").expect("failed to load music");
    music.play_stream();
    let damage_sound = audio.new_sound("assets/hit1.ogg").expect("Failed to load damage sound");
    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, Color::BLACK);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    let texture_manager = TextureManager::new(&mut window, &raylib_thread);
    let mut game_state = AppState::init(window_width, window_height, block_size as f32, texture_manager);

    while !window.window_should_close() && !game_state.close_window {
        game_state.handle_input(&mut window);
        if game_state.close_window {break;}
        music.update_stream();
        if game_state.hit_frame {
            damage_sound.play();
            game_state.hit_frame = false;
        }
        match game_state.current_screen {
            Screens::Game(_) => {
                // 1. clear framebuffer
                framebuffer.clear();
                
                // 2. draw the maze, passing the maze and block size
                //process_events(&mut window, &mut player, &maze, block_size as f32);
                //render_maze(&mut framebuffer, &maze, block_size, &player);
                render_world(&mut framebuffer, &game_state.player, &game_state.mazes[game_state.current_level], block_size);
                
                for enemy in &game_state.enemies {
                    draw_sprite(&mut framebuffer, &game_state.player, enemy, &game_state.texture_manager);
                }
                render_minimap(&mut framebuffer, &game_state.mazes[game_state.current_level], game_state.block_size as usize, &game_state.player, &game_state.enemies);
                // 3. swap buffers
                framebuffer.swap_buffers(&mut window, &raylib_thread, &game_state);
            }
            Screens::MainMenu(_) => {
                let mut d = window.begin_drawing(&raylib_thread);
                d.clear_background(Color::new(50, 50, 100, 255));
                game_state.current_screen.render(&mut d);
            }
            _ => {
                let mut d = window.begin_drawing(&raylib_thread);
                game_state.current_screen.render(&mut d);
            }
        }

        thread::sleep(Duration::from_millis(8));
    }
}