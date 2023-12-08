<h1 align="center">syncthing_ctl</h1>

<h3 align="center">Starting <a href="https://github.com/syncthing/syncthing">Syncthing</a> Automatically on Windows</h3>

## Install

To install **syncthing_ctl** from source, first ensure you have Rust and Cargo. If not, install them via [rustup.rs](https://rustup.rs).

```sh
cargo install --git https://github.com/zudo/syncthing_ctl
```

This program assumes Syncthing is located at `C:\syncthing\syncthing.exe` by default.

## Usage

To enable or disable automatic startup, simply run `syncthing enable` or `syncthing disable`, respectively.

```sh
syncthing enable -- -no-browser
```

For manual control, use `syncthing start` and `syncthing stop` to start and stop Syncthing.
