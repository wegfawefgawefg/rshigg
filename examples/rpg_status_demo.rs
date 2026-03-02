use glam::{IVec2, UVec2, Vec2};
use raylib::prelude::{Color as RayColor, *};
use rshigg::{
    Button, ButtonToggle, Color as UiColor, Event, Gui, ImageStyle, Label, LeftRightSelector,
    Slider, TaggedEvent, Theme,
};

#[path = "shared/raylib_skin.rs"]
mod raylib_skin;

use raylib_skin::{
    SkinRaylibBackend, SkinTextures, IMG_AURORA_TILE, IMG_ICON_MEAT, IMG_ICON_MOUSE, IMG_PORTRAIT,
    IMG_SLIDER_KNOB, IMG_SLIDER_TRACK, IMG_SOFT_NOISE,
};

const WINDOW_DIMS: UVec2 = UVec2::new(1280, 720);
const DIMS: UVec2 = WINDOW_DIMS;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tag {
    Ability,
    Overdrive,
    AutoBattle,
    DisplayAbilities,
    Back,
}

struct State {
    running: bool,
    gui: Gui<Tag>,
    overdrive_slider_id: u32,
}

impl State {
    fn new() -> Self {
        let mut gui = Gui::new();

        // Main panel background.
        let mut panel_bg = Label::new(Vec2::new(0.13, 0.14), Vec2::new(0.74, 0.72), None);
        let mut panel_img = ImageStyle::tiled(IMG_AURORA_TILE);
        panel_img.tint = UiColor::rgba(98, 120, 172, 82);
        panel_bg.set_background_image(panel_img);
        gui.add_label(panel_bg);

        // Soft frame overlay for subtle texture.
        let mut frame_overlay = Label::new(Vec2::new(0.14, 0.15), Vec2::new(0.72, 0.70), None);
        let mut frame_img = ImageStyle::tiled(IMG_SOFT_NOISE);
        frame_img.tint = UiColor::rgba(212, 224, 255, 46);
        frame_img.draw_over_content = true;
        frame_overlay.set_background_image(frame_img);
        gui.add_label(frame_overlay);

        let help = Label::new(
            Vec2::new(0.16, 0.18),
            Vec2::new(0.52, 0.05),
            Some("Choose an ability set and adjust overdrive.".to_string()),
        );
        gui.add_label(help);

        let mut portrait = Label::new(Vec2::new(0.16, 0.30), Vec2::new(0.11, 0.15), None);
        portrait.set_background_image(ImageStyle::stretched(IMG_PORTRAIT));
        gui.add_label(portrait);

        let mut ability_selector = LeftRightSelector::new(
            Vec2::new(0.30, 0.30),
            Vec2::new(0.50, 0.07),
            0.08,
            vec![
                "Special Attack".to_string(),
                "Meteor Strike".to_string(),
                "Hellfire".to_string(),
            ],
            0,
        );
        style_selector_buttons(&mut ability_selector);
        gui.add_left_right_selector(ability_selector, Tag::Ability);

        let mut overdrive_label = Label::new(
            Vec2::new(0.30, 0.385),
            Vec2::new(0.24, 0.045),
            Some("Overdrive".to_string()),
        );
        let mut overdrive_lbl_bg = ImageStyle::tiled(IMG_AURORA_TILE);
        overdrive_lbl_bg.tint = UiColor::rgba(182, 206, 255, 42);
        overdrive_label.set_background_image(overdrive_lbl_bg);
        gui.add_label(overdrive_label);

        let mut overdrive_slider = Slider::new(
            Vec2::new(0.30, 0.43),
            Vec2::new(0.50, 0.05),
            0.04,
            0.0,
            100.0,
            1.0,
            62.0,
            0.0,
            Some("62%".to_string()),
        );
        let mut track_img = ImageStyle::tiled(IMG_SLIDER_TRACK);
        track_img.tint = UiColor::rgba(124, 152, 214, 160);
        overdrive_slider.set_track_image(track_img);
        let mut knob_img = ImageStyle::centered(IMG_SLIDER_KNOB);
        knob_img.tint = UiColor::rgba(255, 220, 120, 220);
        overdrive_slider.set_thumb_image(knob_img);
        let overdrive_slider_id = overdrive_slider.id;
        gui.add_slider(overdrive_slider, Tag::Overdrive);

        let mut auto_toggle = ButtonToggle::new(
            Vec2::new(0.30, 0.50),
            Vec2::new(0.30, 0.06),
            "Manual".to_string(),
            "Auto".to_string(),
            true,
        );
        style_button_with_texture(&mut auto_toggle.left_button);
        style_button_with_texture(&mut auto_toggle.right_button);
        gui.add_button_toggle(auto_toggle, Tag::AutoBattle);

        let stats_left = ["Strength", "Defense", "Magic", "Magic Defense"];
        let stats_right = ["Agility", "Luck", "Evasion", "Accuracy"];
        let values_left = [21, 36, 26, 23];
        let values_right = [17, 17, 10, 12];
        for i in 0..stats_left.len() {
            add_stat_row(
                &mut gui,
                0.30,
                0.59 + i as f32 * 0.055,
                stats_left[i],
                values_left[i],
                IMG_ICON_MEAT,
            );
            add_stat_row(
                &mut gui,
                0.55,
                0.59 + i as f32 * 0.055,
                stats_right[i],
                values_right[i],
                IMG_ICON_MOUSE,
            );
        }

        let mut display_button = Button::new(
            Vec2::new(0.46, 0.82),
            Vec2::new(0.24, 0.065),
            Some("Display Abilities".to_string()),
        );
        style_button_with_texture(&mut display_button);
        gui.add_button(display_button, Tag::DisplayAbilities);

        let mut back_button = Button::new(
            Vec2::new(0.72, 0.82),
            Vec2::new(0.11, 0.065),
            Some("Back".to_string()),
        );
        style_button_with_texture(&mut back_button);
        gui.add_button(back_button, Tag::Back);

        Self {
            running: true,
            gui,
            overdrive_slider_id,
        }
    }
}

fn style_button_with_texture(button: &mut Button) {
    let mut bg = ImageStyle::tiled(IMG_AURORA_TILE);
    bg.tint = UiColor::rgba(112, 132, 196, 160);
    button.set_background_image(bg);
}

fn style_selector_buttons(selector: &mut LeftRightSelector) {
    style_button_with_texture(&mut selector.left_button);
    style_button_with_texture(&mut selector.right_button);
}

fn add_stat_row(gui: &mut Gui<Tag>, x: f32, y: f32, name: &str, value: i32, icon_id: u64) {
    let mut icon = Label::new(Vec2::new(x, y), Vec2::new(0.028, 0.042), None);
    let mut icon_img = ImageStyle::centered(icon_id);
    icon_img.tint = UiColor::rgba(220, 230, 255, 180);
    icon.set_background_image(icon_img);
    gui.add_label(icon);

    let mut name_label = Label::new(
        Vec2::new(x + 0.03, y),
        Vec2::new(0.16, 0.042),
        Some(name.to_string()),
    );
    let mut name_bg = ImageStyle::tiled(IMG_AURORA_TILE);
    name_bg.tint = UiColor::rgba(142, 162, 220, 78);
    name_label.set_background_image(name_bg);
    gui.add_label(name_label);

    let mut value_label = Label::new(
        Vec2::new(x + 0.19, y),
        Vec2::new(0.05, 0.042),
        Some(format!("{}", value)),
    );
    let mut value_bg = ImageStyle::tiled(IMG_AURORA_TILE);
    value_bg.tint = UiColor::rgba(100, 120, 182, 130);
    value_label.set_background_image(value_bg);
    gui.add_label(value_label);
}

fn main() {
    let mut state = State::new();
    let (mut rl, rlt) = raylib::init().title("RShiGG RPG Status Demo").build();

    rl.set_window_size(WINDOW_DIMS.x as i32, WINDOW_DIMS.y as i32);
    center_window(&mut rl);
    rl.set_target_fps(144);

    let mut render_texture = rl
        .load_render_texture(&rlt, DIMS.x, DIMS.y)
        .unwrap_or_else(|e| {
            println!("Error creating render texture: {}", e);
            std::process::exit(1);
        });
    let skins = SkinTextures::load(&mut rl, &rlt);

    while state.running && !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            state.running = false;
        }

        let mouse = rl.get_mouse_position();
        let mouse_pressed = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);
        let events =
            state
                .gui
                .step_pixels(Vec2::new(mouse.x, mouse.y), DIMS.as_vec2(), mouse_pressed);
        handle_events(&mut state, events);

        let mut d = rl.begin_drawing(&rlt);
        {
            let low_res_d = &mut d.begin_texture_mode(&rlt, &mut render_texture);
            low_res_d.clear_background(RayColor::new(14, 21, 44, 255));

            let theme = Theme {
                control_color: UiColor::rgba(82, 98, 148, 190),
                text_color: UiColor::rgb(236, 244, 255),
                shadow_color: UiColor::rgb(10, 14, 24),
                highlight_color: UiColor::rgb(166, 184, 232),
                track_color: UiColor::rgb(72, 84, 132),
                hover_shade: 0.92,
                pressed_shade: 0.78,
                bevel_size_px: 1.0,
                font_size_px: 24.0,
            };
            let mut backend = SkinRaylibBackend::new(low_res_d, &skins);
            rshigg::draw_gui(&state.gui, &mut backend, DIMS.as_vec2(), &theme);
        }
        scale_and_blit_render_texture_to_window(&mut d, &mut render_texture);
    }
}

fn handle_events(state: &mut State, events: Vec<TaggedEvent<Tag>>) {
    for tagged in events {
        match (tagged.tag, tagged.event) {
            (Tag::Ability, Event::SelectionChanged { .. }) => {
                if let Some(selector) = state.gui.get_left_right_selector(tagged.element_id) {
                    println!("Ability => {}", selector.selected_option().unwrap_or("n/a"));
                }
            }
            (Tag::Overdrive, Event::SliderMoved { value }) => {
                if let Some(slider) = state.gui.get_slider_mut(state.overdrive_slider_id) {
                    slider.label = Some(format!("{:.0}%", value));
                }
            }
            (Tag::AutoBattle, Event::ButtonToggleChanged { toggled_left }) => {
                println!("Mode => {}", if toggled_left { "Manual" } else { "Auto" });
            }
            (Tag::DisplayAbilities, Event::ButtonReleased) => println!("Display abilities"),
            (Tag::Back, Event::ButtonReleased) => println!("Back"),
            _ => {}
        }
    }
}

fn center_window(rl: &mut RaylibHandle) {
    let screen_dims = IVec2::new(rl.get_screen_width(), rl.get_screen_height());
    let screen_center = screen_dims / 2;
    let window_center = WINDOW_DIMS.as_ivec2() / 2;
    let offset = IVec2::new(
        screen_center.x - window_center.x,
        screen_center.y - window_center.y,
    );
    rl.set_window_position(offset.x, offset.y);
}

fn scale_and_blit_render_texture_to_window(
    draw_handle: &mut RaylibDrawHandle,
    render_texture: &mut RenderTexture2D,
) {
    let source_rec = Rectangle::new(
        0.0,
        0.0,
        render_texture.texture.width as f32,
        -render_texture.texture.height as f32,
    );
    let dest_rec = Rectangle::new(0.0, 0.0, WINDOW_DIMS.x as f32, WINDOW_DIMS.y as f32);
    draw_handle.draw_texture_pro(
        render_texture,
        source_rec,
        dest_rec,
        Vector2::new(0.0, 0.0),
        0.0,
        RayColor::WHITE,
    );
}
