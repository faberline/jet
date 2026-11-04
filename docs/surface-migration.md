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

The complete `scripts/verify-advanced-wasm-gates.sh` run passed with exit 0.
It includes live MUI and AntD DOM/WASM visual journeys, a real WebGPU
large-table probe, lowering gates and Chromium oracle checks. The compiled
generated-counter test also passed after copying the root lockfile into
the generated project.

Strict runtime Clippy reports three existing findings: `large_enum_variant`
in renderer and debug types, and `doc_lazy_continuation` in the text shaper.
The untouched base reproduces all three. No lint suppression or public
enum boxing was added to make this migration appear clean.

Surface is public. Hosted run 41767721585 at implementation commit
`41ecdf95aeaa2723f862d1007fe2d1eb6c7718e1` fetched the dependency and
reported success. Its parity command explicitly skipped the unwired CI
oracle because no results were available. That run proves dependency
access, not hosted parity acceptance. Independent acceptance, release
and merge remain open.
