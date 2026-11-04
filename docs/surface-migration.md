# Standalone Surface UI dependency

Jet's `surface` and `ui-runtime` packages now come from
[`faberline/surface`](https://github.com/faberline/surface) at
`1754a2e96e487ea693be558041abdf21ffc63d27`. Keep these two revisions equal.
Core still supplies `cli-std` at `v0.4.14`; that CLI dependency is unchanged.

The standalone UI API uses constructors, builders and read methods. Jet's
native test renderer, browser DOM host, WebGPU host, debug bridge, lowering
output and fixtures now use those public methods. Debug fiber IDs cross the
JS boundary as the same integer values through `FiberId::new` and `get`.

Generated Cargo projects depend on this checkout's `jet-wasm` path. They
therefore receive the same immutable UI pins. Source lookup supports the
standalone repository and the earlier `projects/jet/wasm` layout. A project
outside the repository can use the source path recorded when Jet was built.
Installed binary packaging must retain or locate those sources.

The compiled-counter gate builds actual transpiler output in a new Cargo
project. It verifies initial state, event dispatch, flush and the next view.
Its lockfile must contain one `surface` and one `ui-runtime`, both on the
standalone source. Existing snapshot and renderer assertions stay in place.

## Local evidence

On 2026-11-04, macOS and Rust 1.96.1 passed:

- 142 runtime/renderer/text/hook tests with `jet-wasm` debug enabled.
- 83 WASM build configuration and source-location tests.
- 26 lowering tests, including compiling and running the generated counter.
- `wasm32-unknown-unknown` compilation with all Jet WASM host features.

The aggregate `scripts/verify-advanced-wasm-gates.sh` browser/parity gate is
running separately. These local results alone do not establish DOM/WASM
parity, hosted CI, or independent acceptance.

Surface is private at preparation time. Authenticated local builds work.
Public consumers and hosted CI need dependency access before this can land.
No repository visibility, release tag, tracker acceptance or merge changed.
