use glam::Vec2;
use rshigg::{Button, Draggable, Event, Gui, Label, TaggedEvent, VerticalSlider};

pub struct ElemRow {
    pub label: u32,
    pub elements: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SettingsWindowElementTag {
    MoveWindowDraggable,
    MinimizeWindowButton,
    CloseWindowButton,
    ScrollVerticalSlider,

    Potato,
}

pub struct SettingsWindow {
    pub gui: Gui<SettingsWindowElementTag>,

    pub pos: Vec2,
    pub size: Vec2,
    pub move_window_draggable: u32,
    pub minimize_window_button: u32,
    pub close_window_button: u32,
    pub scroll_vertical_slider: u32,

    pub rows: Vec<ElemRow>,
}

impl SettingsWindow {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        let mut gui = Gui::new();
        let mut rows = Vec::new();

        let move_window_draggable = Draggable::new(
            pos,
            Vec2::new(size.x * 0.8, size.y * 0.1),
            Some("Move Window".to_string()),
        );
        let move_window_draggable_id = move_window_draggable.id;
        gui.add_draggable(
            move_window_draggable,
            SettingsWindowElementTag::MoveWindowDraggable,
        );

        let minimize_window_button = Button::new(
            Vec2::new(pos.x + size.x * 0.8, pos.y),
            Vec2::new(size.x * 0.1, size.y * 0.1),
            Some("-".to_string()),
        );
        let minimize_window_button_id = minimize_window_button.id;
        gui.add_button(
            minimize_window_button,
            SettingsWindowElementTag::MinimizeWindowButton,
        );

        let close_window_button = Button::new(
            Vec2::new(pos.x + size.x * 0.9, pos.y),
            Vec2::new(size.x * 0.1, size.y * 0.1),
            Some("X".to_string()),
        );
        let close_window_button_id = close_window_button.id;
        gui.add_button(
            close_window_button,
            SettingsWindowElementTag::CloseWindowButton,
        );

        let scroll_vertical_slider = VerticalSlider::new(
            Vec2::new(pos.x + size.x * 0.9, pos.y + size.y * 0.1),
            Vec2::new(size.x * 0.1, size.y * 0.8),
            0.05 * size.y,
            0.0,
            100.0,
            1.0,
            50.0,
            0.05,
            Some("Scroll".to_string()),
        );
        let scroll_vertical_slider_id = scroll_vertical_slider.id;
        gui.add_vertical_slider(
            scroll_vertical_slider,
            SettingsWindowElementTag::ScrollVerticalSlider,
        );

        let nullpos = Vec2::new(0.0, 0.0);
        let nullsize = Vec2::new(0.0, 0.0);

        // option row: potato
        let label = Label::new(nullpos, nullsize, Some("Potato".to_string()));
        let button = Button::new(nullpos, nullsize, None);
        let potato_row = ElemRow {
            label: label.id,
            elements: vec![button.id],
        };
        rows.push(potato_row);
        gui.add_label(label);
        gui.add_button(button, SettingsWindowElementTag::Potato);

        let mut settings_window = SettingsWindow {
            gui,
            pos,
            size,
            move_window_draggable: move_window_draggable_id,
            minimize_window_button: minimize_window_button_id,
            close_window_button: close_window_button_id,
            scroll_vertical_slider: scroll_vertical_slider_id,

            rows,
        };
        settings_window.layout_from_state();
        settings_window
    }

    pub fn step(
        &mut self,
        mouse_position: Vec2,
        mouse_pressed: bool,
    ) -> Vec<TaggedEvent<SettingsWindowElementTag>> {
        let events = self.gui.step(mouse_position, mouse_pressed);
        for event in &events {
            if event.tag == SettingsWindowElementTag::MoveWindowDraggable {
                if let Event::DraggableMoved { new_pos } = event.event {
                    self.pos = new_pos;
                    self.layout_from_state();
                }
            }
        }
        events
    }

    fn layout_from_state(&mut self) {
        let top_bar_height = self.size.y * 0.1;
        let frame_button_width = self.size.x * 0.1;

        if let Some(move_handle) = self.gui.get_draggable_mut(self.move_window_draggable) {
            move_handle.position = self.pos;
            move_handle.size = Vec2::new(self.size.x * 0.8, top_bar_height);
        }
        if let Some(minimize) = self.gui.get_button_mut(self.minimize_window_button) {
            minimize.position = Vec2::new(self.pos.x + self.size.x * 0.8, self.pos.y);
            minimize.size = Vec2::new(frame_button_width, top_bar_height);
        }
        if let Some(close) = self.gui.get_button_mut(self.close_window_button) {
            close.position = Vec2::new(self.pos.x + self.size.x * 0.9, self.pos.y);
            close.size = Vec2::new(frame_button_width, top_bar_height);
        }
        if let Some(scroll) = self
            .gui
            .get_vertical_slider_mut(self.scroll_vertical_slider)
        {
            scroll.position =
                Vec2::new(self.pos.x + self.size.x * 0.9, self.pos.y + top_bar_height);
            scroll.size = Vec2::new(frame_button_width, self.size.y * 0.8);
            scroll.thumb_height = self.size.y * 0.05;
        }

        let row_height = self.size.y * 0.1;
        let label_width = self.size.x * 0.4;
        let element_width = self.size.x * 0.4;
        let gap = self.size.x * 0.02;
        let mut row_cursor = Vec2::new(self.pos.x, self.pos.y + top_bar_height + gap);

        for row in &self.rows {
            if let Some(label) = self.gui.get_label_mut(row.label) {
                label.position = row_cursor;
                label.size = Vec2::new(label_width, row_height);
            }

            let mut element_cursor = Vec2::new(row_cursor.x + label_width + gap, row_cursor.y);
            for element_id in &row.elements {
                if let Some(button) = self.gui.get_button_mut(*element_id) {
                    button.position = element_cursor;
                    button.size = Vec2::new(element_width, row_height);
                }
                if let Some(slider) = self.gui.get_slider_mut(*element_id) {
                    slider.position = element_cursor;
                    slider.size = Vec2::new(element_width, row_height);
                }
                if let Some(slider) = self.gui.get_vertical_slider_mut(*element_id) {
                    slider.position = element_cursor;
                    slider.size = Vec2::new(row_height, row_height * 2.0);
                }
                element_cursor.x += element_width + gap;
            }
            row_cursor.y += row_height * 1.1;
        }
    }
}
