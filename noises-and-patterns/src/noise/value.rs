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

	    let a = self.hash21(i);
        let b = self.hash21(i + FP2::new(1.0, 0.0));
        let c = self.hash21(i + FP2::new(0.0, 1.0));
        let d = self.hash21(i + FP2::new(1.0, 1.0));

        let u = FP2::new( f.x * f.x * (3.0 - 2.0 * f.x), f.y * f.y * (3.0 - 2.0 * f.y));

        let xx = self.mix(&a, &b, &u.x);
        let yy = self.mix(&c, &d, &u.x);

        self.mix(&xx, &yy, &u.y)
    }

}