FROM --platform=linux/amd64 ubuntu:latest AS vlayer-base
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    git \
    build-essential \
    clang \
    dumb-init

RUN git config --global http.version HTTP/1.1

RUN useradd -m -s /bin/bash -u 1001 vlayer
USER vlayer
WORKDIR /home/vlayer

RUN curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL 'https://sh.rustup.rs' | sh -s -- -y
ENV PATH="/home/vlayer/.cargo/bin:${PATH}"

RUN rustup update
RUN cargo install cargo-chef --locked

RUN curl -L https://risczero.com/install | bash
ENV PATH="/home/vlayer/.risc0/bin:${PATH}"
RUN rzup install cargo-risczero v1.2.0
RUN cargo risczero install

ENV CC_riscv32im_risc0_zkvm_elf=clang
ENV CFLAGS_riscv32im_risc0_zkvm_elf="-march=rv32im -nostdlib -DRING_CORE_NOSTDLIBINC=1 -D__ILP32__=1"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

#################################################################
FROM --platform=linux/amd64 vlayer-base AS vlayer-prepare

WORKDIR /home/vlayer/rust
COPY rust .
RUN cargo chef prepare --recipe-path recipe.json

#################################################################
FROM --platform=linux/amd64 vlayer-base AS vlayer-cook
ARG cargo_profile=debug

WORKDIR /home/vlayer/rust
COPY --from=vlayer-prepare /home/vlayer/rust/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry,uid=1001,gid=1001 \
    --mount=type=cache,target=/home/vlayer/rust/target,uid=1001,gid=1001 \
    export build_flag=$(if [ "$cargo_profile" = "release" ]; then echo "--release"; else echo ""; fi) && \
    cargo chef cook ${build_flag} --recipe-path recipe.json

RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/home/vlayer/.foundry/bin:${PATH}"
RUN foundryup

COPY rust .
ADD contracts /home/vlayer/contracts
RUN --mount=type=cache,target=/usr/local/cargo/registry,uid=1001,gid=1001 \
    --mount=type=cache,target=/home/vlayer/rust/target,uid=1001,gid=1001 \
    export build_flag=$(if [ "$cargo_profile" = "release" ]; then echo "--release"; else echo ""; fi) && \
    cargo build ${build_flag} --workspace && \
    mkdir bin && \
    cp target/${cargo_profile}/chain_server bin && \
    cp target/${cargo_profile}/vlayer bin && \
    cp target/${cargo_profile}/worker bin && \
    cp -r target/assets assets

#################################################################
FROM --platform=linux/amd64 ghcr.io/foundry-rs/foundry:latest AS vlayer-contracts

COPY --from=vlayer-cook /home/vlayer/contracts /home/vlayer/contracts
COPY --from=vlayer-cook /home/vlayer/rust/assets/ /home/vlayer/contracts/vlayer/src/

WORKDIR /home/vlayer/contracts/vlayer
RUN forge soldeer install
RUN forge build

#################################################################
FROM --platform=linux/amd64 vlayer-base AS vlayer-bin

COPY --from=vlayer-cook /home/vlayer/rust/bin/ /home/vlayer/
WORKDIR /home/vlayer

ENTRYPOINT ["/usr/bin/dumb-init", "--"]
