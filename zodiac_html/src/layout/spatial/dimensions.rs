use std::ops::Sub;
use serde::*;
use zodiac::Dimensions;
use super::distance::*;
use super::offset::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutDimensions {
    width: LayoutDistance,
    height: LayoutDistance
}

impl LayoutDimensions {
    pub fn new(width: LayoutDistance, height: LayoutDistance) -> Self  {
        Self { width, height }
    }
}

impl From<&Dimensions> for LayoutDimensions {
    fn from(dimensions: &Dimensions) -> Self {
        Self {
            width: LayoutDistance::from(dimensions.width),
            height: LayoutDistance::from(dimensions.height)
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedLayoutDimensions {
    width: ResolvedLayoutDistance,
    height: ResolvedLayoutDistance
}

impl ResolvedLayoutDimensions {
    pub fn width(&self) -> ResolvedLayoutDistance {
        self.width
    }

    pub fn height(&self) -> ResolvedLayoutDistance {
        self.height
    }
}

impl Sub<ResolvedLayoutOffsetRect> for ResolvedLayoutDimensions {
    type Output = ResolvedLayoutDimensions;

    fn sub(self, rhs: ResolvedLayoutOffsetRect) -> Self::Output {
        Self {
            width: self.width - (rhs.left() + rhs.right()),
            height: self.height - (rhs.top() + rhs.bottom()),
        }
    }
}

impl Into<(u16, u16)> for ResolvedLayoutDimensions {
    fn into(self) -> (u16, u16) {
        (self.width.into(), self.height.into())
    }
} 

impl ResolvedLayoutDimensions {
    pub fn resolve_from_parent(&mut self, current: &LayoutDimensions, parent: &ResolvedLayoutDimensions) {
        self.width = current.width.resolve_from_parent(&self.width, &parent.width);
        self.height = current.height.resolve_from_parent(&self.height, &parent.height);
    }

    pub fn resolve_from_child(&mut self, current: &LayoutDimensions, child: &ResolvedLayoutDimensions) {
        self.width = current.width.resolve_from_child(&self.width, &child.width);
        self.height = current.height.resolve_from_child(&self.height, &child.height);
    }

    pub fn complete_children_resolution(&mut self, current: &LayoutDimensions, resolved_offset: ResolvedLayoutOffsetRect) {
        self.width = current.width.complete_children_resolution(&self.width, resolved_offset.left() + resolved_offset.right());
        self.height = current.height.complete_children_resolution(&self.height, resolved_offset.top() + resolved_offset.bottom());
    }

}

impl From<LayoutDimensions> for ResolvedLayoutDimensions {
    fn from(dimensions: LayoutDimensions) -> Self {
        Self {
            width: ResolvedLayoutDistance::from(dimensions.width),
            height: ResolvedLayoutDistance::from(dimensions.height)
        }
    }
}
