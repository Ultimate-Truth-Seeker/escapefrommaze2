use raylib::prelude::*;

use crate::gui::element::Element;

#[derive(Clone, Debug)]
pub struct Label {
    pub text: String,
    pub position: Vector2,
    pub font_size: i32,
    pub color: Color,
}

impl Label {
    pub fn new(text: impl Into<String>, position: Vector2, font_size: i32, color: Color) -> Self {
        Label {
            text: text.into(),
            position,
            font_size,
            color,
        }
    }
}

impl Element for Label {
    fn draw_element(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(&self.text, self.position.x as i32, self.position.y as i32, self.font_size, self.color);
    }

    fn update(&mut self, _window: &mut RaylibHandle) {
        // Labels do not react to input (static element)
    }
}