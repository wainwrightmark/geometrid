# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.#


[crates.io]: https://crates.io/crates/geometrid

## v0.5.0 (2023-11-06)
- Added `is_edge` and `is_corner` and `adjacent_tile_count` to tile
- Added `ALL` and `is_empty` and `with_bit_set` to tile_set

## v0.4.0 (2023-10-05)

- Fixed a bug in `Polyomino.tiles()`
- Added `TryFromDynamic` for `Tile` and `Vertex`
- Bumped dependency versions

## v0.3.0 (2023-7-18)

- Added more polyominos
- Breaking Changes - all polyominos are now in normalized form
- Breaking Changes - replaced `Point` with `glam::f32::Vec2`. This and the `HasCenter` trait are behind the `glam` feature.
- Added `EnumIs` derives to all enums


## v0.2.0 (2023-7-02)

- Rename `Location` to `Point`
- Changed some polyominos

## v0.1.0 (2022-11-15)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/geometrid
