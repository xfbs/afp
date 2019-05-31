# Ham Radio Exam Trainer [![Build Status](https://travis-ci.org/xfbs/afp.svg?branch=master)](https://travis-ci.org/xfbs/afp)

Translation: [🇩🇪 Deutsch](README.de.md)

A small tool to study for the (German) ham radio exam. It's written in Rust, uses GTK+3 and [gtk-rs](https://github.com/gtk-rs/gtk) to be cross-platform and look somewhat decent. Documentation is available [here](https://xfbs.github.io/afp/afp).

## Installation

Should work on all Platforms somehow, but only macOS and Linux are officially supported. You need to install GTK+3, which on macOS you can do with

    brew install gtk+3 pkg-config
    export PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig"

You can build it with cargo:

    cargo build

You can run it:

    cargo run

And it's advised to run tests to see if everything works as intended.

    cargo test

## License

See [LICENSE.md](LICENSE.md).
