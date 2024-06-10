#![no_main]

risc0_zkvm::guest::entry!(main);

use alloy_sol_types::sol;

sol! {
    interface Simple {
        function sum(uint256, uint256) public pure returns (uint256);
    }

}

fn main() {
    println!("Hello, world!")
}
