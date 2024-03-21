use glam::Vec2;
use raylib::color::Color;
use std::sync::atomic::{AtomicU32, Ordering};

static ELEMENT_NEXT_ID: AtomicU32 = AtomicU32::new(0);

pub enum Event {
    ButtonPressed,
    ButtonReleased,
    SliderMoved,
    SliderReleased,
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

pub struct Slider {
    pub id: u32,
    pub position: Vec2,
    pub size: Vec2,
    pub thumb_width: f32,
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
impl Slider {
    pub fn new(
        position: Vec2,
        size: Vec2,
        thumb_width: f32,
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
            thumb_width,
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

    fn step(&mut self, mouse_position: Vec2, mouse_pressed: bool) -> Option<Event> {
        let mut event: Option<Event> = None;

        if self.was_pressed && !mouse_pressed {
            event = Some(Event::SliderReleased);
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

                let total = br.x - tl.x;
                let local_p = mouse_position.x - tl.x;
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
                    event = Some(Event::SliderMoved);
                }

                self.was_pressed = true;
            }
        } else {
            self.hovered = false;
        }

        event
    }
}

pub struct Gui {
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            sliders: Vec::new(),
        }
    }

    pub fn add_button(&mut self, button: Button) {
        self.buttons.push(button);
    }

    pub fn add_slider(&mut self, slider: Slider) {
        self.sliders.push(slider);
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
        for slider in self.sliders.iter_mut() {
            if let Some(event) = slider.step(mouse_position, mouse_pressed) {
                events.push(event);
            }
        }
        events
    }
}
