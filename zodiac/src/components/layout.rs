use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u16,
    pub height: u16
}

impl Dimensions {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl From<(u16, u16)> for Dimensions {
    fn from(dimensions: (u16, u16)) -> Self {
        Self::new(dimensions.0, dimensions.1)
    }
}

impl From<(u32, u32)> for Dimensions {
    fn from(dimensions: (u32, u32)) -> Self {
        Self::new(dimensions.0 as u16, dimensions.1 as u16)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentLayoutConstraints {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LayoutRequest {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}

impl From<&Dimensions> for LayoutRequest {
    fn from(dimensions: &Dimensions) -> Self {
        LayoutRequest {
            left: 0, 
            top: 0, 
            width: 
            dimensions.width, 
            height: dimensions.height
        }
    }
}

impl From<&CurrentLayoutConstraints> for LayoutRequest {
    fn from(current_layout_constraints: &CurrentLayoutConstraints) -> Self {
        LayoutRequest {
            left: current_layout_constraints.left, 
            top: current_layout_constraints.top, 
            width: current_layout_constraints.width,
            height: current_layout_constraints.height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LayoutChange {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}
