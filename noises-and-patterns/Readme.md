A curated list of common noises and patterns in computer graphics. Mostly taken from implementations in [Shadertoy](www.shadertoy.com). All implementations are under the MIT or similar.

This library is intended for users who need access to raw, unprocessed noise values in Rust. This is not intended to be an effect or post processing library. All returned values are between [-1..1] or [0..1] as indicated.

Noise classes support both raw noises as well as smooth FBM variants.

This library is mostly a port of GPU noise and pattern implementations indended for computer graphics applications and games. If you need highest quality noise implementations you may have to look elsewhere.

This library uses the [nalgebra-glm](https://docs.rs/nalgebra-glm/latest/nalgebra_glm/) crate as the math library (and is also its only dependency). The API however has no external dependencies.

### Precision

By default the library compiles to ```f32``` you can change the values in *lib.rs* as instructed and compile to f64 the library to f64 if needed.

### The Traits

The traits for noises and patterns are very simple. Noises are currently 2D only, 3D coming later.

```rust
pub trait Noise {

    fn new() -> Self;

    /// 2D noise for the given position
    fn noise_2d(&self, p: (FP, FP)) -> FP;

    // 2D fbm for the given position and the octaves
    fn fbm_2d(&self, p: (FP, FP), octaves: i32) -> FP;
}

pub trait Pattern {

    fn new() -> Self;

    /// 2D pattern for the given position
    fn pattern_2d(&self, p: (FP, FP)) -> FP;
}

```

# Noises

## Value Noise

Based on [1D, 2D & 3D Value Noise ](https://www.shadertoy.com/view/4dS3Wd)

```rust
    let mut pixels = vec![0;width * height * 4];
    let value = Value::new();

    for y in 0..height {
        for x in 0..width {
            // let v = value.noise_2d(((x as FP) * 0.1, (y as FP) * 0.1));
            let v = value.fbm_2d(((x as FP) * 0.1, (y as FP) * 0.1), 5);

            let v_u8 = (v * 255.0) as u8;
            let color = [v_u8, v_u8, v_u8, 255];

            let d = x * 4 + y * width * 4;

            pixels[d..d+4].copy_from_slice(&color);
        }
    }
```

# Patterns

## Bricks

```rust
    let mut pixels = vec![0;width * height * 4];
    let bricks = Bricks::new();

    for y in 0..height {
        for x in 0..width {
            let scale = 0.8; // Scale as needed
            let v = bricks.pattern_2d(((x as FP / width as FP) * scale, (y as FP / height as FP) * scale));

            let v_u8 = (v * 255.0) as u8;
            let color = [v_u8, v_u8, v_u8, 255];

            let d = x * 4 + y * width * 4;

            pixels[d..d+4].copy_from_slice(&color);
        }
    }
```
