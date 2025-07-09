//! Graphics rendering abstractions
//!
//! This module provides platform-agnostic graphics rendering interfaces.

pub mod renderer;

#[cfg(target_arch = "wasm32")]
pub mod webgl;

#[cfg(not(target_arch = "wasm32"))]
pub mod opengl;