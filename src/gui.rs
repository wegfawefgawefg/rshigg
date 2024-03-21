use glam::Vec2;
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle, RaylibTextureMode},
    text::measure_text_ex,
};

use crate::{
    rshigg::{Button, Gui},
    DIMS,
};

enum Textures {}

pub enum FoodSelector {
    SelectionPotato,
    SelectionHotChip,
    SelectionIceCream,
    SelectionSteak,
}

pub fn def_gui() -> Gui {
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
    gui.add_button(button);
    gui
}

pub fn draw_gui(gui: &Gui, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let resolution = Vec2::new(DIMS.x as f32, DIMS.y as f32);
    for button in &gui.buttons {
        draw_button(d, button, resolution);
    }
}

pub fn draw_button(d: &mut RaylibTextureMode<RaylibDrawHandle>, button: &Button, resolution: Vec2) {
    let absolute_position = resolution * button.position;
    let absolute_dimensions = resolution * button.size;

    let ap = absolute_position;
    let ad = absolute_dimensions;
    let min_dim = ad.x.min(ad.y);
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

/*
def draw_button(surface, button, resolution):
    absolute_position = resolution * button.position
    absolute_dimensions = resolution * button.scale

    ap = absolute_position
    ad = absolute_dimensions
    min_dim = min(ad.x, ad.y)
    offset = glm.vec2(1, 1)

    if not button.pressed:
        if not button.hovered:
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
                button.color,
                ((ap + offset).to_tuple(), (ad - offset).to_tuple()),
            )
        else:  # button hovered
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
                    button.color[0] * 0.65,
                    button.color[1] * 0.65,
                    button.color[2] * 0.65,
                ),
                ((ap + offset).to_tuple(), (ad - offset).to_tuple()),
            )
    elif button.pressed:
        # under shadow
        pygame.draw.rect(
            surface,
            (255, 255, 255),
            (ap.to_tuple(), (ad + offset).to_tuple()),
        )

        # under highlight
        pygame.draw.rect(
            surface,
            (0, 0, 0),
            (ap.to_tuple(), (ad).to_tuple()),
        )

        # button
        pygame.draw.rect(
            surface,
            (
                button.color[0] * 0.65,
                button.color[1] * 0.65,
                button.color[2] * 0.65,
            ),
            ((ap + offset).to_tuple(), (ad - offset).to_tuple()),
        )

    if button.image:
        # scale the image to fit the button
        # scale both dimensions by the same amount, but scale the larger dimension to fit the button
        image_dimensions = glm.vec2(button.image.get_width(), button.image.get_height())
        larger_dimension = max(image_dimensions.x, image_dimensions.y)
        scale_factor = (ad * 0.9) / larger_dimension
        scaled_dimensions = image_dimensions * scale_factor
        centered_offset = scaled_dimensions * 0.05

        image_position = offset + ap + ad / 2 - scaled_dimensions / 2

        # draw the image
        if not button.pressed:
            surface.blit(
                pygame.transform.scale(
                    button.image,
                    (int(scaled_dimensions.x), int(scaled_dimensions.y)),
                ),
                image_position.to_tuple(),
            )
        else:
            image_position = glm.vec2(
                image_position.x,
                image_position.y + centered_offset.y,
            )
            surface.blit(
                pygame.transform.scale(
                    button.image,
                    (int(scaled_dimensions.x), int(scaled_dimensions.y)),
                ),
                image_position.to_tuple(),
            )

    elif button.label:
        font_offset = 0

        font = pygame.font.SysFont("Arial", 24)
        text = font.render(button.label, True, (0, 0, 0))
        text_position = (
            ap.x + ad.x / 2 - text.get_width() / 2,
            ap.y + ad.y / 2 - text.get_height() / 2,
        )

        if button.pressed:
            # offset text_position down by offset.y
            text_position = (
                text_position[0],
                text_position[1] + offset.y,
            )

        surface.blit(
            text,
            text_position,
        )
*/
