FROM --platform=linux/amd64 ghcr.io/foundry-rs/foundry:latest
COPY /test_contracts/web-proof/ /app/test_contracts/web-proof
COPY /contracts /app/contracts
WORKDIR /app/test_contracts/web-proof
RUN forge soldeer install

ENTRYPOINT ["sh", "-c", "anvil --host 0.0.0.0 & sleep 2 && forge create src/WebProofProver.sol:WebProofProver --private-key=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url=http://localhost:8545 && tail -f /dev/null"]

