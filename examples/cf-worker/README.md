# Cloudflare Workers Example

Get `worker-build` with

```sh
cargo install worker-build
```

Build & run with:

```sh
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' worker-build
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' npx wrangler dev
```
## Relevant Docs

- <https://developers.cloudflare.com/workers/languages/rust/>
- <https://docs.rs/getrandom/latest/getrandom/#webassembly-support>
