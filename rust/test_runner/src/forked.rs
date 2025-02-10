#![allow(clippy::all)]
#![allow(clippy::unneeded_field_pattern)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::useless_let_if_seq)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::if_then_some_else_none)]
#![allow(clippy::assigning_clones)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::large_futures)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::unused_self)]

#[rustfmt::skip]
pub mod cli;
#[rustfmt::skip]
mod runner;
#[rustfmt::skip]
mod filter;
#[rustfmt::skip]
mod install;
#[rustfmt::skip]
mod multi_runner;
#[rustfmt::skip]
mod progress;
#[rustfmt::skip]
mod summary;
#[rustfmt::skip]
mod test_executor;
#[rustfmt::skip]
pub(crate)mod watch;
