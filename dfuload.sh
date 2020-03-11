#!/bin/sh
dfu-util -a 0 --dfuse-address 0x08000000:leave -D $1.bin
