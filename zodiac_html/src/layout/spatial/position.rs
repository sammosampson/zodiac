use serde::*;
use std::ops::Add;
use super::distance::*;
use super::offset::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedLayoutPosition {
    left: u16,
    top: u16
}

impl ResolvedLayoutPosition {
    pub fn add_width(self, width: ResolvedLayoutDistance) -> ResolvedLayoutPosition {
        Self {
            left: width + self.left,
            top: self.top
        }
    }

    pub fn add_height(self, height: ResolvedLayoutDistance) -> ResolvedLayoutPosition {
        Self {
            left: self.left,
            top: height + self.top
        }
    }
}

impl Add<ResolvedLayoutOffsetRect> for ResolvedLayoutPosition {
    type Output = ResolvedLayoutPosition;

    fn add(self, rhs: ResolvedLayoutOffsetRect) -> Self::Output {
        Self {
            left: rhs.left() + self.left,
            top: rhs.top() + self.top
        }
    }
}

impl Into<(u16, u16)> for ResolvedLayoutPosition {
    fn into(self) -> (u16, u16) {
        (self.left, self.top)
    }
}