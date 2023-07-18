# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.#


[crates.io]: https://crates.io/crates/geometrid

## v0.3.0 (2023-7-18)

- Added more polyominos
- Breaking Changes - all polyominos are now in normalized form
- Breaking Changes - replaced `Point` with `glam::f32::Vec2`. This and the `HasCenter` trait are behind the `glam` feature.


## v0.2.0 (2023-7-02)

- Rename `Location` to `Point`
- Changed some polyominos

## v0.1.0 (2022-11-15)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/geometrid
