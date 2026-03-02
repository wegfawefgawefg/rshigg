use glam::Vec2;
use std::collections::{HashMap, HashSet};

use crate::{
    ButtonToggle, Draggable, Label, LeftRightSelector, MoveAndResizeThumbs, VerticalSlider,
};

use super::{Button, Slider, TaggedEvent};

pub struct Gui<T: Clone + Copy> {
    pub el_to_tag_map: HashMap<u32, T>,
    pub hidden_ids: HashSet<u32>,
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
            hidden_ids: HashSet::new(),
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
        self.hidden_ids.remove(&button.id);
        self.buttons.push(button);
    }

    pub fn add_slider(&mut self, slider: Slider, tag: T) {
        self.el_to_tag_map.insert(slider.id, tag);
        self.hidden_ids.remove(&slider.id);
        self.sliders.push(slider);
    }

    pub fn add_vertical_slider(&mut self, vertical_slider: VerticalSlider, tag: T) {
        self.el_to_tag_map.insert(vertical_slider.id, tag);
        self.hidden_ids.remove(&vertical_slider.id);
        self.vertical_sliders.push(vertical_slider);
    }

    pub fn add_draggable(&mut self, draggable: Draggable, tag: T) {
        self.el_to_tag_map.insert(draggable.id, tag);
        self.hidden_ids.remove(&draggable.id);
        self.draggables.push(draggable);
    }

    pub fn add_label(&mut self, label: Label) {
        self.hidden_ids.remove(&label.id);
        self.labels.push(label);
    }

    pub fn add_left_right_selector(&mut self, left_right_selector: LeftRightSelector, tag: T) {
        self.el_to_tag_map.insert(left_right_selector.id, tag);
        self.hidden_ids.remove(&left_right_selector.id);
        self.left_right_selectors.push(left_right_selector);
    }

    pub fn add_button_toggle(&mut self, button_toggle: ButtonToggle, tag: T) {
        self.el_to_tag_map.insert(button_toggle.id, tag);
        self.hidden_ids.remove(&button_toggle.id);
        self.button_toggles.push(button_toggle);
    }

    pub fn add_move_and_resize_thumbs(
        &mut self,
        move_and_resize_thumbs: MoveAndResizeThumbs,
        tag: T,
    ) {
        self.el_to_tag_map.insert(move_and_resize_thumbs.id, tag);
        self.hidden_ids.remove(&move_and_resize_thumbs.id);
        self.move_and_resize_thumbs.push(move_and_resize_thumbs);
    }

    pub fn set_visible(&mut self, id: u32, visible: bool) {
        if visible {
            self.hidden_ids.remove(&id);
        } else {
            self.hidden_ids.insert(id);
        }
    }

    pub fn is_visible(&self, id: u32) -> bool {
        !self.hidden_ids.contains(&id)
    }

    //// REMOVE ELEMENTS
    pub fn remove_button(&mut self, id: u32) {
        self.buttons.retain(|button| button.id != id);
        self.el_to_tag_map.remove(&id);
        self.hidden_ids.remove(&id);
    }

    pub fn remove_slider(&mut self, id: u32) {
        self.sliders.retain(|slider| slider.id != id);
        self.el_to_tag_map.remove(&id);
        self.hidden_ids.remove(&id);
    }

    pub fn remove_vertical_slider(&mut self, id: u32) {
        self.vertical_sliders
            .retain(|vertical_slider| vertical_slider.id != id);
        self.el_to_tag_map.remove(&id);
        self.hidden_ids.remove(&id);
    }

    pub fn remove_draggable(&mut self, id: u32) {
        self.draggables.retain(|draggable| draggable.id != id);
        self.el_to_tag_map.remove(&id);
        self.hidden_ids.remove(&id);
    }

    pub fn remove_label(&mut self, id: u32) {
        self.labels.retain(|label| label.id != id);
        self.hidden_ids.remove(&id);
    }

    pub fn remove_left_right_selector(&mut self, id: u32) {
        self.left_right_selectors
            .retain(|selector| selector.id != id);
        self.el_to_tag_map.remove(&id);
        self.hidden_ids.remove(&id);
    }

    pub fn remove_button_toggle(&mut self, id: u32) {
        self.button_toggles.retain(|toggle| toggle.id != id);
        self.el_to_tag_map.remove(&id);
        self.hidden_ids.remove(&id);
    }

    pub fn remove_move_and_resize_thumbs(&mut self, id: u32) {
        self.move_and_resize_thumbs.retain(|thumbs| thumbs.id != id);
        self.el_to_tag_map.remove(&id);
        self.hidden_ids.remove(&id);
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

    /// Step the gui using mouse coordinates in this gui's pixel space.
    pub fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Vec<TaggedEvent<T>> {
        let mut tagged_events = Vec::new();
        for button in self.buttons.iter_mut() {
            if self.hidden_ids.contains(&button.id) {
                continue;
            }
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
            if self.hidden_ids.contains(&slider.id) {
                continue;
            }
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
            if self.hidden_ids.contains(&vertical_slider.id) {
                continue;
            }
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
            if self.hidden_ids.contains(&draggable.id) {
                continue;
            }
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
            if self.hidden_ids.contains(&selector.id) {
                continue;
            }
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
            if self.hidden_ids.contains(&toggle.id) {
                continue;
            }
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
            if self.hidden_ids.contains(&thumbs.id) {
                continue;
            }
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

    /// Step gui from a sub-rectangle in another surface.
    ///
    /// - `mouse_position_surface`: mouse position in the parent surface (pixels)
    /// - `subsurface_position`: top-left of the gui rectangle in that parent surface (pixels)
    /// - `subsurface_size`: size of the gui rectangle in the parent surface (pixels)
    /// - `gui_size`: logical pixel size used by this gui
    pub fn step_in_rect(
        &mut self,
        mouse_position_surface: Vec2,
        subsurface_position: Vec2,
        subsurface_size: Vec2,
        gui_size: Vec2,
        mouse_pressed: bool,
    ) -> Vec<TaggedEvent<T>> {
        if subsurface_size.x <= 0.0
            || subsurface_size.y <= 0.0
            || gui_size.x <= 0.0
            || gui_size.y <= 0.0
        {
            return self.step(Vec2::new(-1.0, -1.0), mouse_pressed);
        }
        let local_surface = mouse_position_surface - subsurface_position;
        let gui_mouse = Vec2::new(
            local_surface.x * gui_size.x / subsurface_size.x,
            local_surface.y * gui_size.y / subsurface_size.y,
        );
        self.step(gui_mouse, mouse_pressed)
    }
}

impl<T: Clone + Copy> Default for Gui<T> {
    fn default() -> Self {
        Self::new()
    }
}
