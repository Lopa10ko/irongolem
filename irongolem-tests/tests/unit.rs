//! Unit test suite (mirrors GOLEM `test/unit/`).

#[path = "unit/custom.rs"]
mod custom;

#[path = "unit/adapter/mod.rs"]
mod adapter;

#[path = "unit/api/api.rs"]
mod api;

#[path = "unit/dag/mod.rs"]
mod dag;

#[path = "unit/optimizers/mod.rs"]
mod optimizers;

#[path = "unit/serialization/mod.rs"]
mod serialization;

#[path = "unit/structural_analysis/mod.rs"]
mod structural_analysis;

#[path = "unit/utilities/mod.rs"]
mod utilities;
