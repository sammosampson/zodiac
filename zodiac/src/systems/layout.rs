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
    fn into_width_subdivider<'a>(&self, minimum_width_map: &'a MinimumWidthMap) -> LayoutConstraintsSubDivider<LayoutConstraintsWidthResizer<'a>> {
        LayoutConstraintsSubDivider::<LayoutConstraintsWidthResizer<'a>>
            ::from_resizer(LayoutConstraintsWidthResizer(*self, minimum_width_map))
    }
    fn into_height_subdivider<'a>(&self, minimum_height_map: &'a MinimumHeightMap) -> LayoutConstraintsSubDivider<LayoutConstraintsHeightResizer<'a>> {
        LayoutConstraintsSubDivider::<LayoutConstraintsHeightResizer<'a>>
            ::from_resizer(LayoutConstraintsHeightResizer(*self, minimum_height_map))
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

trait LayoutConstraintsResizer {
    fn resize(&self, culmative_size: u16, size: u16) -> LayoutConstraints;
    fn get_size(&self) -> u16;
    fn get_minimum_size(&self, entity: &Entity) -> Option<u16>;
}

struct LayoutConstraintsWidthResizer<'a> (LayoutConstraints, &'a MinimumWidthMap);

impl<'a> LayoutConstraintsResizer for LayoutConstraintsWidthResizer<'a>  {
    fn resize(&self, culmative_size: u16, size: u16) -> LayoutConstraints {
        LayoutConstraints {
            left: self.0.left + culmative_size,
            top: self.0.top,
            width: size,
            height: self.0.height
        }
    }

    fn get_size(&self) -> u16 {
        self.0.width
    }

    fn get_minimum_size(&self, entity: &Entity) -> Option<u16> {
        if let Some(width) = self.1.get(entity) {
            return Some(width.width);
        }
        None
    }
}

struct LayoutConstraintsHeightResizer<'a> (LayoutConstraints, &'a MinimumHeightMap);

impl<'a> LayoutConstraintsResizer for LayoutConstraintsHeightResizer<'a> {
    fn resize(&self, culmative_size: u16, size: u16) -> LayoutConstraints {
        LayoutConstraints {
            left: self.0.left,
            top: self.0.top + culmative_size,
            width: self.0.width,
            height: size
        }
    }

    fn get_size(&self) -> u16 {
        self.0.height
    }

    fn get_minimum_size(&self, entity: &Entity) -> Option<u16> {
        if let Some(height) = self.1.get(entity) {
            return Some(height.height);
        }
        None
    }
}

struct LayoutConstraintsSubDivider<TResizer: LayoutConstraintsResizer> {
    resizer: TResizer,
    subdivisions: Vec::<LayoutConstraintsSubDivisionType>,
    total_fixed_size: u16,
    total_no_specified_size_items: usize
}

impl <TResizer> LayoutConstraintsSubDivider<TResizer>  where TResizer: LayoutConstraintsResizer {
    fn from_resizer(resizer: TResizer) -> Self {
        LayoutConstraintsSubDivider::<TResizer> {
            resizer,
            subdivisions: vec!(),
            total_fixed_size: 0, 
            total_no_specified_size_items: 0
        }
    }

    fn subdivide_for_entity(&mut self, entity: &Entity) {
        match self.resizer.get_minimum_size(entity) {
            Some(size) => {
                self.subdivisions.push(LayoutConstraintsSubDivisionType::FixedSizeSpecified(*entity, size));
                self.total_fixed_size += size;
            },
            None => {
                self.subdivisions.push(LayoutConstraintsSubDivisionType::SizeNotSpecified(*entity));
                self.total_no_specified_size_items += 1;
            }
        }
    }

    fn iter(&self) -> LayoutConstraintsSubDividerIterator::<TResizer> {
        LayoutConstraintsSubDividerIterator::<TResizer> {
            subdivider: self,
            current_index: 0,
            culmative_size: 0
        }
    }

    fn get_subdivision(&self, index: usize) -> Option<&LayoutConstraintsSubDivisionType> {
        self.subdivisions.get(index)
    }

    fn slice(&self, culmative_size: u16) -> (LayoutConstraints, u16) {
        let size = (self.resizer.get_size() - self.total_fixed_size) / self.total_no_specified_size_items as u16;
        let slice = self.fixed_slice(culmative_size, size);
        (slice, size)
    }
    
    fn fixed_slice(&self, culmative_size: u16, size: u16) -> LayoutConstraints {
        self.resizer.resize(culmative_size, size)
    }
}

struct LayoutConstraintsSubDividerIterator<'a, TResizer: LayoutConstraintsResizer> {
    subdivider: &'a LayoutConstraintsSubDivider<TResizer>,
    current_index: usize,
    culmative_size: u16
}

impl<'a, TResizer> Iterator for LayoutConstraintsSubDividerIterator<'a, TResizer>  where TResizer: LayoutConstraintsResizer{
    type Item = (Entity, LayoutConstraints);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(subdivision_type) = self.subdivider.get_subdivision(self.current_index) {
            let result = match subdivision_type {
                LayoutConstraintsSubDivisionType::SizeNotSpecified(entity) => {
                    let (new_constraints, size) = self.subdivider.slice(self.culmative_size);
                    self.culmative_size += size;
                    Some((*entity, new_constraints))
                },
                LayoutConstraintsSubDivisionType::FixedSizeSpecified(entity, size) => {
                    let new_constraints = self.subdivider.fixed_slice(self.culmative_size, *size);
                    self.culmative_size += size;
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
                LayoutType::Horizontal => layout_horizontal(maps, world, command_buffer, entity, constraints),
                LayoutType::Vertical => layout_vertical(maps, world, command_buffer, entity, constraints)
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
        let mut subdivider = constraints.into_width_subdivider(&maps.minimum_width_map);

        for child in maps.relationship_map.get_children(entity) {
            subdivider.subdivide_for_entity(&child);
        }
        
        for (child, new_constraints) in subdivider.iter() {
            perform_layout(maps, world, command_buffer, &child, &new_constraints);
        }
}

fn layout_vertical(
    maps: &LayoutMaps,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    entity: &Entity, 
    constraints: &LayoutConstraints) {
        let mut subdivider = constraints.into_height_subdivider(&maps.minimum_height_map);

        for child in maps.relationship_map.get_children(entity) {
            subdivider.subdivide_for_entity(&child);
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