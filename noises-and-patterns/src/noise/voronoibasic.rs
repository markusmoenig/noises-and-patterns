use crate::prelude::*;

// Based on https://www.shadertoy.com/view/MslGD8

pub struct VoronoiBasic {}

impl Noise for VoronoiBasic {

    fn new() -> Self {
        Self {

        }
    }

    fn get_2d(&self, p: (FP, FP)) -> FP {

        /// 2D hash, 2 out
        fn hash22(p: FP2 ) -> FP2 {
            let pp = FP2::new(glm::dot(&p, &FP2::new(127.1, 311.7)),
                    glm::dot(&p, &FP2::new(269.5,183.3)));
            glm::fract(&glm::sin(&pp).component_mul( &FP2::new(18.5453, 18.5453 )))
        }

        let x = FP2::new(p.0, p.1);
        let n = glm::floor(&x);
        let f = glm::fract(&x);

        let mut m = FP3::new( 8.0, 8.0, 8.0 );
        for j in -1..=1 {
            for i in -1..=1 {
                let  g = FP2::new( i as FP, j as FP );
                let  o = hash22( n + g );

                let r = g - f + (FP2::new(0.5, 0.5) + 0.5 * glm::sin(&(6.2831*o)));
                let d = glm::dot( &r, &r );
                if d < m.x {
                    m = FP3::new( d, o.x, o.y );
                }
            }
        }

        0.5 + 0.5 * (m.y + m.z)
    }


}