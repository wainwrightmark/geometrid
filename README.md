# geometrid

![GITHUB](https://img.shields.io/github/last-commit/wainwrightmark/geometrid)
![Crates.io](https://img.shields.io/crates/v/geometrid)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/wainwrightmark/geometrid/build.yml)
![docs](https://img.shields.io/docsrs/geometrid)


2d grids, tiles, and vertices, focusing in particular on grids whose size is a compile time constant. Also contains features for Shapes and Polyominos and other common features of 2d grid based games.

_This crate is currently very unstable. I will attempt to stabilize it properly if const traits are ever stabilized._

At the moment, the constant sized types are all internally backed by a `u8`, this means that the largest grid you can build is 16x16. If you want to use larger grids please file an issue and I'll create `u16` and `u32` versions.

Please also file an issue or PR if there are any other useful capabilities that I've missed.

The crate has the following optional features:

| Name    | Description                                  | Default |
| ------- | -------------------------------------------- | ------- |
| `std`   | Required for some floating point functions   | `false` |
| `serde` | `Serialize` and `Deserialize` for most types | `false` |
| `u256`  | Enables `TileSet256`                         | `false` |

One of the hardest problems in creating 2d grids is deciding which way is up. This crate uses compass points to describe directions. Going North corresponds to decreasing the value of the `y` coordinate, Going East corresponds to increasing the value of the `x` coordinate.

A 2x2 grid of tiles looks like this

```
┌───────┬───────┐
│       │       │
│ (0,0) │ (1,0) │
│       │       │
├───────┼───────┤
│       │       │
│ (0,1) │ (1,1) │
│       │       │
└───────┴───────┘
```

The vertices of the same grid look like this
```
 (0,0)   (1,0)   (2,0)
┌───────┬───────┐
│       │       │
│       │       │
│       │       │
│(0,1)  │(1,1)  │(2,1)
├───────┼───────┤
│       │       │
│       │       │
│       │       │
│(0,2)  │(1,2)  │(2,2)
└───────┴───────┘
```

## Getting started

```rust
use geometrid::*;

fn main() {

    let mut grid: TileMap<usize, 3, 3, 9> = TileMap::from_fn(|x| x.into());
    assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
    grid.flip(FlipAxes::Vertical);
    assert_eq!(grid.to_string(), "6|7|8\n3|4|5\n0|1|2");
}
```

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/wainwrightmark/geometrid/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`geometrid` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

- [Mark Wainwright](https://github.com/wainwrightmark)
