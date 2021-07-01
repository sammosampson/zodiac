use serde::*;
use zodiac::*;
use super::direction::*;
use super::spatial::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StyleLayoutBox {
    direction: LayoutDirection,
    offset: LayoutOffsetRect,
    content_offset: LayoutOffsetRect,
    dimensions: LayoutDimensions
}

impl From<&Dimensions> for StyleLayoutBox {
    fn from(dimensions: &Dimensions) -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            offset: LayoutOffsetRect::default(),
            content_offset: LayoutOffsetRect::default(),
            dimensions: LayoutDimensions::from(dimensions)
        }
    }
}

impl zodiac::PropertySet<(LayoutDirection, LayoutOffsetRect, LayoutOffsetRect, LayoutDimensions)> for StyleLayoutBox {
    fn set(&mut self, to_set: (LayoutDirection, LayoutOffsetRect, LayoutOffsetRect, LayoutDimensions)) {
        self.direction = to_set.0;
        self.offset = to_set.1;
        self.content_offset = to_set.2;
        self.dimensions = to_set.3;
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutBox {
    direction: LayoutDirection,
    offset: LayoutOffsetRect,
    content_offset: LayoutOffsetRect,
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

        if self.content_offset != incumbent.content_offset {
            self.content_offset = incumbent.content_offset;
            has_changed = true;
        }

        if self.dimensions != incumbent.dimensions {
            self.dimensions = incumbent.dimensions;
            has_changed = true;
        }

        has_changed
    }
}

pub enum BoxArea {
    Margin,
    Border,
    Content
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedLayoutBox {
    direction: LayoutDirection,
    offset: ResolvedLayoutOffsetRect,
    content_offset: ResolvedLayoutOffsetRect,
    position: ResolvedLayoutPosition,
    dimensions: ResolvedLayoutDimensions,
}

impl ResolvedLayoutBox {
    pub fn resolve_from_parent(&mut self, current: &LayoutBox, parent: &ResolvedLayoutBox) {
        self.offset.resolve_from_parent(&current.offset, &parent.offset);
        self.content_offset.resolve_from_parent(&current.content_offset, &parent.content_offset);
        self.dimensions.resolve_from_parent(&current.dimensions, &parent.dimensions(BoxArea::Content));
    }

    pub fn resolve_from_child(&mut self, current: &LayoutBox, child: &ResolvedLayoutBox) {
        self.offset.resolve_from_child(&current.offset, &child.offset);
        self.content_offset.resolve_from_child(&current.content_offset, &child.content_offset);
        self.dimensions.resolve_from_child(&current.dimensions, &child.dimensions(BoxArea::Margin));
    }

    pub fn complete_children_resolution(&mut self, current: &LayoutBox) {
        self.offset.complete_children_resolution(&current.offset);
        self.content_offset.complete_children_resolution(&current.content_offset);
        self.dimensions.complete_children_resolution(&current.dimensions, self.offset + self.content_offset);
    }

    pub fn position_from_sibling(&mut self, sibling: &ResolvedLayoutBox) {
        match self.direction {
            LayoutDirection::Horizontal => self.position = sibling.position(BoxArea::Margin).add_width(sibling.dimensions(BoxArea::Margin).width()),
            LayoutDirection::Vertical => self.position = sibling.position(BoxArea::Margin).add_height(sibling.dimensions(BoxArea::Margin).height()),
            LayoutDirection::None => self.position = ResolvedLayoutPosition::default(),
        }
    }

    pub fn position_from_parent(&mut self, parent: &ResolvedLayoutBox) {
        self.position = parent.position(BoxArea::Content);
    }

    pub fn position(&self, area: BoxArea) -> ResolvedLayoutPosition {
        match area {
            BoxArea::Margin => self.position,
            BoxArea::Border => self.position + self.offset,
            BoxArea::Content => self.position + self.offset + self.content_offset
        }
    }

    pub fn dimensions(&self, area: BoxArea) -> ResolvedLayoutDimensions {
        match area {
            BoxArea::Margin => self.dimensions,
            BoxArea::Border => self.dimensions - self.offset,
            BoxArea::Content => self.dimensions - self.offset - self.content_offset
        }
    }
}

impl From<LayoutBox> for ResolvedLayoutBox {
    fn from(layout_box: LayoutBox) -> Self {
        Self {
            direction: layout_box.direction,
            offset: ResolvedLayoutOffsetRect::from(layout_box.offset),
            content_offset: ResolvedLayoutOffsetRect::from(layout_box.content_offset),
            dimensions: ResolvedLayoutDimensions::from(layout_box.dimensions),
            position: ResolvedLayoutPosition::default()
        }
    }
}