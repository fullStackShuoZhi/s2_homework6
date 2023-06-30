#!/bin/sh
./target/release/node-template \
  --node-key 23af1eaa025e4e9e158c8ffeb0ec4aa77e3abc10e7d76c8a70fd926cacaf8f1c \
  --base-path /tmp/node0 \
  --chain ./kictto-chain-raw.json \
  --ws-port 9945 \
  --rpc-port 9933 \
  --port 30333 \
  --validator \
  --name node0