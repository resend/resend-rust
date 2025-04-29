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
> [!NOTE] 
> WASM support was added to `resend-rust` in `v0.12.0`

## Relevant Docs

- <https://developers.cloudflare.com/workers/languages/rust/>
- <https://docs.rs/getrandom/latest/getrandom/#webassembly-support>
