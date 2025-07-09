//! Platform-specific code
//!
//! This module contains platform-specific implementations for different targets.

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;