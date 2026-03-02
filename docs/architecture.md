# RShiGG Architecture and API Direction

## Purpose

This document defines current RShiGG architecture after the backend/theme refactor and records how it diverges from Python `shigg`.

## Design Goals

1. Keep the core crate backend-agnostic.
2. Keep backend integration minimal.
3. Keep widget state simple and explicit.
4. Let themes map widgets to low-level primitives.
5. Make demos show real multi-backend extensibility.

## System Split

RShiGG is organized into four layers:

1. Interaction/state (`Gui<TTag>` + widgets + events)
2. Backend primitive API (`DrawBackend`)
3. Theme renderer (`draw_gui`) that turns widgets into primitives
4. Backend adapters/examples (raylib, command-buffer style custom backend)

## Frame Model

Per frame:

1. Application updates widget positions/sizes/values as needed.
2. `Gui::step(mouse_pos, mouse_pressed)` or `Gui::step_pixels(mouse_px, resolution_px, mouse_pressed)` advances interaction state and returns `Vec<TaggedEvent<TTag>>`.
3. `draw_gui(gui, backend, resolution, theme)` renders the same GUI through the theme.

This is retained-state UI with immediate-style usage patterns in the app loop.

## Visibility Model (No Per-Widget Hidden Field)

Visibility can live in application flow (`if show_settings { ... }`) or in `Gui` visibility control.  
When per-element toggling is needed, it lives in `Gui`, not inside each widget type:

- `Gui::set_visible(id, bool)`
- `Gui::is_visible(id) -> bool`

`Gui` tracks hidden IDs and skips hidden elements during `step`.  
`draw_gui` also checks `is_visible` and skips rendering hidden elements.

This keeps widget structs lean while still supporting culling/layout control.  
`set_visible` is a convenience API, not a mandatory pattern.

## Backend Trait

Required low-level primitives:

- `fill_rect`
- `draw_line`
- `draw_text`

Optional extension hooks (default no-op):

- `push_clip_rect`
- `pop_clip_rect`
- `draw_image`

The optional methods keep the trait small for minimal backends while enabling clipping and textured styling where available.

## Theme + Image Styling

`Theme` owns widget look; backend only executes primitives.

Widgets can optionally provide image styles:

- `Button`, `Label`, `Draggable`: `background_image`
- `Slider`, `VerticalSlider`: `track_image`, `thumb_image`

`ImageStyle` fields:

- `image_id` (backend-defined texture/sprite handle key)
- `layout` (`Stretch`, `Tile`, `Center`)
- `tint`
- `draw_over_content` (under/over content control)

Backends interpret `image_id` mapping. The core crate does not load/manage textures.

## Clipping Strategy

RShiGG uses explicit clip stack calls on the backend (`push_clip_rect`/`pop_clip_rect`) where needed (for example, scroll regions in demos), inspired by the same general pattern used in ImGui draw lists.

## Public API Surface

Core:

- `Gui<TTag>`
- widgets: `Button`, `Slider`, `VerticalSlider`, `Draggable`, `Label`, `LeftRightSelector`, `ButtonToggle`, `MoveAndResizeThumbs`
- events: `Event`, `TaggedEvent<TTag>`

Rendering:

- `DrawBackend`
- `Rect`
- `Color`
- `ImageStyle`, `ImageLayout`
- `Theme`
- `draw_gui(...)`

Utilities:

- `transform_mouse_to_normalized_subsurface_coords(...)`

## Differences from Python `shigg`

1. Strongly typed tags/events (`Gui<Tag>`).
2. Backend/theme split is explicit and reusable.
3. Rendering is primitive-based, not backend-specific widget code paths.
4. Visibility/culling is centralized at `Gui` level.
5. Image styling is optional and backend-defined.

## Guardrails

1. Keep core backend-independent.
2. Prefer additive API changes.
3. Keep default backend trait implementation easy to satisfy.
4. Keep widget types focused on interaction data, not renderer internals.
