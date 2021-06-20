use std::ops::Add;
use legion::*;
use zodiac::*;
use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutConstraints {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}

impl LayoutConstraints {
    pub fn into_width_subdivider<'a>(&self, minimum_width_map: &'a MinimumWidthMap) -> LayoutConstraintsSubDivider<LayoutConstraintsWidthResizer<'a>> {
        LayoutConstraintsSubDivider::<LayoutConstraintsWidthResizer<'a>>
            ::from_resizer(LayoutConstraintsWidthResizer(*self, minimum_width_map))
    }
    pub fn into_height_subdivider<'a>(&self, minimum_height_map: &'a MinimumHeightMap) -> LayoutConstraintsSubDivider<LayoutConstraintsHeightResizer<'a>> {
        LayoutConstraintsSubDivider::<LayoutConstraintsHeightResizer<'a>>
            ::from_resizer(LayoutConstraintsHeightResizer(*self, minimum_height_map))
    }
}

impl From<&LayoutConstraints> for CurrentLayoutConstraints {
    fn from(constraints: &LayoutConstraints) -> Self {
        Self {
            left: constraints.left, 
            top: constraints.top, 
            width: constraints.width,
            height: constraints.height,
        }
    }
}

impl Add<Left> for  LayoutConstraints {
    type Output = Self;
    fn add(self, other: Left) -> Self {
        Self {
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
        Self {
            left: self.left,
            top: self.top + other.top,
            width: self.width,
            height: self.height
        }
    }
}

impl From<&LayoutRequest> for LayoutConstraints {
    fn from(request: &LayoutRequest) -> Self {
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

pub trait LayoutConstraintsResizer {
    fn resize(&self, culmative_size: u16, size: u16) -> LayoutConstraints;
    fn get_size(&self) -> u16;
    fn get_minimum_size(&self, entity: &Entity) -> Option<u16>;
}

pub struct LayoutConstraintsWidthResizer<'a> (LayoutConstraints, &'a MinimumWidthMap);

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

pub struct LayoutConstraintsHeightResizer<'a> (LayoutConstraints, &'a MinimumHeightMap);

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

pub struct LayoutConstraintsSubDivider<TResizer: LayoutConstraintsResizer> {
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

    pub fn subdivide_for_entity(&mut self, entity: &Entity) {
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

    pub fn iter(&self) -> LayoutConstraintsSubDividerIterator::<TResizer> {
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

pub struct LayoutConstraintsSubDividerIterator<'a, TResizer: LayoutConstraintsResizer> {
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