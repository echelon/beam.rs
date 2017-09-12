// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Beam.rs is a library that contains various laser projection algorithms.

#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_mut)]
#![deny(unused_qualifications)]
#![deny(unused_variables)]

// Fluent assertions for tests.
#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

extern crate point;

pub mod rotate;
pub mod scale;

pub use point::PipelinePoint;
