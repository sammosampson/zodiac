use std::collections::HashMap;
use serde::*;
use legion::*;
use legion::world::*;
use zodiac::*;
use crate::{dimensions::*, size::*};

pub fn layout_box_tree<'a>(world: &mut SubWorld, relationship_map: &'a RelationshipMap) -> LayoutBoxTree<'a> {
    LayoutBoxTree::<'a>::new(world, relationship_map)
}

pub struct LayoutBoxTree<'a>(&'a RelationshipMap, HashMap::<Entity, LayoutBox>);

impl<'a> LayoutBoxTree<'a> {
    fn new(world: &mut SubWorld, relationship_map: &'a RelationshipMap) -> Self {
        let layout_boxes: HashMap::<Entity, LayoutBox> = <(Entity, &LayoutBox)>::query()
            .iter(world)
            .map(|(entity, layout_box)| (*entity, *layout_box))
            .collect();

        Self(relationship_map, layout_boxes)
    }

    pub fn get_children(&'a self, entity: Entity) -> LayoutBoxTreeChildrenIterator<'a> {
        LayoutBoxTreeChildrenIterator::<'a>::new( &self, entity)
    }

    fn relationship_map(&self) -> &'a RelationshipMap {
        self.0
    }

    fn get_layout_box(&self, entity: &Entity) -> Option<&LayoutBox> {
        self.1.get(entity)
    } 
}

pub struct LayoutBoxTreeChildrenIterator<'a> {
    children: ChildrenRelationshipIterator<'a>,
    tree: &'a LayoutBoxTree<'a>
}

impl<'a> LayoutBoxTreeChildrenIterator<'a> {
    fn new(tree: &'a LayoutBoxTree, parent: Entity) -> Self {
        Self {
            children: tree.relationship_map().get_children(&parent), 
            tree
        }
    }
}

impl <'a> Iterator for LayoutBoxTreeChildrenIterator<'a> {
    type Item = (Entity, LayoutBox);
    fn next(&mut self) -> Option<(Entity, LayoutBox)> {
        if let Some(child) = self.children.next() {
            if let Some(layout_box) = self.tree.get_layout_box(&child) {
                return Some((child, *layout_box));
            }
        }
        None
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Layout {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}


#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LayoutRequest {
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutDirection {
    None,
    Horizontal,
    Vertical
}

impl Default for LayoutDirection {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutDistance {
    None,
    FromParent(f32),
    FromChildren(f32),
    Fixed(u16)
}

impl Default for LayoutDistance {
    fn default() -> Self {
        Self::None
    }
}

impl From<u16> for LayoutDistance {
    fn from(distance: u16) -> Self {
        Self::Fixed(distance)
    }
}

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
            height: LayoutDistance::from(dimensions.height),        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncumbentLayoutBox {
    direction: LayoutDirection,
    offset: LayoutOffsetRect,
    dimensions: LayoutDimensions
}

impl From<&Dimensions> for IncumbentLayoutBox {
    fn from(dimensions: &Dimensions) -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            offset: LayoutOffsetRect::default(),
            dimensions: LayoutDimensions::from(dimensions)
        }
    }
}

impl zodiac::PropertySet<(LayoutDirection, LayoutOffsetRect, LayoutDimensions)> for IncumbentLayoutBox {
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
    pub fn apply(&self, _incumbent: &IncumbentLayoutBox) -> bool {
        todo!()
        // this will merrge in the changes from the incumbent and return true if anything differed
    }
}

pub struct LayoutNode {
}

impl From<&LayoutBox> for LayoutNode {
    fn from(_layout_box: &LayoutBox) -> Self {
        todo!()
        //suck in box constraints to use in layout impl
    }
}

impl LayoutNode {
    pub fn apply_parent_layout(self, _parent: &LayoutNode) -> Self {        
        todo!()
        // this should push constraints down
    }

    pub fn apply_child_layout(&self, _child: &LayoutNode) {
        todo!()
        // this should allow resizing based on child sizes
    }

    pub fn layout(&self) -> Layout {
        todo!()
        // this should deduce the layout given full parent and children layouts applied
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Span {
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Div {
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum DisplayTypes {
    None,
    Inline,
    Block
}

impl Default for DisplayTypes {
    fn default() -> Self {
        DisplayTypes::None
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Display(DisplayTypes);

impl Display {
    pub fn inline() -> Self {
        Self(DisplayTypes::Inline)
    }

    pub fn block() -> Self {
        Self(DisplayTypes::Block)
    }
}

impl From<DisplayTypes> for Display {
    fn from(types: DisplayTypes) -> Self {
        Self(types)
    }
}

impl Into<DisplayTypes> for &Display {
    fn into(self) -> DisplayTypes {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum MarginSize {
    None,
    Inherit,
    Auto,
    Specific(Size)
}

impl Default for MarginSize {
    fn default() -> Self {
        Self::None
    }
}

impl Into<Size> for MarginSize {
    fn into(self) -> Size {
        match self {
            MarginSize::Specific(size) => size,
            _ => Size::default()
        }
    }
}


impl Into<LayoutDistance> for MarginSize {
    fn into(self) -> LayoutDistance {
        match self {
            MarginSize::Specific(size) => LayoutDistance::Fixed(size.into()),
            _ => LayoutDistance::default()
        }
    }
}

impl Into<MarginSize> for Size {
    fn into(self) -> MarginSize {
        MarginSize::Specific(self)
    }
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct MarginSizes(MarginSize, MarginSize, MarginSize, MarginSize);

impl Into<Dimensions> for MarginSizes {
    fn into(self) -> Dimensions {
        WrappedDimensions::from((self.0.into(), self.1.into(), self.2.into(), self.3.into())).into()
    }
}

impl Into<LayoutOffsetRect> for MarginSizes {
    fn into(self) -> LayoutOffsetRect {
        LayoutOffsetRect::from((self.0.into(), self.1.into(), self.2.into(), self.3.into()))
    }
}

impl Into<MarginSizes> for (MarginSize, MarginSize, MarginSize, MarginSize) {
    fn into(self) -> MarginSizes {
        MarginSizes(self.0, self.1, self.2, self.3)
    }
}

impl Into<MarginSizes> for (MarginSize, MarginSize, MarginSize) {
    fn into(self) -> MarginSizes {
        MarginSizes(self.0, self.1, self.2, self.1)
    }
}

impl Into<MarginSizes> for (MarginSize, MarginSize) {
    fn into(self) -> MarginSizes {
        MarginSizes(self.0, self.1, self.0, self.1)
    }
}

impl Into<MarginSizes> for MarginSize {
    fn into(self) -> MarginSizes {
        MarginSizes(self, self, self, self)
    }
}

impl Into<MarginSizes> for Size {
    fn into(self) -> MarginSizes {
        MarginSizes(self.into(), self.into(), self.into(), self.into())
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Margin(MarginSizes, bool);

impl zodiac::PropertySetCheck for Margin {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl From<MarginSizes> for Margin {
    fn from(values: MarginSizes) -> Self {
        Self(values, true)
    }
}

impl Into<Dimensions> for &Margin {
    fn into(self) -> Dimensions {
        self.0.into()
    }
}

impl Into<LayoutOffsetRect> for &Margin {
    fn into(self) -> LayoutOffsetRect {
        self.0.into()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum PaddingSize {
    None,
    Inherit,
    Specific(Size)
}

impl Default for PaddingSize {
    fn default() -> Self {
        Self::None
    }
}

impl Into<PaddingSize> for Size {
    fn into(self) -> PaddingSize {
        PaddingSize::Specific(self)
    }
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct PaddingSizes(PaddingSize, PaddingSize, PaddingSize, PaddingSize);

impl Into<PaddingSizes> for (PaddingSize, PaddingSize, PaddingSize, PaddingSize) {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self.0, self.1, self.2, self.3)
    }
}

impl Into<PaddingSizes> for (PaddingSize, PaddingSize, PaddingSize) {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self.0, self.1, self.2, self.1)
    }
}

impl Into<PaddingSizes> for (PaddingSize, PaddingSize) {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self.0, self.1, self.0, self.1)
    }
}

impl Into<PaddingSizes> for PaddingSize {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self, self, self, self)
    }
}

impl Into<PaddingSizes> for Size {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self.into(), self.into(), self.into(), self.into())
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Padding(PaddingSizes, bool);

impl zodiac::PropertySetCheck for Padding {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl From<PaddingSizes> for Padding {
    fn from(values: PaddingSizes) -> Self {
        Self(values, true)
    }
}