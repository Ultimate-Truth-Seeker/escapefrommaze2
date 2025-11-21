use raylib::color::Color;

use crate::framebuffer::{self, Framebuffer};
use crate::player::Player;
use crate::maze::Maze;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: usize,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw: bool,
) -> Intersect {
    let mut d = 0.1;
    let max_distance = block_size as f32 *200.0;

    // Precompute maze dimensions in world units
    let maze_rows = maze.len();
    let maze_cols = maze[0].len();
    let world_width  = (maze_cols * block_size) as isize;
    let world_height = (maze_rows * block_size) as isize;

    framebuffer.set_current_color(Color::WHITE);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();

        let wx = player.pos.x + cos;
        let wy = player.pos.y + sin;

        // if we go outside the world bounds, stop and return "no hit" far away
        if wx < 0.0 || wy < 0.0 || wx >= world_width as f32 || wy >= world_height as f32 {
            return Intersect {
                distance: max_distance,
                impact: ' ', // or some sentinel
                tx: 0,
            };
        }

        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        // extra safety: check maze indices
        if j >= maze_rows || i >= maze_cols {
            return Intersect {
                distance: max_distance,
                impact: ' ',
                tx: 0,
            };
        }

        if maze[j][i] != ' ' && maze[j][i] != 'g' {
            let hitx = x - i * block_size;
            let hity = y - j * block_size;
            let mut maxhit = hity;
            
            if 1 < hitx && hitx < block_size - 1 {
                maxhit = hitx;
            }

            let tx = ((maxhit as f32 * 128.0) / block_size as f32) as usize;
            
            return Intersect{
                distance: d,
                impact: maze[j][i],
                tx: tx,
            };
        }
        

        if draw {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        d += 1.0;

        // safety net: in case we somehow never hit a wall or leave the map
        if d > max_distance {
            return Intersect {
                distance: max_distance,
                impact: ' ',
                tx: 0,
            };
        }
    }
}