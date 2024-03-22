use std::sync::atomic::AtomicU32;

use glam::Vec2;

pub static ELEMENT_NEXT_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, Debug)]
pub struct TaggedEvent<T: Clone + Copy> {
    pub tag: T,
    pub event: Event,
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    ButtonPressed,
    ButtonReleased,
    SliderMoved { value: f32 },
    SliderReleased { value: f32 },
}

pub trait Element {
    fn get_id(&self) -> u32;
    fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event>;
}
