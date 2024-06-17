use alloy_primitives::{address, Address};
use host::Host;
use vlayer_steel::contract::CallTxData;

pub mod host;

const CONTRACT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");
const CALLER: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

fn main() -> anyhow::Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // This is the abi encoded call data (lhs = 1, rhs = 2) for the sum function in the Simple contract.
    let raw_call_data: Vec<u8> = vec![
        202, 208, 137, 155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    ];

    let mut call_data = CallTxData::<()>::new_from_bytes(CONTRACT, raw_call_data.clone());
    call_data.caller = CALLER;

    let _return_data = Host::try_new()?.run(raw_call_data, call_data)?;

    Ok(())
}
