FROM --platform=linux/amd64 ubuntu:latest AS vlayer-builder
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    git \
    build-essential \
    clang 

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
FROM --platform=linux/amd64 vlayer-builder AS vlayer-prepare

WORKDIR /home/vlayer/rust
COPY rust .
RUN cargo chef prepare --recipe-path recipe.json

#################################################################
FROM --platform=linux/amd64 vlayer-builder AS vlayer-cook
ARG cargo_profile

WORKDIR /home/vlayer/rust
COPY --from=vlayer-prepare /home/vlayer/rust/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry,uid=1001,gid=1001 \
    --mount=type=cache,target=/home/vlayer/rust/target,uid=1001,gid=1001 \
    cargo chef cook ${cargo_profile} --recipe-path recipe.json
 
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/home/vlayer/.foundry/bin:${PATH}"
RUN foundryup

COPY rust .
ADD contracts /home/vlayer/contracts
RUN --mount=type=cache,target=/usr/local/cargo/registry,uid=1001,gid=1001 \
    --mount=type=cache,target=/home/vlayer/rust/target,uid=1001,gid=1001 \
    cargo build ${cargo_profile} --workspace


# FROM vlayer-builder as vlayer-build
# RUN mkdir /home/vlayer/vlayer
# ADD contracts /home/vlayer/vlayer/contracts
# ADD rust /home/vlayer/vlayer/rust
# USER root
# RUN chown -R vlayer:vlayer /home/vlayer/vlayer
# RUN chmod 755 /home/vlayer/vlayer

# USER vlayer
# WORKDIR /home/vlayer/vlayer/rust
# # Set the default compiler for building guest
# ENV CC_riscv32im_risc0_zkvm_elf=clang
# # Set required C flags; in particular, in guest, we don't want ring crate to use stdlib
# ENV CFLAGS_riscv32im_risc0_zkvm_elf="-march=rv32im -nostdlib -DRING_CORE_NOSTDLIBINC=1 -D__ILP32__=1"
# # CARGO_NET_GIT_FETCH_WITH_CLI=true will force cargo to use system-provided git instead of using
# # the baked-in libgit2. I have been experiencing spurious network errors when trying to fetch git+https
# # dependencies such as tlsn in Docker, and falling back to system git with forced HTTP1.1 version fixed
# # all issues for me.
# RUN CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build --bin vlayer

# ENTRYPOINT ["/usr/bin/dumb-init", "--", "/home/vlayer/vlayer/rust/target/debug/vlayer"]