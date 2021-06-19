use serde::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum BorderStyles {
    None,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    Hidden
}


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderStyle(BorderStyles, bool);

impl Default for BorderStyle {
    fn default() -> Self {
        Self(BorderStyles::None, false)
    }
}

impl From<BorderStyles> for BorderStyle {
    fn from(styles: BorderStyles) -> Self {
        Self(styles, true)
    }
}

impl Into<BorderStyles> for &BorderStyle {
    fn into(self) -> BorderStyles {
        self.0
    }
}

impl zodiac::PropertySetCheck for BorderStyle {
    fn is_set(&self) -> bool {
        self.1
    }
}