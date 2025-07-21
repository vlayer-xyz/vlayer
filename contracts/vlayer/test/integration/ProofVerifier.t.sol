// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.4/src/Test.sol";

import {Prover} from "../../src/Prover.sol";
import {Proof, ProofLib} from "../../src/Proof.sol";
import {IProofVerifier} from "../../src/proof_verifier/IProofVerifier.sol";
import {CallAssumptions} from "../../src/CallAssumptions.sol";
import {Seal, ProofMode} from "../../src/Seal.sol";

import {Repository} from "../../src/Repository.sol";

import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";

import {Groth16VerifierSelector} from "../helpers/Groth16VerifierSelector.sol";

// Proofs have been generated using SimpleProver from examples/simple
// All the pinned values have been obtained using the following instruction: https://github.com/vlayer-xyz/vlayer/pull/577#issuecomment-2355839549
interface PinnedSimpleProver {
    function balance(address _owner) external returns (Proof memory, address, uint256);
}

contract FakeProofVerifierUnderTest is FakeProofVerifier {
    constructor() FakeProofVerifier(new Repository(address(this), address(this))) {
        IMAGE_ID_REPOSITORY.addImageIdSupport(ProofFixtures.FIXED_CALL_GUEST_ID);
    }
}

contract Groth16ProofVerifierUnderTest is Groth16ProofVerifier {
    constructor() Groth16ProofVerifier(new Repository(address(this), address(this))) {
        IMAGE_ID_REPOSITORY.addImageIdSupport(ProofFixtures.FIXED_CALL_GUEST_ID);
    }
}

contract PinnedProofVerifer_Tests is Test {
    bytes32 internal fuzzingSeed;

    function setUp() public {
        // Proof has been generated with anvil, whereas we are checking against forge chain,
        // therefore blockhashes do not match.
        vm.roll(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER + 1);
    }

    function test_canVerifyFakeProof() public {
        vm.setBlockhash(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER, ProofFixtures.FIXED_FAKE_SETTLE_BLOCK_HASH);
        vm.chainId(ProofFixtures.FIXED_SETTLE_CHAIN_ID);
        IProofVerifier verifier = new FakeProofVerifierUnderTest();
        (Proof memory proof, bytes32 journalHash) = ProofFixtures.fakeProofFixture();

        verifier.verify(proof, journalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR);
    }

    function test_canVerifyGroth16Proof() public {
        vm.setBlockhash(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER, ProofFixtures.FIXED_GROTH16_SETTLE_BLOCK_HASH);
        vm.chainId(ProofFixtures.FIXED_SETTLE_CHAIN_ID);
        IProofVerifier verifier = new Groth16ProofVerifierUnderTest();
        (Proof memory proof, bytes32 journalHash) = ProofFixtures.groth16ProofFixture();

        verifier.verify(proof, journalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR);
    }

    function _randomBool() internal returns (bool) {
        fuzzingSeed = keccak256(abi.encode(fuzzingSeed));
        return uint256(fuzzingSeed) % 2 == 1;
    }

    // Enums can't be fuzzed https://github.com/foundry-rs/foundry/issues/871
    struct FuzzableProof {
        FuzzableSeal seal;
        bytes32 callGuestId;
        uint256 length;
        CallAssumptions callAssumptions;
    }

    struct FuzzableSeal {
        bytes4 verifierSelector;
        bytes32[8] seal;
        uint256 mode;
    }

    function _fromFuzzable(FuzzableProof memory proof) internal pure returns (Proof memory) {
        return Proof(_fromFuzzable(proof.seal), proof.callGuestId, proof.length, proof.callAssumptions);
    }

    function _fromFuzzable(FuzzableSeal memory seal) internal pure returns (Seal memory) {
        return Seal(seal.verifierSelector, seal.seal, ProofMode(seal.mode % uint256(type(ProofMode).max)));
    }

    function _arbitraryProof(Proof memory originalProof, Proof memory randomProof)
        internal
        returns (Proof memory, bytes32)
    {
        Proof memory arbitraryProof;
        arbitraryProof.seal.verifierSelector =
            _randomBool() ? randomProof.seal.verifierSelector : originalProof.seal.verifierSelector;
        arbitraryProof.seal.seal = _randomBool() ? randomProof.seal.seal : originalProof.seal.seal;
        arbitraryProof.seal.mode = _randomBool() ? randomProof.seal.mode : originalProof.seal.mode;
        arbitraryProof.callGuestId = _randomBool() ? randomProof.callGuestId : originalProof.callGuestId;
        arbitraryProof.length = originalProof.length; // Not actually verified
        arbitraryProof.callAssumptions.proverContractAddress = _randomBool()
            ? randomProof.callAssumptions.proverContractAddress
            : originalProof.callAssumptions.proverContractAddress;
        arbitraryProof.callAssumptions.functionSelector = _randomBool()
            ? randomProof.callAssumptions.functionSelector
            : originalProof.callAssumptions.functionSelector;
        arbitraryProof.callAssumptions.settleChainId =
            _randomBool() ? randomProof.callAssumptions.settleChainId : originalProof.callAssumptions.settleChainId;
        arbitraryProof.callAssumptions.settleBlockNumber = _randomBool()
            ? randomProof.callAssumptions.settleBlockNumber
            : originalProof.callAssumptions.settleBlockNumber;
        arbitraryProof.callAssumptions.settleBlockHash =
            _randomBool() ? randomProof.callAssumptions.settleBlockHash : originalProof.callAssumptions.settleBlockHash;
        return (
            arbitraryProof,
            ProofFixtures.journalHash(
                randomProof.callAssumptions, ProofFixtures.FIXED_OWNER, ProofFixtures.FIXED_BALANCE
            )
        );
    }

    function testFuzz_cannotVerifyManipulatedGroth16Proof(
        FuzzableProof calldata randomFuzzableProof,
        bytes32 _fuzzingSeed
    ) public {
        fuzzingSeed = _fuzzingSeed;
        Proof memory randomProof = _fromFuzzable(randomFuzzableProof);

        vm.setBlockhash(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER, ProofFixtures.FIXED_GROTH16_SETTLE_BLOCK_HASH);
        vm.chainId(ProofFixtures.FIXED_SETTLE_CHAIN_ID);
        IProofVerifier verifier = new Groth16ProofVerifierUnderTest();
        (Proof memory proof,) = ProofFixtures.groth16ProofFixture();

        (Proof memory arbitraryProof, bytes32 arbitraryJournalHash) = _arbitraryProof(proof, randomProof);
        vm.assume(keccak256(abi.encode(arbitraryProof)) != keccak256(abi.encode(proof)));

        try verifier.verify(
            arbitraryProof, arbitraryJournalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR
        ) {
            revert("Should fail");
        } catch {}
    }
}

library ProofFixtures {
    bytes32 public constant FIXED_CALL_GUEST_ID =
        bytes32(0xe4fa67ba8e47afa449599dc8e5664934caab39d82b28db49d6618120f13cd617);
    address public constant FIXED_PROVER_ADDRESS = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0);
    bytes4 public constant FIXED_SELECTOR = PinnedSimpleProver.balance.selector;
    uint256 public constant FIXED_SETTLE_CHAIN_ID = 31_337;
    uint256 public constant FIXED_SETTLE_BLOCK_NUMBER = 6;
    bytes32 public constant FIXED_GROTH16_SETTLE_BLOCK_HASH =
        bytes32(0xc097ebaa8c376eef219eba639b8106445245184e6340589aade6be8f952b8213);
    bytes32 public constant FIXED_FAKE_SETTLE_BLOCK_HASH =
        bytes32(0x60b4606c6d21d31bf382363d05aa5aecfd9a729782080bd58d5db24f16a3bdd6);

    address public constant FIXED_OWNER = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
    uint256 public constant FIXED_BALANCE = 10000000;

    function fakeProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x71c344b512f2adc731cc604d8ca9d0c754532998b34bdf3fb1497e058f5e8d67),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000)
        ];

        Seal memory seal = Seal(bytes4(0xdeafbeef), sealBytes, ProofMode.FAKE);

        return generateProof(seal, ProofFixtures.FIXED_FAKE_SETTLE_BLOCK_HASH);
    }

    function groth16ProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x271c6738507468f4a9fd292475876d4e989af05f16131422f9048669b64e616e),
            bytes32(0x2b416f89ba8d5547f92ec149770adcff6376198d9631c24272e2580f419e4fee),
            bytes32(0x1301810121e28114301d0b222b1dcc01ade6e463c70e2212d08805e847bf3451),
            bytes32(0x2d52bd27b4d9255c4c6e2578cd8f3a36a1943d6ac2174e7bb1abfc41990eb7ec),
            bytes32(0x131eb1197b012e93ae5d7bc39043d7ffc7678b453bbababc9f9c3362f6829eb3),
            bytes32(0x22ea0234222c68f189b99b48913767e39e6dc300c802beb4b1599c5edef8e696),
            bytes32(0x0e9c4a5a9b33a7269851b424e1765cb6968bae628884e4de097fad916d3b70fd),
            bytes32(0x2bbf47e7e96436cc2f08ecfeef50b13bbc263474754cc95974dc3966b5e3525a)
        ];

        Seal memory seal = Seal(Groth16VerifierSelector.STABLE_VERIFIER_SELECTOR, sealBytes, ProofMode.GROTH16);

        return generateProof(seal, ProofFixtures.FIXED_GROTH16_SETTLE_BLOCK_HASH);
    }

    function generateProof(Seal memory seal, bytes32 blockHash) private pure returns (Proof memory, bytes32) {
        CallAssumptions memory callAssumptions = CallAssumptions(
            FIXED_PROVER_ADDRESS, FIXED_SELECTOR, FIXED_SETTLE_CHAIN_ID, FIXED_SETTLE_BLOCK_NUMBER, blockHash
        );

        uint256 length = 0; // it is not used in verification, so can be set to 0

        Proof memory proof = Proof(seal, FIXED_CALL_GUEST_ID, length, callAssumptions);
        return (proof, journalHash(callAssumptions, FIXED_OWNER, FIXED_BALANCE));
    }

    function journalHash(CallAssumptions memory callAssumptions, address owner, uint256 balance)
        public
        pure
        returns (bytes32)
    {
        bytes memory journal = abi.encode(callAssumptions, ProofLib.emptyProof(), owner, balance);
        return sha256(journal);
    }
}
