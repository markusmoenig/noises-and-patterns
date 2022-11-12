A curated list of common noises and patterns in computer graphics. Mostly taken from implementations in [Shadertoy](www.shadertoy.com). All implementations are under the MIT or similar.

This library is intended for users who need access to raw, unprocessed noise values in Rust. This is not intended to be an effect or post processing library. All returned values are between [-1..1] or [0..1] as indicated.

Noise classes support both raw noises as well as smooth FBM variants.

This library is mostly a port of GPU noise and pattern implementations indended for computer graphics applications and games. If you need highest quality noise implementations you may have to look elsewhere.

This library uses the [nalgebra-glm](https://docs.rs/nalgebra-glm/latest/nalgebra_glm/) crate as the math library (and is also its only dependency). The API however has no external dependencies.

### The Trait

The trait for all noises is very simple. Currently 2D only, 3D coming soon.

```rust
pub trait Noise {

    fn new() -> Self;

    /// 2D noise for the given position
    fn get_2d(&self, p: (FP, FP)) -> FP;

    // 2D fbm for the given position and the octaves
    fn fbm_2d(&self, p: (FP, FP), octaves: i32) -> FP;
}

```

# Noises

## Value Noise

```rust

    let mut pixels = vec![0;width * height * 4];

    let value = Value::new();

    for y in 0..height {
        for x in 0..width {
            // let v = value.get_2d(((x as FP) * 0.1, (y as FP) * 0.1));
            let v = value.fbm_2d(((x as FP) * 0.1, (y as FP) * 0.1), 5);

            let v_u8 = (v * 255.0) as u8;
            let color = [v_u8, v_u8, v_u8, 255];

            let d = x * 4 + y * width * 4;

            pixels[d..d+4].copy_from_slice(&color);
        }
    }
```

<table><tr>
<td> <img src="images/value2d.png" alt="Value 2D" style="width: 500px;"/> </td>
<td> <img src="images/value2d_fbm.png" alt="Value 2D FBM" style="width: 500px;"/> </td>
</tr></table>
