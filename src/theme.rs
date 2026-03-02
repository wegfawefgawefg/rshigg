use glam::Vec2;

use crate::{
    Button, ButtonToggle, Color, Draggable, DrawBackend, Gui, ImageStyle, Label, LeftRightSelector,
    MoveAndResizeThumbs, Rect, Slider, VerticalSlider,
};

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub control_color: Color,
    pub text_color: Color,
    pub shadow_color: Color,
    pub highlight_color: Color,
    pub track_color: Color,
    pub hover_shade: f32,
    pub pressed_shade: f32,
    pub bevel_size_px: f32,
    pub font_size_px: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            control_color: Color::rgb(200, 200, 200),
            text_color: Color::rgb(0, 0, 0),
            shadow_color: Color::rgb(0, 0, 0),
            highlight_color: Color::rgb(255, 255, 255),
            track_color: Color::rgb(100, 100, 100),
            hover_shade: 0.65,
            pressed_shade: 0.65,
            bevel_size_px: 1.0,
            font_size_px: 20.0,
        }
    }
}

pub fn draw_gui<T: Clone + Copy, B: DrawBackend>(gui: &Gui<T>, backend: &mut B, theme: &Theme) {
    for label in &gui.labels {
        if !gui.is_visible(label.id) {
            continue;
        }
        draw_label(backend, label, theme);
    }
    for button in &gui.buttons {
        if !gui.is_visible(button.id) {
            continue;
        }
        draw_button(backend, button, theme);
    }
    for slider in &gui.sliders {
        if !gui.is_visible(slider.id) {
            continue;
        }
        draw_slider(backend, slider, theme);
    }
    for slider in &gui.vertical_sliders {
        if !gui.is_visible(slider.id) {
            continue;
        }
        draw_vertical_slider(backend, slider, theme);
    }
    for draggable in &gui.draggables {
        if !gui.is_visible(draggable.id) {
            continue;
        }
        draw_draggable(backend, draggable, theme);
    }
    for selector in &gui.left_right_selectors {
        if !gui.is_visible(selector.id) {
            continue;
        }
        draw_left_right_selector(backend, selector, theme);
    }
    for toggle in &gui.button_toggles {
        if !gui.is_visible(toggle.id) {
            continue;
        }
        draw_button_toggle(backend, toggle, theme);
    }
    for thumbs in &gui.move_and_resize_thumbs {
        if !gui.is_visible(thumbs.id) {
            continue;
        }
        draw_move_and_resize_thumbs(backend, thumbs, theme);
    }
}

fn draw_label<B: DrawBackend>(backend: &mut B, label: &Label, theme: &Theme) {
    let rect = Rect::new(label.position, label.size);
    if rect.size.x <= 0.0 || rect.size.y <= 0.0 {
        return;
    }

    match label.background_image {
        Some(image) if !image.draw_over_content => backend.draw_image(image, rect),
        _ => backend.fill_rect(rect, theme.control_color),
    }

    if let Some(text) = &label.text {
        let text_pos = rect.position + Vec2::new(6.0, 4.0);
        backend.draw_text(text, text_pos, theme.font_size_px, theme.text_color);
    }

    if let Some(image) = label.background_image {
        if image.draw_over_content {
            backend.draw_image(image, rect);
        }
    }
}

fn draw_button<B: DrawBackend>(backend: &mut B, button: &Button, theme: &Theme) {
    draw_button_visual(
        backend,
        theme,
        button.position,
        button.size,
        button.label.as_deref(),
        button.hovered,
        button.pressed,
        button.background_image,
    );
}

fn draw_button_visual<B: DrawBackend>(
    backend: &mut B,
    theme: &Theme,
    position: Vec2,
    size: Vec2,
    label: Option<&str>,
    hovered: bool,
    pressed: bool,
    background_image: Option<ImageStyle>,
) {
    let rect = Rect::new(position, size);
    if rect.size.x <= 0.0 || rect.size.y <= 0.0 {
        return;
    }
    draw_beveled_box(backend, rect, theme, hovered, pressed, theme.control_color);

    if let Some(image) = background_image {
        if !image.draw_over_content {
            backend.draw_image(image, rect);
        }
    }

    if let Some(label) = label {
        let text_pos = rect.position + Vec2::new(6.0, 4.0);
        let text_offset = if pressed {
            Vec2::new(0.0, theme.bevel_size_px)
        } else {
            Vec2::ZERO
        };
        backend.draw_text(
            label,
            text_pos + text_offset,
            theme.font_size_px,
            theme.text_color,
        );
    }

    if let Some(image) = background_image {
        if image.draw_over_content {
            backend.draw_image(image, rect);
        }
    }
}

fn draw_slider<B: DrawBackend>(backend: &mut B, slider: &Slider, theme: &Theme) {
    let body = Rect::new(slider.position, slider.size);
    if body.size.x <= 0.0 || body.size.y <= 0.0 {
        return;
    }

    match slider.track_image {
        Some(image) if !image.draw_over_content => backend.draw_image(image, body),
        _ => backend.fill_rect(body, theme.track_color),
    }

    let range = (slider.maximum - slider.minimum).abs();
    let value_fraction = if range <= f32::EPSILON {
        0.0
    } else {
        ((slider.value - slider.minimum) / (slider.maximum - slider.minimum)).clamp(0.0, 1.0)
    };

    let thumb_x = body.position.x + value_fraction * body.size.x;
    let thumb_width = slider.thumb_width.max(1.0);
    let thumb_rect = Rect::new(
        Vec2::new(thumb_x - thumb_width / 2.0, body.position.y),
        Vec2::new(thumb_width, body.size.y),
    );

    draw_beveled_box(
        backend,
        thumb_rect,
        theme,
        slider.hovered,
        false,
        theme.control_color,
    );

    if let Some(image) = slider.thumb_image {
        backend.draw_image(image, thumb_rect);
    }

    if let Some(image) = slider.track_image {
        if image.draw_over_content {
            backend.draw_image(image, body);
        }
    }
}

fn draw_vertical_slider<B: DrawBackend>(backend: &mut B, slider: &VerticalSlider, theme: &Theme) {
    let body = Rect::new(slider.position, slider.size);
    if body.size.x <= 0.0 || body.size.y <= 0.0 {
        return;
    }

    match slider.track_image {
        Some(image) if !image.draw_over_content => backend.draw_image(image, body),
        _ => backend.fill_rect(body, theme.track_color),
    }

    let range = (slider.maximum - slider.minimum).abs();
    let value_fraction = if range <= f32::EPSILON {
        0.0
    } else {
        ((slider.value - slider.minimum) / (slider.maximum - slider.minimum)).clamp(0.0, 1.0)
    };

    let thumb_y = body.position.y + value_fraction * body.size.y;
    let thumb_height = slider.thumb_height.max(1.0);
    let thumb_rect = Rect::new(
        Vec2::new(body.position.x, thumb_y - thumb_height / 2.0),
        Vec2::new(body.size.x, thumb_height),
    );

    draw_beveled_box(
        backend,
        thumb_rect,
        theme,
        slider.hovered,
        false,
        theme.control_color,
    );

    if let Some(image) = slider.thumb_image {
        backend.draw_image(image, thumb_rect);
    }

    if let Some(image) = slider.track_image {
        if image.draw_over_content {
            backend.draw_image(image, body);
        }
    }
}

fn draw_draggable<B: DrawBackend>(backend: &mut B, draggable: &Draggable, theme: &Theme) {
    let rect = Rect::new(draggable.position, draggable.size);
    if rect.size.x <= 0.0 || rect.size.y <= 0.0 {
        return;
    }

    draw_beveled_box(
        backend,
        rect,
        theme,
        draggable.hovered,
        false,
        theme.control_color,
    );

    if let Some(image) = draggable.background_image {
        if !image.draw_over_content {
            backend.draw_image(image, rect);
        }
    }

    let line_start_x = rect.position.x + rect.size.x * 0.3;
    let line_end_x = rect.position.x + rect.size.x * 0.7;
    let upper_line_y = rect.position.y + rect.size.y * 0.3;
    let lower_line_y = rect.position.y + rect.size.y * 0.7;
    let line_color = theme.control_color.scaled(0.3);

    backend.draw_line(
        Vec2::new(line_start_x, upper_line_y),
        Vec2::new(line_end_x, upper_line_y),
        line_color,
        2.0,
    );
    backend.draw_line(
        Vec2::new(line_start_x, lower_line_y),
        Vec2::new(line_end_x, lower_line_y),
        line_color,
        2.0,
    );

    if let Some(label) = &draggable.label {
        let text_pos = rect.position + Vec2::new(6.0, 4.0);
        backend.draw_text(label, text_pos, theme.font_size_px, theme.text_color);
    }

    if let Some(image) = draggable.background_image {
        if image.draw_over_content {
            backend.draw_image(image, rect);
        }
    }
}

fn draw_left_right_selector<B: DrawBackend>(
    backend: &mut B,
    selector: &LeftRightSelector,
    theme: &Theme,
) {
    let rect = Rect::new(selector.position, selector.size);
    let center_position = rect.position + Vec2::new(selector.button_width, 0.0);
    let center_size = Vec2::new(rect.size.x - selector.button_width * 2.0, rect.size.y);
    if center_size.x > 0.0 {
        backend.fill_rect(Rect::new(center_position, center_size), theme.track_color);
    }

    draw_button(backend, &selector.left_button, theme);
    draw_button(backend, &selector.right_button, theme);

    if let Some(selected) = selector.selected_option() {
        let text_pos = center_position + Vec2::new(6.0, 4.0);
        backend.draw_text(selected, text_pos, theme.font_size_px, theme.text_color);
    }
}

fn draw_button_toggle<B: DrawBackend>(backend: &mut B, toggle: &ButtonToggle, theme: &Theme) {
    draw_button_visual(
        backend,
        theme,
        toggle.left_button.position,
        toggle.left_button.size,
        toggle.left_button.label.as_deref(),
        toggle.left_button.hovered,
        toggle.toggled_left,
        toggle.left_button.background_image,
    );
    draw_button_visual(
        backend,
        theme,
        toggle.right_button.position,
        toggle.right_button.size,
        toggle.right_button.label.as_deref(),
        toggle.right_button.hovered,
        !toggle.toggled_left,
        toggle.right_button.background_image,
    );
}

fn draw_move_and_resize_thumbs<B: DrawBackend>(
    backend: &mut B,
    thumbs: &MoveAndResizeThumbs,
    theme: &Theme,
) {
    draw_draggable(backend, &thumbs.move_thumb, theme);
    draw_draggable(backend, &thumbs.resize_thumb, theme);
}

fn draw_beveled_box<B: DrawBackend>(
    backend: &mut B,
    rect: Rect,
    theme: &Theme,
    hovered: bool,
    pressed: bool,
    color: Color,
) {
    let offset = Vec2::splat(theme.bevel_size_px);
    let inner_size = Vec2::new(
        (rect.size.x - offset.x).max(1.0),
        (rect.size.y - offset.y).max(1.0),
    );

    if !pressed {
        backend.fill_rect(
            Rect::new(rect.position, rect.size + offset),
            theme.shadow_color,
        );
        backend.fill_rect(Rect::new(rect.position, rect.size), theme.highlight_color);

        let mut fill = color;
        if hovered {
            fill = fill.scaled(theme.hover_shade);
        }
        backend.fill_rect(Rect::new(rect.position + offset, inner_size), fill);
    } else {
        backend.fill_rect(
            Rect::new(rect.position, rect.size + offset),
            theme.highlight_color,
        );
        backend.fill_rect(rect, theme.shadow_color);
        backend.fill_rect(
            Rect::new(rect.position + offset, inner_size),
            color.scaled(theme.pressed_shade),
        );
    }
}
