// Only preflight. Fast
mod preflight;
// Both preflight and guest. Slow
mod with_guest;

// Checks that we don't perform redundant RPC calls
mod number_or_rpc_calls;
