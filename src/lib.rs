#![feature(rustc_private)]
#![feature(exhaustive_patterns)]
#![allow(clippy::similar_names)]
#![allow(clippy::single_match_else)]
#![allow(clippy::too_many_lines)]
#![deny(warnings)]

extern crate rustc_const_eval; // Requires `rustup component add rustc-dev`
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_trait_selection;

mod changes;
mod mapping;
mod mismatch;
mod translate;
mod traverse;
mod typeck;

pub use self::traverse::{run_analysis, run_traversal};
