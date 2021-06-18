mod embedding;
mod building;
mod running;
mod changes;

pub use embedding::*;
pub use changes::*;
pub use building::*;
pub use running::*;

pub trait PropertySetCheck {
    fn is_set(&self) -> bool;
}
