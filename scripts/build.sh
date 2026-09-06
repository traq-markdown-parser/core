#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target}"
cargo build --locked --release --target wasm32-unknown-unknown -p traq-markdown-wasm
cargo run --locked --release -p traq-markdown --features contracts --bin export-contracts -- packages/browser/generated
cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/traq_markdown_wasm.wasm" packages/browser/parser.wasm
cp crates/markdown/THIRD_PARTY_NOTICES.md packages/browser/THIRD_PARTY_NOTICES.md
cp LICENSE packages/browser/LICENSE
cp LICENSE packages/renderer/LICENSE
if command -v node >/dev/null; then NODE_CMD=node; else NODE_CMD=node.exe; fi
"$NODE_CMD" scripts/generate-bindings.mjs
"$NODE_CMD" scripts/contract.mjs
