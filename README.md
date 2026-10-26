# wawk-hello

Example wawk extension -- a minimal greeting extension for learning extension development.

## Overview

This extension demonstrates the wawk v1.0 extension architecture using WIT (WebAssembly Interface Types) bindings. It provides greeting functions that can be called from AWK scripts through the extension dispatch system.

- **Namespace:** `hello`
- **Capabilities:** functions

## Functions

| Function | Args | Description |
|----------|------|-------------|
| `greet(name)` | 1 | Returns "Hello, {name}!" |
| `greet_lang(name)` | 1 | Returns a greeting in one of 10 languages |

## Build

```bash
./build.sh
```

`build.sh` runs the tests and then builds a WASM **component** with
`cargo component build --release --target wasm32-wasip1`, copying the result to
`./wawk-hello.wasm`.

The wasip1 component target is required: it is the artifact `wac plug` can
compose into the wawk hosts (wawk-wasi, wawk-bindgen, wawk-mcp). A plain
`cargo build --target wasm32-unknown-unknown` produces a core module that
cannot be plugged, so do not substitute it.

To compose this extension into a host manually:

```bash
wac plug --plug wawk-hello.wasm <host-component>.wasm -o composed.wasm
```

## Security

This extension runs in the wawk Wasm sandbox. It has no network access, no filesystem access, and no ability to execute system commands. See the wawk-rs SECURITY.md for details on the sandbox architecture.

## License

MIT
