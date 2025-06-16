FROM --platform=linux/amd64 ubuntu:24.04 AS build
ARG VLAYER_RELEASE=nightly
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates

RUN curl -L https://github.com/risc0/risc0/releases/download/v2.1.0/cargo-risczero-x86_64-unknown-linux-gnu.tgz -o cargo-risczero-x86_64-unknown-linux-gnu.tgz
RUN mkdir -p cargo-risczero
RUN tar -xzf cargo-risczero-x86_64-unknown-linux-gnu.tgz -C cargo-risczero

RUN curl -L https://vlayer-releases.s3.eu-north-1.amazonaws.com/latest-${VLAYER_RELEASE}/binaries-linux-amd64.tar.gz -o binaries-linux-amd64.tar.gz
RUN mkdir -p vlayer
RUN tar -xzf binaries-linux-amd64.tar.gz -C vlayer

FROM --platform=linux/amd64 ubuntu:24.04
RUN apt-get update && apt-get install -y --no-install-recommends dumb-init curl
COPY --from=build /cargo-risczero/r0vm /bin/r0vm
COPY --from=build /vlayer/bin/call_server /bin/call_server

ENTRYPOINT ["/usr/bin/dumb-init", "--", "/bin/call_server"]
