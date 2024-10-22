
FROM --platform=linux/amd64 ubuntu:24.04 as vlayer-builder
RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates clang curl libssl-dev pkg-config build-essential gnupg wget
RUN curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL 'https://sh.rustup.rs' | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
# RUN curl -L https://risczero.com/install | bash

# ENV PATH="/root/.risc0/bin:${PATH}"
# RUN rzup install

RUN wget --no-check-certificate -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN echo "deb http://apt.llvm.org/focal/ llvm-toolchain-focal main" >> /etc/apt/sources.list
RUN apt-get update
RUN apt-get install -y clang-18 lldb-18 lld-18

COPY rust/ /vlayer/rust
COPY contracts/ /vlayer/contracts
WORKDIR vlayer/rust
RUN cargo build

