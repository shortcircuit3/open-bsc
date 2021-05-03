//! Generalization of a state machine for a consensus engine.

mod bsc_consts;
mod impls;
mod traits;

pub use self::{impls::*, traits::*};
