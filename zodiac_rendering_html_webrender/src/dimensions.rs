use glium::glutin::dpi::PhysicalSize;
use zodiac::*;
use zodiac_html::*;

pub struct WrappedDimensions(Dimensions);
impl From<PhysicalSize<u32>> for WrappedDimensions {
    fn from(size: PhysicalSize<u32>) -> Self {
        Self(Dimensions { width: size.width as u16, height: size.height as u16 })
    }
}

impl From<WrappedDimensions> for PhysicalSize<u32>  {
    fn from(dimensions: WrappedDimensions) -> Self {
        Self::new(dimensions.0.width as u32, dimensions.0.height as u32)
    }
}

impl From<Dimensions> for WrappedDimensions {
    fn from(size: Dimensions) -> Self {
        Self(size)
    }
}

impl From<&Dimensions> for WrappedDimensions {
    fn from(size: &Dimensions) -> Self {
        Self(*size)
    }
}

impl From<(u16, u16)> for WrappedDimensions {
    fn from(size: (u16, u16)) -> Self {
        Self(Dimensions::from(size))
    }
}

impl Into<Dimensions> for WrappedDimensions {
    fn into(self) -> Dimensions {
        self.0
    }
}

pub struct WrappedLayout(ResolvedLayoutBox);

impl From<&ResolvedLayoutBox> for WrappedLayout  {
    fn from(layout: &ResolvedLayoutBox) -> Self {
        Self(*layout)
    }
}

impl Into<webrender::euclid::Rect<f32, webrender::api::units::LayoutPixel>> for WrappedLayout {
    fn into(self) -> webrender::euclid::Rect<f32, webrender::api::units::LayoutPixel> {
        let position: (u16, u16) = self.0.position(BoxArea::Border).into();
        let dimensions: (u16, u16) = self.0.dimensions(BoxArea::Border).into();
        webrender::euclid::Rect::new(
            webrender::euclid::point2(position.0 as f32, position.1 as f32),
            webrender::euclid::size2(dimensions.0 as f32, dimensions.1 as f32))
    }
}

