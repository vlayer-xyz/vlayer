FROM --platform=linux/amd64 ubuntu:latest as vlayer-builder
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y --no-install-recommends  curl  ca-certificates

RUN curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL 'https://sh.rustup.rs' | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


RUN curl -L https://risczero.com/install | bash
ENV PATH="/root/.risc0/bin:${PATH}"
RUN rzup install cargo-risczero v1.0.5

RUN curl -L  https://vlayer-releases.s3.eu-north-1.amazonaws.com/latest/nightly-linux-amd64.tar.gz -o nightly-linux-amd64.tar.gz
RUN tar -xvf nightly-linux-amd64.tar.gz 
CMD ["/bin/vlayer", "serve","--proof","fake","--host", "0.0.0.0", "--rpc-url", "31337:http://anvil:8545"]



