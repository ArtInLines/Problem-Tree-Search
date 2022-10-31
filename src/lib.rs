//! # Overview
//!
//! This library aims to allow easily solving problems as tree-searches.
//! Using the ProblemTree trait, the particular problem can implement the necessary interface to be solved via tree-searching.
//! Many standard algorithms are then offered (in differently optimized versions) to allow for solving these problems easily and efficiently.
//!
//! # Examples
//!
//! ...
//!
//! # Version 0.1.0
//!
//! ...

mod data_structures;
mod search;
mod tests;
mod traits;

pub use data_structures::*;
pub use search::*;
pub use traits::*;
