# wawk-hello

Example wawk plugin -- a minimal greeting plugin for learning plugin development.

## Overview

This plugin demonstrates the wawk plugin architecture using WIT (WebAssembly Interface Types) bindings. It provides greeting functions that can be called from AWK scripts through the plugin dispatch system.

## Functions

| Function | Args | Description |
|----------|------|-------------|
| greet(name) | 1 | Returns "Hello, {name}!" |
| greet_lang(name) | 1 | Returns a greeting in one of 10 languages |

## Build

```bash
./build.sh
```

## Security

This plugin runs in the wawk Wasm sandbox. It has no network access, no filesystem access, and no ability to execute system commands. See the [wawk-rs SECURITY.md](https://github.com/ailurlabs/wawk-rs/blob/main/SECURITY.md) for details on the sandbox architecture.

## License

MIT
