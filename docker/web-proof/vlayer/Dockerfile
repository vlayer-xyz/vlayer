FROM --platform=linux/amd64 ubuntu:latest as vlayer-builder

# update and install needed packages
RUN apt-get update
RUN apt-get install -y --no-install-recommends  curl  ca-certificates

# install rust and add it to the path 
RUN curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL 'https://sh.rustup.rs' | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


# install risc0 in version v1.0.5! (important for vlayer compatibility) 
RUN curl -L https://risczero.com/install | bash
ENV PATH="/root/.risc0/bin:${PATH}"
RUN rzup install cargo-risczero v1.0.5

# install vlayer from last nightly build hosted on s3
RUN curl -L  https://vlayer-releases.s3.eu-north-1.amazonaws.com/latest/nightly-linux-amd64.tar.gz -o nightly-linux-amd64.tar.gz
RUN tar -xvf nightly-linux-amd64.tar.gz 

#this makes assumption that anvil is running on the same network as vlayer
CMD ["/bin/vlayer", "serve","--proof","fake","--host", "0.0.0.0", "--rpc-url", "31337:http://anvil:8545"]



