# Amateurfunkprüfer [![Build Status](https://travis-ci.org/xfbs/afp.svg?branch=master)](https://travis-ci.org/xfbs/afp)

Ein kleines Tool zum üben für die Amateurfunkprüfung. Geschrieben in [Rust](https://rust-lang.org), welches dank [gtk](https://www.gtk.org) und [gtk-rs](https://gtk-rs.org/) auf allen möglichen Platformen läuft. Es steht unter der [MIT Lizenz](LICENSE.md).

## Installieren

Das Tool kann mit `cargo` kompiliert und ausgeführt werden. Unter umständen

    export PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig"
    cargo build
    cargo run

Mitgelieferte tests können ebenso ausgeführt werden.

    cargo test

