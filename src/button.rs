use std::sync::atomic::Ordering;

use super::{common::ELEMENT_NEXT_ID, Event};

use glam::Vec2;
use raylib::color::Color;
pub struct Button {
    pub id: u32,
    pub position: Vec2,
    pub size: Vec2,

    pub color: Color,
    pub label: Option<String>,

    pub hovered: bool,
    pub pressed: bool,
    pub was_pressed: bool,
}

impl Button {
    pub fn new(position: Vec2, size: Vec2, color: Color, label: Option<String>) -> Self {
        Self {
            id: ELEMENT_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            position,
            size,

            color,
            label,

            hovered: false,
            pressed: false,
            was_pressed: false,
        }
    }

    pub fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event> {
        let mut event: Option<Event> = None;
        if !mouse_pressed && self.was_pressed {
            event = Some(Event::ButtonReleased);
            self.was_pressed = false;
        }

        self.hovered = mouse_position.x > self.position.x
            && mouse_position.x < self.position.x + self.size.x
            && mouse_position.y > self.position.y
            && mouse_position.y < self.position.y + self.size.y;

        if mouse_pressed && self.hovered {
            if !self.pressed {
                event = Some(Event::ButtonPressed);
            }
            self.pressed = true;
            self.was_pressed = true;
        } else {
            self.pressed = false;
        }

        event
    }
}
