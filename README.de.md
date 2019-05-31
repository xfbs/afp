# Amateurfunkprüfer [![Build Status](https://travis-ci.org/xfbs/afp.svg?branch=master)](https://travis-ci.org/xfbs/afp)

Übersetzung: [🇬🇧 Englisch](README.md)

Ein kleines Tool zum Üben für die Amateurfunkprüfung. Geschrieben in [Rust](https://rust-lang.org), welches dank [gtk](https://www.gtk.org) und [gtk-rs](https://gtk-rs.org/) auf allen möglichen Platformen läuft. Es steht unter der [MIT Lizenz](LICENSE.md).

## Installieren

Es wird GTK+3 benötigt, das kann (unter macOS) einfach mit Homebrew installiert werden. Außerdem muss ein Pfad gesetzt werden, damit pkg-config libffi findet.

    brew install gtk+3 pkg-config
    export PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig"

Das Tool kann mit `cargo` kompiliert und ausgeführt werden.

    cargo build
    cargo run

Mitgelieferte Tests können ebenso ausgeführt werden.

    cargo test

## Lizenz

Siehe [LICENSE.md](LICENSE.md).
