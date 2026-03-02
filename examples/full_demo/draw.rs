use glam::IVec2;
use raylib::{
    drawing::RaylibDraw,
    math::Vector2,
    prelude::{Color as RayColor, *},
};
use rshigg::{DrawBackend, Rect, Theme};

use crate::raylib_skin::{SkinRaylibBackend, SkinTextures};
use crate::state::{settings_scroll_clip_rect, DemoState, DIMS, WINDOW_DIMS};

pub fn draw_scene(
    state: &DemoState,
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    skins: &SkinTextures,
) {
    d.draw_text("RShiGG Full Demo", 12, 10, 24, RayColor::WHITE);
    d.draw_text(
        "Drag green thumbs to move/resize preview panel",
        12,
        38,
        18,
        RayColor::new(200, 220, 200, 255),
    );

    let preview = Rect::from_normalized(
        state.preview_rect_pos,
        state.preview_rect_size,
        DIMS.as_vec2(),
    );
    d.draw_rectangle(
        preview.position.x as i32,
        preview.position.y as i32,
        preview.size.x as i32,
        preview.size.y as i32,
        RayColor::new(80, 80, 120, 255),
    );
    d.draw_rectangle_lines(
        preview.position.x as i32,
        preview.position.y as i32,
        preview.size.x as i32,
        preview.size.y as i32,
        RayColor::new(230, 230, 240, 255),
    );
    d.draw_text(
        "Preview Area",
        (preview.position.x + 12.0) as i32,
        (preview.position.y + 12.0) as i32,
        20,
        RayColor::new(230, 230, 240, 255),
    );

    let resolution = DIMS.as_vec2();
    let mut backend = SkinRaylibBackend::new(d, skins);
    let theme = Theme::default();
    rshigg::draw_gui(&state.main_gui, &mut backend, resolution, &theme);
    if state.settings_open {
        let scroll_clip = settings_scroll_clip_rect(state);
        backend.push_clip_rect(scroll_clip);
        rshigg::draw_gui(&state.settings_gui, &mut backend, resolution, &theme);
        backend.pop_clip_rect();
    }
}

pub fn center_window(rl: &mut RaylibHandle) {
    let screen_dims = IVec2::new(rl.get_screen_width(), rl.get_screen_height());
    let screen_center = screen_dims / 2;
    let window_center = WINDOW_DIMS.as_ivec2() / 2;
    let offset = IVec2::new(
        screen_center.x - window_center.x,
        screen_center.y - window_center.y,
    );
    rl.set_window_position(offset.x, offset.y);
}

pub fn scale_and_blit_render_texture_to_window(
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
