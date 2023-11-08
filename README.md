# satshunter

Find the correct word order for a shuffled bitcoin seed phrase with a known address.
Perhaps you're playing the [Hunting Sats](https://www.huntingsats.com/) game.

Note that only native SegWit (P2WPKH) (BIP84) addresses are supported at the moment
because they're the most popular. Others can be added quite easily.

While this program uses the [Rayon](https://docs.rs/rayon/latest/rayon/) library to
get parallelization, the use of it could probably be improved upon to get more performance.

## Installation

Embrace [Nix](https://nixos.org/download) and run `cargo build --release`. A binary will be
written to `target/release/satshunter`.

## Usage

```shell
satshunter --words 'hint cheese wife sugar cute boss win twin wall erode blush rival' \
  --address bc1qatpcs4tjj0c0mhfdpdypvxl7l66pcrqlcq02ce
```

Make sure you enclose the words in quotes so they can be parsed properly.

By default, the first 10 addresses are searched. If you want to change this pass
the `--depth` argument.
