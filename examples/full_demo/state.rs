use glam::{UVec2, Vec2};
use rshigg::{
    Button, ButtonToggle, Draggable, Event, Gui, Label, LeftRightSelector, MoveAndResizeThumbs,
    Slider, TaggedEvent, VerticalSlider,
};

use crate::raylib_skin::{
    IMG_GOLD_ARROW, IMG_OPTION_BUTTON, IMG_ROW_STRIP, IMG_SLIDER_KNOB, IMG_SLIDER_TRACK,
};

pub const WINDOW_DIMS: UVec2 = UVec2::new(1280, 720);
pub const DIMS: UVec2 = WINDOW_DIMS;

fn px(x: f32, y: f32) -> Vec2 {
    Vec2::new(x * DIMS.x as f32, y * DIMS.y as f32)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Tag {
    OpenSettings,
    CloseSettings,
    MoveWindow,
    ScrollMenu,
    MoveAndResizePreview,
    RowButton(usize),
    RowSlider(usize),
    RowSelector(usize),
    RowToggle(usize),
}

#[derive(Clone, Copy)]
pub enum RowControl {
    Button(u32),
    Slider(u32),
    LeftRightSelector(u32),
    ButtonToggle(u32),
}

pub struct Row {
    pub label_id: u32,
    pub control: RowControl,
}

pub struct DemoState {
    pub running: bool,
    pub main_gui: Gui<Tag>,
    pub settings_gui: Gui<Tag>,
    pub settings_open: bool,
    pub settings_pos: Vec2,
    pub settings_size: Vec2,
    pub rows: Vec<Row>,
    pub open_settings_button_id: u32,
    pub scroll_value: f32,
    pub scroll_slider_id: u32,
    pub move_window_id: u32,
    pub close_window_id: u32,
    pub preview_rect_pos: Vec2,
    pub preview_rect_size: Vec2,
}

impl DemoState {
    pub fn new() -> Self {
        let mut main_gui = Gui::new();
        let mut settings_gui = Gui::new();

        let open_button = Button::new(px(0.02, 0.02), px(0.14, 0.07), Some("Settings".to_string()));
        let mut open_button = open_button;
        open_button.set_background_image(rshigg::ImageStyle::stretched(IMG_OPTION_BUTTON));
        let open_settings_button_id = open_button.id;
        main_gui.add_button(open_button, Tag::OpenSettings);

        let preview_pos = px(0.58, 0.18);
        let preview_size = px(0.34, 0.24);
        let preview_thumbs =
            MoveAndResizeThumbs::new(Vec2::new(18.0, 18.0), preview_pos, preview_size);
        main_gui.add_move_and_resize_thumbs(preview_thumbs, Tag::MoveAndResizePreview);

        let settings_pos = px(0.08, 0.08);
        let settings_size = px(0.55, 0.82);

        let move_window = Draggable::new(
            settings_pos,
            Vec2::new(settings_size.x * 0.88, settings_size.y * 0.08),
            Some("Settings".to_string()),
        );
        let mut move_window = move_window;
        move_window.set_background_image(rshigg::ImageStyle::stretched(IMG_ROW_STRIP));
        let move_window_id = move_window.id;
        settings_gui.add_draggable(move_window, Tag::MoveWindow);

        let close_window = Button::new(
            Vec2::new(settings_pos.x + settings_size.x * 0.88, settings_pos.y),
            Vec2::new(settings_size.x * 0.12, settings_size.y * 0.08),
            Some("X".to_string()),
        );
        let mut close_window = close_window;
        close_window.set_background_image(rshigg::ImageStyle::stretched(IMG_OPTION_BUTTON));
        let close_window_id = close_window.id;
        settings_gui.add_button(close_window, Tag::CloseSettings);

        let scroll_slider = VerticalSlider::new(
            Vec2::ZERO,
            Vec2::ZERO,
            24.0,
            0.0,
            1.0,
            0.05,
            0.0,
            0.0,
            Some("Scroll".to_string()),
        );
        let mut scroll_slider = scroll_slider;
        scroll_slider.set_track_image(rshigg::ImageStyle::tiled(IMG_SLIDER_TRACK));
        scroll_slider.set_thumb_image(rshigg::ImageStyle::centered(IMG_SLIDER_KNOB));
        let scroll_slider_id = scroll_slider.id;
        settings_gui.add_vertical_slider(scroll_slider, Tag::ScrollMenu);

        let mut rows = Vec::new();
        let row_count = 36;
        for i in 0..row_count {
            let label = Label::new(Vec2::ZERO, Vec2::ZERO, Some(format!("Option {}", i + 1)));
            let mut label = label;
            label.set_background_image(rshigg::ImageStyle::stretched(IMG_ROW_STRIP));
            let label_id = label.id;
            settings_gui.add_label(label);

            let control = match i % 4 {
                0 => {
                    let mut button = Button::new(Vec2::ZERO, Vec2::ZERO, Some("Apply".to_string()));
                    button.set_background_image(rshigg::ImageStyle::stretched(IMG_OPTION_BUTTON));
                    let id = button.id;
                    settings_gui.add_button(button, Tag::RowButton(i));
                    RowControl::Button(id)
                }
                1 => {
                    let mut slider = Slider::new(
                        Vec2::ZERO,
                        Vec2::ZERO,
                        16.0,
                        0.0,
                        100.0,
                        1.0,
                        50.0,
                        0.0,
                        Some("Slider".to_string()),
                    );
                    slider.set_track_image(rshigg::ImageStyle::tiled(IMG_SLIDER_TRACK));
                    slider.set_thumb_image(rshigg::ImageStyle::centered(IMG_SLIDER_KNOB));
                    let id = slider.id;
                    settings_gui.add_slider(slider, Tag::RowSlider(i));
                    RowControl::Slider(id)
                }
                2 => {
                    let mut selector = LeftRightSelector::new(
                        Vec2::ZERO,
                        Vec2::ZERO,
                        32.0,
                        vec![
                            "Low".to_string(),
                            "Medium".to_string(),
                            "High".to_string(),
                            "Ultra".to_string(),
                        ],
                        1,
                    );
                    selector
                        .left_button
                        .set_background_image(rshigg::ImageStyle::stretched(IMG_OPTION_BUTTON));
                    selector
                        .right_button
                        .set_background_image(rshigg::ImageStyle::stretched(IMG_OPTION_BUTTON));
                    let id = selector.id;
                    settings_gui.add_left_right_selector(selector, Tag::RowSelector(i));
                    RowControl::LeftRightSelector(id)
                }
                _ => {
                    let mut toggle = ButtonToggle::new(
                        Vec2::ZERO,
                        Vec2::ZERO,
                        "Off".to_string(),
                        "On".to_string(),
                        i % 2 == 0,
                    );
                    toggle
                        .left_button
                        .set_background_image(rshigg::ImageStyle::stretched(IMG_OPTION_BUTTON));
                    let mut right_image = rshigg::ImageStyle::centered(IMG_GOLD_ARROW);
                    right_image.draw_over_content = true;
                    toggle.right_button.set_background_image(right_image);
                    let id = toggle.id;
                    settings_gui.add_button_toggle(toggle, Tag::RowToggle(i));
                    RowControl::ButtonToggle(id)
                }
            };

            rows.push(Row { label_id, control });
        }

        let mut state = Self {
            running: true,
            main_gui,
            settings_gui,
            settings_open: true,
            settings_pos,
            settings_size,
            rows,
            open_settings_button_id,
            scroll_value: 0.0,
            scroll_slider_id,
            move_window_id,
            close_window_id,
            preview_rect_pos: preview_pos,
            preview_rect_size: preview_size,
        };
        sync_main_visibility(&mut state);
        layout_settings(&mut state);
        state
    }
}

pub fn handle_main_events(state: &mut DemoState, events: Vec<TaggedEvent<Tag>>) {
    for tagged in events {
        match (tagged.tag, tagged.event) {
            (Tag::OpenSettings, Event::ButtonReleased) => {
                state.settings_open = true;
                sync_main_visibility(state);
            }
            (
                Tag::MoveAndResizePreview,
                Event::MoveAndResizeThumbsChanged {
                    target_position,
                    target_size,
                },
            ) => {
                state.preview_rect_pos = target_position;
                state.preview_rect_size = target_size;
            }
            _ => {}
        }
    }
}

pub fn handle_settings_events(state: &mut DemoState, events: Vec<TaggedEvent<Tag>>) {
    for tagged in events {
        match (tagged.tag, tagged.event) {
            (Tag::CloseSettings, Event::ButtonReleased) => {
                state.settings_open = false;
                sync_main_visibility(state);
            }
            (Tag::MoveWindow, Event::DraggableMoved { new_pos }) => {
                state.settings_pos = new_pos;
                layout_settings(state);
            }
            (Tag::ScrollMenu, Event::SliderMoved { value }) => {
                state.scroll_value = value;
                layout_settings(state);
            }
            (Tag::RowButton(idx), Event::ButtonReleased) => println!("row button {idx} released"),
            (Tag::RowSlider(idx), Event::SliderMoved { value }) => {
                println!("row slider {idx} => {value}")
            }
            (
                Tag::RowSelector(idx),
                Event::SelectionChanged {
                    selected_option_index,
                    ..
                },
            ) => {
                if let Some(selector) = state
                    .settings_gui
                    .get_left_right_selector(tagged.element_id)
                {
                    println!(
                        "row selector {idx} => {} ({selected_option_index})",
                        selector.selected_option().unwrap_or("n/a")
                    );
                }
            }
            (Tag::RowToggle(idx), Event::ButtonToggleChanged { toggled_left }) => {
                println!(
                    "row toggle {idx} => {}",
                    if toggled_left { "Off" } else { "On" }
                );
            }
            _ => {}
        }
    }
}

pub fn layout_settings(state: &mut DemoState) {
    let pos = state.settings_pos;
    let size = state.settings_size;
    let top_h = size.y * 0.08;
    let padding = size.x * 0.02;
    let row_h = size.y * 0.06;
    let row_gap = row_h * 0.2;
    let row_stride = row_h + row_gap;
    let label_w = size.x * 0.36;
    let control_w = size.x * 0.44;
    let scroll_w = size.x * 0.06;

    if let Some(move_handle) = state.settings_gui.get_draggable_mut(state.move_window_id) {
        move_handle.position = pos;
        move_handle.size = Vec2::new(size.x - scroll_w - padding, top_h);
    }
    if let Some(close) = state.settings_gui.get_button_mut(state.close_window_id) {
        close.position = Vec2::new(pos.x + size.x - scroll_w, pos.y);
        close.size = Vec2::new(scroll_w, top_h);
    }

    let viewport_top = pos.y + top_h + padding;
    let viewport_bottom = pos.y + size.y - padding;
    let viewport_h = (viewport_bottom - viewport_top).max(0.01);
    let content_h = state.rows.len() as f32 * row_stride;
    let max_scroll = (content_h - viewport_h).max(0.0);

    state.scroll_value = state.scroll_value.clamp(0.0, max_scroll);
    if let Some(scroll) = state
        .settings_gui
        .get_vertical_slider_mut(state.scroll_slider_id)
    {
        scroll.position = Vec2::new(pos.x + size.x - scroll_w, viewport_top);
        scroll.size = Vec2::new(scroll_w, viewport_h);
        scroll.minimum = 0.0;
        scroll.maximum = max_scroll;
        scroll.step_size = row_stride.max(0.001);
        let visible_fraction = if content_h <= 0.0 {
            1.0
        } else {
            (viewport_h / content_h).clamp(0.05, 1.0)
        };
        scroll.thumb_height = (viewport_h * visible_fraction).clamp(18.0, viewport_h);
        scroll.value = state.scroll_value;
    }

    for (i, row) in state.rows.iter().enumerate() {
        let y = viewport_top + i as f32 * row_stride - state.scroll_value;
        let visible = y + row_h >= viewport_top && y <= viewport_bottom;
        state.settings_gui.set_visible(row.label_id, visible);
        if visible {
            if let Some(label) = state.settings_gui.get_label_mut(row.label_id) {
                label.position = Vec2::new(pos.x + padding, y);
                label.size = Vec2::new(label_w, row_h);
            }
        }

        let control_pos = Vec2::new(pos.x + padding + label_w + padding, y);
        match row.control {
            RowControl::Button(id) => {
                state.settings_gui.set_visible(id, visible);
                if let Some(control) = state.settings_gui.get_button_mut(id) {
                    if visible {
                        control.position = control_pos;
                        control.size = Vec2::new(control_w, row_h);
                    }
                }
            }
            RowControl::Slider(id) => {
                state.settings_gui.set_visible(id, visible);
                if let Some(control) = state.settings_gui.get_slider_mut(id) {
                    if visible {
                        control.position = control_pos;
                        control.size = Vec2::new(control_w, row_h);
                    }
                }
            }
            RowControl::LeftRightSelector(id) => {
                state.settings_gui.set_visible(id, visible);
                if let Some(control) = state.settings_gui.get_left_right_selector_mut(id) {
                    if visible {
                        control.set_position(control_pos);
                        control.set_size(Vec2::new(control_w, row_h));
                    }
                }
            }
            RowControl::ButtonToggle(id) => {
                state.settings_gui.set_visible(id, visible);
                if let Some(control) = state.settings_gui.get_button_toggle_mut(id) {
                    if visible {
                        control.set_position(control_pos);
                        control.set_size(Vec2::new(control_w, row_h));
                    }
                }
            }
        }
    }
}

pub fn settings_scroll_clip_rect(state: &DemoState) -> rshigg::Rect {
    let pos = state.settings_pos;
    let size = state.settings_size;
    let clip_pos = pos;
    let clip_size = size;
    rshigg::Rect::new(clip_pos, clip_size)
}

fn sync_main_visibility(state: &mut DemoState) {
    state
        .main_gui
        .set_visible(state.open_settings_button_id, !state.settings_open);
}
