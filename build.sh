#!/bin/sh
export https_proxy=http://127.0.0.1:7890 http_proxy=http://127.0.0.1:7890 all_proxy=socks5://127.0.0.1:7890

while true
do
#cargo install cargo-contract
cargo build --release --features runtime-benchmarks
if [ "$?" = "0" ]; then
	echo 'install success';
	exit;
else
	echo 'retry install...';
fi
done