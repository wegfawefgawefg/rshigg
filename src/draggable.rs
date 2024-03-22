use std::sync::atomic::Ordering;

use glam::Vec2;
use raylib::color::Color;

use super::{Event, ELEMENT_NEXT_ID};

/*
from ._element_event import ElementEvent
from ._element import Element


class DraggableReleased(ElementEvent):
    def __init__(self, tag) -> None:
        super().__init__(tag)


class DraggablePressed(ElementEvent):
    def __init__(self, tag) -> None:
        super().__init__(tag)


class DraggableMoved(ElementEvent):
    def __init__(self, tag) -> None:
        super().__init__(tag)


class Draggable(Element):
    def __init__(
        self,
        position,
        scale,
        color=(200, 200, 200),
        text=None,
        texture=None,
        label=None,
        label_color=None,
        image=None,
        pressed_tag=None,
        released_tag=None,
        moved_tag=None,
    ) -> None:
        super().__init__()
        self.position = position
        self.scale = scale
        self.color = color
        self.text = text
        self.texture = texture
        self.label = label
        self.label_color = label_color
        self.image = image

        self.pressed_tag = pressed_tag
        self.released_tag = released_tag
        self.moved_tag = moved_tag

        self.hovered = False
        self.being_dragged = True
        self.mouse_last_position = None
        self.was_pre_hovered = False
        self.pre_hover_countdown = 0

    def step(self, mouse_position, mouse_pressed) -> ElementEvent:
        event = None

        """For the section below, order of these clauses does matter.
        """

        # check hovered
        if (
            mouse_position.x > self.position.x
            and mouse_position.x < self.position.x + self.scale.x
            and mouse_position.y > self.position.y
            and mouse_position.y < self.position.y + self.scale.y
        ):
            self.hovered = True
        else:
            self.hovered = False

        # check if mouse hovered without clicking first
        # this is important so we dont also shove all draggables we drag through
        if self.hovered and not mouse_pressed and not self.being_dragged:
            self.was_pre_hovered = True
            self.pre_hover_countdown = 3

        # if you didnt get dragged within a frame, disable prehover
        self.pre_hover_countdown -= 1
        self.pre_hover_countdown = max(self.pre_hover_countdown, 0)
        if self.pre_hover_countdown == 0:
            self.was_pre_hovered = False

        # check drag start
        if (
            self.was_pre_hovered
            and not self.being_dragged
            and mouse_pressed
            and self.hovered
        ):
            self.being_dragged = True
            self.mouse_last_position = mouse_position
            event = DraggablePressed(self.pressed_tag)
            self.was_pre_hovered = False
            self.pre_hover_countdown = 0

        # check drag finished
        if self.being_dragged and not mouse_pressed:
            self.being_dragged = False
            event = DraggableReleased(self.released_tag)

        # drag move
        if self.being_dragged and mouse_position != self.mouse_last_position:
            delta = mouse_position - self.mouse_last_position
            self.position += delta
            self.mouse_last_position = mouse_position
            event = DraggableMoved(self.moved_tag)

        return event

*/

pub struct Draggable {
    pub id: u32,
    pub position: Vec2,
    pub size: Vec2,

    pub color: Color,
    pub label: Option<String>,

    pub hovered: bool,
    pub being_dragged: bool,
    pub mouse_last_position: Option<Vec2>,
    pub was_pre_hovered: bool,
    pub pre_hover_countdown: i32,
}

#[allow(clippy::too_many_arguments)]
impl Draggable {
    pub fn new(position: Vec2, size: Vec2, color: Color, label: Option<String>) -> Self {
        Self {
            id: ELEMENT_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            position,
            size,

            color,
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

        if in_bounds {
            self.hovered = true;
        } else {
            self.hovered = false;
        }

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
