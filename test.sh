#/bin/bash

LEAF_ROOT="./leafroot"
DL_CACHE="./dl_cache"
LOGLEVEL="warn"

set -e

DIR=$(dirname "$0")
ROOTDIR="$DIR/$LEAF_ROOT"

cd "$DIR" || exit
cargo build

echo "Removing old root directory $ROOTDIR"
rm -rf "$ROOTDIR"

echo "Update"
RUST_LOG="$LOGLEVEL" "$DIR"/target/debug/leaf --root "$ROOTDIR" update

echo "Install"
RUST_LOG="$LOGLEVEL" "$DIR"/target/debug/leaf --root "$ROOTDIR" --download-cache "$DL_CACHE" install "$@"
