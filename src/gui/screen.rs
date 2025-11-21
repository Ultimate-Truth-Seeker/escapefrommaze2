use std::collections::HashMap;

use raylib::prelude::*;

use crate::gui::element::{Element, Elements};

pub struct Screen {
    pub background: Option<Texture2D>,
    pub elements: HashMap<String, Elements>
}

impl Element for Screen {
    fn draw_element(&self, d: &mut RaylibDrawHandle) {
        if let Some(bg) = &self.background {
            d.draw_texture(bg, 0, 0, Color::WHITE);
        }

        for e in self.elements.values(){
            e.draw_element(d);
        }
    }
    fn update(&mut self, window: &mut RaylibHandle) {
        for e in self.elements.values_mut() {
            e.update(window);
        }
    }
}


