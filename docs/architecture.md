# RShiGG Architecture and API Direction

## Purpose

This document defines the post-refactor direction for RShiGG and records how it intentionally diverges from the original Python `shigg`.

## Scope

RShiGG is split into:

1. Core interaction/state engine (backend-agnostic)
2. Rendering abstraction (`DrawBackend`)
3. Theme renderer (`draw_gui`) that maps widgets to draw primitives
4. Backend adapters (raylib example today, more later)

## Core Rules

1. Core crate must not depend on a graphics backend.
2. Widget logic and event generation must remain deterministic from input state.
3. Rendering must happen through low-level primitives, not widget-specific backend hooks.

## Why a Tiny Backend Trait

A small trait is stable and easy for users to implement:

- `fill_rect`
- `draw_line`
- `draw_text`

This keeps backend integration simple for raylib, SDL2, macroquad, engine-native wrappers, or headless command buffers.

## Theme Layer

`rshigg::draw_gui(gui, backend, resolution, theme)` owns the mapping:

- button -> bevel rectangles + text
- slider -> track + thumb
- draggable -> bevel + grip lines
- label -> text box

Backends only execute primitives. Theme decisions stay centralized.

## Public API Direction

### Stable Core Types

- `Gui<TTag>`
- widgets:
  - `Button`
  - `Slider`
  - `VerticalSlider`
  - `Draggable`
  - `Label`
  - `LeftRightSelector`
  - `ButtonToggle`
  - `MoveAndResizeThumbs`
- events: `Event`, `TaggedEvent<TTag>`

### Stable Render Types

- `DrawBackend`
- `Rect`
- `Color`
- `Theme`
- `draw_gui(...)`

### Utility

- `transform_mouse_to_normalized_subsurface_coords(...)`

## Differences from Python ShiGG

1. Rust API is explicit and typed (`Gui<Tag>` + enum tags).
2. Rendering is no longer tied to pygame-style code paths.
3. Event queue model is simplified as frame-returned vectors from `Gui::step`.
4. Composite widgets are first-class elements and render through the same theme/backend path.

## Backend Strategy

Current:

- Raylib adapter in `examples/basic/draw.rs`
- Headless/custom adapter in `examples/custom_backend.rs`
- Full UI demo in `examples/full_demo.rs`

Planned:

1. `rshigg-raylib` helper crate (optional)
2. `rshigg-sdl2` helper crate
3. dedicated input adapters per backend (mouse + wheel + key mapping helpers)

## Porting Roadmap (Short Term)

1. Improve clipping/viewport behavior for large scroll containers.
2. Add optional helper crates for concrete backends (`rshigg-raylib`, `rshigg-sdl2`).
3. Add additional docs/examples for style customization.
4. Add API docs and compile-tested code snippets.

## Guardrails

1. Keep core backend-independent.
2. Keep backend trait minimal.
3. Keep theme behavior composable and overridable.
4. Prefer additive changes over breaking signatures unless unavoidable.
