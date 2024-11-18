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
#![rustfmt::skip]

pub mod cli;
mod contract_runner;
mod filter;
mod install;
mod multi_runner_run;
mod progress;
mod summary;
mod test_executor;
