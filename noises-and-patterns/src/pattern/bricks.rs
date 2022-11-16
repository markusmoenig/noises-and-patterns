use crate::prelude::*;

pub struct Bricks {
    ratio               : FP,
    brick               : FP,
    cell                : FP,
    gap                 : FP,
    bevel               : FP,
    round               : FP,
}

impl Pattern for Bricks {

    fn new() -> Self {



        Self {
            ratio       : 2.0,
            brick       : 1.0,
            cell        : 16.0,
            gap         : 0.08,
            bevel       : 0.07,
            round       : 0.25,
        }
    }

    fn set_property(&mut self, name: &str, value: FP) {
        if name == "ratio" {
            self.ratio = value;
        } else
        if name == "brick" {
            self.brick = value;
        } else
        if name == "cell" {
            self.cell = value;
        } else
        if name == "gap" {
            self.gap = value;
        } else
        if name == "bevel" {
            self.bevel = value;
        } else
        if name == "round" {
            self.round = value;
        }
    }

    fn pattern_2d(&self, p: (FP, FP)) -> (FP, FP) {
        let uv = FP2::new(p.0, p.1);

        let mut u = uv + FP2::new(10000.0, 10000.0);

        let bevel = FP2::new(self.bevel, self.bevel);
        let gap = FP2::new(self.gap, self.gap);
        let round = self.round;

        let w = FP2::new(self.ratio,1.0);
        u = u.component_mul(&FP2::new(self.cell, self.cell).component_div(&w));

        if self.brick == 1.0 {
            u.x += 0.5 * u.y.floor() % 2.0;
        }

        let t = glm::fract(&u) - FP2::new(1.0, 1.0) / 2.0;
        let s = w.component_mul(&t);

        let a = w / 2.0 - gap - glm::abs(&s);
        let b = a.component_mul(&FP2::new(2.0, 2.0)).component_div(&bevel);
        let mut m = b.x.min(b.y);
        if a.x < round && a.y < round {
           m = (round - glm::length(&(FP2::new(round, round) - a))) * 2.0 / glm::dot(&bevel,&glm::normalize(&(FP2::new(round, round) - a)));
        }

        (m.clamp(0.0, 1.0), self.hash21(glm::floor(&u)))
    }

}