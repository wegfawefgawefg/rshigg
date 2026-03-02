use glam::Vec2;
use rshigg::{Draggable, VerticalSlider};

use crate::rshigg::{Button, Gui, Slider};
use crate::DIMS;

fn px(x: f32, y: f32) -> Vec2 {
    Vec2::new(x * DIMS.x as f32, y * DIMS.y as f32)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Tag {
    CloseMenu,
    MinimizeMenu,
    SelectionPotato,
    SelectionHotChip,
    SetTemperature,
    SetHeight,
    MoveThumb,
    OpenMenu,
}

pub struct TestElements {
    pub drag_thumb: u32,
    pub close_window_button: u32,
    pub minimize_window_button: u32,
    pub potato_button: u32,
    pub hot_chip_button: u32,
    pub slider: u32,
    pub vertical_slider: u32,
}

impl TestElements {
    pub fn new() -> Self {
        Self {
            drag_thumb: 0,
            close_window_button: 0,
            minimize_window_button: 0,
            potato_button: 0,
            hot_chip_button: 0,
            slider: 0,
            vertical_slider: 0,
        }
    }
}

pub fn def_test_elements_gui() -> (Gui<Tag>, TestElements) {
    let mut test_elements = TestElements::new();

    let mut gui = Gui::new();
    let mut cursor = px(0.2, 0.2);
    let element_dims = px(0.1, 0.05);
    let button = Button::new(cursor, element_dims, Some("Potato".to_string()));
    test_elements.potato_button = button.id;
    gui.add_button(button, Tag::SelectionPotato);

    cursor.y += DIMS.y as f32 * 0.1;
    let button = Button::new(cursor, element_dims, Some("Hot Chip".to_string()));
    test_elements.hot_chip_button = button.id;
    gui.add_button(button, Tag::SelectionHotChip);

    // slider now
    cursor.y += DIMS.y as f32 * 0.2;
    let slider = Slider::new(
        cursor,
        px(0.2, 0.05),
        16.0,
        0.0,
        100.0,
        1.0,
        50.0,
        0.05,
        Some("Temperature".to_string()),
    );
    test_elements.slider = slider.id;
    gui.add_slider(slider, Tag::SetTemperature);

    // vertical slider now
    cursor.y += DIMS.y as f32 * 0.2;
    let vertical_slider = VerticalSlider::new(
        cursor,
        px(0.05, 0.2),
        24.0,
        0.0,
        100.0,
        1.0,
        50.0,
        0.05,
        Some("Height".to_string()),
    );
    test_elements.vertical_slider = vertical_slider.id;
    gui.add_vertical_slider(vertical_slider, Tag::SetHeight);

    // draggable
    cursor = px(0.2, 0.2);
    // let aspect_ratio = DIMS.x as f32 / DIMS.y as f32;
    // let d_width = 0.2;
    let draggable = Draggable::new(cursor, px(0.2, 0.05), Some("Thumb".to_string()));
    test_elements.drag_thumb = draggable.id;
    gui.add_draggable(draggable, Tag::MoveThumb);

    // minimize window button
    // to the right of the draggable
    cursor.x += DIMS.x as f32 * 0.2;
    let button = Button::new(cursor, px(0.05, 0.05), Some("-".to_string()));
    test_elements.minimize_window_button = button.id;
    gui.add_button(button, Tag::MinimizeMenu);

    // close window button
    // to the right of the minimize window button
    cursor.x += DIMS.x as f32 * 0.05;
    let button = Button::new(cursor, px(0.05, 0.05), Some("X".to_string()));
    test_elements.close_window_button = button.id;
    gui.add_button(button, Tag::CloseMenu);

    let button = Button::new(px(0.0, 0.9), px(0.1, 0.1), Some("Menu".to_string()));
    gui.add_button(button, Tag::OpenMenu);

    (gui, test_elements)
}
