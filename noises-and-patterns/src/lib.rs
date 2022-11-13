extern crate nalgebra_glm as glm;

pub type FP = f32;
pub type FP2 = glm::Vec2;
pub type FP3 = glm::Vec3;

// If f64 support is needed, change to

// pub type FP = f64;
// pub type FP2 = glm::DVec2;
// pub type FP3 = glm::DVec3;

pub mod noise;

pub mod pattern;

pub mod prelude {
    pub use crate::FP;
    pub use crate::FP2;
    pub use crate::FP3;

    pub use nalgebra::*;

    pub use crate::noise::Noise;
    pub use crate::pattern::Pattern;

    pub use crate::noise::value::Value;
    pub use crate::noise::voronoibasic::VoronoiBasic;

    pub use crate::pattern::bricks::Bricks;
}