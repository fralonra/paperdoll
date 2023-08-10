use std::ops;

use serde::{Deserialize, Serialize};

/// A coordinate used in `paperdoll`.
///
/// The positive X-axis is rightward.
/// The positive Y-axis is downward.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Point {
    /// The distance from the Y-axis
    pub x: f32,
    /// The distance from the X-axis
    pub y: f32,
}

impl ops::Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

pub(crate) fn is_false(b: &bool) -> bool {
    !b
}

pub(crate) fn is_zero(u: &u32) -> bool {
    *u == 0
}
