use std::sync::atomic::Ordering;

use glam::Vec2;

use super::{Event, ELEMENT_NEXT_ID};

pub struct Draggable {
    pub id: u32,
    pub position: Vec2,
    pub size: Vec2,

    pub label: Option<String>,

    pub hovered: bool,
    pub being_dragged: bool,
    pub mouse_last_position: Option<Vec2>,
    pub was_pre_hovered: bool,
    pub pre_hover_countdown: i32,
}

#[allow(clippy::too_many_arguments)]
impl Draggable {
    pub fn new(position: Vec2, size: Vec2, label: Option<String>) -> Self {
        Self {
            id: ELEMENT_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            position,
            size,

            label,

            hovered: false,
            being_dragged: false,
            mouse_last_position: None,
            was_pre_hovered: false,
            pre_hover_countdown: 0,
        }
    }

    pub fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event> {
        let mut event: Option<Event> = None;

        let in_bounds = mouse_position.x > self.position.x
            && mouse_position.x < self.position.x + self.size.x
            && mouse_position.y > self.position.y
            && mouse_position.y < self.position.y + self.size.y;
        self.hovered = in_bounds;

        if self.hovered && !mouse_pressed && !self.being_dragged {
            self.was_pre_hovered = true;
            self.pre_hover_countdown = 3;
        }

        if self.pre_hover_countdown > 0 {
            self.pre_hover_countdown -= 1;
        }

        if self.pre_hover_countdown == 0 {
            self.was_pre_hovered = false;
        }

        if self.was_pre_hovered && !self.being_dragged && mouse_pressed && self.hovered {
            self.being_dragged = true;
            self.mouse_last_position = Some(mouse_position);
            event = Some(Event::DraggablePressed);

            self.was_pre_hovered = false;
            self.pre_hover_countdown = 0;
        }

        if self.being_dragged && !mouse_pressed {
            self.being_dragged = false;
            event = Some(Event::DraggableReleased {
                new_pos: self.position,
            });
        }

        if self.being_dragged && mouse_position != self.mouse_last_position.unwrap() {
            let delta = mouse_position - self.mouse_last_position.unwrap();
            self.position += delta;
            self.mouse_last_position = Some(mouse_position);
            event = Some(Event::DraggableMoved {
                new_pos: self.position,
            });
        }

        event
    }
}
