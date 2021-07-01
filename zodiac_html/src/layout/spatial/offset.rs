use std::ops::Add;

use serde::*;
use super::distance::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutOffsetRect {
    top: LayoutDistance,
    right: LayoutDistance,
    bottom: LayoutDistance,
    left: LayoutDistance,
}

impl From<(LayoutDistance, LayoutDistance, LayoutDistance, LayoutDistance)> for LayoutOffsetRect {
    fn from(dimensions: (LayoutDistance, LayoutDistance, LayoutDistance, LayoutDistance)) -> Self {
        Self {
            top: dimensions.0,
            right: dimensions.1,        
            bottom: dimensions.2,
            left: dimensions.3,
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedLayoutOffsetRect {
    top: ResolvedLayoutDistance,
    right: ResolvedLayoutDistance,
    bottom: ResolvedLayoutDistance,
    left: ResolvedLayoutDistance,
}

impl From<LayoutOffsetRect> for ResolvedLayoutOffsetRect {
    fn from(rect: LayoutOffsetRect) -> Self {
        Self {
            top: ResolvedLayoutDistance::from(rect.top),
            right: ResolvedLayoutDistance::from(rect.right),
            bottom: ResolvedLayoutDistance::from(rect.bottom),
            left: ResolvedLayoutDistance::from(rect.left),
        }
    }
}

impl Add for ResolvedLayoutOffsetRect {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            top: self.top + rhs.top,
            right: self.right + rhs.right,
            bottom: self.bottom + rhs.bottom,
            left: self.left + rhs.left,
        }
    }
}

impl ResolvedLayoutOffsetRect {
    pub fn top(&self) -> ResolvedLayoutDistance {
        self.top
    }

    pub fn right(&self) -> ResolvedLayoutDistance {
        self.right
    }

    pub fn bottom(&self) -> ResolvedLayoutDistance {
        self.bottom
    }

    pub fn left(&self) -> ResolvedLayoutDistance {
        self.left
    }

    pub fn resolve_from_parent(&mut self, current: &LayoutOffsetRect, parent: &ResolvedLayoutOffsetRect) {
        self.top = current.top.resolve_from_parent(&self.top, &parent.top);
        self.right = current.right.resolve_from_parent(&self.right, &parent.right);
        self.bottom = current.bottom.resolve_from_parent(&self.bottom, &parent.bottom);
        self.left = current.left.resolve_from_parent(&self.left, &parent.left);
    }

    pub fn resolve_from_child(&mut self, current: &LayoutOffsetRect, child: &ResolvedLayoutOffsetRect) {
        self.top = current.top.resolve_from_child(&self.top, &child.top);
        self.right = current.right.resolve_from_child(&self.right, &child.right);
        self.bottom = current.bottom.resolve_from_child(&self.bottom, &child.bottom);
        self.left = current.left.resolve_from_child(&self.left, &child.left);
    }

    pub fn complete_children_resolution(&mut self, current: &LayoutOffsetRect) {
        self.top = current.top.complete_children_resolution(&self.top , ResolvedLayoutDistance::default());
        self.right = current.right.complete_children_resolution(&self.right, ResolvedLayoutDistance::default());
        self.bottom = current.bottom.complete_children_resolution(&self.bottom, ResolvedLayoutDistance::default());
        self.left = current.left.complete_children_resolution(&self.left, ResolvedLayoutDistance::default());
    }
}