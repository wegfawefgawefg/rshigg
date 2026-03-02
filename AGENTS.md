# AGENTS.md

Project-local engineering guidance for contributors and coding agents.

## Style and Structure

1. Prefer a C+-style coding feel:
   - straightforward control flow
   - explicit state updates
   - limited macro-heavy abstraction
   - no over-engineered type gymnastics
2. Respect obvious Rust idioms where they provide safety or clarity.
3. Keep related types and functions grouped by responsibility.
4. Target file size between 300 and 500 lines max.
5. Split modules by concern before files become sprawling.

## Architecture Expectations

1. Core `rshigg` logic must remain backend-agnostic.
2. Rendering backend integrations should be adapters over `DrawBackend`.
3. Widget-to-primitive mapping belongs in theme/render code, not backend adapters.
4. Examples should demonstrate:
   - default raylib usage
   - custom backend implementation

## API Design Principles

1. Keep public APIs small and explicit.
2. Favor additive evolution and avoid churn.
3. Keep event/tag flow simple (`Gui<Tag>` + `TaggedEvent<Tag>`).
4. Prefer plain data structures over deep inheritance-like layers.

## Testing and Validation

1. Run `cargo check` after library changes.
2. Run `cargo check --examples` after demo or backend changes.
3. Keep examples compiling; examples are part of the API story.

## Documentation

1. Update `README.md` for user-facing changes.
2. Update `docs/architecture.md` for design-level changes.
3. Keep docs aligned with real code, not aspirational-only APIs.
