#!/bin/bash

mkdir -p out/notary-keys

# Generate private key
openssl ecparam -name secp256k1 -genkey -noout | openssl pkcs8 -topk8 -nocrypt -inform PEM -out out/notary-keys/notary.key

# Extract the public key from the private key
openssl ec -in out/notary-keys/notary.key -pubout -out out/notary-keys/notary.pub
