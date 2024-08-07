# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

[crates.io]: https://crates.io/crates/geometrid

## Unreleased

### Added

- Added 'tiles_before' to tile sets

## v0.9.0 (2024-16-07)

### Breaking Changes

- Changed the signatures of some const methods in `Vector` to not take references

### Fixes

- Fixed a bug when iterating through a tile set

## v0.8.0 (2024-26-06)

### Breaking Changes

- Changed the signatures of `Tile` `iter_adjacent` and `iter_contiguous` to remove lifetimes
- Removed `TileByRowIter` and `TileByColIter`
- `Tile` `iter_by_row` and `iter_by_col` are now not const and return type erased iterators
- Renamed `line_of_sight_tiles` to `iter_line_of_sight_tiles`
- Renamed `RectangleIterator` to `CornersIter`
- Replaced `TileSet` `from_iter` with a FromIterator implementation
- Removed `Copy` implementation from `TileSetIter` and `TrueTilesIter`

### Added

- Implement `nth` and `nth_back` for tile set iterator
- `transpose` to `Tile`

## v0.7.0 (2024-19-03)

- Added `TileByRowIter`. Improved tile iterator methods
- Add `first`, `pop`, `last`, `pop_last` to tile sets
- More efficient `iter_true_tiles` in tile sets
- More efficient `row_mask` and `col_mask` in tile sets
- Tile set `iter_true_tiles` now implements `FusedIterator` and `DoubleEndedIterator`
- Bumped glam to 0.25.0

## v0.6.0 (2023-11-09)

- Improved `rotate` for tile_map and added `with_rotate` and `with_flip`
- Added `is_subset`, `is_superset` and `symmetric difference` for tile_set and `tile_set256`
- Added `ALL` and `is_empty` and `with_bit_set` to `tile_set256`
- Improved performance of `iter_true_tiles` for `tile_set256`

## v0.5.0 (2023-11-06)

- Added `is_edge` and `is_corner` and `adjacent_tile_count` to tile
- Added `ALL` and `is_empty` and `with_bit_set` to tile_set
- Bumped versions of dependencies
- Huge performance improvements for tile_set `iter_true_tiles`, especially for sparse sets

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

