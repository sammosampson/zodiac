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

pub struct WrappedLayout(Layout);

impl From<&Layout> for WrappedLayout  {
    fn from(layout: &Layout) -> Self {
        Self(*layout)
    }
}

impl Into<webrender::euclid::Rect<f32, webrender::api::units::LayoutPixel>> for WrappedLayout {
    fn into(self) -> webrender::euclid::Rect<f32, webrender::api::units::LayoutPixel> {
        webrender::euclid::Rect::new(
            webrender::euclid::point2(self.0.left as f32, self.0.top as f32),
            webrender::euclid::size2(self.0.width as f32, self.0.height as f32))
    }
}

