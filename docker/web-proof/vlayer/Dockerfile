FROM --platform=linux/amd64 ubuntu:latest as vlayer-builder
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y --no-install-recommends  curl  ca-certificates

RUN curl -L  https://vlayer-releases.s3.eu-north-1.amazonaws.com/latest/nightly-linux-amd64.tar.gz -o nightly-linux-amd64.tar.gz
RUN tar -xvf nightly-linux-amd64.tar.gz 
CMD ["/bin/vlayer", "serve","--proof","fake","--host", "0.0.0.0", "--rpc-url", "31337:http://anvil:8545"]



