# Yet Another Rust Implementation of the Cryptopals Challenges

**WARNING**: *Spoilers inside*.

If you're doing the Cryptopals Crypto Challenges for yourself, I recommend you don't read this code! It's much more fun if you work it out on your own. Really, it is.

If you've already done the challenges and are interested in different ways to do it using different techniques and different programming languages, welcome and read on!

## Running

First, make sure you've got Rust installed (duh). [Rustup](https://www.rustup.rs) is the recommended way of installing Rust, but you know that, right?

To run without printing output:

```sh
$ cargo test
```

Simple! If you see ''test-result: OK'' print it, it means the code is working. Hoorah!

To run with output printed, and also with backtraces printed out for use during development:

```sh
$ RUST_BACKTRACE=1; cargo test -- --nocapture
```

Now you've got no excuse to get coding.

## Any Questions?

If you've got any questions, please write them all down in invisible ink on a roll of parchment, squeeze the parchment into a glass bottle and throw into the Atlantic Ocean. If you haven't heard back within 13 days, it means that the mighty Ægir has deemed your questions unworthy of response.

Or, y'know, just use email or whatever.
