use std::sync::atomic::Ordering;

use glam::Vec2;

use super::{Button, Event, ELEMENT_NEXT_ID};

pub struct ButtonToggle {
    pub id: u32,
    pub position: Vec2,
    pub size: Vec2,
    pub left_option: String,
    pub right_option: String,
    pub toggled_left: bool,
    pub left_button: Button,
    pub right_button: Button,
}

impl ButtonToggle {
    pub fn new(
        position: Vec2,
        size: Vec2,
        left_option: String,
        right_option: String,
        toggled_left: bool,
    ) -> Self {
        let mut toggle = Self {
            id: ELEMENT_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            position,
            size,
            left_option: left_option.clone(),
            right_option: right_option.clone(),
            toggled_left,
            left_button: Button::new(position, Vec2::ZERO, Some(left_option)),
            right_button: Button::new(position, Vec2::ZERO, Some(right_option)),
        };
        toggle.sync_internal_buttons();
        toggle
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.sync_internal_buttons();
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
        self.sync_internal_buttons();
    }

    pub fn selected_option(&self) -> &str {
        if self.toggled_left {
            self.left_option.as_str()
        } else {
            self.right_option.as_str()
        }
    }

    pub fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event> {
        let left_event = self.left_button.step(mouse_position, mouse_pressed);
        let right_event = self.right_button.step(mouse_position, mouse_pressed);

        if matches!(left_event, Some(Event::ButtonReleased)) {
            self.toggled_left = true;
            return Some(Event::ButtonToggleChanged { toggled_left: true });
        }

        if matches!(right_event, Some(Event::ButtonReleased)) {
            self.toggled_left = false;
            return Some(Event::ButtonToggleChanged {
                toggled_left: false,
            });
        }

        None
    }

    fn sync_internal_buttons(&mut self) {
        let half_width = self.size.x / 2.0;
        self.left_button.position = self.position;
        self.left_button.size = Vec2::new(half_width, self.size.y);

        self.right_button.position = Vec2::new(self.position.x + half_width, self.position.y);
        self.right_button.size = Vec2::new(half_width, self.size.y);
    }
}
