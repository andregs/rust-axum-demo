#![forbid(unsafe_code)]

// we need a lib.rs if we want to do some black box tests under tests/ dir

pub mod config;
mod controller;
pub mod model;
mod validation;
