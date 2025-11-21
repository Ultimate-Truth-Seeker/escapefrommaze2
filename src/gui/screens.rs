use std::collections::HashMap;

use raylib::prelude::*;

use crate::gui::{button::Button, element::{Element, Elements}, label::Label, panel::Panel, *};
pub enum Screens {
    MainMenu(Screen),
    Game(Screen),
    Pause(Screen),
    Victory(Screen),
    Defeat(Screen),
}

impl Screens {
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        match self {
            Screens::MainMenu(sc) => sc.draw_element(d),
            Screens::Game(sc) => sc.draw_element(d),
            Screens::Pause(sc) => sc.draw_element(d),
            Screens::Victory(sc) => sc.draw_element(d),
            Screens::Defeat(sc) => sc.draw_element(d),
        }
    }
    pub fn main_menu(screen_w: i32, screen_h: i32) -> Self {
        let mut elements = HashMap::new();

        // Title label
        let title = Label::new(
            "Maze Raycaster",
            Vector2 { x: (screen_w / 2 - 150) as f32, y: 80.0 },
            40,
            Color::WHITE,
        );
        elements.insert("title".to_string(), Elements::Label(title));

        // Panel for level buttons
        let panel_rect = Rectangle {
            x: (screen_w / 2 - 150) as f32,
            y: 160.0,
            width: 300.0,
            height: 200.0,
        };
        let mut level_panel = Panel::new(panel_rect, Some(Color::DARKGRAY));

        // Example: 3 levels
        let level_names = ["Level 1", "Level 2", "Level 3"];
        let button_height = 40.0;
        let spacing = 10.0;
        let mut y = panel_rect.y + 20.0;

        for (i, name) in level_names.iter().enumerate() {
            let rect = Rectangle {
                x: panel_rect.x + 20.0,
                y,
                width: panel_rect.width - 40.0,
                height: button_height,
            };

            let btn = Button::new(
                rect,
                Some(name.to_string()),
                Color::GRAY,
                Color::LIGHTGRAY,
            );

            level_panel.add_element(format!("level_{}", i), Elements::Button(btn));

            y += button_height + spacing;
        }

        elements.insert("levels_panel".to_string(), Elements::Panel(level_panel));

        // Play button
        let play_rect = Rectangle {
            x: (screen_w / 2 - 100) as f32,
            y: panel_rect.y + panel_rect.height + 40.0,
            width: 200.0,
            height: 50.0,
        };
        let play_button = Button::new(
            play_rect,
            Some("PLAY".to_string()),
            Color::DARKGREEN,
            Color::GREEN,
        );
        elements.insert("play".to_string(), Elements::Button(play_button));

        // Quit button
        let quit_rect = Rectangle {
            x: (screen_w / 2 - 100) as f32,
            y: play_rect.y + 70.0,
            width: 200.0,
            height: 50.0,
        };
        let quit_button = Button::new(
            quit_rect,
            Some("QUIT".to_string()),
            Color::MAROON,
            Color::RED,
        );
        elements.insert("quit".to_string(), Elements::Button(quit_button));

        Screens::MainMenu(Screen {
            background: None, // or Some(texture)
            elements,
        })
    }
    pub fn game(screen_w: i32, _screen_h: i32) -> Self {
        let mut elements = HashMap::new();

        // Small hint in the corner
        let hint = Label::new(
            "ESC - Pause",
            Vector2 { x: 20.0, y: 20.0 },
            20,
            Color::WHITE,
        );
        elements.insert("hint_esc".to_string(), Elements::Label(hint));

        Screens::Game(Screen {
            background: None, // game world is drawn separately by the raycaster
            elements,
        })
    }
    pub fn pause(screen_w: i32, screen_h: i32) -> Self {
        let mut elements = HashMap::new();

        // Dim background hint (you can draw semi-transparent rect if you want later)
        let title = Label::new(
            "Paused",
            Vector2 { x: (screen_w / 2 - 70) as f32, y: 80.0 },
            40,
            Color::WHITE,
        );
        elements.insert("title".to_string(), Elements::Label(title));

        // Central panel
        let panel_rect = Rectangle {
            x: (screen_w / 2 - 150) as f32,
            y: 160.0,
            width: 300.0,
            height: 200.0,
        };
        let mut panel = Panel::new(panel_rect, Some(Color::DARKGRAY));

        let button_height = 40.0;
        let spacing = 10.0;
        let mut y = panel_rect.y + 20.0;

        // Resume
        let resume_rect = Rectangle {
            x: panel_rect.x + 20.0,
            y,
            width: panel_rect.width - 40.0,
            height: button_height,
        };
        let resume_btn = Button::new(
            resume_rect,
            Some("Resume".to_string()),
            Color::DARKGREEN,
            Color::GREEN,
        );
        panel.add_element("pause_resume".to_string(), Elements::Button(resume_btn));
        y += button_height + spacing;

        // Main menu
        let menu_rect = Rectangle {
            x: panel_rect.x + 20.0,
            y,
            width: panel_rect.width - 40.0,
            height: button_height,
        };
        let menu_btn = Button::new(
            menu_rect,
            Some("Main Menu".to_string()),
            Color::GRAY,
            Color::LIGHTGRAY,
        );
        panel.add_element("pause_menu".to_string(), Elements::Button(menu_btn));
        y += button_height + spacing;

        // Quit
        let quit_rect = Rectangle {
            x: panel_rect.x + 20.0,
            y,
            width: panel_rect.width - 40.0,
            height: button_height,
        };
        let quit_btn = Button::new(
            quit_rect,
            Some("Quit".to_string()),
            Color::MAROON,
            Color::RED,
        );
        panel.add_element("pause_quit".to_string(), Elements::Button(quit_btn));

        elements.insert("pause_panel".to_string(), Elements::Panel(panel));

        Screens::Pause(Screen {
            background: None,
            elements,
        })
    }
    pub fn victory(screen_w: i32, _screen_h: i32) -> Self {
        let mut elements = HashMap::new();

        let title = Label::new(
            "You escaped the maze!",
            Vector2 { x: (screen_w / 2 - 180) as f32, y: 80.0 },
            40,
            Color::WHITE,
        );
        elements.insert("title".to_string(), Elements::Label(title));

        let panel_rect = Rectangle {
            x: (screen_w / 2 - 150) as f32,
            y: 160.0,
            width: 300.0,
            height: 220.0,
        };
        let mut panel = Panel::new(panel_rect, Some(Color::DARKGREEN));

        let button_height = 40.0;
        let spacing = 10.0;
        let mut y = panel_rect.y + 20.0;

        // Next level
        let next_rect = Rectangle {
            x: panel_rect.x + 20.0,
            y,
            width: panel_rect.width - 40.0,
            height: button_height,
        };
        let next_btn = Button::new(
            next_rect,
            Some("Next Level".to_string()),
            Color::DARKGREEN,
            Color::GREEN,
        );
        panel.add_element("victory_next".to_string(), Elements::Button(next_btn));
        y += button_height + spacing;

        // Replay current level
        let restart_rect = Rectangle {
            x: panel_rect.x + 20.0,
            y,
            width: panel_rect.width - 40.0,
            height: button_height,
        };
        let restart_btn = Button::new(
            restart_rect,
            Some("Replay Level".to_string()),
            Color::GRAY,
            Color::LIGHTGRAY,
        );
        panel.add_element("victory_restart".to_string(), Elements::Button(restart_btn));
        y += button_height + spacing;

        // Back to menu
        let menu_rect = Rectangle {
            x: panel_rect.x + 20.0,
            y,
            width: panel_rect.width - 40.0,
            height: button_height,
        };
        let menu_btn = Button::new(
            menu_rect,
            Some("Main Menu".to_string()),
            Color::MAROON,
            Color::RED,
        );
        panel.add_element("victory_menu".to_string(), Elements::Button(menu_btn));

        elements.insert("victory_panel".to_string(), Elements::Panel(panel));

        Screens::Victory(Screen {
            background: None,
            elements,
        })
    }
    pub fn defeat(screen_w: i32, _screen_h: i32) -> Self {
        let mut elements = HashMap::new();

        let title = Label::new(
            "You got lost in the maze!",
            Vector2 { x: (screen_w / 2 - 200) as f32, y: 80.0 },
            40,
            Color::WHITE,
        );
        elements.insert("title".to_string(), Elements::Label(title));

        let panel_rect = Rectangle {
            x: (screen_w / 2 - 150) as f32,
            y: 160.0,
            width: 300.0,
            height: 220.0,
        };
        let mut panel = Panel::new(panel_rect, Some(Color::DARKGRAY));

        let button_height = 40.0;
        let spacing = 10.0;
        let mut y = panel_rect.y + 20.0;

        // Retry current level
        let retry_rect = Rectangle {
            x: panel_rect.x + 20.0,
            y,
            width: panel_rect.width - 40.0,
            height: button_height,
        };
        let retry_btn = Button::new(
            retry_rect,
            Some("Retry Level".to_string()),
            Color::GRAY,
            Color::LIGHTGRAY,
        );
        panel.add_element("defeat_restart".to_string(), Elements::Button(retry_btn));
        y += button_height + spacing;

        // Back to menu
        let menu_rect = Rectangle {
            x: panel_rect.x + 20.0,
            y,
            width: panel_rect.width - 40.0,
            height: button_height,
        };
        let menu_btn = Button::new(
            menu_rect,
            Some("Main Menu".to_string()),
            Color::DARKBLUE,
            Color::BLUE,
        );
        panel.add_element("defeat_menu".to_string(), Elements::Button(menu_btn));
        y += button_height + spacing;

        // Quit
        let quit_rect = Rectangle {
            x: panel_rect.x + 20.0,
            y,
            width: panel_rect.width - 40.0,
            height: button_height,
        };
        let quit_btn = Button::new(
            quit_rect,
            Some("Quit".to_string()),
            Color::MAROON,
            Color::RED,
        );
        panel.add_element("defeat_quit".to_string(), Elements::Button(quit_btn));

        elements.insert("defeat_panel".to_string(), Elements::Panel(panel));

        Screens::Defeat(Screen {
            background: None,
            elements,
        })
    }
}