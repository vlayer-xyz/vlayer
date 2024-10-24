FROM  ghcr.io/foundry-rs/foundry:latest
COPY /test_contracts/ /vlayer/test_contracts
COPY /contracts /vlayer/contracts

# we do not build contract here we make assumption that contract are already built
# to avoid building rust 

WORKDIR /vlayer/contracts
RUN forge soldeer install

WORKDIR /vlayer/test_contracts
RUN forge soldeer install

CMD ["anvil --host 0.0.0.0"]

