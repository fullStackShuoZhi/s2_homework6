#!/bin/sh
./target/release/node-template key insert \
  --base-path /tmp/node0 \
  --chain kictto-chain-raw.json \
  --scheme Sr25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type babe

./target/release/node-template key insert \
  --base-path /tmp/node0 \
  --chain kictto-chain-raw.json \
  --scheme Ed25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type gran

./target/release/node-template key insert \
  --base-path /tmp/node1 \
  --chain kictto-chain-raw.json \
  --scheme Sr25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type babe

./target/release/node-template key insert \
  --base-path /tmp/node1 \
  --chain kictto-chain-raw.json \
  --scheme Ed25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type gran

./target/release/node-template key insert \
  --base-path /tmp/node2 \
  --chain kictto-chain-raw.json \
  --scheme Sr25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type babe

./target/release/node-template key insert \
  --base-path /tmp/node2 \
  --chain kictto-chain-raw.json \
  --scheme Ed25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type gran

./target/release/node-template key insert \
  --base-path /tmp/node3 \
  --chain kictto-chain-raw.json \
  --scheme Sr25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type babe

./target/release/node-template key insert \
  --base-path /tmp/node3 \
  --chain kictto-chain-raw.json \
  --scheme Ed25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type gran

./target/release/node-template key insert \
  --base-path /tmp/node4 \
  --chain kictto-chain-raw.json \
  --scheme Sr25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type babe

./target/release/node-template key insert \
  --base-path /tmp/node4 \
  --chain kictto-chain-raw.json \
  --scheme Ed25519 \
  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
  --password-interactive \
  --key-type gran

#./target/release/node-template key insert \
#  --base-path /tmp/node5 \
#  --chain kictto-chain-raw.json \
#  --scheme Sr25519 \
#  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
#  --password-interactive \
#  --key-type babe
#
#./target/release/node-template key insert \
#  --base-path /tmp/node5 \
#  --chain kictto-chain-raw.json \
#  --scheme Ed25519 \
#  --suri "icon coral pioneer chalk story seek picture tone fetch buzz collect broom" \
#  --password-interactive \
#  --key-type gran