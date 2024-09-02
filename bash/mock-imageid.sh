cd "./contracts"
IMAGE_ID_FILE=src/ImageID.sol
rm "${IMAGE_ID_FILE}"

cat <<EOF >"${IMAGE_ID_FILE}"
pragma solidity ^0.8.20;

library ImageID {
    bytes32 public constant RISC0_CALL_GUEST_ID =
        bytes32(0xea31876753a57ad8325b121b5cbd77fab2becc0457755d8024affec18c889944);
}
EOF