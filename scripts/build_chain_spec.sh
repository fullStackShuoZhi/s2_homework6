#!/bin/sh
./target/release/node-template build-spec --chain staging > kchain-staging.json
./target/release/node-template build-spec --chain=kchain-staging.json --raw > kchain-staging-raw.json