use glam::Vec2;
use raylib::{drawing::RaylibDraw, math::Vector2, prelude::Color as RaylibColor};
use rshigg::{Color, DrawBackend, Gui, Rect, Theme};

pub fn draw_gui<T: Clone + Copy, D: RaylibDraw>(gui: &Gui<T>, draw: &mut D, resolution: Vec2) {
    let mut backend = RaylibBackend { draw };
    let theme = Theme::default();
    rshigg::draw_gui(gui, &mut backend, resolution, &theme);
}

struct RaylibBackend<'a, D: RaylibDraw> {
    draw: &'a mut D,
}

impl<D: RaylibDraw> DrawBackend for RaylibBackend<'_, D> {
    fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.draw.draw_rectangle(
            rect.position.x as i32,
            rect.position.y as i32,
            rect.size.x as i32,
            rect.size.y as i32,
            to_raylib_color(color),
        );
    }

    fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color, thickness: f32) {
        if thickness <= 1.0 {
            self.draw.draw_line(
                start.x as i32,
                start.y as i32,
                end.x as i32,
                end.y as i32,
                to_raylib_color(color),
            );
            return;
        }

        self.draw.draw_line_ex(
            Vector2::new(start.x, start.y),
            Vector2::new(end.x, end.y),
            thickness,
            to_raylib_color(color),
        );
    }

    fn draw_text(&mut self, text: &str, position: Vec2, font_size: f32, color: Color) {
        self.draw.draw_text(
            text,
            position.x as i32,
            position.y as i32,
            font_size as i32,
            to_raylib_color(color),
        );
    }
}

fn to_raylib_color(color: Color) -> RaylibColor {
    RaylibColor::new(color.r, color.g, color.b, color.a)
}
