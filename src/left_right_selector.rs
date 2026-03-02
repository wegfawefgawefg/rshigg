use std::sync::atomic::Ordering;

use glam::Vec2;

use super::{Button, Event, ELEMENT_NEXT_ID};

pub struct LeftRightSelector {
    pub id: u32,
    pub position: Vec2,
    pub size: Vec2,
    pub button_width: f32,
    pub options: Vec<String>,
    pub selected_option_index: usize,
    pub left_button: Button,
    pub right_button: Button,
}

impl LeftRightSelector {
    pub fn new(
        position: Vec2,
        size: Vec2,
        button_width: f32,
        options: Vec<String>,
        starting_option_index: usize,
    ) -> Self {
        let selected_option_index = if options.is_empty() {
            0
        } else {
            starting_option_index.min(options.len() - 1)
        };

        let mut selector = Self {
            id: ELEMENT_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            position,
            size,
            button_width,
            options,
            selected_option_index,
            left_button: Button::new(position, Vec2::ZERO, Some("<".to_string())),
            right_button: Button::new(position, Vec2::ZERO, Some(">".to_string())),
        };
        selector.sync_internal_buttons();
        selector
    }

    pub fn selected_option(&self) -> Option<&str> {
        self.options
            .get(self.selected_option_index)
            .map(String::as_str)
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.sync_internal_buttons();
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
        self.sync_internal_buttons();
    }

    pub fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event> {
        let left_event = self.left_button.step(mouse_position, mouse_pressed);
        let right_event = self.right_button.step(mouse_position, mouse_pressed);

        if self.options.is_empty() {
            return None;
        }

        if matches!(left_event, Some(Event::ButtonReleased)) {
            if self.selected_option_index > 0 {
                self.selected_option_index -= 1;
                return Some(Event::SelectionChanged {
                    selected_option_index: self.selected_option_index,
                    end_of_options_reached: self.selected_option_index == 0,
                });
            }
            return Some(Event::SelectionChanged {
                selected_option_index: self.selected_option_index,
                end_of_options_reached: true,
            });
        }

        if matches!(right_event, Some(Event::ButtonReleased)) {
            let max_index = self.options.len() - 1;
            if self.selected_option_index < max_index {
                self.selected_option_index += 1;
                return Some(Event::SelectionChanged {
                    selected_option_index: self.selected_option_index,
                    end_of_options_reached: self.selected_option_index == max_index,
                });
            }
            return Some(Event::SelectionChanged {
                selected_option_index: self.selected_option_index,
                end_of_options_reached: true,
            });
        }

        None
    }

    fn sync_internal_buttons(&mut self) {
        self.left_button.position = self.position;
        self.left_button.size = Vec2::new(self.button_width, self.size.y);

        self.right_button.position = Vec2::new(
            self.position.x + self.size.x - self.button_width,
            self.position.y,
        );
        self.right_button.size = Vec2::new(self.button_width, self.size.y);
    }
}
