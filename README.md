# Tether

[![Cargo](https://img.shields.io/crates/v/tether.svg)](https://crates.io/crates/tether)
[![Documentation](https://docs.rs/tether/badge.svg)](https://docs.rs/tether)

- [Getting Started](#getting-started)
- [Usage on Windows](#usage-on-windows)
- [Platform Information](#platform-information)

Extremely simple bindings to the OS' web view, for anyone who wants to make a
GUI application in Rust.

## Getting Started

There's no getting started guide, but it'd be nice if you made one! On the other
hand, the API surface is *tiny*, so reading an [example](examples) and maybe the
[documentation](https://docs.rs/tether) is the way to go.

## Usage on Windows

You need to add `#![windows_subsystem = "windows"]` to the top of your
`main.rs`, like in the [minimal example](eaxmples/minimal.rs). This tells
Windows that you don't want a console window.

You also have to run the executable in the UWP sandbox, or it'll stack overflow.
The simplest way to do this is to [make an app manifest](https://docs.microsoft.com/en-us/uwp/schemas/appxpackage/how-to-create-a-basic-package-manifest),
making sure to add appropriate permissions, and run the following script after a
build:

```batch
powershell add-appxpackage -register AppxManifest.xml
```

Finally, you need to use the MSVC compiler that comes with Visual Studio,
because MinGW doesn't support C++/CX. Visual Studio also comes with a few
different command prompts, so make sure you use the right one when compiling.

## Platform Information

|------------------|------------|
| Operating System | Library    |
|------------------|------------|
| Windows          | UWP        |
| macOS            | WebKit     |
| Anything Else    | Webkit2GTK |
|------------------|------------|
