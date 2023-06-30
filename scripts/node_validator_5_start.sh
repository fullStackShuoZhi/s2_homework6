#!/bin/sh
./target/release/node-template \
  --base-path /tmp/node5 \
  --chain ./kictto-chain-raw.json \
  --ws-port 9950 \
  --rpc-port 9938 \
  --port 30338 \
  --validator \
  --name node5