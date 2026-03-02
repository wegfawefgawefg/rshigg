use raylib::prelude::*;
use rshigg::Event;

#[path = "full_demo/draw.rs"]
mod draw;
#[path = "full_demo/state.rs"]
mod state;

use draw::{center_window, draw_scene, scale_and_blit_render_texture_to_window};
use state::{
    handle_main_events, handle_settings_events, layout_settings, DemoState, DIMS, WINDOW_DIMS,
};

fn main() {
    let mut state = DemoState::new();
    let (mut rl, rlt) = raylib::init().title("RShiGG Full Demo").build();

    rl.set_window_size(WINDOW_DIMS.x as i32, WINDOW_DIMS.y as i32);
    center_window(&mut rl);
    rl.set_target_fps(144);

    let mut render_texture = rl
        .load_render_texture(&rlt, DIMS.x, DIMS.y)
        .unwrap_or_else(|e| {
            println!("Error creating render texture: {}", e);
            std::process::exit(1);
        });

    while state.running && !rl.window_should_close() {
        process_input(&mut rl, &mut state);
        step(&mut rl, &mut state);

        let mut d = rl.begin_drawing(&rlt);
        {
            let low_res_d = &mut d.begin_texture_mode(&rlt, &mut render_texture);
            low_res_d.clear_background(Color::new(45, 45, 52, 255));
            draw_scene(&state, low_res_d);
        }
        scale_and_blit_render_texture_to_window(&mut d, &mut render_texture);
    }
}

fn process_input(rl: &mut RaylibHandle, state: &mut DemoState) {
    if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }
}

fn step(rl: &mut RaylibHandle, state: &mut DemoState) {
    let mouse = rl.get_mouse_position();
    let nmp = glam::Vec2::new(mouse.x / DIMS.x as f32, mouse.y / DIMS.y as f32);
    let mouse_pressed = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);

    let main_events = state.main_gui.step(nmp, mouse_pressed);
    handle_main_events(state, main_events);

    if state.settings_open {
        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            if let Some(scroll) = state
                .settings_gui
                .get_vertical_slider_mut(state.scroll_slider_id)
            {
                let event = if wheel > 0.0 {
                    scroll.scroll_down_one_step()
                } else {
                    scroll.scroll_up_one_step()
                };
                if let Event::SliderMoved { value } = event {
                    state.scroll_value = value;
                    layout_settings(state);
                }
            }
        }

        let settings_events = state.settings_gui.step(nmp, mouse_pressed);
        handle_settings_events(state, settings_events);
    }
}
