use glam::Vec2;
use std::sync::atomic::AtomicU32;

pub static ELEMENT_NEXT_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, Debug)]
pub struct TaggedEvent<T: Clone + Copy> {
    pub tag: T,
    pub element_id: u32,
    pub event: Event,
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    ButtonPressed,
    ButtonReleased,
    SliderMoved {
        value: f32,
    },
    SliderReleased {
        value: f32,
    },
    DraggablePressed,
    DraggableReleased {
        new_pos: Vec2,
    },
    DraggableMoved {
        new_pos: Vec2,
    },
    SelectionChanged {
        selected_option_index: usize,
        end_of_options_reached: bool,
    },
    ButtonToggleChanged {
        toggled_left: bool,
    },
    MoveAndResizeThumbsChanged {
        target_position: Vec2,
        target_size: Vec2,
    },
}
