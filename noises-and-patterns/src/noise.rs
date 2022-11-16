use crate::prelude::*;

pub mod value;
pub mod voronoibasic;

pub trait Noise {

    fn new() -> Self;

    /// 2D noise for the given position
    fn get_2d(&self, p: (FP, FP)) -> FP;

    // 2D fbm for the given position and the octaves
    fn fbm_2d(&self, p: (FP, FP), octaves: i32) -> FP {
        let mut x = FP2::new(p.0, p.1);

        let mut f = 0.0;
        let mut a = 0.5;
        let shift = FP2::new(100.0, 100.0);

        // Rotate to reduce axial bias
        let rot =  Matrix2::new(1.6,  1.2, -1.2,  1.6);

        for _i in 0..octaves {
            f += a * self.get_2d((x.x, x.y));
            x = rot * x * 2.0 + shift;
            a *= 0.5;
        }
        f.clamp(0.0, 1.0)
    }

    /// 2D hash, taken from https://www.shadertoy.com/view/4djSRW
    #[inline(always)]
    fn hash21(&self, p: FP2) -> FP {
        let mut p3 = glm::fract(&FP3::new(p.x * 0.1031, p.y * 0.1031, p.x * 0.1031));

        let dot = glm::dot(&p3, &(FP3::new(p3.y + 33.333, p3.z + 33.333, p3.x + 33.333)));

        p3.x += dot; p3.y += dot; p3.z += dot;
        ((p3.x + p3.y) * p3.z).fract()
    }

    /// Mix for FP
    #[inline(always)]
    fn mix(&self, a: &FP, b: &FP, v: &FP) -> FP {
        (1.0 - v) * a + b * v
    }
}
