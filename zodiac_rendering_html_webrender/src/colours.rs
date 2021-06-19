use zodiac_html::*;

pub struct ColourF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Colour> for ColourF {
    fn from(colour: Colour) -> Self {
        Self {
            r: colour.r as f32 / 255.0,
            g: colour.g as f32 / 255.0,
            b: colour.b as f32 / 255.0,
            a: colour.a as f32 / 255.0
        }    
    }
}

impl From<&BackgroundColour> for ColourF {
    fn from(colour: &BackgroundColour) -> Self {
        ColourF::from(colour.0)
    }
}

impl From<&BorderColour> for ColourF {
    fn from(colour: &BorderColour) -> Self {
        let colour: Colour = colour.into();
        ColourF::from(colour)
    }
}

impl Into<webrender::api::ColorF> for ColourF {
    fn into(self) -> webrender::api::ColorF {
        webrender::api::ColorF::new(self.r, self.g, self.b, self.a)
    }
}