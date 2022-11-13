use crate::prelude::*;

pub mod bricks;

pub trait Pattern {

    fn new() -> Self;

    /// 2D pattern for the given position and display ratio (width / height)
    fn pattern_2d(&self, p: (FP, FP)) -> FP;

    /// For setting pattern properties
    fn set_property(&mut self, name: &str, value: FP);
}
