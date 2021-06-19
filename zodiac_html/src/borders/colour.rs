use serde::*;
use crate::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderColour(Colour, bool);

impl From<Colour> for BorderColour {
    fn from(colour: Colour) -> Self {
        Self(colour, true)
    }
}

impl zodiac::PropertySetCheck for &BorderColour {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl Into<Colour> for &BorderColour {
    fn into(self) -> Colour {
        self.0
    }
}