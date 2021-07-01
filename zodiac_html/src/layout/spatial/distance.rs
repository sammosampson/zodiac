use std::ops::{Add, Sub};
use serde::*;

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
    pub fn resolve_from_parent(&self, current: &ResolvedLayoutDistance, parent: &ResolvedLayoutDistance) -> ResolvedLayoutDistance {
        if current == &ResolvedLayoutDistance::Unresolved {
            if let Self::FromParent(multiplier) = self {
                if let ResolvedLayoutDistance::Resolved(parent_fixed_distance) = parent {
                    return ResolvedLayoutDistance::Resolved((*parent_fixed_distance as f32 * multiplier) as u16);
                }
            }
        }

        *current
    }

    pub fn resolve_from_child(&self, current: &ResolvedLayoutDistance, _child: &ResolvedLayoutDistance) -> ResolvedLayoutDistance {
        if current == &ResolvedLayoutDistance::Unresolved {
            if let Self::FromChildren(_multiplier) = self {
                return ResolvedLayoutDistance::Resolved(0);
            }
        }

        *current
    }

    pub fn complete_children_resolution(&self, current: &ResolvedLayoutDistance, offset: ResolvedLayoutDistance) -> ResolvedLayoutDistance {
        if current == &ResolvedLayoutDistance::Unresolved {
            if let Self::FromChildren(_multiplier) = self {
                return ResolvedLayoutDistance::Resolved(0) + offset;
            }
        }

        *current
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
                if rhs_distance > distance {
                    return ResolvedLayoutDistance::Resolved(0);
                }
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
            LayoutDistance::None => Self::Resolved(0),
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