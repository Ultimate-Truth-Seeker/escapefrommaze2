use raylib::prelude::*;

use crate::gui::element::Element;


pub struct Button {
    pub rect: Rectangle,
    pub text: Option<String>,
    pub clicked: bool,
    pub color: Color,
    pub hover_color: Color,
    pub is_rounded: bool,
    pub selected: bool,
}

impl Button {
    pub fn new(rect: Rectangle, text: Option<String>, color: Color, hover_color: Color) -> Self {
        Button {
            rect,
            text,
            clicked: false,
            color,
            hover_color,
            is_rounded: false,
            selected: false
        }
    }

}

impl Element for Button {
    /// Update button state based on mouse input.
    /// This should be called from the Screen / state handler each frame.
    fn update(&mut self, window: &mut RaylibHandle) {
        let mouse = window.get_mouse_position();
    
        let hovered = mouse.x >= self.rect.x
            && mouse.x <= self.rect.x + self.rect.width
            && mouse.y >= self.rect.y
            && mouse.y <= self.rect.y + self.rect.height;
    
        self.clicked = hovered && window.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        if hovered && window.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.selected = true;
        } else if window.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.selected = false;
        }
    }
    fn draw_element(&self, d: &mut RaylibDrawHandle) {
        // Determine hover purely for visual feedback (independent from `clicked`)
        let mouse = d.get_mouse_position();
        let hovered = mouse.x >= self.rect.x
            && mouse.x <= self.rect.x + self.rect.width
            && mouse.y >= self.rect.y
            && mouse.y <= self.rect.y + self.rect.height;

        let fill_color = if hovered || self.selected { self.hover_color } else { self.color };

        if self.is_rounded {
            d.draw_rectangle_rounded(self.rect, 0.3, 8, fill_color);
        } else {
            d.draw_rectangle_rec(self.rect, fill_color);
        }

        if let Some(text) = &self.text {
            let font_size = 20; // you can make this configurable later
            let text_width = d.measure_text(text, font_size);

            let x = self.rect.x + (self.rect.width - text_width as f32) / 2.0;
            let y = self.rect.y + (self.rect.height - font_size as f32) / 2.0;

            d.draw_text(text, x as i32, y as i32, font_size, Color::WHITE);
        }
    }
}