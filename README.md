# syncthing_ctl

A super simple program for starting and stopping Syncthing on Windows.

Inspired by <https://docs.syncthing.net/users/autostart.html#autostart-windows-startup> and `systemctl`.

## Starting Syncthing automatically on Windows

This program requires that the [Syncthing](https://github.com/syncthing/syncthing) binary (`syncthing.exe`) is located at `C:\syncthing\syncthing.exe`. For a different location, specify with `--path=C:\path\to\syncthing.exe`.

```sh
syncthing enable
```

This will create a startup script in `C:\Users\<username>\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\syncthing.bat`.
You can navigate to this folder easily by pressing `Win + R` and typing `shell:startup`.

### Passing arguments to Syncthing

Optionally if you want to pass arguments to Syncthing for example `-no-browser`:

```sh
syncthing enable -- -no-browser

```

## Install

To install `syncthing_ctl` from source, first ensure you have Rust and Cargo. If not, install them via <https://rustup.rs/>.

```sh
cargo install syncthing_ctl
```

## Usage

Syncthing can be manually controlled with the `start` and `stop` commands.

To automatically start Syncthing on Windows, use the `enable` command.
To disable automatic startup, use the `disable` command.
