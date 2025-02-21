#![no_main]

risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;

fn main() {
    let input = env::read();

    call_guest::main(input);
}
