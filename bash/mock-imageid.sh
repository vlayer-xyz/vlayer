cd "./contracts"
IMAGE_ID_FILE=src/ImageID.sol
rm "${IMAGE_ID_FILE}"

cat <<EOF >"${IMAGE_ID_FILE}"
pragma solidity ^0.8.20;

library ImageID {
    bytes32 public constant RISC0_CALL_GUEST_ID =
        bytes32(0x0000000000000000000000000000000000000000000000000000000000000000);
}
EOF