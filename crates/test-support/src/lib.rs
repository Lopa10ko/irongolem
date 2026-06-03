//! Shared types and fixtures for irongolem unit tests (mirrors GOLEM test helpers).

pub mod fixtures;
pub mod golem;

pub mod prelude {
    pub use crate::fixtures::*;
    pub use crate::golem::*;
}
