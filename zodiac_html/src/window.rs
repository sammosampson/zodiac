use serde::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Window;


#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowOpen;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Title(String);

impl From<String> for Title {
    fn from(title: String) -> Self {
        Self(title)
    }
}

impl Into<String> for &Title {
    fn into(self) -> String {
        self.0.clone()
    }
}