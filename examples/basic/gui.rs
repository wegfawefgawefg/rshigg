use glam::Vec2;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle, RaylibTextureMode},
    text::measure_text_ex,
};
use rshigg::{Draggable, VerticalSlider};

use crate::{
    rshigg::{Button, Gui, Slider},
    sketch::reposition_test_elements,
    DIMS,
};

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

pub fn def_gui() -> (Gui<Tag>, TestElements) {
    let mut test_elements = TestElements::new();

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
    test_elements.potato_button = button.id;
    gui.add_button(button, Tag::SelectionPotato);

    cursor.y += 0.1;
    let button = Button::new(
        cursor,
        element_dims,
        default_color,
        Some("Hot Chip".to_string()),
    );
    test_elements.hot_chip_button = button.id;
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
    test_elements.slider = slider.id;
    gui.add_slider(slider, Tag::SetTemperature);

    // vertical slider now
    cursor.y += 0.2;
    let vertical_slider = VerticalSlider::new(
        cursor,
        Vec2::new(0.05, 0.2),
        0.05,
        0.0,
        100.0,
        1.0,
        50.0,
        0.05,
        default_color,
        Some("Height".to_string()),
    );
    test_elements.vertical_slider = vertical_slider.id;
    gui.add_vertical_slider(vertical_slider, Tag::SetHeight);

    // draggable
    cursor = Vec2::new(0.2, 0.2);
    let aspect_ratio = DIMS.x as f32 / DIMS.y as f32;
    let d_width = 0.2;
    let draggable = Draggable::new(
        cursor,
        Vec2::new(0.2, 0.05),
        default_color,
        Some("Thumb".to_string()),
    );
    test_elements.drag_thumb = draggable.id;
    gui.add_draggable(draggable, Tag::MoveThumb);

    // minimize window button
    // to the right of the draggable
    cursor.x += 0.2;
    let button = Button::new(
        cursor,
        Vec2::new(0.05, 0.05),
        default_color,
        Some("-".to_string()),
    );
    test_elements.minimize_window_button = button.id;
    gui.add_button(button, Tag::MinimizeMenu);

    // close window button
    // to the right of the minimize window button
    cursor.x += 0.05;
    let button = Button::new(
        cursor,
        Vec2::new(0.05, 0.05),
        default_color,
        Some("X".to_string()),
    );
    test_elements.close_window_button = button.id;
    gui.add_button(button, Tag::CloseMenu);

    let button = Button::new(
        Vec2::new(0.0, 0.9),
        Vec2::new(0.1, 0.1),
        default_color,
        Some("Menu".to_string()),
    );
    gui.add_button(button, Tag::OpenMenu);

    (gui, test_elements)
}

pub fn draw_gui(gui: &Gui<Tag>, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let resolution = Vec2::new(DIMS.x as f32, DIMS.y as f32);
    for button in &gui.buttons {
        draw_button(d, button, resolution);
    }
    for slider in &gui.sliders {
        draw_slider(d, slider, resolution);
    }
    for vertical_slider in &gui.vertical_sliders {
        draw_vertical_slider(d, vertical_slider, resolution);
    }
    for draggable in &gui.draggables {
        draw_draggable(d, draggable, resolution);
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

pub fn draw_vertical_slider(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    slider: &VerticalSlider,
    resolution: Vec2,
) {
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
    let rel_position_y = value_fraction * slider.size.y; // [0.0, slider_rel_width]
    let absolute_thumb_y = absolute_position.y + rel_position_y * resolution.y;

    let absolute_thumb_height = resolution.y * slider.thumb_height;
    let half_thumb_height = absolute_thumb_height / 2.0;

    let thumb_position = Vec2::new(absolute_position.x, absolute_thumb_y - half_thumb_height);
    let absolute_thumb_dimensions = Vec2::new(absolute_dimensions.x, absolute_thumb_height);

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

pub fn draw_draggable(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    draggable: &Draggable,
    resolution: Vec2,
) {
    let absolute_position = resolution * draggable.position;
    let absolute_dimensions = resolution * draggable.size;

    let ap = absolute_position;
    let ad = absolute_dimensions;
    let min_dim = ad.x.min(ad.y);
    let offset = Vec2::new(1.0, 1.0);

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

    // draggable
    d.draw_rectangle(
        (ap.x + offset.x) as i32,
        (ap.y + offset.y) as i32,
        (ad.x - offset.x) as i32,
        (ad.y - offset.y) as i32,
        draggable.color,
    );

    // draw 2 lines to indicate the draggable is grabbable
    // draw one line 30% from the top, and one 30% from the bottom
    let line_start_x = ad.x * 0.3 + ap.x;
    let line_end_x = ad.x * 0.7 + ap.x;

    let upper_line_height = ad.y * 0.3 + ap.y;
    let lower_line_height = ad.y * 0.7 + ap.y;

    let line_thickness = 2;

    let color_mod = 0.3;
    let line_color = raylib::color::Color::new(
        (draggable.color.r as f32 * color_mod) as u8,
        (draggable.color.g as f32 * color_mod) as u8,
        (draggable.color.b as f32 * color_mod) as u8,
        draggable.color.a,
    );
    d.draw_line(
        line_start_x as i32,
        upper_line_height as i32,
        line_end_x as i32,
        upper_line_height as i32,
        line_color,
    );
    d.draw_line(
        line_start_x as i32,
        lower_line_height as i32,
        line_end_x as i32,
        lower_line_height as i32,
        line_color,
    );
}

/*

def draw_draggable(surface, draggable, resolution):
    absolute_position = resolution * draggable.position
    absolute_dimensions = resolution * draggable.scale

    ap = absolute_position
    ad = absolute_dimensions
    min_dim = min(ad.x, ad.y)
    offset = glm.vec2(1, 1)

    if not draggable.hovered:
        # shadow
        pygame.draw.rect(
            surface,
            (0, 0, 0),
            (ap.to_tuple(), (ad + offset).to_tuple()),
        )

        # highlight
        pygame.draw.rect(
            surface,
            (255, 255, 255),
            (ap.to_tuple(), (ad).to_tuple()),
        )

        # draggable
        pygame.draw.rect(
            surface,
            draggable.color,
            ((ap + offset).to_tuple(), (ad - offset).to_tuple()),
        )

        # draw 2 lines to indicate the draggable is grabbable
        # draw one line 30% from the top, and one 30% from the bottom
        line_start_x = ad.x * 0.3 + ap.x
        line_end_x = ad.x * 0.7 + ap.x

        upper_line_height = ad.y * 0.3 + ap.y
        lower_line_height = ad.y * 0.7 + ap.y

        line_thickness = 2

        color_mod = 0.3
        line_color = [ce * color_mod for ce in draggable.color]
        pygame.draw.line(
            surface,
            line_color,
            (line_start_x, upper_line_height),
            (line_end_x, upper_line_height),
            line_thickness,
        )
        pygame.draw.line(
            surface,
            line_color,
            (line_start_x, lower_line_height),
            (line_end_x, lower_line_height),
            line_thickness,
        )

    else:  # draggable hovered
        # shadow
        pygame.draw.rect(
            surface,
            (0, 0, 0),
            (ap.to_tuple(), (ad + offset).to_tuple()),
        )

        # highlight
        pygame.draw.rect(
            surface,
            (255, 255, 255),
            (ap.to_tuple(), (ad).to_tuple()),
        )

        # button
        pygame.draw.rect(
            surface,
            (
                draggable.color[0] * 0.65,
                draggable.color[1] * 0.65,
                draggable.color[2] * 0.65,
            ),
            ((ap + offset).to_tuple(), (ad - offset).to_tuple()),
        )

    if draggable.image:
        # scale the image to fit the button
        # scale both dimensions by the same amount, but scale the larger dimension to fit the button
        image_dimensions = glm.vec2(
            draggable.image.get_width(), draggable.image.get_height()
        )
        larger_dimension = max(image_dimensions.x, image_dimensions.y)
        scale_factor = (ad * 0.9) / larger_dimension
        scaled_dimensions = image_dimensions * scale_factor
        centered_offset = scaled_dimensions * 0.05

        image_position = offset + ap + ad / 2 - scaled_dimensions / 2

        # draw the image
        surface.blit(
            pygame.transform.scale(
                draggable.image,
                (int(scaled_dimensions.x), int(scaled_dimensions.y)),
            ),
            image_position.to_tuple(),
        )

    elif draggable.label:
        font_offset = 0

        font = pygame.font.SysFont("Arial", 24)
        text = font.render(draggable.label, True, (0, 0, 0))
        text_position = (
            ap.x + ad.x / 2 - text.get_width() / 2,
            ap.y + ad.y / 2 - text.get_height() / 2,
        )

        surface.blit(
            text,
            text_position,
        )

*/
