use glam::Vec2;
use raylib::prelude::*;

use crate::{
    gui::{def_gui, draw_gui, Tag},
    rshigg::{Event, Gui, TaggedEvent},
    DIMS,
};

pub const FRAMES_PER_SECOND: u32 = 60;

pub struct Settings {
    pub potato: bool,
    pub hot_chip: bool,
    pub ice_cream: bool,
    pub steak: bool,

    pub temperature: f32,
    pub height: f32,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            potato: false,
            hot_chip: false,
            ice_cream: false,
            steak: false,
            temperature: 50.0,
            height: 50.0,
        }
    }
}

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,
    pub gui: Gui<Tag>,
    pub settings: Settings,
}

impl State {
    pub fn new() -> Self {
        Self {
            running: true,
            time_since_last_update: 0.0,
            gui: def_gui(),
            settings: Settings::new(),
        }
    }
}

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }
}

pub fn normalize_coord(pos: Vec2) -> Vec2 {
    Vec2::new(pos.x / DIMS.x as f32, pos.y / DIMS.y as f32)
}

pub fn handle_gui_events(state: &mut State, tagged_events: Vec<TaggedEvent<Tag>>) {
    for tagged_event in tagged_events {
        match (tagged_event.tag, tagged_event.event) {
            (Tag::SelectionPotato, Event::ButtonReleased) => {
                state.settings.potato = !state.settings.potato;
                println!("Potato: {}", state.settings.potato);
            }
            (Tag::SetTemperature, Event::SliderMoved { value }) => {
                state.settings.temperature = value;
                println!("Temp set to {}", value);
            }
            (Tag::SetHeight, Event::SliderMoved { value }) => {
                state.settings.height = value;
                println!("Height set to {}", value);
            }
            _ => {}
        }
    }
}

pub fn step(rl: &mut RaylibHandle, state: &mut State) {
    let mouse_pos = rl.get_mouse_position();
    let nmp = normalize_coord(Vec2::new(mouse_pos.x, mouse_pos.y));
    let mouse_pressed = rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON);

    let tagged_events = state.gui.step(nmp, mouse_pressed);
    handle_gui_events(state, tagged_events);
}

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    d.draw_text("Low Res Sketch!", 12, 12, 12, Color::WHITE);
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);

    let angle = d.get_time() as f32;

    let center = Vec2::new(d.get_screen_width() as f32, d.get_screen_height() as f32) / 2.0;
    let offset = center / 4.0;

    for i in 0..3 {
        let rot = glam::Mat2::from_angle(angle + i as f32 * 90.0);
        let rect_pos_rotated = rot * offset + center;

        let size =
            (((d.get_time() as f32 + i as f32 * 1.0) * 2.0).sin() + 1.0) / 2.0 * offset.y + 4.0;
        d.draw_rectangle(
            rect_pos_rotated.x as i32,
            rect_pos_rotated.y as i32,
            size as i32,
            size as i32,
            Color::RED,
        );
    }

    draw_gui(&state.gui, d);

    // render the settings in the top left
    let list_items = vec![
        format!("Potato: {}", state.settings.potato),
        format!("Temperature: {}", state.settings.temperature),
        format!("Height: {}", state.settings.height),
    ];
    render_list(d, &list_items, Vec2::new(12.0, 48.0), 24, Color::WHITE);
}

pub fn render_list(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    list: &Vec<String>,
    position: Vec2,
    font_size: i32,
    color: Color,
) {
    let mut cursor = position;
    let cursor_offset = Vec2::new(0.0, 24.0);

    for item in list {
        d.draw_text(item, cursor.x as i32, cursor.y as i32, font_size, color);
        cursor += cursor_offset;
    }
}
