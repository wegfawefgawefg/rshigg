use std::sync::atomic::Ordering;

use glam::Vec2;

use super::{Draggable, Event, ELEMENT_NEXT_ID};

pub struct MoveAndResizeThumbs {
    pub id: u32,
    pub thumb_size: Vec2,
    pub target_position: Vec2,
    pub target_size: Vec2,
    pub move_thumb: Draggable,
    pub resize_thumb: Draggable,
}

impl MoveAndResizeThumbs {
    pub fn new(thumb_size: Vec2, target_position: Vec2, target_size: Vec2) -> Self {
        let mut thumbs = Self {
            id: ELEMENT_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            thumb_size,
            target_position,
            target_size,
            move_thumb: Draggable::new(Vec2::ZERO, thumb_size, None),
            resize_thumb: Draggable::new(Vec2::ZERO, thumb_size, None),
        };
        thumbs.sync_thumbs_from_target();
        thumbs
    }

    pub fn set_target(&mut self, position: Vec2, size: Vec2) {
        self.target_position = position;
        self.target_size = size;
        self.sync_thumbs_from_target();
    }

    pub fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event> {
        let move_event = self.move_thumb.step(mouse_position, mouse_pressed);
        let resize_event = self.resize_thumb.step(mouse_position, mouse_pressed);
        let mut changed = false;

        if matches!(move_event, Some(Event::DraggableMoved { .. })) {
            self.target_position = self.move_thumb.position + self.thumb_size;
            self.resize_thumb.position = self.target_position + self.target_size;
            changed = true;
        }

        if matches!(resize_event, Some(Event::DraggableMoved { .. })) {
            let min_size = self.thumb_size.max(Vec2::new(1.0, 1.0));
            let mut new_size = self.resize_thumb.position - self.target_position;
            new_size.x = new_size.x.max(min_size.x);
            new_size.y = new_size.y.max(min_size.y);
            self.target_size = new_size;
            self.resize_thumb.position = self.target_position + self.target_size;
            changed = true;
        }

        self.sync_thumbs_from_target();

        if changed {
            return Some(Event::MoveAndResizeThumbsChanged {
                target_position: self.target_position,
                target_size: self.target_size,
            });
        }

        None
    }

    fn sync_thumbs_from_target(&mut self) {
        self.move_thumb.size = self.thumb_size;
        self.resize_thumb.size = self.thumb_size;
        self.move_thumb.position = self.target_position - self.thumb_size;
        self.resize_thumb.position = self.target_position + self.target_size;
    }
}
