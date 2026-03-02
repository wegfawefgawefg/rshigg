use glam::Vec2;
use std::collections::HashMap;

use crate::{
    ButtonToggle, Draggable, Label, LeftRightSelector, MoveAndResizeThumbs, VerticalSlider,
};

use super::{Button, Slider, TaggedEvent};

pub struct Gui<T: Clone + Copy> {
    pub el_to_tag_map: HashMap<u32, T>,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
    pub vertical_sliders: Vec<VerticalSlider>,
    pub draggables: Vec<Draggable>,
    pub labels: Vec<Label>,
    pub left_right_selectors: Vec<LeftRightSelector>,
    pub button_toggles: Vec<ButtonToggle>,
    pub move_and_resize_thumbs: Vec<MoveAndResizeThumbs>,
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
            left_right_selectors: Vec::new(),
            button_toggles: Vec::new(),
            move_and_resize_thumbs: Vec::new(),
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

    pub fn add_left_right_selector(&mut self, left_right_selector: LeftRightSelector, tag: T) {
        self.el_to_tag_map.insert(left_right_selector.id, tag);
        self.left_right_selectors.push(left_right_selector);
    }

    pub fn add_button_toggle(&mut self, button_toggle: ButtonToggle, tag: T) {
        self.el_to_tag_map.insert(button_toggle.id, tag);
        self.button_toggles.push(button_toggle);
    }

    pub fn add_move_and_resize_thumbs(
        &mut self,
        move_and_resize_thumbs: MoveAndResizeThumbs,
        tag: T,
    ) {
        self.el_to_tag_map.insert(move_and_resize_thumbs.id, tag);
        self.move_and_resize_thumbs.push(move_and_resize_thumbs);
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

    pub fn remove_left_right_selector(&mut self, id: u32) {
        self.left_right_selectors
            .retain(|selector| selector.id != id);
        self.el_to_tag_map.remove(&id);
    }

    pub fn remove_button_toggle(&mut self, id: u32) {
        self.button_toggles.retain(|toggle| toggle.id != id);
        self.el_to_tag_map.remove(&id);
    }

    pub fn remove_move_and_resize_thumbs(&mut self, id: u32) {
        self.move_and_resize_thumbs.retain(|thumbs| thumbs.id != id);
        self.el_to_tag_map.remove(&id);
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

    pub fn get_left_right_selector(&self, id: u32) -> Option<&LeftRightSelector> {
        self.left_right_selectors
            .iter()
            .find(|selector| selector.id == id)
    }

    pub fn get_button_toggle(&self, id: u32) -> Option<&ButtonToggle> {
        self.button_toggles.iter().find(|toggle| toggle.id == id)
    }

    pub fn get_move_and_resize_thumbs(&self, id: u32) -> Option<&MoveAndResizeThumbs> {
        self.move_and_resize_thumbs
            .iter()
            .find(|thumbs| thumbs.id == id)
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

    pub fn get_left_right_selector_mut(&mut self, id: u32) -> Option<&mut LeftRightSelector> {
        self.left_right_selectors
            .iter_mut()
            .find(|selector| selector.id == id)
    }

    pub fn get_button_toggle_mut(&mut self, id: u32) -> Option<&mut ButtonToggle> {
        self.button_toggles
            .iter_mut()
            .find(|toggle| toggle.id == id)
    }

    pub fn get_move_and_resize_thumbs_mut(&mut self, id: u32) -> Option<&mut MoveAndResizeThumbs> {
        self.move_and_resize_thumbs
            .iter_mut()
            .find(|thumbs| thumbs.id == id)
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
        for selector in self.left_right_selectors.iter_mut() {
            if let Some(event) = selector.step(mouse_position, mouse_pressed) {
                if let Some(tag) = self.el_to_tag_map.get(&selector.id) {
                    tagged_events.push(TaggedEvent {
                        tag: *tag,
                        element_id: selector.id,
                        event,
                    });
                }
            }
        }
        for toggle in self.button_toggles.iter_mut() {
            if let Some(event) = toggle.step(mouse_position, mouse_pressed) {
                if let Some(tag) = self.el_to_tag_map.get(&toggle.id) {
                    tagged_events.push(TaggedEvent {
                        tag: *tag,
                        element_id: toggle.id,
                        event,
                    });
                }
            }
        }
        for thumbs in self.move_and_resize_thumbs.iter_mut() {
            if let Some(event) = thumbs.step(mouse_position, mouse_pressed) {
                if let Some(tag) = self.el_to_tag_map.get(&thumbs.id) {
                    tagged_events.push(TaggedEvent {
                        tag: *tag,
                        element_id: thumbs.id,
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
