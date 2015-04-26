# emdrv
[![Build Status](https://travis-ci.org/RustyGecko/emdrv.svg)](https://travis-ci.org/RustyGecko/emdrv)

Bindings for Silicon Labs emdrv, drivers for [emlib](https://github.com/RustyGecko/emlib).

## Building
The library needs to be built for the ARM Coretex M3. Also, a kit needs to be specified as a feature.
```
$ cargo build --target thumbv7m-none-eabi --features stk3700
```

## Using
Add as a target-dependency in your `Cargo.toml`.
```toml
[target.thumbv7m-none-eabi.dependencies.emdrv]
git = "https://github.com/RustyGecko/emdrv.git"
```
