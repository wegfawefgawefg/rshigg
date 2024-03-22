use glam::Vec2;
use std::collections::HashMap;

use crate::{Draggable, Label, VerticalSlider};

use super::{Button, Slider, TaggedEvent};

pub struct Gui<T: Clone + Copy> {
    pub el_to_tag_map: HashMap<u32, T>,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
    pub vertical_sliders: Vec<VerticalSlider>,
    pub draggables: Vec<Draggable>,
    pub labels: Vec<Label>,
}

impl<T: Clone + Copy> Gui<T> {
    pub fn new() -> Self {
        Self {
            el_to_tag_map: HashMap::new(),
            buttons: Vec::new(),
            sliders: Vec::new(),
            vertical_sliders: Vec::new(),
            draggables: Vec::new(),
            labels: Vec::new(),
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

    pub fn add_vertical_slider(&mut self, vertical_slider: VerticalSlider, tag: T) {
        self.el_to_tag_map.insert(vertical_slider.id, tag);
        self.vertical_sliders.push(vertical_slider);
    }

    pub fn add_draggable(&mut self, draggable: Draggable, tag: T) {
        self.el_to_tag_map.insert(draggable.id, tag);
        self.draggables.push(draggable);
    }

    pub fn add_label(&mut self, label: Label) {
        self.labels.push(label);
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

    pub fn remove_vertical_slider(&mut self, id: u32) {
        self.vertical_sliders
            .retain(|vertical_slider| vertical_slider.id != id);
        self.el_to_tag_map.remove(&id);
    }

    pub fn remove_draggable(&mut self, id: u32) {
        self.draggables.retain(|draggable| draggable.id != id);
        self.el_to_tag_map.remove(&id);
    }

    pub fn remove_label(&mut self, id: u32) {
        self.labels.retain(|label| label.id != id);
    }

    //// GET ELEMENTS
    pub fn get_button(&self, id: u32) -> Option<&Button> {
        self.buttons.iter().find(|button| button.id == id)
    }

    pub fn get_slider(&self, id: u32) -> Option<&Slider> {
        self.sliders.iter().find(|slider| slider.id == id)
    }

    pub fn get_vertical_slider(&self, id: u32) -> Option<&VerticalSlider> {
        self.vertical_sliders
            .iter()
            .find(|vertical_slider| vertical_slider.id == id)
    }

    pub fn get_draggable(&self, id: u32) -> Option<&Draggable> {
        self.draggables.iter().find(|draggable| draggable.id == id)
    }

    pub fn get_label(&self, id: u32) -> Option<&Label> {
        self.labels.iter().find(|label| label.id == id)
    }

    //// GET ELEMENTS MUT
    pub fn get_button_mut(&mut self, id: u32) -> Option<&mut Button> {
        self.buttons.iter_mut().find(|button| button.id == id)
    }

    pub fn get_slider_mut(&mut self, id: u32) -> Option<&mut Slider> {
        self.sliders.iter_mut().find(|slider| slider.id == id)
    }

    pub fn get_vertical_slider_mut(&mut self, id: u32) -> Option<&mut VerticalSlider> {
        self.vertical_sliders
            .iter_mut()
            .find(|vertical_slider| vertical_slider.id == id)
    }

    pub fn get_draggable_mut(&mut self, id: u32) -> Option<&mut Draggable> {
        self.draggables
            .iter_mut()
            .find(|draggable| draggable.id == id)
    }

    pub fn get_label_mut(&mut self, id: u32) -> Option<&mut Label> {
        self.labels.iter_mut().find(|label| label.id == id)
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
                    tagged_events.push(TaggedEvent {
                        tag: *tag,
                        element_id: button.id,
                        event,
                    });
                }
            }
        }
        for slider in self.sliders.iter_mut() {
            if let Some(event) = slider.step(mouse_position, mouse_pressed) {
                if let Some(tag) = self.el_to_tag_map.get(&slider.id) {
                    tagged_events.push(TaggedEvent {
                        tag: *tag,
                        element_id: slider.id,
                        event,
                    });
                }
            }
        }
        for vertical_slider in self.vertical_sliders.iter_mut() {
            if let Some(event) = vertical_slider.step(mouse_position, mouse_pressed) {
                if let Some(tag) = self.el_to_tag_map.get(&vertical_slider.id) {
                    tagged_events.push(TaggedEvent {
                        tag: *tag,
                        element_id: vertical_slider.id,
                        event,
                    });
                }
            }
        }
        for draggable in self.draggables.iter_mut() {
            if let Some(event) = draggable.step(mouse_position, mouse_pressed) {
                if let Some(tag) = self.el_to_tag_map.get(&draggable.id) {
                    tagged_events.push(TaggedEvent {
                        tag: *tag,
                        element_id: draggable.id,
                        event,
                    });
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
