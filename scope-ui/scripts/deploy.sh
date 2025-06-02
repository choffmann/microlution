#!/usr/bin/env bash

TARGET=aarch64-unknown-linux-gnu
USER=pi

docker build -t rust-cross-compile .
docker run --rm --user "$(id -u)":"$(id -g)" -v "$(pwd)":/app -w /app rust-cross-compile cargo build --release --target ${TARGET}

#
# scp -i ~/.ssh/id_choffmann -r ./target/$TARGET/release/display $USER@$PI_IP:/tmp/
# ssh -i ~/.ssh/id_choffmann $USER@$PI_IP /tmp/display
