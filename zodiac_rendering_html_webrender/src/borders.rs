use std::marker::PhantomData;
use zodiac_html::*;
use crate::{
    colours::*,
    render_primitive::*
};

pub struct WrappedSize(Size);

impl From<Size> for WrappedSize  {
    fn from(size: Size) -> Self {
        Self(size)
    }
}

impl Into<webrender::api::units::LayoutSideOffsets> for WrappedSize {
    fn into(self) -> webrender::api::units::LayoutSideOffsets {
        webrender::api::units::LayoutSideOffsets {
            top: self.0.into(),
            right: self.0.into(),
            bottom: self.0.into(),
            left: self.0.into(),
            _unit: PhantomData::<webrender::api::units::LayoutPixel>::default(),
        }
    }
}

impl Into<webrender::euclid::Size2D<f32, webrender::api::units::LayoutPixel>> for WrappedSize {
    fn into(self) -> webrender::euclid::Size2D<f32, webrender::api::units::LayoutPixel> {
        webrender::euclid::size2(self.0.into(), self.0.into())
    }
}

pub struct WrappedBorderStyles(BorderStyles);

impl From<BorderStyles> for WrappedBorderStyles {
    fn from(styles: BorderStyles) -> Self {
        Self(styles)
    }
}

impl Into<webrender::api::BorderStyle> for WrappedBorderStyles {
    fn into(self) -> webrender::api::BorderStyle {
        match self.0 {
            BorderStyles::None => webrender::api::BorderStyle::None,
            BorderStyles::Dotted => webrender::api::BorderStyle::Dotted,
            BorderStyles::Dashed => webrender::api::BorderStyle::Dashed,
            BorderStyles::Solid => webrender::api::BorderStyle::Solid,
            BorderStyles::Double => webrender::api::BorderStyle::Double,
            BorderStyles::Groove => webrender::api::BorderStyle::Groove,
            BorderStyles::Ridge => webrender::api::BorderStyle::Ridge,
            BorderStyles::Inset => webrender::api::BorderStyle::Inset,
            BorderStyles::Outset => webrender::api::BorderStyle::Outset,
            BorderStyles::Hidden => webrender::api::BorderStyle::Hidden,
        }
    }
}

pub struct WrappedBorderSide(Colour, BorderStyles);

impl From<(&Border, BorderStyles)> for WrappedBorderSide {
    fn from(props: (&Border, BorderStyles)) -> Self {
        Self(props.0.colour, props.1)
    }
}


impl Into<webrender::api::BorderSide> for WrappedBorderSide {
    fn into(self) -> webrender::api::BorderSide {
        webrender::api::BorderSide {
            color: ColourF::from(self.0).into(),
            style: WrappedBorderStyles::from(self.1).into()
        }
    }
}

pub struct WrappedBorderRadius(BorderRadius);

impl From<BorderRadius> for WrappedBorderRadius {
    fn from(radius: BorderRadius) -> Self {
        Self(radius)
    }
}

impl Into<webrender::api::BorderRadius> for WrappedBorderRadius {
    fn into(self) -> webrender::api::BorderRadius {
        webrender::api::BorderRadius {
            top_left: WrappedSize::from(self.0.top_left).into(),
            top_right: WrappedSize::from(self.0.top_right).into(),
            bottom_left: WrappedSize::from(self.0.bottom_left).into(),
            bottom_right: WrappedSize::from(self.0.bottom_right).into(),
        }
    }
}

pub struct WrappedBorder(Border);

impl From<&Border> for WrappedBorder {
    fn from(border: &Border) -> Self {
        WrappedBorder(*border)
    }
}

impl Into<Option<RenderPrimitiveBorder>> for WrappedBorder {
    fn into(self) -> Option<RenderPrimitiveBorder> {
        if self.0.width.is_zero() {
            None
        } else {
            Some(self.into())
        }
    }
}

impl Into<RenderPrimitiveBorder> for WrappedBorder {
    fn into(self) -> RenderPrimitiveBorder {

        let radius = WrappedBorderRadius::from(self.0.radius).into();

        let details =  webrender::api::BorderDetails::Normal(
            webrender::api::NormalBorder {
                left: WrappedBorderSide::from((&self.0, self.0.style.left)).into(),
                right: WrappedBorderSide::from((&self.0, self.0.style.right)).into(),
                top: WrappedBorderSide::from((&self.0, self.0.style.top)).into(),
                bottom: WrappedBorderSide::from((&self.0, self.0.style.bottom)).into(),
                radius,
                do_aa: true
            });

        let widths  = WrappedSize::from(self.0.width).into();
        
        RenderPrimitiveBorder {
            radius,
            widths,
            details
        }
    }
}