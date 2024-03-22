use std::sync::atomic::Ordering;

use glam::Vec2;
use raylib::color::Color;

use super::{Event, ELEMENT_NEXT_ID};

pub struct VerticalSlider {
    pub id: u32,
    pub position: Vec2,
    pub size: Vec2,
    pub thumb_height: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub step_size: f32,
    pub snap_sensetivity_fraction: f32,
    pub value: f32,
    pub color: Color,
    pub label: Option<String>,
    pub hovered: bool,
    pub was_pressed: bool,
}

#[allow(clippy::too_many_arguments)]
impl VerticalSlider {
    pub fn new(
        position: Vec2,
        size: Vec2,
        thumb_height: f32,
        minimum: f32,
        maximum: f32,
        step_size: f32,
        default_value: f32,
        snap_sensetivity_fraction: f32,
        color: Color,
        label: Option<String>,
    ) -> Self {
        Self {
            id: ELEMENT_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            position,
            size,
            thumb_height,
            minimum,
            maximum,
            step_size,
            snap_sensetivity_fraction,
            value: default_value,
            color,
            label,
            hovered: false,
            was_pressed: false,
        }
    }

    pub fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event> {
        let mut event: Option<Event> = None;

        if self.was_pressed && !mouse_pressed {
            event = Some(Event::SliderReleased { value: self.value });
            self.was_pressed = false;
        }

        let tl = self.position;
        let br = tl + self.size;

        if mouse_position.x > tl.x
            && mouse_position.x < br.x
            && mouse_position.y > tl.y
            && mouse_position.y < br.y
        {
            self.hovered = true;
            if mouse_pressed {
                let old_value = self.value;

                let total = br.y - tl.y;
                let local_p = mouse_position.y - tl.y;
                let fraction = local_p / total;
                self.value = self.minimum + fraction * (self.maximum - self.minimum);

                // if value is within 5% of the minimum or maximum, snap to it
                if self.snap_sensetivity_fraction > 0.0 {
                    if self.value > self.maximum * (1.0 - self.snap_sensetivity_fraction) {
                        self.value = self.maximum;
                    }
                    if self.value < (self.maximum - self.minimum) * self.snap_sensetivity_fraction {
                        self.value = self.minimum;
                    }
                }

                // round to nearest 100th, needs to work for negative and 0
                self.value = (self.value * 100.0).round() / 100.0;

                // round to nearest step size
                self.value = (self.value / self.step_size).round() * self.step_size;

                // only emit event if value changed
                if self.value != old_value {
                    event = Some(Event::SliderMoved { value: self.value });
                }

                self.was_pressed = true;
            }
        } else {
            self.hovered = false;
        }

        event
    }
}
