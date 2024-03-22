use glam::Vec2;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle, RaylibTextureMode},
    text::measure_text_ex,
};

use crate::{
    rshigg::{Button, Gui, Slider},
    DIMS,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Tag {
    SelectionPotato,
    SelectionHotChip,
    SetTemperature,
}

pub fn def_gui() -> Gui<Tag> {
    let default_color = Color::new(200, 200, 200, 255);

    let mut gui = Gui::new();
    let mut cursor = Vec2::new(0.2, 0.2);
    let element_dims = Vec2::new(0.1, 0.05);
    let button = Button::new(
        cursor,
        element_dims,
        default_color,
        Some("Potato".to_string()),
    );
    gui.add_button(button, Tag::SelectionPotato);

    cursor.y += 0.1;
    let button = Button::new(
        cursor,
        element_dims,
        default_color,
        Some("Hot Chip".to_string()),
    );
    gui.add_button(button, Tag::SelectionHotChip);

    // slider now
    cursor.y += 0.2;
    let slider = Slider::new(
        cursor,
        Vec2::new(0.2, 0.05),
        0.02,
        0.0,
        100.0,
        1.0,
        50.0,
        0.05,
        default_color,
        Some("Temperature".to_string()),
    );
    gui.add_slider(slider, Tag::SetTemperature);

    gui
}

pub fn draw_gui(gui: &Gui<Tag>, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let resolution = Vec2::new(DIMS.x as f32, DIMS.y as f32);
    for button in &gui.buttons {
        draw_button(d, button, resolution);
    }
    for slider in &gui.sliders {
        draw_slider(d, slider, resolution);
    }
}

pub fn draw_button(d: &mut RaylibTextureMode<RaylibDrawHandle>, button: &Button, resolution: Vec2) {
    let absolute_position = resolution * button.position;
    let absolute_dimensions = resolution * button.size;

    let ap = absolute_position;
    let ad = absolute_dimensions;
    let offset = Vec2::new(1.0, 1.0);

    if !button.pressed {
        if !button.hovered {
            // shadow
            d.draw_rectangle(
                ap.x as i32,
                ap.y as i32,
                (ad.x + offset.x) as i32,
                (ad.y + offset.y) as i32,
                raylib::color::Color::BLACK,
            );

            // highlight
            d.draw_rectangle(
                ap.x as i32,
                ap.y as i32,
                ad.x as i32,
                ad.y as i32,
                raylib::color::Color::WHITE,
            );

            // button
            d.draw_rectangle(
                (ap.x + offset.x) as i32,
                (ap.y + offset.y) as i32,
                (ad.x - offset.x) as i32,
                (ad.y - offset.y) as i32,
                button.color,
            );
        } else {
            // shadow
            d.draw_rectangle(
                ap.x as i32,
                ap.y as i32,
                (ad.x + offset.x) as i32,
                (ad.y + offset.y) as i32,
                raylib::color::Color::BLACK,
            );

            // highlight
            d.draw_rectangle(
                ap.x as i32,
                ap.y as i32,
                ad.x as i32,
                ad.y as i32,
                raylib::color::Color::WHITE,
            );

            // button
            d.draw_rectangle(
                (ap.x + offset.x) as i32,
                (ap.y + offset.y) as i32,
                (ad.x - offset.x) as i32,
                (ad.y - offset.y) as i32,
                raylib::color::Color::new(
                    (button.color.r as f32 * 0.65) as u8,
                    (button.color.g as f32 * 0.65) as u8,
                    (button.color.b as f32 * 0.65) as u8,
                    button.color.a,
                ),
            );
        }
    } else {
        // under shadow
        d.draw_rectangle(
            ap.x as i32,
            ap.y as i32,
            (ad.x + offset.x) as i32,
            (ad.y + offset.y) as i32,
            raylib::color::Color::WHITE,
        );

        // under highlight
        d.draw_rectangle(
            ap.x as i32,
            ap.y as i32,
            ad.x as i32,
            ad.y as i32,
            raylib::color::Color::BLACK,
        );

        // button
        d.draw_rectangle(
            (ap.x + offset.x) as i32,
            (ap.y + offset.y) as i32,
            (ad.x - offset.x) as i32,
            (ad.y - offset.y) as i32,
            raylib::color::Color::new(
                (button.color.r as f32 * 0.65) as u8,
                (button.color.g as f32 * 0.65) as u8,
                (button.color.b as f32 * 0.65) as u8,
                button.color.a,
            ),
        );
    }

    if let Some(label) = &button.label {
        let font = d.get_font_default();
        let text_shape = measure_text_ex(font, label, 20.0, 1.0);
        let text_center = text_shape / 2.0;

        d.draw_text(
            label,
            (ap.x + ad.x / 2.0 - text_center.x) as i32,
            (ap.y + ad.y / 2.0 - text_center.y) as i32,
            20,
            raylib::color::Color::BLACK,
        );
    }
}

pub fn draw_slider(d: &mut RaylibTextureMode<RaylibDrawHandle>, slider: &Slider, resolution: Vec2) {
    let absolute_position = resolution * slider.position;
    let absolute_dimensions = resolution * slider.size;

    let ap = absolute_position;
    let ad = absolute_dimensions;

    // draw body
    d.draw_rectangle(
        ap.x as i32,
        ap.y as i32,
        ad.x as i32,
        ad.y as i32,
        raylib::color::Color::new(100, 100, 100, 255),
    );

    // draw body
    let value_fraction = (slider.value - slider.minimum) / (slider.maximum - slider.minimum); // range [0.0 , 1.0]
    let rel_position_x = value_fraction * slider.size.x; // [0.0, slider_rel_width]
    let absolute_thumb_x = absolute_position.x + rel_position_x * resolution.x;

    let absolute_thumb_width = resolution.x * slider.thumb_width;
    let half_thumb_width = absolute_thumb_width / 2.0;

    let thumb_position = Vec2::new(absolute_thumb_x - half_thumb_width, absolute_position.y);
    let absolute_thumb_dimensions = Vec2::new(absolute_thumb_width, absolute_dimensions.y);

    let tp = thumb_position;
    let td = absolute_thumb_dimensions;
    let offset = Vec2::new(1.0, 1.0);

    // shadow
    d.draw_rectangle(
        tp.x as i32,
        tp.y as i32,
        (td.x + offset.x) as i32,
        (td.y + offset.y) as i32,
        raylib::color::Color::BLACK,
    );

    // highlight
    d.draw_rectangle(
        tp.x as i32,
        tp.y as i32,
        td.x as i32,
        td.y as i32,
        raylib::color::Color::WHITE,
    );

    // slider center
    d.draw_rectangle(
        (tp.x + offset.x) as i32,
        (tp.y + offset.y) as i32,
        (td.x - offset.x) as i32,
        (td.y - offset.y) as i32,
        slider.color,
    );
}
