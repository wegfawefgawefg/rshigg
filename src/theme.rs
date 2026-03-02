use glam::Vec2;

use crate::{
    Button, ButtonToggle, Color, Draggable, DrawBackend, Gui, Label, LeftRightSelector,
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

pub fn draw_gui<T: Clone + Copy, B: DrawBackend>(
    gui: &Gui<T>,
    backend: &mut B,
    resolution: Vec2,
    theme: &Theme,
) {
    for label in &gui.labels {
        draw_label(backend, label, resolution, theme);
    }
    for button in &gui.buttons {
        draw_button(backend, button, resolution, theme);
    }
    for slider in &gui.sliders {
        draw_slider(backend, slider, resolution, theme);
    }
    for slider in &gui.vertical_sliders {
        draw_vertical_slider(backend, slider, resolution, theme);
    }
    for draggable in &gui.draggables {
        draw_draggable(backend, draggable, resolution, theme);
    }
    for selector in &gui.left_right_selectors {
        draw_left_right_selector(backend, selector, resolution, theme);
    }
    for toggle in &gui.button_toggles {
        draw_button_toggle(backend, toggle, resolution, theme);
    }
    for thumbs in &gui.move_and_resize_thumbs {
        draw_move_and_resize_thumbs(backend, thumbs, resolution, theme);
    }
}

fn draw_label<B: DrawBackend>(backend: &mut B, label: &Label, resolution: Vec2, theme: &Theme) {
    let rect = Rect::from_normalized(label.position, label.size, resolution);
    if rect.size.x > 0.0 && rect.size.y > 0.0 {
        backend.fill_rect(rect, theme.control_color);
    }

    if let Some(text) = &label.text {
        let text_pos = rect.position + Vec2::new(6.0, 4.0);
        backend.draw_text(text, text_pos, theme.font_size_px, theme.text_color);
    }
}

fn draw_button<B: DrawBackend>(backend: &mut B, button: &Button, resolution: Vec2, theme: &Theme) {
    let rect = Rect::from_normalized(button.position, button.size, resolution);
    draw_beveled_box(
        backend,
        rect,
        theme,
        button.hovered,
        button.pressed,
        theme.control_color,
    );

    if let Some(label) = &button.label {
        let text_pos = rect.position + Vec2::new(6.0, 4.0);
        let text_offset = if button.pressed {
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
}

fn draw_slider<B: DrawBackend>(backend: &mut B, slider: &Slider, resolution: Vec2, theme: &Theme) {
    let body = Rect::from_normalized(slider.position, slider.size, resolution);
    backend.fill_rect(body, theme.track_color);

    let value_fraction = (slider.value - slider.minimum) / (slider.maximum - slider.minimum);
    let rel_position_x = value_fraction * slider.size.x;
    let thumb_x = body.position.x + rel_position_x * resolution.x;

    let thumb_width = resolution.x * slider.thumb_width;
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
}

fn draw_vertical_slider<B: DrawBackend>(
    backend: &mut B,
    slider: &VerticalSlider,
    resolution: Vec2,
    theme: &Theme,
) {
    let body = Rect::from_normalized(slider.position, slider.size, resolution);
    backend.fill_rect(body, theme.track_color);

    let value_fraction = (slider.value - slider.minimum) / (slider.maximum - slider.minimum);
    let rel_position_y = value_fraction * slider.size.y;
    let thumb_y = body.position.y + rel_position_y * resolution.y;

    let thumb_height = resolution.y * slider.thumb_height;
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
}

fn draw_draggable<B: DrawBackend>(
    backend: &mut B,
    draggable: &Draggable,
    resolution: Vec2,
    theme: &Theme,
) {
    let rect = Rect::from_normalized(draggable.position, draggable.size, resolution);
    draw_beveled_box(
        backend,
        rect,
        theme,
        draggable.hovered,
        false,
        theme.control_color,
    );

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

fn draw_left_right_selector<B: DrawBackend>(
    backend: &mut B,
    selector: &LeftRightSelector,
    resolution: Vec2,
    theme: &Theme,
) {
    let rect = Rect::from_normalized(selector.position, selector.size, resolution);
    let center_position = rect.position + Vec2::new(selector.button_width * resolution.x, 0.0);
    let center_size = Vec2::new(
        rect.size.x - selector.button_width * resolution.x * 2.0,
        rect.size.y,
    );
    if center_size.x > 0.0 {
        backend.fill_rect(Rect::new(center_position, center_size), theme.track_color);
    }

    draw_button(backend, &selector.left_button, resolution, theme);
    draw_button(backend, &selector.right_button, resolution, theme);

    if let Some(selected) = selector.selected_option() {
        let text_pos = center_position + Vec2::new(6.0, 4.0);
        backend.draw_text(selected, text_pos, theme.font_size_px, theme.text_color);
    }
}

fn draw_button_toggle<B: DrawBackend>(
    backend: &mut B,
    toggle: &ButtonToggle,
    resolution: Vec2,
    theme: &Theme,
) {
    let mut left = Button {
        id: toggle.left_button.id,
        position: toggle.left_button.position,
        size: toggle.left_button.size,
        label: toggle.left_button.label.clone(),
        hovered: toggle.left_button.hovered,
        pressed: toggle.toggled_left,
        was_pressed: toggle.left_button.was_pressed,
    };
    let mut right = Button {
        id: toggle.right_button.id,
        position: toggle.right_button.position,
        size: toggle.right_button.size,
        label: toggle.right_button.label.clone(),
        hovered: toggle.right_button.hovered,
        pressed: !toggle.toggled_left,
        was_pressed: toggle.right_button.was_pressed,
    };

    // Pressed style reflects toggle state, while hover still comes from input.
    left.pressed = toggle.toggled_left;
    right.pressed = !toggle.toggled_left;
    draw_button(backend, &left, resolution, theme);
    draw_button(backend, &right, resolution, theme);
}

fn draw_move_and_resize_thumbs<B: DrawBackend>(
    backend: &mut B,
    thumbs: &MoveAndResizeThumbs,
    resolution: Vec2,
    theme: &Theme,
) {
    draw_draggable(backend, &thumbs.move_thumb, resolution, theme);
    draw_draggable(backend, &thumbs.resize_thumb, resolution, theme);
}
