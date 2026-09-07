//! Exact Circuit-SAT/#SAT implementation. All gates have fan-in at most two;
//! fanout is unrestricted. Counts use arbitrary-precision nonnegative integers.
//!
//! The default pipeline follows the SAT note: exact graph reductions,
//! constructive fixed-epsilon layout, then enumeration or frontier counting.
//! Resource caps return errors. Explicit exact-search and greedy layout modes
//! remain available for experiments and do not carry the paper's time bound.

pub mod circuit;
pub mod examples;
pub mod layout;
pub mod natural;
mod network;
pub mod solver;

pub use circuit::{Circuit, Gate, Wire};
pub use layout::{Epsilon, LayoutKind, LayoutMode};
pub use natural::Natural;
pub use solver::{solve, Algorithm, Method, Options, Report};
