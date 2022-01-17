# roset

A collection of helpful Rust macro from my personal projects, see example and document in [lib.rs](/src/lib.rs)

## Feature

- `derive(EnumFrom)`
  - `enum_from(str = "🤔")` implement `FromStr` for specific `enum` variant inner type
  - `enum_from(inner)` implement `From<T>` for specific `enum` variant inner type
- `derive(EnumFromWrapped)` implement `From<T>` for every variant inner type in `enum`
- `derive(EnumIntoWrapped)` implement `TryFrom<T>` for every variant inner type in `enum`
