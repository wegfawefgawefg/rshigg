# RShiGG

Rust implementation of ShiGG: a compact, game-oriented GUI library.

Status: usable today for small projects and demos, but not complete yet.

## Current Status

- Core widget logic is backend-agnostic.
- Raylib is used in the `basic` example.
- A larger scrollable panel demo is provided in `full_demo`.
- `full_demo` now uses textured image regions for row skins/icons.
- A backend extension example is provided in `custom_backend`.
- A stylized RPG-status-inspired image demo is provided in `rpg_status_demo`.
- All current widgets render through the same backend/theme architecture.
- GUI-level visibility control is available via `Gui::set_visible(id, bool)`.

## Run

```bash
cargo run
```

Shows quick usage guidance.

```bash
cargo run --example basic
```

Runs the raylib demo.

```bash
cargo run --example full_demo
```

Runs the larger settings-panel demo inspired by the original Python project.

```bash
cargo run --example rpg_status_demo
```

Runs a texture-skinned RPG status panel demo inspired by reference menu UIs in `refs/`.

```bash
cargo run --example custom_backend
```

Runs a non-raylib backend implementation that records draw commands.

## Core Concepts

1. Build a `Gui<Tag>` and add elements.
2. Step GUI state with pixel mouse coordinates via `Gui::step(...)`.
3. For GUI rendered in sub-rectangles/scaled surfaces, use `Gui::step_in_rect(...)`.
4. Handle returned `TaggedEvent<Tag>` values.
5. Render widgets using:
   - your own backend implementing `DrawBackend`
   - `rshigg::draw_gui(...)` with a `Theme`
6. Optionally set widget image styles (background, track, thumb) and let the backend decide how to render images.

## Widgets

- `Button`
- `Slider`
- `VerticalSlider`
- `Draggable`
- `Label`
- `LeftRightSelector`
- `ButtonToggle`
- `MoveAndResizeThumbs`

## Minimal Example

```rust
use glam::Vec2;
use rshigg::{Button, Gui, Slider};

#[derive(Clone, Copy, Debug)]
enum Tag {
    ToggleMute,
    SetVolume,
}

let mut gui = Gui::new();
gui.add_button(
    Button::new(Vec2::new(32.0, 24.0), Vec2::new(160.0, 54.0), Some("Mute".into())),
    Tag::ToggleMute,
);
gui.add_slider(
    Slider::new(
        Vec2::new(32.0, 108.0),
        Vec2::new(576.0, 43.2),
        24.0,
        0.0,
        100.0,
        1.0,
        50.0,
        0.05,
        Some("Volume".into()),
    ),
    Tag::SetVolume,
);

let events = gui.step(Vec2::new(120.0, 84.0), true);
```

## Backend API

The library rendering surface is intentionally small:

```rust
pub trait DrawBackend {
    fn fill_rect(&mut self, rect: Rect, color: Color);
    fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color, thickness: f32);
    fn draw_text(&mut self, text: &str, position: Vec2, font_size: f32, color: Color);
    fn push_clip_rect(&mut self, rect: Rect) { ... }  // optional
    fn pop_clip_rect(&mut self) { ... }               // optional
    fn draw_image(&mut self, image: ImageStyle, rect: Rect) { ... } // optional
}
```

Theme/widget rendering lives in `rshigg::draw_gui(...)`, which maps widgets to these primitives.

## Utilities

`transform_mouse_to_subsurface_coords(...)` is provided for mouse coordinate remapping when drawing GUI into subregions.

## Design Docs

See:

- `docs/architecture.md`
- `docs/design_decisions_and_refactors.md`

It documents:

- backend strategy
- public API direction
- differences vs original Python ShiGG
