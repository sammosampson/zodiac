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

pub struct WrappedBorderSide(Colour, BorderStyles, Size);

impl From<(Size, BorderStyles, Colour)> for WrappedBorderSide {
    fn from(props: (Size, BorderStyles, Colour)) -> Self {
        Self(props.2, props.1, props.0)
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

pub struct WrappedBorder(FullBorder);

impl From<&FullBorder> for WrappedBorder {
    fn from(border: &FullBorder) -> Self {
        WrappedBorder(*border)
    }
}

impl Into<Option<RenderPrimitiveBorder>> for WrappedBorder {
    fn into(self) -> Option<RenderPrimitiveBorder> {
        if !self.0.is_visible() {
            None
        } else {
            Some(self.into())
        }
    }
}

impl Into<RenderPrimitiveBorder> for WrappedBorder {
    fn into(self) -> RenderPrimitiveBorder {

        let (top, left, bottom, right, radius) = self.0.into();
        let left: (Size, BorderStyles, Colour) = left.into();
        let right: (Size, BorderStyles, Colour) = right.into();
        let top: (Size, BorderStyles, Colour) = top.into();
        let bottom: (Size, BorderStyles, Colour) = bottom.into();
        let radius = WrappedBorderRadius::from(radius).into();

        let details =  webrender::api::BorderDetails::Normal(
            webrender::api::NormalBorder {
                left: WrappedBorderSide::from(left).into(),
                right: WrappedBorderSide::from(right).into(),
                top: WrappedBorderSide::from(top).into(),
                bottom: WrappedBorderSide::from(bottom).into(),
                radius,
                do_aa: true
            });

        let widths = webrender::api::units::LayoutSideOffsets {
            top: top.0.into(),
            right: right.0.into(),
            bottom: bottom.0.into(),
            left: left.0.into(),
            _unit: PhantomData::<webrender::api::units::LayoutPixel>::default(),
        };

        RenderPrimitiveBorder {
            radius,
            widths,
            details
        }
    }
}