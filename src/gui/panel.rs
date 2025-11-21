use raylib::prelude::*;
use std::collections::HashMap;

use crate::gui::element::{Element, Elements};

pub struct Panel {
    pub rect: Rectangle,
    pub background_color: Option<Color>,
    pub elements: HashMap<String, Elements>,
}

impl Panel {
    pub fn new(rect: Rectangle, background_color: Option<Color>) -> Self {
        Panel {
            rect,
            background_color,
            elements: HashMap::new(),
        }
    }

    pub fn add_element(&mut self, id: impl Into<String>, element: Elements) {
        self.elements.insert(id.into(), element);
    }
}

impl Element for Panel {
    fn draw_element(&self, d: &mut RaylibDrawHandle) {
        // Draw background rect if specified
        if let Some(color) = self.background_color {
            d.draw_rectangle_rec(self.rect, color);
        }

        // Draw all child elements (positions are assumed to be in global coordinates)
        for element in self.elements.values() {
            element.draw_element(d);
        }
    }

    fn update(&mut self, window: &mut RaylibHandle) {
        // Forward input/update to all children
        for element in self.elements.values_mut() {
            element.update(window);
        }
    }
}