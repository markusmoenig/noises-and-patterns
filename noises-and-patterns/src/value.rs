use crate::prelude::*;

// Based on https://www.shadertoy.com/view/4dS3Wd

pub struct Value {}

impl Noise for Value {

    fn new() -> Self {
        Self {

        }
    }

    fn get_2d(&self, p: (FP, FP)) -> FP {
        let x = FP2::new(p.0, p.1);
        let i = glm::floor(&x);
        let f = glm::fract(&x);

	    // Four corners in 2D of a tile
	    let a = self.hash2d(i);
        let b = self.hash2d(i + FP2::new(1.0, 0.0));
        let c = self.hash2d(i + FP2::new(0.0, 1.0));
        let d = self.hash2d(i + FP2::new(1.0, 1.0));

        // Simple 2D lerp using smoothstep envelope between the values.
        // return vec3(mix(mix(a, b, smoothstep(0.0, 1.0, f.x)),
        //			mix(c, d, smoothstep(0.0, 1.0, f.x)),
        //			smoothstep(0.0, 1.0, f.y)));

        // Same code, with the clamps in smoothstep and common subexpressions
        // optimized away.

        let u = FP2::new( f.x * f.x * (3.0 - 2.0 * f.x), f.y * f.y * (3.0 - 2.0 * f.y));

        self.mix(&a, &b, &u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y
    }

    fn fbm_2d(&self, p: (FP, FP), octaves: i32) -> FP {
        let mut x = FP2::new(p.0, p.1);

        let mut v = 0.0;
        let mut a = 0.5;
        let shift = FP2::new(100.0, 100.0);

        // Rotate to reduce axial bias
        let rot =  Matrix2::new(0.87758256189, 0.4794255386, -0.4794255386, 0.87758256189);

        for _i in 0..octaves {
            v += a * self.get_2d((x.x, x.y));
            x = rot * x * 2.0 + shift;
            a *= 0.5;
        }
        v
    }
}