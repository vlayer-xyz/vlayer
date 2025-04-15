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
        bytes32(0x27af6fa6848425013a1e3829d0004247ee6123bd4bafb264546627d19235096b);
    address public constant FIXED_PROVER_ADDRESS = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0);
    bytes4 public constant FIXED_SELECTOR = PinnedSimpleProver.balance.selector;
    uint256 public constant FIXED_SETTLE_CHAIN_ID = 31_337;
    uint256 public constant FIXED_SETTLE_BLOCK_NUMBER = 6;
    bytes32 public constant FIXED_GROTH16_SETTLE_BLOCK_HASH =
        bytes32(0x108374c602edb7492fcb83f266c022056980cc292b6c75c6cbdcd35c0ddb5020);
    bytes32 public constant FIXED_FAKE_SETTLE_BLOCK_HASH =
        bytes32(0x7b7f1af934e42703670329a42b6df211fbafa0c8b2a6805816f9db6e9ea47dc0);

    address public constant FIXED_OWNER = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
    uint256 public constant FIXED_BALANCE = 10000000;

    function fakeProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x437de4f7c2e014d2062a44164ac4b7b3613d4f02da31293f3a33215d06a2d981),
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
            bytes32(0x2d1c27ce49d0d02887b2389952390eeb0aa01c385668de1321256b11ac94fa9b),
            bytes32(0x2f53731dfa5d3e861335c39417fd033522d31210220581878065d0ef278e97be),
            bytes32(0x062f43a31905646a546681cbca0cdd9b0fbf123074f80cc0fdc0292fdb6b70ca),
            bytes32(0x194711a30a0493b3a0d5a8cc2f60c83ca123816c0ec2cee72215bf1b8ee5ba85),
            bytes32(0x0a7bdccce263f3c88a2068d6679a3fb134e6ba798fb1370dec93492c4cec9a91),
            bytes32(0x176903a80c73d733739f8b75798625a770811d448d598507eb281c6648f23843),
            bytes32(0x2ee28cf9759f80b1c4dea8b7727e6e2e8c08c5676c7aced91666cd7043a11ffe),
            bytes32(0x1db943bcdac6ed047178dce405f09b0d48e23d447353c2fe9557d2eb20bfb42f)
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
