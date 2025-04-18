FROM --platform=linux/amd64 ubuntu:24.04 AS build
ARG VLAYER_RELEASE=nightly
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates

RUN curl -L https://vlayer-releases.s3.eu-north-1.amazonaws.com/latest-${VLAYER_RELEASE}/binaries-linux-amd64.tar.gz -o binaries-linux-amd64.tar.gz
RUN mkdir -p vlayer
RUN tar -xzf binaries-linux-amd64.tar.gz -C vlayer

FROM --platform=linux/amd64 ubuntu:24.04
RUN apt-get update && apt-get install -y --no-install-recommends dumb-init curl 

COPY --from=build /vlayer/bin/dns_server /bin/dns_server

EXPOSE 3002

ENTRYPOINT ["/usr/bin/dumb-init", "--", "/bin/dns_server"]
