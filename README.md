# Simple plotting library for Bevy

Draws line plots in 2d using gizmos.

![output](https://github.com/user-attachments/assets/6e6f0b2b-8fd2-4650-b07d-96a4c81b6d09)

### Installation

```sh
cargo add --git https://github.com/tbillington/bevy_plotting
```

### Usage

```rust
// Data
let points = [Vec2::new(0., 0.), Vec2::new(1., 1.), Vec2::new(2., 0.5)];
// Position & size of plot
let bounds = Rect::from_center_size(Vec2::ZERO, Vec2::splat(100.));

draw_gizmo_line_plot_2d_raw(
    points.iter(),
    &mut gizmos,
    bounds,
    Vec2::ZERO,
    Vec2::new(2., 1.),
    Color::WHITE,
);
```

### License

Except where noted, all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

#### Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
