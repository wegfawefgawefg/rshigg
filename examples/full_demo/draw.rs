use glam::{IVec2, Vec2};
use raylib::{
    drawing::RaylibDraw,
    math::Vector2,
    prelude::{Color as RayColor, *},
};
use rshigg::{Color, DrawBackend, Rect, Theme};

use crate::state::{settings_scroll_clip_rect, DemoState, DIMS, WINDOW_DIMS};

pub fn draw_scene(state: &DemoState, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
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
    let mut backend = RaylibBackend {
        draw: d,
        scissor_stack: Vec::new(),
    };
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

struct RaylibBackend<'a, D: RaylibDraw> {
    draw: &'a mut D,
    scissor_stack: Vec<Rect>,
}

impl<D: RaylibDraw> DrawBackend for RaylibBackend<'_, D> {
    fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.draw.draw_rectangle(
            rect.position.x as i32,
            rect.position.y as i32,
            rect.size.x as i32,
            rect.size.y as i32,
            RayColor::new(color.r, color.g, color.b, color.a),
        );
    }

    fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color, thickness: f32) {
        if thickness <= 1.0 {
            self.draw.draw_line(
                start.x as i32,
                start.y as i32,
                end.x as i32,
                end.y as i32,
                RayColor::new(color.r, color.g, color.b, color.a),
            );
        } else {
            self.draw.draw_line_ex(
                Vector2::new(start.x, start.y),
                Vector2::new(end.x, end.y),
                thickness,
                RayColor::new(color.r, color.g, color.b, color.a),
            );
        }
    }

    fn draw_text(&mut self, text: &str, position: Vec2, font_size: f32, color: Color) {
        self.draw.draw_text(
            text,
            position.x as i32,
            position.y as i32,
            font_size as i32,
            RayColor::new(color.r, color.g, color.b, color.a),
        );
    }

    fn push_clip_rect(&mut self, rect: Rect) {
        self.scissor_stack.push(rect);
        let top = self.scissor_stack[self.scissor_stack.len() - 1];
        unsafe {
            raylib::ffi::BeginScissorMode(
                top.position.x as i32,
                top.position.y as i32,
                top.size.x as i32,
                top.size.y as i32,
            );
        }
    }

    fn pop_clip_rect(&mut self) {
        if self.scissor_stack.is_empty() {
            return;
        }
        self.scissor_stack.pop();
        unsafe {
            raylib::ffi::EndScissorMode();
        }
        if let Some(top) = self.scissor_stack.last().copied() {
            unsafe {
                raylib::ffi::BeginScissorMode(
                    top.position.x as i32,
                    top.position.y as i32,
                    top.size.x as i32,
                    top.size.y as i32,
                );
            }
        }
    }

    fn draw_image(&mut self, image: rshigg::ImageStyle, rect: Rect) {
        // Placeholder image rendering path: backend integrators should map image_id to textures.
        let mut tint = RayColor::new(image.tint.r, image.tint.g, image.tint.b, image.tint.a);
        if image.draw_over_content {
            tint.a = 110;
        }
        self.draw.draw_rectangle(
            rect.position.x as i32,
            rect.position.y as i32,
            rect.size.x as i32,
            rect.size.y as i32,
            tint,
        );
    }
}
