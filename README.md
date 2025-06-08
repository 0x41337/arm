## Android Rust Mod (ARM)

This is a simple template with as little code as possible to run Rust on Android and create mods with it.

## Prerequisites

Here are some prerequisites to ensure everything works correctly.

- You need to have the `NDK` installed and configured to export `ANDROID_NDK_HOME`.
- You need to have `Rust` installed and configured.
- You need to have `cargo-ndk` installed.

## How to build the library

To build the project you can use the standard command for ndk with cargo and if you want other architectures you can change it without any problems.

```sh
$  cargo ndk -t arm64-v8a build
```

If everything goes well you should have a working library for android that can be loaded with `System.loadLibrary()` or any other known loading method.

## My intention with this

My intention with this project is to register a template for myself, so don't expect support or anything like that here, this is more personal than a general contribution. But I decided to make it public in case someone finds it useful and wants to use it to create Android mods written in Rust.
