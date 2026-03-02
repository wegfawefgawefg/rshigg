use std::collections::HashMap;

use glam::Vec2;
use raylib::{
    drawing::RaylibDraw,
    math::Vector2,
    prelude::{Color as RayColor, RaylibHandle, RaylibThread, Rectangle, Texture2D},
};
use rshigg::{Color, DrawBackend, ImageLayout, ImageStyle, Rect};

pub const IMG_STATUS_BG: u64 = 1;
pub const IMG_HELP_BAR: u64 = 2;
pub const IMG_ROW_STRIP: u64 = 3;
pub const IMG_SLIDER_TRACK: u64 = 4;
pub const IMG_SLIDER_KNOB: u64 = 5;
pub const IMG_GOLD_ARROW: u64 = 6;
pub const IMG_PORTRAIT: u64 = 7;
pub const IMG_OPTION_BUTTON: u64 = 8;
pub const IMG_ICON_GEAR: u64 = 9;
pub const IMG_ICON_POTATO: u64 = 10;
pub const IMG_AURORA_TILE: u64 = 100;
pub const IMG_SOFT_NOISE: u64 = 101;
pub const IMG_ICON_MEAT: u64 = 102;
pub const IMG_ICON_MOUSE: u64 = 103;

#[derive(Clone, Copy)]
struct SpriteRegion {
    texture_index: usize,
    source: Rectangle,
}

pub struct SkinTextures {
    textures: Vec<Texture2D>,
    regions: HashMap<u64, SpriteRegion>,
}

impl SkinTextures {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut skins = Self {
            textures: Vec::new(),
            regions: HashMap::new(),
        };

        if let Some(status_tex) = skins.try_load_texture(rl, thread, "refs/image copy 2.png") {
            skins.add_region(IMG_STATUS_BG, status_tex, 0.0, 0.0, 500.0, 281.0);
            skins.add_region(IMG_HELP_BAR, status_tex, 34.0, 14.0, 188.0, 21.0);
            skins.add_region(IMG_ROW_STRIP, status_tex, 122.0, 84.0, 318.0, 22.0);
            skins.add_region(IMG_SLIDER_TRACK, status_tex, 121.0, 184.0, 320.0, 16.0);
            skins.add_region(IMG_SLIDER_KNOB, status_tex, 215.0, 184.0, 28.0, 16.0);
            skins.add_region(IMG_PORTRAIT, status_tex, 60.0, 121.0, 56.0, 56.0);
            skins.add_region(IMG_SOFT_NOISE, status_tex, 438.0, 2.0, 56.0, 56.0);
        }

        if let Some(menu_tex) = skins.try_load_texture(rl, thread, "refs/image copy 5.png") {
            skins.add_region(IMG_OPTION_BUTTON, menu_tex, 196.0, 99.0, 508.0, 34.0);
            skins.add_region(IMG_GOLD_ARROW, menu_tex, 168.0, 107.0, 28.0, 17.0);
        }

        if let Some(title_tex) = skins.try_load_texture(rl, thread, "refs/image copy.png") {
            skins.add_region(IMG_AURORA_TILE, title_tex, 460.0, 4.0, 280.0, 70.0);
        }

        if let Some(gear_tex) = skins.try_load_texture(rl, thread, "assets/gear.png") {
            skins.add_region(IMG_ICON_GEAR, gear_tex, 0.0, 0.0, 24.0, 24.0);
        }
        if let Some(potato_tex) = skins.try_load_texture(rl, thread, "assets/potato.png") {
            skins.add_region(IMG_ICON_POTATO, potato_tex, 0.0, 0.0, 24.0, 24.0);
        }
        if let Some(meat_tex) = skins.try_load_texture(rl, thread, "assets/meat.png") {
            skins.add_region(IMG_ICON_MEAT, meat_tex, 0.0, 0.0, 24.0, 24.0);
        }
        if let Some(mouse_tex) = skins.try_load_texture(rl, thread, "assets/mouse.png") {
            skins.add_region(IMG_ICON_MOUSE, mouse_tex, 0.0, 0.0, 23.0, 32.0);
        }

        skins
    }

    fn try_load_texture(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
    ) -> Option<usize> {
        match rl.load_texture(thread, path) {
            Ok(texture) => {
                let next_index = self.textures.len();
                self.textures.push(texture);
                Some(next_index)
            }
            Err(err) => {
                eprintln!("warning: failed to load texture '{}': {}", path, err);
                None
            }
        }
    }

    fn add_region(&mut self, image_id: u64, texture_index: usize, x: f32, y: f32, w: f32, h: f32) {
        self.regions.insert(
            image_id,
            SpriteRegion {
                texture_index,
                source: Rectangle::new(x, y, w, h),
            },
        );
    }

    fn lookup(&self, image_id: u64) -> Option<(&Texture2D, Rectangle)> {
        let region = self.regions.get(&image_id)?;
        let texture = self.textures.get(region.texture_index)?;
        Some((texture, region.source))
    }
}

pub struct SkinRaylibBackend<'a, D: RaylibDraw> {
    draw: &'a mut D,
    skins: &'a SkinTextures,
    scissor_stack: Vec<Rect>,
}

impl<'a, D: RaylibDraw> SkinRaylibBackend<'a, D> {
    pub fn new(draw: &'a mut D, skins: &'a SkinTextures) -> Self {
        Self {
            draw,
            skins,
            scissor_stack: Vec::new(),
        }
    }

    fn draw_sprite(
        &mut self,
        texture: &Texture2D,
        source: Rectangle,
        image: ImageStyle,
        rect: Rect,
    ) {
        match image.layout {
            ImageLayout::Stretch => {
                let destination = Rectangle::new(
                    rect.position.x,
                    rect.position.y,
                    rect.size.x.max(0.0),
                    rect.size.y.max(0.0),
                );
                self.draw.draw_texture_pro(
                    texture,
                    source,
                    destination,
                    Vector2::new(0.0, 0.0),
                    0.0,
                    to_ray_color(image.tint),
                );
            }
            ImageLayout::Center => {
                let src_w = source.width.abs().max(1.0);
                let src_h = source.height.abs().max(1.0);
                let dst_w = rect.size.x.max(0.0);
                let dst_h = rect.size.y.max(0.0);
                let scale = (dst_w / src_w).min(dst_h / src_h).min(1.0);
                let draw_w = src_w * scale;
                let draw_h = src_h * scale;
                let destination = Rectangle::new(
                    rect.position.x + (dst_w - draw_w) * 0.5,
                    rect.position.y + (dst_h - draw_h) * 0.5,
                    draw_w,
                    draw_h,
                );
                self.draw.draw_texture_pro(
                    texture,
                    source,
                    destination,
                    Vector2::new(0.0, 0.0),
                    0.0,
                    to_ray_color(image.tint),
                );
            }
            ImageLayout::Tile => {
                let tile_w = source.width.abs().max(1.0);
                let tile_h = source.height.abs().max(1.0);
                let x_end = rect.position.x + rect.size.x.max(0.0);
                let y_end = rect.position.y + rect.size.y.max(0.0);
                let x_dir = if source.width >= 0.0 { 1.0 } else { -1.0 };
                let y_dir = if source.height >= 0.0 { 1.0 } else { -1.0 };

                let mut y = rect.position.y;
                while y < y_end {
                    let h = (y_end - y).min(tile_h);
                    let mut x = rect.position.x;
                    while x < x_end {
                        let w = (x_end - x).min(tile_w);
                        let src = Rectangle::new(source.x, source.y, w * x_dir, h * y_dir);
                        let destination = Rectangle::new(x, y, w, h);
                        self.draw.draw_texture_pro(
                            texture,
                            src,
                            destination,
                            Vector2::new(0.0, 0.0),
                            0.0,
                            to_ray_color(image.tint),
                        );
                        x += w;
                    }
                    y += h;
                }
            }
        }
    }
}

impl<D: RaylibDraw> DrawBackend for SkinRaylibBackend<'_, D> {
    fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.draw.draw_rectangle(
            rect.position.x as i32,
            rect.position.y as i32,
            rect.size.x.max(0.0) as i32,
            rect.size.y.max(0.0) as i32,
            to_ray_color(color),
        );
    }

    fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color, thickness: f32) {
        if thickness <= 1.0 {
            self.draw.draw_line(
                start.x as i32,
                start.y as i32,
                end.x as i32,
                end.y as i32,
                to_ray_color(color),
            );
            return;
        }
        self.draw.draw_line_ex(
            Vector2::new(start.x, start.y),
            Vector2::new(end.x, end.y),
            thickness,
            to_ray_color(color),
        );
    }

    fn draw_text(&mut self, text: &str, position: Vec2, font_size: f32, color: Color) {
        self.draw.draw_text(
            text,
            position.x as i32,
            position.y as i32,
            font_size.max(1.0) as i32,
            to_ray_color(color),
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

    fn draw_image(&mut self, image: ImageStyle, rect: Rect) {
        if let Some((texture, source)) = self.skins.lookup(image.image_id) {
            self.draw_sprite(texture, source, image, rect);
            return;
        }
        self.fill_rect(rect, Color::rgba(255, 0, 255, 140));
    }
}

fn to_ray_color(color: Color) -> RayColor {
    RayColor::new(color.r, color.g, color.b, color.a)
}
