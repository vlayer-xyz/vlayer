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
        bytes32(0x2c142a3531ffb7cc3913541cac2a5735496a0f642b35d6c23a0881c1ff4a4d72);
    address public constant FIXED_PROVER_ADDRESS = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0);
    bytes4 public constant FIXED_SELECTOR = PinnedSimpleProver.balance.selector;
    uint256 public constant FIXED_SETTLE_CHAIN_ID = 31_337;
    uint256 public constant FIXED_SETTLE_BLOCK_NUMBER = 6;
    bytes32 public constant FIXED_GROTH16_SETTLE_BLOCK_HASH =
        bytes32(0xf0ac2a4ad2735a2d24a34c10bf493fcf98b58a6fbfc2d79639e25652e34cf89f);
    bytes32 public constant FIXED_FAKE_SETTLE_BLOCK_HASH =
        bytes32(0x68c9d7e5f84b4ec324a15c46330a5994530df55e7f437bfdcc2e48d572dc93bb);

    address public constant FIXED_OWNER = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
    uint256 public constant FIXED_BALANCE = 10000000;

    function fakeProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0xc63b5190f650e4579feedd76edd57342f1de95291f9172f4964e74b9cedbf99c),
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
            bytes32(0x2cce35b1cd14b67a009a78d1868d0a25def9102601c896217b2b61ce5152fbb3),
            bytes32(0x1df9360af6ed2897c5d75818ba39cde5aa84c67f99ea12758c1061aba98f01e3),
            bytes32(0x004c5eb2882f9940f45acf8d1041b34fff99a127dac18255002aa10f29a30e91),
            bytes32(0x2815565e95bc35311e5ad0dfe9594692529dd0c35011e46f1ac4536b3d083afd),
            bytes32(0x02216c3605e64b7ee21911bd9e3a512d50c9034a9e4147dd25c9c9d446eaccd7),
            bytes32(0x16c4ab934e53a6b30af1c19d08de11a7ee99e36646387ceda5dcf6121521b175),
            bytes32(0x0bf04d292bb5b64cfb15d4d592528c160684f580125f32f13720298c080572a1),
            bytes32(0x0753237c2346db7efe693c999d97981c8349b332f4d86bc71290fc7d45e41b26)
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
