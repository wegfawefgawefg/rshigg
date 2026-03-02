use std::sync::atomic::Ordering;

use glam::Vec2;

use super::ELEMENT_NEXT_ID;

pub struct Label {
    pub id: u32,
    pub position: Vec2,
    pub size: Vec2,

    pub text: Option<String>,
}

#[allow(clippy::too_many_arguments)]
impl Label {
    pub fn new(position: Vec2, size: Vec2, text: Option<String>) -> Self {
        Self {
            id: ELEMENT_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            position,
            size,

            text,
        }
    }
}
