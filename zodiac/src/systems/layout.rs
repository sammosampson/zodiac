use std::ops::Add;
use std::collections::{ HashMap };
use legion::*;
use legion::world::*;
use legion::systems::*;
use crate::systems::relationships::*;
use crate::systems::measurement::*;
use zodiac_entities::components::*;

pub type LeftOffsetMap = HashMap<Entity, Left>;

pub fn create_left_offset_map() -> LeftOffsetMap {
    LeftOffsetMap::new()
}

pub type TopOffsetMap = HashMap<Entity, Top>;

pub fn create_top_offset_map() -> TopOffsetMap {
    TopOffsetMap::new()
}

pub type LayoutTypeMap = HashMap<Entity, LayoutType>;

pub fn create_layout_type_map() -> LayoutTypeMap {
    LayoutTypeMap::new()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct LayoutConstraints {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}

impl LayoutConstraints {
    fn into_width_subdivider(&self) -> LayoutConstraintsWidthSubDivider {
        LayoutConstraintsWidthSubDivider::from_constraints(*self)
    }
}

impl Add<Left> for  LayoutConstraints {
    type Output = Self;
    fn add(self, other: Left) -> Self {
        LayoutConstraints {
            left: self.left + other.left,
            top: self.top,
            width: self.width,
            height: self.height
        }
    }
}

impl Add<Top> for LayoutConstraints {
    type Output = Self;
    fn add(self, other: Top) -> Self {
        LayoutConstraints {
            left: self.left,
            top: self.top + other.top,
            width: self.width,
            height: self.height
        }
    }
}

impl From<&ResizeRequest> for LayoutConstraints {
    fn from(request: &ResizeRequest) -> Self {
        LayoutConstraints {
            left: request.left,
            top: request.top,
            width: request.width,
            height: request.height
        }
    }
}

enum LayoutConstraintsSubDivisionType {
    SizeNotSpecified(Entity),
    FixedSizeSpecified(Entity, u16)
}

struct LayoutConstraintsWidthSubDivider {
    from_constraints: LayoutConstraints,
    subdivisions: Vec::<LayoutConstraintsSubDivisionType>,
    total_fixed_width: u16,
    total_no_specified_width_columns: usize
}

impl LayoutConstraintsWidthSubDivider {
    fn from_constraints(constraints: LayoutConstraints) -> Self {
        LayoutConstraintsWidthSubDivider {
            from_constraints: constraints,
            subdivisions: vec!(),
            total_fixed_width: 0, 
            total_no_specified_width_columns: 0
        }
    }

    fn subdivide_for_entity(&mut self, entity: &Entity, minimum_width_map: &MinimumWidthMap) {
        match minimum_width_map.get(entity) {
            Some(width) => {
                self.subdivisions.push(LayoutConstraintsSubDivisionType::FixedSizeSpecified(*entity, width.width));
                self.total_fixed_width += width.width;
            },
            None => {
                self.subdivisions.push(LayoutConstraintsSubDivisionType::SizeNotSpecified(*entity));
                self.total_no_specified_width_columns += 1;
            }
        }
    }

    fn width_slice(&self, culmative_width: u16) -> LayoutConstraints {
        self.fixed_width_slice(
            culmative_width, 
            (self.from_constraints.width - self.total_fixed_width) / self.total_no_specified_width_columns as u16)
    }
    
    fn fixed_width_slice(&self, culmative_width: u16, width: u16) -> LayoutConstraints {
        LayoutConstraints {
            left: self.from_constraints.left + culmative_width,
            top: self.from_constraints.top,
            width,
            height: self.from_constraints.height
        }
    }

    fn iter(&self) -> LayoutConstraintsWidthSubDividerIterator {
        LayoutConstraintsWidthSubDividerIterator {
            subdivider: self,
            current_index: 0,
            culmative_width: 0
        }
    }
}

struct LayoutConstraintsWidthSubDividerIterator<'a> {
    subdivider: &'a LayoutConstraintsWidthSubDivider,
    current_index: usize,
    culmative_width: u16
}

impl<'a> Iterator for LayoutConstraintsWidthSubDividerIterator<'a> {
    type Item = (Entity, LayoutConstraints);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(subdivision_type) = self.subdivider.subdivisions.get(self.current_index) {
            let result = match subdivision_type {
                LayoutConstraintsSubDivisionType::SizeNotSpecified(entity) => {
                    let new_constraints = self.subdivider.width_slice(self.culmative_width);
                    self.culmative_width += new_constraints.width;
                    Some((*entity, new_constraints))
                },
                LayoutConstraintsSubDivisionType::FixedSizeSpecified(entity, width) => {
                    let new_constraints = self.subdivider.fixed_width_slice(self.culmative_width, *width);
                    self.culmative_width += width;
                    Some((*entity, new_constraints))
                }
            };
            self.current_index += 1;
            result
        }
        else {
            None
        }
    }
}

impl From<&LayoutConstraints> for LayoutChange {
    fn from(constraints: &LayoutConstraints) -> Self {
        LayoutChange {
            left: constraints.left,
            top: constraints.top,
            width: constraints.width,
            height: constraints.height
        }
    }
}

struct LayoutMaps<'a> {
    relationship_map: &'a RelationshipMap,
    layout_map: &'a LayoutTypeMap,
    left_map: &'a LeftOffsetMap,
    top_map: &'a TopOffsetMap,
    width_map: &'a WidthMap,
    minimum_width_map: &'a MinimumWidthMap,
    height_map: &'a HeightMap,
    minimum_height_map: &'a MinimumHeightMap
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_layout_type_map(
    #[resource] layout_map: &mut LayoutTypeMap,
    entity: &Entity,
    layout: &LayoutContent) {
        layout_map.insert(*entity, layout.layout_type);
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_left_offset_map(
    #[resource] offset_map: &mut LeftOffsetMap,
    entity: &Entity,
    offset: &Left) {
        offset_map.insert(*entity, *offset);
}

#[system(for_each)]
#[filter(!component::<Mapped>())]
pub fn build_top_offset_map(
    #[resource] offset_map: &mut TopOffsetMap,
    entity: &Entity,
    offset: &Top) {
        offset_map.insert(*entity, *offset);
}

#[system(simple)]
#[read_component(RootWindowResized)]
#[read_component(Root)]
#[write_component(ResizeRequest)]
pub fn resize_screen(world: &mut SubWorld, command_buffer: &mut CommandBuffer) {
    for (entity, window_resized) in <(Entity, &RootWindowResized)>::query()
        .iter(world) {
            for root in <Entity>::query()
                .filter(component::<Root>())
                .iter(world) {
                    command_buffer.add_component(*root, ResizeRequest::from(window_resized));
            }
            command_buffer.remove(*entity);
    } 
}

#[system(for_each)]
pub fn resize(
    #[resource] relationship_map: &RelationshipMap,
    #[resource] layout_map: &LayoutTypeMap,
    #[resource] left_map: &LeftOffsetMap,
    #[resource] top_map: &TopOffsetMap,
    #[resource] width_map: &WidthMap,
    #[resource] minimum_width_map: &MinimumWidthMap,
    #[resource] height_map: &HeightMap,
    #[resource] minimum_height_map: &MinimumHeightMap,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    resize_request: &ResizeRequest) {
        perform_resize(
            &LayoutMaps {
                relationship_map, 
                layout_map, 
                left_map,
                top_map,
                width_map, 
                minimum_width_map,
                height_map,
                minimum_height_map,
            },
            world,
            command_buffer,
            entity,
            &LayoutConstraints::from(resize_request));
}

fn perform_resize(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        command_buffer.add_component(*entity, Width { width: constraints.width });
        command_buffer.add_component(*entity, Height { height: constraints.height });
        command_buffer.add_component(*entity, Resized {});
        command_buffer.remove_component::<ResizeRequest>(*entity);
        perform_layout(maps, world, command_buffer, entity, constraints);
}

fn perform_layout(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        if let Some(layout_type) = maps.layout_map.get(entity) {
            match layout_type {
                LayoutType::Canvas => layout_canvas(maps, world, command_buffer, entity, constraints),
                LayoutType::Horizontal => layout_horizontal(maps, world, command_buffer, entity, constraints)
            }
        } else {
            layout_renderable(maps, command_buffer, entity, constraints);
        }
}

fn layout_canvas(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        let mut new_constraints = *constraints;
        if let Some(left) = maps.left_map.get(entity) {
            new_constraints = new_constraints + *left;
        }
        if let Some(top) = maps.top_map.get(entity) {
            new_constraints = new_constraints + *top;
        }
        for child in maps.relationship_map.get_children(entity) {
            perform_layout(maps, world, command_buffer, &child, &new_constraints);
        } 
}

fn layout_horizontal(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        let mut subdivider = constraints.into_width_subdivider();

        for child in maps.relationship_map.get_children(entity) {
            subdivider.subdivide_for_entity(&child, &maps.minimum_width_map);
        }
        
        for (child, new_constraints) in subdivider.iter() {
            perform_layout(maps, world, command_buffer, &child, &new_constraints);
        }
}

fn layout_renderable(
    maps: &LayoutMaps,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        let mut layout_change = LayoutChange::from(constraints);
        
        if let Some(left) = maps.left_map.get(entity) {
            layout_change = layout_change + *left;
        }
        if let Some(top) = maps.top_map.get(entity) {
            layout_change = layout_change + *top;
        }
        if let Some(width) = maps.width_map.get(entity) {
            layout_change.width = width.width;
        }
        if let Some(height) = maps.height_map.get(entity) {
            layout_change.height = height.height;
        }
        println!("{:?}", layout_change);
        command_buffer.add_component(*entity, layout_change);
}