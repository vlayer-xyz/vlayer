use alloy_primitives::{Address, address};
use alloy_sol_types::sol;
use lazy_static::lazy_static;

pub mod usdt {
    use super::*;

    pub const USDT: Address = address!("dAC17F958D2ee523a2206206994597C13D831ec7");
    pub const BLOCK_NO: u64 = 19_493_153;
    pub const OPTIMISM_USDT: Address = address!("94b008aA00579c1307B0EF2c499aD98a8ce58e58");
    pub const OPTIMISM_BLOCK_NO: u64 = 128_507_722;
    sol! {
        #[derive(Debug, PartialEq, Eq)]
        interface IERC20 {
            function balanceOf(address account) external view returns (uint);
        }
    }
}

pub mod uniswap {
    use super::*;

    pub const UNISWAP: Address = address!("1F98431c8aD98523631AE4a59f267346ea31F984");
    sol! {
        #[derive(Debug, PartialEq, Eq)]
        interface IUniswapV3Factory {
            function owner() external view returns (address);
        }
    }
}

pub mod view {
    use super::*;

    pub const VIEW_CALL: Address = address!("C5096d96dbC7594B3d0Ba50e708ba654A7ae1F3E");
    pub const BLOCK_NO: u64 = 5_702_743;
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
}

sol! {
    #[derive(Debug)]
    enum ProofMode {
        GROTH16,
        FAKE
    }
    #[derive(Debug)]
    struct Seal {
        bytes4 verifierSelector;
        bytes32[8] seal;
        ProofMode mode;
    }
    #[derive(Debug)]
    struct CallAssumptions {
        address proverContractAddress;
        bytes4 functionSelector;
        uint256 settleBlockNumber; // Block number for which assumptions was made.
        bytes32 settleBlockHash; // Hash of the block at the specified block number.
    }
}

pub mod simple {
    use super::*;

    // Generated using `simple` example
    pub const SIMPLE: Address = address!("6050ea72b58525d3d470c96604bcd62b7f464e17");
    // Block where verifier was deployed: https://sepolia-optimism.etherscan.io/tx/0x461050173aadd23142df65edcef1e847706795750398a01ed548c37bf6f58087
    pub const BLOCK_NO: u64 = 22_616_952;

    sol! {
        #[derive(Debug)]
        struct Proof {
            Seal seal;
            bytes32 callGuestId;
            uint256 length;
            CallAssumptions callAssumptions;
        }
        #[derive(Debug)]
        contract SimpleProver {
            function balance(address _owner) public returns (Proof memory, address, uint256);
        }
    }
}

pub mod teleport {
    use alloy_primitives::{B256, Uint, hex, uint};
    use optimism::{NumHash, types::SequencerOutput};

    use super::*;

    // Generated using `simple_teleport` example
    pub const SIMPLE_TELEPORT: Address = address!("9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0");
    pub const JOHN: Address = address!("e2148eE53c0755215Df69b2616E552154EdC584f");
    pub const BLOCK_NO: u64 = 3;
    pub const TOKEN: Erc20Token = Erc20Token {
        addr: TOKEN_ADDR,
        chainId: OP_ANVIL_CHAIN_ID,
        blockNumber: OP_BLOCK_NO,
    };
    const OP_BLOCK_NO: Uint<256, 4> = uint!(3_U256);
    const OP_ANVIL_CHAIN_ID: Uint<256, 4> = uint!(31338_U256);
    const TOKEN_ADDR: Address = address!("da52b25ddB0e3B9CC393b0690Ac62245Ac772527");

    lazy_static! {
        static ref STATE_ROOT: B256 =
            B256::from(hex!("25d65fff68c2248f9b0c0b04d2ce9749dbdb088bd0fe16962476f18794373e5f"));
        static ref WITHDRAWAL_STORAGE_ROOT: B256 =
            B256::from(hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"));
        static ref FINALIZED_L2_HASH: B256 =
            B256::from(hex!("8a3162ac8009f30a115f905e15c1206c89d5bde102e5cf1f72e425d3aec03fbd"));
        pub static ref OUTPUT: SequencerOutput = SequencerOutput::new(
            *STATE_ROOT,
            *WITHDRAWAL_STORAGE_ROOT,
            NumHash::new(3, *FINALIZED_L2_HASH)
        );
    }

    sol! {
        #[derive(Debug)]
        struct Proof {
            Seal seal;
            bytes32 callGuestId;
            uint256 length;
            CallAssumptions callAssumptions;
        }
        #[derive(Debug)]
        struct Erc20Token {
            address addr;
            uint256 chainId;
            uint256 blockNumber;
        }
        #[derive(Debug)]
        contract SimpleTeleportProver {
            function crossChainBalanceOf(address owner, Erc20Token[] memory tokens) public returns (Proof memory, address, uint256);
        }
    }
}

pub mod time_travel {
    use super::*;

    // Generated using `simple_time_travel` example
    pub const SIMPLE_TIME_TRAVEL: Address = address!("0d5556591b2ffe5e7d54a4887638508f039771d3");
    pub const BLOCK_NO: u64 = 20_064_547_u64;
    const TOKEN_OWNER: Address = address!("E6b08c02Dbf3a0a4D3763136285B85A9B492E391");
    sol!(
        #[derive(Debug)]
        struct Proof {
            Seal seal;
            bytes32 callGuestId;
            uint256 length;
            CallAssumptions callAssumptions;
        }
        #[sol(all_derives = true)]
        contract AverageBalance {
            #[sol(all_derives = true)]
            function averageBalanceOf(address _owner) public returns (Proof memory, address, uint256);
        }
    );

    pub const AVERAGE_BALANCE_OF_CALL: AverageBalance::averageBalanceOfCall =
        AverageBalance::averageBalanceOfCall {
            _owner: TOKEN_OWNER,
        };
}

pub mod web_proof {
    use alloy_primitives::address;

    use super::*;

    pub const WEB_PROOF_PROVER: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");
    // Required to be passed to the `main` function, but not utilized within it, as the code panics beforehand
    pub const ACCOUNT_ADDRESS: Address = address!("0000000000000000000000000000000000000000");

    lazy_static! {
        pub static ref WEB_PROOF: String = "web_proof".to_string();
    }

    sol!(
        #[derive(Debug)]
        struct Proof {
            Seal seal;
            bytes32 callGuestId;
            uint256 length;
            CallAssumptions callAssumptions;
        }

        #[derive(Debug)]
        struct WebProof {
            string webProofJson;
        }

        #[sol(all_derives = true)]
        contract WebProofProver {
            #[sol(all_derives = true)]
            function main(WebProof webProof, address account)
                public
                returns (Proof memory, string memory, address);
        }
    );
}
