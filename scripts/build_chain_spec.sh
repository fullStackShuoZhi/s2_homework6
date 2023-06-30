#!/bin/sh
./target/release/node-template build-spec --chain kictto > kictto-chain.json
./target/release/node-template build-spec --chain=kictto-chain.json --raw > kictto-chain-raw.json