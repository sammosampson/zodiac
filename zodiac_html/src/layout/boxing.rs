use serde::*;
use zodiac::*;
use super::direction::*;
use super::spatial::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StyleLayoutBox {
    direction: LayoutDirection,
    margin: LayoutOffsetRect,
    border: LayoutOffsetRect,
    content: LayoutOffsetRect,
    dimensions: LayoutDimensions
}

impl From<&Dimensions> for StyleLayoutBox {
    fn from(dimensions: &Dimensions) -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            margin: LayoutOffsetRect::default(),
            border: LayoutOffsetRect::default(),
            content: LayoutOffsetRect::default(),
            dimensions: LayoutDimensions::from(dimensions)
        }
    }
}

impl zodiac::PropertySet<(LayoutDirection, LayoutOffsetRect, LayoutOffsetRect,  LayoutOffsetRect, LayoutDimensions)> for StyleLayoutBox {
    fn set(&mut self, to_set: (LayoutDirection, LayoutOffsetRect, LayoutOffsetRect,  LayoutOffsetRect, LayoutDimensions)) {
        self.direction = to_set.0;
        self.margin = to_set.1;
        self.border = to_set.2;
        self.content = to_set.3;
        self.dimensions = to_set.4;
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutBox {
    direction: LayoutDirection,
    margin: LayoutOffsetRect,
    border: LayoutOffsetRect,
    content: LayoutOffsetRect,
    dimensions: LayoutDimensions
}

impl LayoutBox {
    pub fn apply(&mut self, incumbent: &StyleLayoutBox) -> bool {
        let mut has_changed = false;

        if self.direction != incumbent.direction {
            self.direction = incumbent.direction;
            has_changed = true;
        }

        if self.margin != incumbent.margin {
            self.margin = incumbent.margin;
            has_changed = true;
        }

        if self.border != incumbent.border {
            self.border = incumbent.border;
            has_changed = true;
        }

        if self.content != incumbent.content {
            self.content = incumbent.content;
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
    margin: ResolvedLayoutOffsetRect,
    border: ResolvedLayoutOffsetRect,
    content: ResolvedLayoutOffsetRect,
    position: ResolvedLayoutPosition,
    dimensions: ResolvedLayoutDimensions,
}

impl ResolvedLayoutBox {
    pub fn resolve_from_parent(&mut self, current: &LayoutBox, parent: &ResolvedLayoutBox) {
        self.margin.resolve_from_parent(&current.margin, &parent.margin);
        self.content.resolve_from_parent(&current.content, &parent.content);
        self.dimensions.resolve_from_parent(&current.dimensions, &parent.dimensions(BoxArea::Content));
    }

    pub fn resolve_from_child(&mut self, current: &LayoutBox, child: &ResolvedLayoutBox) {
        self.margin.resolve_from_child(&current.margin, &child.margin);
        self.content.resolve_from_child(&current.content, &child.content);
        self.dimensions.resolve_from_child(&current.dimensions, &child.dimensions(BoxArea::Margin));
    }

    pub fn complete_children_resolution(&mut self, current: &LayoutBox) {
        self.margin.complete_children_resolution(&current.margin);
        self.content.complete_children_resolution(&current.content);
        self.dimensions.complete_children_resolution(&current.dimensions, self.margin + self.border + self.content);
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
            BoxArea::Border => self.position + self.margin,
            BoxArea::Content => self.position + self.margin + self.border + self.content
        }
    }

    pub fn dimensions(&self, area: BoxArea) -> ResolvedLayoutDimensions {
        match area {
            BoxArea::Margin => self.dimensions,
            BoxArea::Border => self.dimensions - self.margin,
            BoxArea::Content => self.dimensions - self.margin - self.border - self.content
        }
    }
}

impl From<LayoutBox> for ResolvedLayoutBox {
    fn from(layout_box: LayoutBox) -> Self {
        Self {
            direction: layout_box.direction,
            margin: ResolvedLayoutOffsetRect::from(layout_box.margin),
            border: ResolvedLayoutOffsetRect::from(layout_box.border),
            content: ResolvedLayoutOffsetRect::from(layout_box.content),
            dimensions: ResolvedLayoutDimensions::from(layout_box.dimensions),
            position: ResolvedLayoutPosition::default()
        }
    }
}