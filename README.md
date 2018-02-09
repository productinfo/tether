# Tether

[![Cargo](https://img.shields.io/crates/v/tether.svg)](https://crates.io/crates/tether)

- [Getting Started](#getting-started)
- [Usage on Windows](#usage-on-windows)
- [Platform Information](#platform-information)
- [Comparison with zserge/webview](#comparison-with-zsergewebview)

Extremely simple bindings to the OS' web view, for anyone who wants to make a
GUI application in Rust.

## Getting Started

There's no getting started guide, but it'd be nice if you made one! Meanwhile,
the API surface is *tiny*, so reading an [example](examples) is the way to go.

## Usage on Windows

You need to add `#![windows_subsystem = "windows"]` to the top of your
`main.rs`, like in the [minimal example](examples/minimal.rs). This tells
Windows that you don't want a console window. You also have to run the
executable in the UWP sandbox, or it'll stack overflow. The simplest way to do
this is to [make an app manifest](https://docs.microsoft.com/en-us/uwp/schemas/appxpackage/how-to-create-a-basic-package-manifest),
making sure to add appropriate permissions, and run the following script after a
build:

```batch
powershell add-appxpackage -register AppxManifest.xml
```

Finally, you need to use the MSVC compiler that comes with Visual Studio,
because MinGW doesn't support C++/CX. Visual Studio also comes with a few
different command prompts, so make sure you use the right one when compiling.

## Platform Information

| Operating System | Library    | System Requirements |
| ---------------- | ---------- | ------------------- |
| Windows          | UWP        | Windows 10          |
| macOS            | WebKit     | macOS 10.10         |
| Anything Else    | Webkit2GTK | Webkit2GTK 2.8      |

## Comparison with zserge/webview

In my opinion the main benefit of this library is that it's a **lot more
ergonomic**, but there are some technical differences, too.

| Feature | Tether | Libraries Based on [zserge/webview](https://github.com/zserge/webview) |
| --- | --- | --- |
| System Requirements | Higher | **Lower** |
| Dialogs | Use Libraries | **Builtin** |
| Engine on Windows | **EdgeHTML (Microsoft Edge)** | MSHTML (Internet Explorer) |
| String Escaping | **Builtin** | External Libraries |
| iOS + Android Support | Planned™️ | ??? |
