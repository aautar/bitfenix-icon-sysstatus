# bitfenix-icon-sysstatus
bitfenix-icon-system status is a small utility program to surface some basic system stats to the [Bitfenix ICON display](https://www.bitfenix.com/products/chassis-2/micro-atx/pandora/).

## Things to know
- The display is written to via the HIDAPI display
- The display has a very low refresh rate and the screen goes black during an update
- The display is a 240x320 with 16-bit color (5-6-5 format)
- This is a Rust program intended to be run on a Ubuntu system

## Technical details
For information on how to write to the display and techniques used in this program see:
- [Writing to the Bitfenix ICON display with Rust (part 1, writing images)](https://semisignal.com/writing-to-the-bitfenix-icon-display-with-rust/)
- [Writing to the Bitfenix ICON display with Rust (part 2, writing text)](https://semisignal.com/writing-to-the-bitfenix-icon-display-with-rust-part-2-writing-text/)
