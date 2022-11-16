use crate::prelude::*;

pub mod bricks;

pub trait Pattern {

    fn new() -> Self;

    /// 2D pattern for the given position, returns mask and id
    fn pattern_2d(&self, p: (FP, FP)) -> (FP, FP);

    /// For setting pattern properties
    fn set_property(&mut self, name: &str, value: FP);

    /// 2D hash, taken from https://www.shadertoy.com/view/4djSRW
    #[inline(always)]
    fn hash21(&self, p: FP2) -> FP {
        let mut p3 = glm::fract(&FP3::new(p.x * 0.1031, p.y * 0.1031, p.x * 0.1031));

        let dot = glm::dot(&p3, &(FP3::new(p3.y + 33.333, p3.z + 33.333, p3.x + 33.333)));

        p3.x += dot; p3.y += dot; p3.z += dot;
        ((p3.x + p3.y) * p3.z).fract()
    }
}
