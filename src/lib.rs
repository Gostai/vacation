//! A program for vacation management
#![deny(missing_docs)]
#![forbid(unsafe_code)]


pub mod processor; 
pub mod instruction;
pub mod error;


#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
