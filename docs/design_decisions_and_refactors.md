# RShiGG Design Decisions and Refactor Backlog

This document tracks architecture/usability decisions we should explicitly make, plus refactors that simplify both the implementation and API usage.

Status legend:
- `done`: decided and implemented
- `open`: needs decision
- `planned`: decision made, implementation pending

## Decision List

1. Coordinate policy (pixel vs normalized)
   - status: `done`
   - decision: pixel-native core API and internals.
   - notes: `Gui::step(...)` uses pixel mouse coords; `Gui::step_in_rect(...)` and `transform_mouse_to_subsurface_coords(...)` cover remapping cases.

2. GUI ownership model (retained vs frame-built)
   - status: `open`
   - decision needed: keep retained-only, or add a frame-built command API in parallel.
   - tradeoff: retained is simple for stateful game UI; frame-built can reduce stale state bugs.

3. Input API shape
   - status: `planned`
   - target: keep one canonical `step(...)` path and one explicit mapping helper (`step_in_rect(...)`).
   - cleanup: avoid adding overlapping step variants.

4. Visibility strategy
   - status: `open`
   - decision needed: promote app-level branching as default, keep `Gui::set_visible` as convenience, or standardize on one pattern in docs/examples.

5. Composite event semantics
   - status: `open`
   - decision needed: whether child widget events inside composites should ever bubble, or only composite-level events should be emitted.

6. Unit consistency for widget internals
   - status: `planned`
   - target: keep all size/position-like fields explicitly in pixels (`thumb_width`, `thumb_height`, selector button width, etc.) and document this in API docs.

7. Layout stance
   - status: `open`
   - decision needed: provide minimal layout helpers (`stack`, `row`, `padding`) or keep manual layout math as a deliberate scope constraint.

8. Render layering model
   - status: `open`
   - decision needed: standardize explicit layers (background/content/overlay/debug) to simplify theme overdraw/image behavior.

9. ID/tag model
   - status: `open`
   - decision needed: auto-generated IDs only vs optional user-supplied stable IDs.

10. Scroll container abstraction
    - status: `open`
    - decision needed: keep scroll logic demo-local or add a small reusable core abstraction for clipping + offset + wheel handling.

11. Widget interaction state simplification
    - status: `open`
    - opportunities: standardize pressed/hover patterns, simplify drag pre-hover/countdown behavior, reduce duplicated state transitions across widgets.

12. Core vs demo-support boundaries
    - status: `planned`
    - target: keep crate core minimal and move skin/theme-heavy helpers into demo/shared modules or optional backend helper crates.

## Recommended Next Pass (small, high-impact)

1. Decide and document visibility recommendation (`if branch` default + when to use `set_visible`).
2. Decide layout stance (no helper vs tiny helper set).
3. Decide scroll abstraction scope (core vs demo utility).
4. After those decisions, do one cleanup pass to remove dead branches and normalize naming/docs.
