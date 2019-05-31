//! # AfP
//!
//! This crate is a small app for practising ham radio exam questions. It exposes
//! a database of questions along with facilities to record interactions (answering
//! questions) in DataStore.
//!
//! It also exposes some modules in ui that are used to draw a GUI to show and
//! interact with the questions.

mod datastore;
pub use datastore::*;

pub mod ui;
