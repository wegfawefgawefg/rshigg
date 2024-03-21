use glam::Vec2;
use raylib::color::Color;
use std::sync::atomic::{AtomicU32, Ordering};

static ELEMENT_NEXT_ID: AtomicU32 = AtomicU32::new(0);

pub enum Event {
    ButtonPressed,
    ButtonReleased,
}

pub trait Element {
    fn get_id(&self) -> u32;
    fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event>;
}

pub struct Button {
    pub id: u32,
    pub position: Vec2,
    pub size: Vec2,

    pub color: Color,
    pub label: Option<String>,

    pub hovered: bool,
    pub pressed: bool,
    pub was_pressed: bool,
}

impl Button {
    pub fn new(position: Vec2, size: Vec2, color: Color, label: Option<String>) -> Self {
        Self {
            id: ELEMENT_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            position,
            size,

            color,
            label,

            hovered: false,
            pressed: false,
            was_pressed: false,
        }
    }

    fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event> {
        let mut event: Option<Event> = None;
        if !mouse_pressed && self.was_pressed {
            event = Some(Event::ButtonReleased);
            self.was_pressed = false;
        }

        self.hovered = mouse_position.x > self.position.x
            && mouse_position.x < self.position.x + self.size.x
            && mouse_position.y > self.position.y
            && mouse_position.y < self.position.y + self.size.y;

        if mouse_pressed && self.hovered {
            if !self.pressed {
                event = Some(Event::ButtonPressed);
            }
            self.pressed = true;
            self.was_pressed = true;
        } else {
            self.pressed = false;
        }

        event
    }
}

pub struct Gui {
    pub buttons: Vec<Button>,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }

    pub fn add_button(&mut self, button: Button) {
        self.buttons.push(button);
    }

    ///Step the gui, and all elements within.
    /// Mouse position should ideally be normalized between [0.0, 1.0].
    /// Values outside the range [0.0, 1.0] can be treated as outside the gui rect.
    /// This isnt required but will make things a whole lot easier for you.
    pub fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();
        for button in self.buttons.iter_mut() {
            if let Some(event) = button.step(mouse_position, mouse_pressed) {
                events.push(event);
            }
        }
        events
    }
}
