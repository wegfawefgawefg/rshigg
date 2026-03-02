use glam::Vec2;

pub fn transform_mouse_to_normalized_subsurface_coords(
    normalized_mouse_pos: Vec2,
    surface_resolution: Vec2,
    subsurface_position: Vec2,
    subsurface_resolution: Vec2,
) -> Vec2 {
    let subsurface_normalized_position = subsurface_position / surface_resolution;
    let subsurface_normalized_size = subsurface_resolution / surface_resolution;

    (normalized_mouse_pos - subsurface_normalized_position) / subsurface_normalized_size
}
