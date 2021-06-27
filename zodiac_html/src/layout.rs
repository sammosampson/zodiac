use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use serde::*;
use legion::*;
use legion::world::*;
use zodiac::*;
use crate::{dimensions::*, size::*};

pub fn layout_tree<'a>(world: &mut SubWorld, relationship_map: &'a RelationshipMap) -> LayoutTree<'a> {
    LayoutTree::<'a>::new(world, relationship_map)
}

pub struct LayoutTree<'a>(&'a RelationshipMap, HashMap::<Entity, RefCell<LayoutNode>>);

impl<'a> LayoutTree<'a> {
    fn new(world: &mut SubWorld, relationship_map: &'a RelationshipMap) -> Self {
        let layout_nodes: HashMap::<Entity, RefCell<LayoutNode>> = <(Entity, &LayoutBox, &ResolvedLayoutBox, Option<&LayoutRequest>)>::query()
            .iter(world)
            .map(|(entity, layout_box, resolved_layout_box, request)| 
                (*entity, RefCell::new(LayoutNode::new(*entity, *layout_box, *resolved_layout_box, request.into()))))
            .collect();

        Self(relationship_map, layout_nodes)
    }

    pub fn layout(&self, root: &Entity) {
        let mut root_node = self.1.get(root).unwrap().borrow_mut();
        root_node.layout(self);
    }  

    pub fn position(&self, root: &Entity) {
        let mut root_node = self.1.get(root).unwrap().borrow_mut();
        root_node.position(self);
    }

    fn get(&self, entity: &Entity) -> Option<&RefCell<LayoutNode>> {
        self.1.get(entity)
    }

    fn get_parent(&self, entity: &Entity) -> Option<&RefCell<LayoutNode>> {
        if let Some(parent)= self.relationship_map().get_parent(&entity) {
            return self.1.get(&parent)
        }
        None
    }
    
    fn get_previous_sibling(&self, entity: &Entity) -> Option<&RefCell<LayoutNode>> {
        if let Some(parent)= self.relationship_map().get_previous_sibling(&entity) {
            return self.1.get(&parent)
        }
        None
    }

    fn get_children(&'a self, entity: Entity) -> LayoutTreeChildrenIterator<'a> {
        LayoutTreeChildrenIterator::<'a>::new( &self, entity)
    }

    fn relationship_map(&self) -> &'a RelationshipMap {
        self.0
    }
}

pub struct LayoutTreeChildrenIterator<'a> {
    children: ChildrenRelationshipIterator<'a>,
    tree: &'a LayoutTree<'a>
}

impl<'a> LayoutTreeChildrenIterator<'a> {
    fn new(tree: &'a LayoutTree, parent: Entity) -> Self {
        Self {
            children: tree.relationship_map().get_children(&parent), 
            tree
        }
    }
}

impl <'a> Iterator for LayoutTreeChildrenIterator<'a> {
    type Item = (Entity, &'a RefCell<LayoutNode>);
    fn next(&mut self) -> Option<(Entity, &'a RefCell<LayoutNode>)> {
        if let Some(child) = self.children.next() {
            if let Some(layout_node) = self.tree.get(&child) {
                return Some((child, layout_node));
            }
        }
        None
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutStatus {
    Requested,
    Resolved,
    Resolving,
    Positioning,
    Positioned
}

impl Into<LayoutStatus> for Option<&LayoutRequest> {
    fn into(self) -> LayoutStatus {
        match self {
            None => LayoutStatus::Resolved,
            _ => LayoutStatus::Requested
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutNode {
    entity: Entity,
    layout_box: LayoutBox,
    resolved_layout_box: ResolvedLayoutBox, 
    status: LayoutStatus
}

impl LayoutNode {
    fn new(
        entity: Entity, 
        layout_box: LayoutBox, 
        resolved_layout_box: ResolvedLayoutBox, 
        status: LayoutStatus) -> Self {
        Self {
            entity,
            layout_box,
            resolved_layout_box,
            status
        }
    }

    fn layout<'a>(&mut self, tree: &LayoutTree<'a>) {
        if self.status == LayoutStatus::Requested {
            self.resolve_layout(tree);
            return;
        }

        for (_child, child_layout) in tree.get_children(self.entity) {
            child_layout.borrow_mut().layout(tree);
        }
    }

    fn resolve_layout<'a>(&mut self, tree: &LayoutTree<'a>) {
        self.status = LayoutStatus::Resolving;   
        self.resolved_layout_box = ResolvedLayoutBox::from(self.layout_box);

        if let Some(parent_layout) = tree.get_parent(&self.entity) {
            self.resolved_layout_box.resolve_from_parent(&self.layout_box, &parent_layout.borrow().resolved_layout_box);
        }
                
        for (_child, child_layout) in tree.get_children(self.entity) {
            child_layout.borrow_mut().resolve_layout(tree);
            self.resolved_layout_box.resolve_from_child(&self.layout_box, &child_layout.borrow().resolved_layout_box);
        }

        self.status = LayoutStatus::Resolved;   
    }

    fn position<'a>(&mut self, tree: &LayoutTree<'a>) {
        if self.status == LayoutStatus::Resolved {
            self.resolve_position(tree);
            return;
        }

        for (_child, child_layout) in tree.get_children(self.entity) {
            child_layout.borrow_mut().position(tree);
        }
    }

    fn resolve_position<'a>(&mut self, tree: &LayoutTree<'a>) {
        self.status = LayoutStatus::Positioning;   

        if let Some(previous_sibling_layout) = tree.get_previous_sibling(&self.entity) {
            self.resolved_layout_box.position_from_sibling(&previous_sibling_layout.borrow().resolved_layout_box);
        } else if let Some(parent_layout) = tree.get_parent(&self.entity) {
            self.resolved_layout_box.position_from_parent(&parent_layout.borrow().resolved_layout_box);
        } 

        for (_child, child_layout) in tree.get_children(self.entity) {
            child_layout.borrow_mut().resolve_position(tree);
        }
        
        self.status = LayoutStatus::Positioned;   
        
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LayoutRequest {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LayoutChange {
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

impl LayoutDistance {
    fn resolve_from_parent(&self, current: &ResolvedLayoutDistance, parent: &ResolvedLayoutDistance) -> ResolvedLayoutDistance {
        if current == &ResolvedLayoutDistance::Unresolved {
            if let Self::FromParent(multiplier) = self {
                if let ResolvedLayoutDistance::Resolved(parent_fixed_distance) = parent {
                    return ResolvedLayoutDistance::Resolved((*parent_fixed_distance as f32 * multiplier) as u16);
                }
            }
        }

        *current
    }

    fn resolve_from_child(&self, current: &ResolvedLayoutDistance, _child: &ResolvedLayoutDistance) -> ResolvedLayoutDistance {
        if current == &ResolvedLayoutDistance::Unresolved {
            if let Self::FromChildren(_multiplier) = self {
                return ResolvedLayoutDistance::Resolved(0);
            }
        }

        *current
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
            height: LayoutDistance::from(dimensions.height)
        }
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
    pub fn apply(&mut self, incumbent: &IncumbentLayoutBox) -> bool {
        let has_changed = false;

        if self.direction != incumbent.direction {
            self.direction = incumbent.direction;
        }

        if self.offset != incumbent.offset {
            self.offset = incumbent.offset;
        }

        if self.dimensions != incumbent.dimensions {
            self.dimensions = incumbent.dimensions;
        }

        has_changed
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResolvedLayoutDistance {
    Unresolved,
    Resolved(u16)
}

impl Default for ResolvedLayoutDistance {
    fn default() -> Self {
        Self::Unresolved
    }
}

impl Add for ResolvedLayoutDistance {
    type Output = ResolvedLayoutDistance;

    fn add(self, rhs: Self) -> Self::Output {
        if let ResolvedLayoutDistance::Resolved(distance) = self {
            if let ResolvedLayoutDistance::Resolved(rhs_distance) = rhs {
                return ResolvedLayoutDistance::Resolved(distance + rhs_distance);
            }   
        } 
        ResolvedLayoutDistance::Unresolved
    }
}

impl Add<u16> for ResolvedLayoutDistance {
    type Output = u16;

    fn add(self, rhs: u16) -> Self::Output {
        if let ResolvedLayoutDistance::Resolved(distance) = self {
            return distance + rhs;
        } 
        0
    }
}

impl Sub for ResolvedLayoutDistance {
    type Output = ResolvedLayoutDistance;

    fn sub(self, rhs: Self) -> Self::Output {
        if let ResolvedLayoutDistance::Resolved(distance) = self {
            if let ResolvedLayoutDistance::Resolved(rhs_distance) = rhs {
                return ResolvedLayoutDistance::Resolved(distance - rhs_distance);
            }   
        } 
        ResolvedLayoutDistance::Unresolved
    }
}

impl From<LayoutDistance> for ResolvedLayoutDistance {
    fn from(distance: LayoutDistance) -> Self {
        match distance {
            LayoutDistance::Fixed(distance) => Self::Resolved(distance),
            _ => Self::Unresolved
        }
    }
}

impl Into<u16> for ResolvedLayoutDistance {
    fn into(self) -> u16 {
        match self {
            ResolvedLayoutDistance::Resolved(distance) => distance,
            _ => 0
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

impl ResolvedLayoutOffsetRect {
    fn resolve_from_parent(&mut self, current: &LayoutOffsetRect, parent: &ResolvedLayoutOffsetRect) {
        self.top = current.top.resolve_from_parent(&self.top, &parent.top);
        self.right = current.right.resolve_from_parent(&self.right, &parent.right);
        self.bottom = current.bottom.resolve_from_parent(&self.bottom, &parent.bottom);
        self.left = current.left.resolve_from_parent(&self.left, &parent.left);
    }

    fn resolve_from_child(&mut self, current: &LayoutOffsetRect, child: &ResolvedLayoutOffsetRect) {
        self.top = current.top.resolve_from_child(&self.top, &child.top);
        self.right = current.right.resolve_from_child(&self.right, &child.right);
        self.bottom = current.bottom.resolve_from_child(&self.bottom, &child.bottom);
        self.left = current.left.resolve_from_child(&self.left, &child.left);
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedLayoutDimensions {
    width: ResolvedLayoutDistance,
    height: ResolvedLayoutDistance
}

impl Sub<ResolvedLayoutOffsetRect> for ResolvedLayoutDimensions {
    type Output = ResolvedLayoutDimensions;

    fn sub(self, rhs: ResolvedLayoutOffsetRect) -> Self::Output {
        Self {
            width: self.width - (rhs.left + rhs.right),
            height: self.width - (rhs.top + rhs.bottom),
        }
    }
}

impl Into<(u16, u16)> for ResolvedLayoutDimensions {
    fn into(self) -> (u16, u16) {
        (self.width.into(), self.height.into())
    }
}

impl ResolvedLayoutDimensions {
    fn resolve_from_parent(&mut self, current: &LayoutDimensions, parent: &ResolvedLayoutDimensions) {
        self.width = current.width.resolve_from_parent(&self.width, &parent.width);
        self.height = current.height.resolve_from_parent(&self.height, &parent.height);
    }

    fn resolve_from_child(&mut self, current: &LayoutDimensions, child: &ResolvedLayoutDimensions) {
        self.width = current.width.resolve_from_child(&self.width, &child.width);
        self.height = current.height.resolve_from_child(&self.height, &child.height);
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
#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolvedLayoutPosition {
    left: u16,
    top: u16
}

impl Add<ResolvedLayoutOffsetRect> for ResolvedLayoutPosition {
    type Output = ResolvedLayoutPosition;

    fn add(self, rhs: ResolvedLayoutOffsetRect) -> Self::Output {
        Self {
            left: rhs.left + self.left,
            top: rhs.top + self.top
        }
    }
}

impl Add<ResolvedLayoutDimensions> for ResolvedLayoutPosition {
    type Output = ResolvedLayoutPosition;

    fn add(self, rhs: ResolvedLayoutDimensions) -> Self::Output {
        Self {
            left: rhs.width + self.left,
            top: rhs.height + self.top
        }
    }
}

impl Into<(u16, u16)> for ResolvedLayoutPosition {
    fn into(self) -> (u16, u16) {
        (self.left, self.top)
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
    fn resolve_from_parent(&mut self, current: &LayoutBox, parent: &ResolvedLayoutBox) {
        self.offset.resolve_from_parent(&current.offset, &parent.offset);
        self.dimensions.resolve_from_parent(&current.dimensions, &parent.dimensions);
    }

    fn resolve_from_child(&mut self, current: &LayoutBox, child: &ResolvedLayoutBox) {
        self.offset.resolve_from_child(&current.offset, &child.offset);
        self.dimensions.resolve_from_child(&current.dimensions, &child.dimensions);
    }

    fn position_from_sibling(&mut self, sibling: &ResolvedLayoutBox) {
        self.position = sibling.position + sibling.dimensions;
    }

    fn position_from_parent(&mut self, parent: &ResolvedLayoutBox) {
        self.position = parent.content_position();
    }

    pub fn content_position(&self) -> ResolvedLayoutPosition {
        self.position + self.offset
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