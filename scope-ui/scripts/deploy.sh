#!/usr/bin/env bash

HOST=$1
BIN_PATH="~"
BIN_NAME="scope-ui"
BUILD_ARCH=cross-aarch64-linux
LOG_LEVEL=debug

if ! [ -x "$(command -v nix)" ]; then
  echo >&2 "Error: nix is not installed"
  exit 1
fi

# cleanup
ssh "$HOST" "rm -rf $BIN_PATH/$BIN_NAME" 

# build
nix build .#$BUILD_ARCH

# copy
scp -prq "result/bin/$BIN_NAME" "$HOST:$BIN_PATH" 

# run
ssh $HOST "RUST_LOG=$LOG_LEVEL $BIN_PATH/$BIN_NAME"
