#!/bin/sh
./target/release/node-template \
  --base-path /tmp/node2 \
  --chain ./kictto-chain-raw.json \
  --ws-port 9947 \
  --rpc-port 9935 \
  --port 30335 \
  --validator \
  --name node2