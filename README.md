# nanoball — Bouncing ball demo for Sipeed Longan Nano
Bart Massey 2020-03-11

This demo bounces a ball around the LCD screen of the
Sipeed
[Longan Nano](https://www.seeedstudio.com/Sipeed-Longan-Nano-RISC-V-GD32VF103CBT6-Development-Board-p-4205.html)
RISC-V embedded board.

Here's a [video](https://youtu.be/huwbYqZ2chk) showing the
code being developed.

The instructions below are for a reasonably current Debian
Linux host. You are on your own for others.

## Setup

Unplug your Longan Nano from USB if plugged in.

Install `99-ftdi.rules` from this directory in
`/etc/udev/rules.d/` and run

    udevadm control --reload-rules

    cargo install cargo-binutils

    rustup component add llvm-tools-preview

* Get the Rust compiler for this chipset.

You are then ready to build a binary.

    cargo objcopy --release -- -O binary nanoball.bin

Now grab and install the Sipeed `dfu-util` from the link
below. Sadly stock Debian `dfu-util` does not appear to work
with this board.

Hook your Longan Nano to your box via USB and reset it by
holding down the "BOOT" button, pressing and releasing the
"RESET" button, then releasing the "BOOT" button.  Now the
Nano is ready to receive code, so
    sh dfuload.sh nanoball

* Hook your Longan Nano to your box via USB. The loader
  doesn't cope well with USB hubs, so hook directly to your
  host's USB.

* Reset the Longan Nano by holding down the "BOOT" button
  (the taller one in the standard case, on the right when
  USB port is at the bottom), then pressing and releasing
  the "RESET" button (the other one).

* Upload the demo. 

        sh dfuload.sh nanoball

  You may need to power the Nano down and back up afterward.

Now the ball should be bouncing!

## Debugging

You'll need a JTAG unit to get started with this. I'm using
the
[Sipeed USB-JTAG/TTL RISC-V Debugger](https://www.seeedstudio.com/Sipeed-USB-JTAG-TTL-RISC-V-Debugger-p-2910.html),
which is about $8. Connect as follows:

     JTAG      Nano
     TDI       JTDI
     TMS       JTMS
     TDO       JTDO
     TCK       JTCK
     RXD       R0
     TXD       T0
     GND       GND
     
You'll also need a version of OpenOCD that supports the
Nano's processor. Sadly, Debian's is not good enough: you'll
want to build and install
[`riscv-openocd`](https://github.com/riscv/riscv-openocd).

Finally, you'll need `gdb-multiarch` or similar to actually
do the debugging.

Once you have everything set up, get the Nano ready for
upload as described at the beginning of this document
(+BOOT,+RESET,-RESET,-BOOT). Then run `sh openocd.sh`. You
should see `openocd` come up, give some mostly-meaningless
errors, then attach to your Nano. Go to a separate terminal
and run `cargo run --release`. You should now see `gdb`
start up, upload `nanoball.bin`, and start it running. Hit
`^C` now to interrupt the running program. Gratz, you're
debugging away!

## Memory Size

There's an older version of the Longan Nano that has less
memory. I doubt anyone has one anymore. Anyway, all the
stuff to handle these is here, but will require some
rewiring to access.

## Resources

* https://dl.sipeed.com/LONGAN/platformio/dl-packages/

    tool-gd32vflash-v0.1.0-linux.tar.gz

* https://pramode.net/2019/10/07/rust-on-riscv-board-sipeed-longan-nano/

* https://blog.tonari.no/rust-simple-hardware-project
