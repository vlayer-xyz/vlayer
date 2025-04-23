EXAMPLE=simple-teleport \
VLAYER_ENV=testnet \
CHAIN_NAME=sepolia \
JSON_RPC_URL=https://thrumming-burned-butterfly.ethereum-sepolia.quiknode.pro/bb59d48b08f14892b90bbdc8d2e75d8a58c9874b \
EXAMPLES_TEST_PRIVATE_KEY=0xb83dc6a499a4da322d55a7fef3a85a267064b22a83cfafea4cb4cbc987775e32 \
QUICKNODE_API_KEY=bb59d48b08f14892b90bbdc8d2e75d8a58c9874b \
QUICKNODE_ENDPOINT=thrumming-burned-butterfly \
BUILD_SERVICES=1 \
JWT_AUTH=off \
./bash/e2e-test.sh