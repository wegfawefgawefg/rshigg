use glam::Vec2;
use rshigg::{
    transform_mouse_to_normalized_subsurface_coords, Button, ButtonToggle, Color, DrawBackend, Gui,
    LeftRightSelector, MoveAndResizeThumbs, Rect, Slider, Theme, VerticalSlider,
};

#[derive(Clone, Copy, Debug)]
enum Tag {
    ToggleMute,
    SetVolume,
    SetDepth,
    SetQuality,
    SetVsync,
    ResizePanel,
}

fn main() {
    let mut gui = Gui::new();
    gui.add_button(
        Button::new(
            Vec2::new(0.05, 0.05),
            Vec2::new(0.25, 0.15),
            Some("Mute".to_string()),
        ),
        Tag::ToggleMute,
    );
    gui.add_slider(
        Slider::new(
            Vec2::new(0.05, 0.3),
            Vec2::new(0.9, 0.12),
            0.04,
            0.0,
            100.0,
            1.0,
            50.0,
            0.05,
            Some("Volume".to_string()),
        ),
        Tag::SetVolume,
    );
    gui.add_vertical_slider(
        VerticalSlider::new(
            Vec2::new(0.95, 0.05),
            Vec2::new(0.03, 0.5),
            0.05,
            0.0,
            1.0,
            0.1,
            0.4,
            0.0,
            Some("Depth".to_string()),
        ),
        Tag::SetDepth,
    );
    gui.add_left_right_selector(
        LeftRightSelector::new(
            Vec2::new(0.05, 0.48),
            Vec2::new(0.6, 0.12),
            0.08,
            vec![
                "Low".to_string(),
                "Medium".to_string(),
                "High".to_string(),
                "Ultra".to_string(),
            ],
            1,
        ),
        Tag::SetQuality,
    );
    gui.add_button_toggle(
        ButtonToggle::new(
            Vec2::new(0.05, 0.64),
            Vec2::new(0.4, 0.12),
            "VSync Off".to_string(),
            "VSync On".to_string(),
            false,
        ),
        Tag::SetVsync,
    );
    gui.add_move_and_resize_thumbs(
        MoveAndResizeThumbs::new(
            Vec2::new(0.03, 0.03),
            Vec2::new(0.55, 0.58),
            Vec2::new(0.2, 0.2),
        ),
        Tag::ResizePanel,
    );

    let window_size = Vec2::new(1280.0, 720.0);
    let render_size = Vec2::new(640.0, 360.0);
    let normalized_mouse = Vec2::new(0.4, 0.35);
    let transformed_mouse = transform_mouse_to_normalized_subsurface_coords(
        normalized_mouse,
        window_size,
        Vec2::ZERO,
        render_size,
    );

    let pressed_events = gui.step(transformed_mouse, true);
    let released_events = gui.step(transformed_mouse, false);

    println!("pressed events: {:?}", pressed_events);
    println!("released events: {:?}", released_events);

    let mut backend = CommandBufferBackend::default();
    rshigg::draw_gui(&gui, &mut backend, render_size, &Theme::default());
    backend.dump();
}

#[derive(Default)]
struct CommandBufferBackend {
    commands: Vec<String>,
}

impl CommandBufferBackend {
    fn dump(&self) {
        println!("\n--- Render Commands (first 24) ---");
        for command in self.commands.iter().take(24) {
            println!("{command}");
        }
        println!("total commands: {}", self.commands.len());
    }
}

impl DrawBackend for CommandBufferBackend {
    fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.commands.push(format!(
            "fill_rect pos=({:.1},{:.1}) size=({:.1},{:.1}) color=({}, {}, {}, {})",
            rect.position.x,
            rect.position.y,
            rect.size.x,
            rect.size.y,
            color.r,
            color.g,
            color.b,
            color.a
        ));
    }

    fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color, thickness: f32) {
        self.commands.push(format!(
            "draw_line ({:.1},{:.1}) -> ({:.1},{:.1}) thickness={:.1} color=({}, {}, {}, {})",
            start.x, start.y, end.x, end.y, thickness, color.r, color.g, color.b, color.a
        ));
    }

    fn draw_text(&mut self, text: &str, position: Vec2, font_size: f32, color: Color) {
        self.commands.push(format!(
            "draw_text '{text}' at ({:.1},{:.1}) size={:.1} color=({}, {}, {}, {})",
            position.x, position.y, font_size, color.r, color.g, color.b, color.a
        ));
    }
}
