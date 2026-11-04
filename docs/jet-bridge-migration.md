# Standalone Jet bridge consumer

Jet depends on public repository `faberline/jet-bridge` at immutable commit
`0fa5ffe78a3743f624b07a36c12816643e351716`. The lockfile records the same
Git source. No local checkout path is required.

The old `jet::browser`, `jet::browser_cli` and `jet::cdp_driver` module
paths re-export the standalone library. The `jet browser` and `jet bb`
command definitions and dispatch call its shared CLI. Jet keeps its
builder, transpiler, test runner and development server.

The two cross-module regression cases remain in
`src/bridge_consumer_tests.rs`. They still compare safe locator JSON with
Jet's bundler escaping, and browser shutdown text with the two development
server warnings. Their original assertions are retained.

## Validation

On the pinned Git dependency, native compilation and binary build pass.
Both cross-module cases and all six browser CLI contract cases pass.
The actual MCP server fixture (two cases), semantic page fixture (one
case), browser CLI fixture (three cases) and browser contexts (nine cases)
pass. The browser CLI fixture builds and runs the real debug WASM app.
These journeys use owned Chromium processes.

The first WASM fixture attempt failed while resolving crates.io DNS.
After fetching locked dependencies, the full CLI and context fixtures
passed with offline dependency resolution. That failed attempt was not
counted as acceptance.

This change stacks on the independent Surface migration. It retains that
change's pinned Surface revision. It does not merge either repository,
release a package or qualify the new Surface application bridge.
