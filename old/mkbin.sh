#!/bin/sh
# XXX Obsolete binary build script.
# Use `cargo objcopy` instead: see the README
# in this distribution.
if [ $# -eq 0 ]
then
    echo "mkbin.sh: usage: mkbin.sh binary [buildmode]" >&2
    exit 1
fi
TARGET="$1"
shift
BUILDMODE=release
case "$1" in
    --debug) BUILDMODE=debug; shift;;
esac
case "$BUILDMODE" in
    debug) BUILDFLAG="" ;;
    release) BUILDFLAG="--release" ;;
    *) echo "mkbin.sh: unknown buildmode $BUILDMODE" >&2; exit 1 ;;
esac
cargo build $BUILDFLAG "$@" &&
riscv-nuclei-elf-objcopy -O binary \
  --only-section=.text --only-section=.rodata \
  target/riscv32imac-unknown-none-elf/$BUILDMODE/$TARGET \
  $TARGET.bin
