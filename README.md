# TinyWx: fetch weather from OpenWeatherMap.

> NOTE: I'm pretty new at this. Any feedback is appreciated!

This is a simple app that uses the OpenWeatherMap API to get the current
weather. I use it for [Polybar](https://polybar.github.io/), so it needs to
display the weather using the least amount of space possible.

## Requirements

- An account on [OpenWeatherMap](https://openweathermap.org/) and
  an [API key](https://home.openweathermap.org/api_keys).
- A [Nerd Font](https://nerdfonts.com/) installed for the weather icons.
  (Modify `wx/src/lib.rs` if your needs differ.)

## Usage

There are two ways to use this app: specifying arguments on the command line, or
using a config file. Note that you cannot use both at the same time.

### Command line

```bash
$ tinywx -c "the hague" -C nl -d icon feels_like -k <YOUR_API_KEY>
 30°
```

### Config file

The config file is in [TOML](https://toml.io/) format. An example file
(`tinywx.toml`) can be found in this project's root directory. Modify it to your
liking and save it somewhere.

```bash
$ tinywx -f ~/.config/tinywx/tinywx.toml
 30°
```

Use `-h` to see available options, and `--help` for the full help text.

## Installation

You will need to have Rust installed. The easiest way is to use `rustup` as 
described on the [Rust website](https://www.rust-lang.org/tools/install).

After you've installed Rust, clone this project and from its root directory, 
use `cargo build --release` to create the binary (`cargo` is part of the 
Rust toolchain). Copy the binary to 
somewhere in your path, for example using `cp ./target/release/tinywx ~/.
local/bin/`.
