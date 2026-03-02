use glam::Vec2;

pub fn transform_mouse_to_subsurface_coords(
    mouse_position_surface: Vec2,
    subsurface_position: Vec2,
    subsurface_size_surface: Vec2,
    subsurface_resolution: Vec2,
) -> Vec2 {
    if subsurface_size_surface.x <= 0.0
        || subsurface_size_surface.y <= 0.0
        || subsurface_resolution.x <= 0.0
        || subsurface_resolution.y <= 0.0
    {
        return Vec2::new(-1.0, -1.0);
    }

    let local_surface = mouse_position_surface - subsurface_position;
    Vec2::new(
        local_surface.x * subsurface_resolution.x / subsurface_size_surface.x,
        local_surface.y * subsurface_resolution.y / subsurface_size_surface.y,
    )
}
