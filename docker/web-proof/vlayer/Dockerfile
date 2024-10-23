
FROM --platform=linux/amd64 ubuntu:20.04 as vlayer-builder
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update
RUN apt-get install -y --no-install-recommends git ca-certificates curl libssl-dev pkg-config build-essential gnupg wget lsb-release software-properties-common
RUN curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL 'https://sh.rustup.rs' | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl -L https://risczero.com/install | bash

ENV PATH="/root/.risc0/bin:${PATH}"
RUN rzup install

RUN wget https://apt.llvm.org/llvm.sh
RUN chmod +x llvm.sh
RUN ./llvm.sh 18  # Replace with the desired version
RUN apt-get install -y clang-18

RUN ln -s /usr/bin/clang-18 /usr/bin/clang
RUN PATH="/usr/bin/clang:${PATH}"
RUN clang --version  
RUN clang -print-targets

COPY rust/target /vlayer/rust/target
COPY contracts/ /vlayer/contracts
COPY .cargo/ /vlayer/.cargo

RUN ls '/vlayer/rust/target/debug'
CMD ["/vlayer/rust/target/x86_64-unknown-linux-gnu/vlayer", "serve"]



