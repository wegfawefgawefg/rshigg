use glam::Vec2;
use std::collections::HashMap;

use super::{Button, Slider, TaggedEvent};

pub struct Gui<T: Clone + Copy> {
    pub el_to_tag_map: HashMap<u32, T>,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
}

impl<T: Clone + Copy> Gui<T> {
    pub fn new() -> Self {
        Self {
            el_to_tag_map: HashMap::new(),
            buttons: Vec::new(),
            sliders: Vec::new(),
        }
    }

    //// ADD ELEMENTS
    pub fn add_button(&mut self, button: Button, tag: T) {
        self.el_to_tag_map.insert(button.id, tag);
        self.buttons.push(button);
    }

    pub fn add_slider(&mut self, slider: Slider, tag: T) {
        self.el_to_tag_map.insert(slider.id, tag);
        self.sliders.push(slider);
    }

    //// REMOVE ELEMENTS
    pub fn remove_button(&mut self, id: u32) {
        self.buttons.retain(|button| button.id != id);
        self.el_to_tag_map.remove(&id);
    }

    pub fn remove_slider(&mut self, id: u32) {
        self.sliders.retain(|slider| slider.id != id);
        self.el_to_tag_map.remove(&id);
    }

    ///Step the gui, and all elements within.
    /// Mouse position should ideally be normalized between [0.0, 1.0].
    /// Values outside the range [0.0, 1.0] can be treated as outside the gui rect.
    /// This isnt required but will make things a whole lot easier for you.
    pub fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Vec<TaggedEvent<T>> {
        let mut tagged_events = Vec::new();
        for button in self.buttons.iter_mut() {
            if let Some(event) = button.step(mouse_position, mouse_pressed) {
                if let Some(tag) = self.el_to_tag_map.get(&button.id) {
                    tagged_events.push(TaggedEvent { tag: *tag, event });
                }
            }
        }
        for slider in self.sliders.iter_mut() {
            if let Some(event) = slider.step(mouse_position, mouse_pressed) {
                if let Some(tag) = self.el_to_tag_map.get(&slider.id) {
                    tagged_events.push(TaggedEvent { tag: *tag, event });
                }
            }
        }
        tagged_events
    }
}

impl<T: Clone + Copy> Default for Gui<T> {
    fn default() -> Self {
        Self::new()
    }
}
