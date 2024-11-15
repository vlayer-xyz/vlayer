use alloy_primitives::{address, Address};
use alloy_sol_types::sol;

pub const USDT: Address = address!("dAC17F958D2ee523a2206206994597C13D831ec7");
pub const USDT_BLOCK_NO: u64 = 19_493_153;
sol! {
    #[derive(Debug, PartialEq, Eq)]
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
    }
}

pub const UNISWAP: Address = address!("1F98431c8aD98523631AE4a59f267346ea31F984");
sol! {
    #[derive(Debug, PartialEq, Eq)]
    interface IUniswapV3Factory {
        function owner() external view returns (address);
    }
}

pub const VIEW_CALL: Address = address!("C5096d96dbC7594B3d0Ba50e708ba654A7ae1F3E");
pub const VIEW_CALL_BLOCK_NO: u64 = 5_702_743;
sol!(
    #[derive(Debug, PartialEq, Eq)]
    contract ViewCallTest {
        /// Tests the SHA256 precompile.
        function testPrecompile() external view returns (bytes32) {
            (bool ok, bytes memory out) = address(0x02).staticcall("");
            require(ok);
            return abi.decode(out, (bytes32));
        }

        /// Tests accessing the code of a nonexistent account.
        function testNonexistentAccount() external view returns (uint256 size) {
            address a = address(uint160(block.prevrandao));
            assembly { size := extcodesize(a) }
        }

        /// Tests accessing the code of the EOA account 0x0000000000000000000000000000000000000000.
        function testEoaAccount() external view returns (uint256 size) {
            assembly { size := extcodesize(0) }
        }

        /// Tests the blockhash opcode.
        function testBlockhash() external view returns (bytes32) {
            return blockhash(block.number - 2);
        }

        /// Tests retrieving the chain ID.
        function testChainid() external view returns (uint256) {
            return block.chainid;
        }

        /// Tests retrieving the gas price.
        function testGasprice() external view returns (uint256) {
            return tx.gasprice;
        }

        /// Tests calling multiple contracts with the same and different storage.
        function testMuliContractCalls() external view returns (uint256) {
            return VALUE0.value() + VALUE42_a.value() + VALUE42_b.value();
        }
    }
);

// Generated using `simple_teleport` example
pub const SIMPLE_TELEPORT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");
pub const BLOCK_NO: u64 = 3;
sol! {
    contract SimpleTravelProver {
        #[derive(Debug)]
        function crossChainBalanceOf(address owner) public returns (address, uint256);
    }
}
