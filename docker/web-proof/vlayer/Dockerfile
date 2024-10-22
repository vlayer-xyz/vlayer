
FROM --platform=linux/amd64 ubuntu:24.04 as vlayer-builder
RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates clang curl libssl-dev pkg-config build-essential
RUN curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL 'https://sh.rustup.rs' | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl -L https://risczero.com/install | bash

ENV PATH="/root/.risc0/bin:${PATH}"
RUN rzup install

COPY rust/ /vlayer/rust
COPY contracts/ /vlayer/contracts
WORKDIR vlayer/rust
RUN cargo run --bin vlayer serve \
            --proof "fake" \
            --rpc-url 31337:http://localhost:8545

