use serde::*;
use zodiac::*;
use super::direction::*;
use super::spatial::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StyleLayoutBox {
    direction: LayoutDirection,
    offset: LayoutOffsetRect,
    dimensions: LayoutDimensions
}

impl From<&Dimensions> for StyleLayoutBox {
    fn from(dimensions: &Dimensions) -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            offset: LayoutOffsetRect::default(),
            dimensions: LayoutDimensions::from(dimensions)
        }
    }
}

impl zodiac::PropertySet<(LayoutDirection, LayoutOffsetRect, LayoutDimensions)> for StyleLayoutBox {
    fn set(&mut self, to_set: (LayoutDirection, LayoutOffsetRect, LayoutDimensions)) {
        self.direction = to_set.0;
        self.offset = to_set.1;
        self.dimensions = to_set.2;
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutBox {
    direction: LayoutDirection,
    offset: LayoutOffsetRect,
    dimensions: LayoutDimensions
}

impl LayoutBox {
    pub fn apply(&mut self, incumbent: &StyleLayoutBox) -> bool {
        let mut has_changed = false;

        if self.direction != incumbent.direction {
            self.direction = incumbent.direction;
            has_changed = true;
        }

        if self.offset != incumbent.offset {
            self.offset = incumbent.offset;
            has_changed = true;
        }

        if self.dimensions != incumbent.dimensions {
            self.dimensions = incumbent.dimensions;
            has_changed = true;
        }

        has_changed
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedLayoutBox {
    direction: LayoutDirection,
    offset: ResolvedLayoutOffsetRect,
    position: ResolvedLayoutPosition,
    dimensions: ResolvedLayoutDimensions,
}

impl ResolvedLayoutBox {
    pub fn resolve_from_parent(&mut self, current: &LayoutBox, parent: &ResolvedLayoutBox) {
        self.offset.resolve_from_parent(&current.offset, &parent.offset);
        self.dimensions.resolve_from_parent(&current.dimensions, &parent.dimensions);
    }

    pub fn resolve_from_child(&mut self, current: &LayoutBox, child: &ResolvedLayoutBox) {
        self.offset.resolve_from_child(&current.offset, &child.offset);
        self.dimensions.resolve_from_child(&current.dimensions, &child.dimensions);
    }

    pub fn complete_children_resolution(&mut self, current: &LayoutBox) {
        self.offset.complete_children_resolution(&current.offset);
        self.dimensions.complete_children_resolution(&current.dimensions, &self.offset);
    }

    pub fn position_from_sibling(&mut self, sibling: &ResolvedLayoutBox) {
        match self.direction {
            LayoutDirection::Horizontal => self.position = sibling.position.add_width(sibling.dimensions.width()),
            LayoutDirection::Vertical => self.position = sibling.position.add_height(sibling.dimensions.height()),
            LayoutDirection::None => todo!(),
        }
    }

    pub fn position_from_parent(&mut self, parent: &ResolvedLayoutBox) {
        self.position = parent.content_position();
    }

    pub fn position(&self) -> ResolvedLayoutPosition {
        self.position
    }

    pub fn content_position(&self) -> ResolvedLayoutPosition {
        self.position + self.offset
    }

    pub fn dimensions(&self) -> ResolvedLayoutDimensions {
        self.dimensions
    }

    pub fn content_dimensions(&self) -> ResolvedLayoutDimensions {
        self.dimensions - self.offset
    }
}

impl From<LayoutBox> for ResolvedLayoutBox {
    fn from(layout_box: LayoutBox) -> Self {
        Self {
            direction: layout_box.direction,
            offset: ResolvedLayoutOffsetRect::from(layout_box.offset),
            dimensions: ResolvedLayoutDimensions::from(layout_box.dimensions),
            position: ResolvedLayoutPosition::default()
        }
    }
}