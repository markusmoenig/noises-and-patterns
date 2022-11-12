use crate::prelude::*;

pub struct Bricks {}

impl Pattern for Bricks {

    fn new() -> Self {
        Self {

        }
    }

    fn pattern_2d(&self, p: (FP, FP)) -> FP {
        let cell = 1.0;
        let ratio = 2.0;
        let brick = 1.0;
        let gap_x = 0.08;

        let uv = FP2::new(p.0, p.1);

        let mut u = uv;// + FP2::new(10000.0, 10000.0);

        let bevelx = 0.07;
        let bevel = FP2::new(bevelx, bevelx);
        let gap = FP2::new(gap_x, gap_x);
        let round = 0.25;
        //let missing = 0.0;

        let w = FP2::new(ratio,1.0);
        u = u.component_mul(&FP2::new(cell, cell).component_div(&w));

        if brick == 1.0 {
            u.x += 0.5 * u.y.floor() % 2.0;
        }

        //hash = hash21(floor(U))

        let t = glm::fract(&u) - FP2::new(1.0, 1.0) / 2.0;
        let s = w.component_mul(&t);

        let a = w / 2.0 - gap - glm::abs(&s);
        let b = a.component_mul(&FP2::new(2.0, 2.0)).component_div(&bevel);
        let mut m = b.x.min(b.y);
        if a.x < round && a.y < round {
           m = (round - glm::length(&(FP2::new(round, round) - a))) * 2.0 / glm::dot(&bevel,&glm::normalize(&(FP2::new(round, round) - a)));
        }

        //if MISSING > missingHash(floor(U)) {
        //    isMissing = true
        //}

        m.clamp(0.0, 1.0)
    }

}