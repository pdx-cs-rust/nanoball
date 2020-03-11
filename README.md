# nanoball — Bouncing ball demo for Sipeed Longan Nano
Bart Massey 2020-03-11

This demo bounces a ball around the LCD screen of the
Sipeed
[Longan Nano](https://www.seeedstudio.com/Sipeed-Longan-Nano-RISC-V-GD32VF103CBT6-Development-Board-p-4205.html)
RISC-V embedded board.

Here's a [video](https://youtu.be/huwbYqZ2chk) showing the
code being developed.

## Build and Run

Currently only tried on Linux host. You are on your own
for others.

### Setup

    rustup target add riscv32imac-unknown-none-elf

Now grab and install the Sipeed tools from the links
below. Sadly stock Debian `dfu-util` does not appear to work
with this board.

    cargo build --release
    sh mkbin.sh

Now hook your Longan Nano to your box via USB and reset it
by holding down the "BOOT" button, then pressing and
releasing the "RESET" button.

    sh dfuload.sh

You may need to power the Nano down and back up
afterward. At that point, the ball should be bouncing!

## Resources

* https://dl.sipeed.com/LONGAN/platformio/dl-packages/

    tool-gd32vflash-v0.1.0-linux.tar.gz

    toolchain-gd32v-v9.2.0-linux.tar.gz

* https://pramode.net/2019/10/07/rust-on-riscv-board-sipeed-longan-nano/
