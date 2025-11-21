use std::f32::consts::PI;

use raylib::prelude::*;

use crate::{gui::{Screen, element::{Element, Elements}, screens::Screens}, maze::{Maze, load_maze}, player::Player, sprite::Enemy};

pub struct AppState {
    pub current_screen: Screens,
    pub width: i32,
    pub height: i32,
    pub current_level: usize,
    pub is_playing: bool,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub mazes: Vec<Maze>,
    pub paused: bool,
    pub enabled_cursor: bool, // true = cursor free/visible, false = captured
    pub block_size: f32,
    pub max_health: i32,
    pub close_window: bool,
}

impl AppState {
    pub fn init(w: i32, h: i32, block_size: f32) -> Self {
        let max_health = 5;
        let current_screen = Screens::main_menu(w, h);
        let maze1 = load_maze("maze1.txt");
        let maze2 = load_maze("maze2.txt");
        let maze3 = load_maze("maze3.txt");
        let player = Player { 
            pos: Vector2::new(block_size, block_size,),
            a: PI/3.0, 
            fov: PI/3.0, 
            health: max_health
        };
        AppState { 
            current_screen,
             width: w, 
             height: h, 
             current_level: 0, 
             is_playing: false, 
             player: player, 
             enemies: vec![], 
             mazes: vec![maze1, maze2, maze3], 
             paused: false, 
             enabled_cursor: true, 
             block_size: block_size, 
             max_health, 
             close_window: false,
        }

    }


    /// Called when we click "Play" in the main menu.
    pub fn start_level(&mut self, window: &mut RaylibHandle) {
        self.is_playing = true;
        self.paused = false;
        self.enabled_cursor = false;

        // Reset player
        self.player.health = self.max_health;
        // e.g. find cell 's' in the maze and set player.pos
        let maze = &self.mazes[self.current_level];
        if let Some((i, j)) = find_start_cell(maze, 's') {
            self.player.pos.x = (i as f32 + 0.5) * self.block_size;
            self.player.pos.y = (j as f32 + 0.5) * self.block_size;
        }
        self.player.a = PI/4.0;

        // Reset enemies for this level
        self.spawn_enemies_for_level();

        // Capture mouse
        window.disable_cursor();
        window.hide_cursor();

        // Switch to game screen overlay (HUD etc.)
        self.current_screen = Screens::game(self.width, self.height);
    }

    fn current_maze(&self) -> &Maze {
        &self.mazes[self.current_level]
    }

    fn current_maze_mut(&mut self) -> &mut Maze {
        &mut self.mazes[self.current_level]
    }

    fn spawn_enemies_for_level(&mut self) {
        let mut pos1 = Vector2::new(1.0, 1.0);
        let mut pos2 = Vector2::new(1.0, 1.0);
        match self.current_level {
            0 => {
                pos1.x = 11.0 * self.block_size; pos1.y = 3.0 * self.block_size;
                pos2.x = 7.0 * self.block_size; pos2.y = 7.0 * self.block_size;
            }
            1 => {
                pos1.x = 11.0 * self.block_size; pos1.y = 3.0 * self.block_size;
                pos2.x = 11.0 * self.block_size; pos2.y = 7.0 * self.block_size;
            }
            2 => {
                pos1.x = 1.0 * self.block_size; pos1.y = 11.0 * self.block_size;
                pos2.x = 1.0 * self.block_size; pos2.y = 5.0 * self.block_size;
            }
            _ => {}
        }
        self.enemies = vec![
            Enemy {pos: pos1, texture_key: 'e'},
            Enemy {pos: pos2, texture_key: 'e'}
        ];
    }

    /// Move enemies one step each frame to some free neighboring cell.
    fn update_enemies(&mut self, dt: f64) {
        let mut maze = self.current_maze().clone();
        use raylib::ffi::GetRandomValue;
        let animation_rate = 1.0;


        for enemy in &mut self.enemies {
            // 4-neighborhood: up, down, left, right
            let dirs = [
                Vector2::new( 1.0,  0.0),
                Vector2::new(-1.0,  0.0),
                Vector2::new( 0.0,  1.0),
                Vector2::new( 0.0, -1.0),
            ];

            let step = self.block_size / 10.0; // enemy step size

            // Try up to 4 random directions
            for _ in 0..4 {
                let idx = unsafe { GetRandomValue(0, 3) } as usize;
                let dir = dirs[idx];

                let next_x = enemy.pos.x + dir.x * step;
                let next_y = enemy.pos.y + dir.y * step;

                if AppState::is_free_cell(&mut maze, next_x, next_y, self.block_size) {
                    enemy.pos.x = next_x;
                    enemy.pos.y = next_y;
                    break;
                }
            }
            if dt%animation_rate < 0.5*animation_rate {
                enemy.texture_key = '#';
            } else {
                enemy.texture_key = 'e';
            }
        }
    }

    /// Check if a world position corresponds to a free maze cell (' ' or maybe 'g').
    fn is_free_cell(maze: &Maze, x: f32, y: f32, block_size: f32) -> bool {
        if x < 0.0 || y < 0.0 {
            return false;
        }
        let i = (x / block_size) as usize;
        let j = (y / block_size) as usize;
        if j >= maze.len() || i >= maze[0].len() {
            return false;
        }
        let c = maze[j][i];
        c == ' ' || c == 'g'
    }

    /// Player wins if standing on a 'g' cell.
    fn is_on_goal(&self) -> bool {
        let maze = self.current_maze();
        let i = (self.player.pos.x / self.block_size) as usize;
        let j = (self.player.pos.y / self.block_size) as usize;
        if j >= maze.len() || i >= maze[0].len() {
            return false;
        }
        maze[j][i] == 'g'
    }

    /// Player loses health when near an enemy.
    fn check_enemy_collisions(&mut self) {
        let damage_distance = self.block_size / 4.0; // tune
        let damage = 1; // HP per frame per collision; you can adjust

        for enemy in &self.enemies {
            let dx = enemy.pos.x - self.player.pos.x;
            let dy = enemy.pos.y - self.player.pos.y;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq <= damage_distance * damage_distance {
                self.player.health -= damage;
                if self.player.health < 0 {
                    self.player.health = 0;
                }
            }
        }
    }

    pub fn render_game_screen_extras(&self, d:&mut RaylibDrawHandle) {
        // Draw the current game screen UI (HUD, panels, etc.)
        self.current_screen.render(d);

        // ---------- HUD OVERLAYS ----------
        let margin = 10;
        let screen_w = self.width;
        let screen_h = self.height;

        // ---- FPS (top-right corner) ----
        let fps = d.get_fps();
        let fps_text = format!("FPS: {}", fps);
        let fps_font_size = 20;

        let fps_text_width = d.measure_text(&fps_text, fps_font_size);
        let fps_x = screen_w - fps_text_width - margin;
        let fps_y = screen_h - margin - 20;
        d.draw_text(&fps_text, fps_x, fps_y, fps_font_size, Color::YELLOW);

        // ---- Health as hearts (bottom-left corner) ----
        let heart_font_size = 30;
        let heart_spacing = heart_font_size; // horizontal spacing between hearts
        let hearts_x = margin;
        let hearts_y = screen_h - heart_font_size - margin;

        for i in 0..self.max_health {
            let x = hearts_x + i * heart_spacing;
            let color = if (i as i32) < self.player.health {
                Color::GREEN
            } else {
                Color::DARKGRAY
            };
            d.draw_text("*", x, hearts_y, heart_font_size, color);
        }

        // Note: The top-left corner is intentionally left free for a future labyrinth/minimap.
    }
}

/// Helper to locate a cell with a specific character in the maze.
fn find_start_cell(maze: &Maze, ch: char) -> Option<(usize, usize)> {
    for (j, row) in maze.iter().enumerate() {
        for (i, &c) in row.iter().enumerate() {
            if c == ch {
                return Some((i, j));
            }
        }
    }
    None
}

pub trait StateHandler {
    fn handle_input(&mut self, window: &mut RaylibHandle);
}

impl StateHandler for AppState {
    fn handle_input(&mut self, window: &mut RaylibHandle) {
        match &mut self.current_screen {
            // =========================
            // MAIN MENU
            // =========================
            Screens::MainMenu(screen) => {
                // Update GUI elements (buttons, labels, panels)
                screen.update(window);

                // 1) Level selection buttons inside a panel "levels_panel"
                if let Some(Elements::Panel(panel)) = screen.elements.get_mut("levels_panel") {
                    for (id, element) in panel.elements.iter_mut() {
                        if let Elements::Button(btn) = element {
                            if btn.clicked {
                                if let Some(index_str) = id.strip_prefix("level_") {
                                    if let Ok(index) = index_str.parse::<usize>() {
                                        if index < self.mazes.len() {
                                            self.current_level = index;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 2) Play button
                if let Some(Elements::Button(play_btn)) = screen.elements.get_mut("play") {
                    if play_btn.clicked {
                        self.start_level(window);
                    }
                    // 3) Quit button
                    else if let Some(Elements::Button(quit_btn)) = screen.elements.get_mut("quit") {
                        if quit_btn.clicked {
                            self.close_window = true;
                        }
                    }
                }

            }

            // =========================
            // GAME / PLAYING
            // =========================
            Screens::Game(screen) => {
                // Update any overlay GUI (HUD, pause button, etc.)
                screen.update(window);

                // ESC toggles pause and cursor
                if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    self.paused = !self.paused;
                    self.enabled_cursor = !self.enabled_cursor;

                    if self.enabled_cursor {
                        window.enable_cursor();
                        window.show_cursor();
                    } else {
                        window.disable_cursor();
                        window.hide_cursor();

                        // optional: recenter mouse to avoid big initial delta
                        let center = Vector2::new(
                            (window.get_screen_width() / 2) as f32,
                            (window.get_screen_height() / 2) as f32,
                        );
                        window.set_mouse_position(center);
                    }
                }

                if !self.paused && self.is_playing {
                    let maze = self.current_maze().clone();
                    let capture_mouse = !self.enabled_cursor;

                    // Move player with keyboard/mouse
                    self.player.process_events(window, &maze, self.block_size);

                    // Move enemies
                    let dt = window.get_time();
                    self.update_enemies(dt);

                    // Enemy collisions
                    self.check_enemy_collisions();

                    // Check win/lose
                    if self.is_on_goal() {
                        self.is_playing = false;
                        self.enabled_cursor = true;
                        window.enable_cursor();
                        window.show_cursor();
                        self.current_screen = Screens::victory(self.width, self.height);
                    } else if self.player.health == 0 {
                        self.is_playing = false;
                        self.enabled_cursor = true;
                        window.enable_cursor();
                        window.show_cursor();
                        self.current_screen = Screens::defeat(self.width, self.height);
                    }
                } else {
                    self.current_screen = Screens::pause(self.width, self.height);
                }
            }
            // ===
            // PAUSE
            // ===
            Screens::Pause(screen) => {
                screen.update(window);
                // ESC toggles pause and cursor
                if window.is_key_pressed(KeyboardKey::KEY_ESCAPE)  {
                    self.paused = !self.paused;
                    self.enabled_cursor = !self.enabled_cursor;
                    
                    if self.enabled_cursor {
                        window.enable_cursor();
                        window.show_cursor();
                    } else {
                        window.disable_cursor();
                        window.hide_cursor();
                        
                        // optional: recenter mouse to avoid big initial delta
                        let center = Vector2::new(
                            (window.get_screen_width() / 2) as f32,
                            (window.get_screen_height() / 2) as f32,
                        );
                        window.set_mouse_position(center);
                    }
                    self.current_screen = Screens::game(self.width, self.height)
                }
                else if let Some(Elements::Panel(pnl)) =screen.elements.get_mut("pause_panel") {
                    if let Some(Elements::Button(play_btn)) = pnl.elements.get_mut("pause_resume") {
                        if play_btn.clicked {
                            self.paused = !self.paused;
                            self.enabled_cursor = !self.enabled_cursor;
                            
                            if self.enabled_cursor {
                                window.enable_cursor();
                                window.show_cursor();
                            } else {
                                window.disable_cursor();
                                window.hide_cursor();
                                
                                // optional: recenter mouse to avoid big initial delta
                                let center = Vector2::new(
                                    (window.get_screen_width() / 2) as f32,
                                    (window.get_screen_height() / 2) as f32,
                                );
                                window.set_mouse_position(center);
                            }
                            self.current_screen = Screens::game(self.width, self.height)
                        } else if let Some(Elements::Button(menu_btn)) = pnl.elements.get_mut("pause_menu") {
                            if menu_btn.clicked {
                                self.is_playing = false;
                                self.current_screen = Screens::main_menu(self.width, self.height);
                            } else if let Some(Elements::Button(quit_btn)) = pnl.elements.get_mut("pause_quit") {
                                if quit_btn.clicked {
                                    self.close_window = true
                                }
                            }
                        }
                    }
                }
            }

            // =========================
            // VICTORY SCREEN
            // =========================
            Screens::Victory(screen) => {
                // Update GUI elements on the victory screen
                screen.update(window);

                // All buttons are inside the "victory_panel" panel
                if let Some(Elements::Panel(panel)) = screen.elements.get_mut("victory_panel") {
                    // Next level
                    if let Some(Elements::Button(next_btn)) = panel.elements.get_mut("victory_next") {
                        if next_btn.clicked {
                            // Advance level if possible
                            if self.current_level + 1 < self.mazes.len() {
                                self.current_level += 1;
                            } else {
                                self.current_level = 0;
                            }
                            // Start the selected level
                            self.start_level(window);
                            return;
                        }
                    }

                    // Replay current level
                    if let Some(Elements::Button(restart_btn)) = panel.elements.get_mut("victory_restart") {
                        if restart_btn.clicked {
                            self.start_level(window);
                            return;
                        }
                    }

                    // Back to main menu
                    if let Some(Elements::Button(menu_btn)) = panel.elements.get_mut("victory_menu") {
                        if menu_btn.clicked {
                            self.is_playing = false;
                            self.paused = false;
                            self.enabled_cursor = true;
                            window.enable_cursor();
                            window.show_cursor();
                            self.current_screen = Screens::main_menu(self.width, self.height);
                            return;
                        }
                    }
                }
            }

            // =========================
            // GAME OVER SCREEN
            // =========================
            Screens::Defeat(screen) => {
                // Update GUI elements on the defeat screen
                screen.update(window);

                // All buttons are inside the "defeat_panel" panel
                if let Some(Elements::Panel(panel)) = screen.elements.get_mut("defeat_panel") {
                    // Retry same level
                    if let Some(Elements::Button(retry_btn)) = panel.elements.get_mut("defeat_restart") {
                        if retry_btn.clicked {
                            self.start_level(window);
                            return;
                        }
                    }

                    // Back to main menu
                    if let Some(Elements::Button(menu_btn)) = panel.elements.get_mut("defeat_menu") {
                        if menu_btn.clicked {
                            self.is_playing = false;
                            self.paused = false;
                            self.enabled_cursor = true;
                            window.enable_cursor();
                            window.show_cursor();
                            self.current_screen = Screens::main_menu(self.width, self.height);
                            return;
                        }
                    }

                    // Quit application
                    if let Some(Elements::Button(quit_btn)) = panel.elements.get_mut("defeat_quit") {
                        if quit_btn.clicked {
                            self.close_window = true;
                            return;
                        }
                    }
                }
            }
        }
    }
}