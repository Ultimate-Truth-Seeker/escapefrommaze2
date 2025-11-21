use raylib::prelude::*;

use crate::gui::{Screen, button::Button, label::Label, panel::Panel, screens::Screens};


pub enum Elements {
    Button(Button),
    Label(Label),
    Panel(Panel),
    Screen(Screen),
}

pub trait Element {
    fn draw_element(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self, window: &mut RaylibHandle);
}

impl Element for Elements {
    fn draw_element(&self, d: &mut RaylibDrawHandle) {
        match self {
            Elements::Button(b) => b.draw_element(d),
            Elements::Label(tb) => tb.draw_element(d),
            Elements::Panel(p) => p.draw_element(d),
            Elements::Screen(s) => s.draw_element(d),
        }
    }
    fn update(&mut self, window: &mut RaylibHandle) {
        match self {
            Elements::Button(b) => b.update(window),
            Elements::Label(tb) => tb.update(window),
            Elements::Panel(p) => p.update(window),
            Elements::Screen(s) => s.update(window),
        }
    }
}