# Markdown core

文法に依存しない Markdown 処理の基盤です。CommonMark や traP の具体的な構文・ノード型・preset は含みません。

| crate | 責務 |
| --- | --- |
| `markdown-ast` | 型付き AST、原文、UTF-8 span |
| `markdown-definitions` / `markdown-definitions-derive` | 宣言の同一性、ノードの型情報 |
| `markdown-parser` | 文法の組み立て、ルール実行、処理量の制限 |
| `markdown-renderer` | AST の描画・テキスト生成基盤 |
| `markdown-extractor` | AST からの情報抽出基盤 |
| `markdown-codec` | 登録されたノード契約に基づく JSON encode / decode |

各 crate の README に API の例があります。

## 開発

Rust は `rust-toolchain.toml` で固定しています。他のリポジトリの checkout は不要です。

```sh
cargo test --workspace --all-features
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run -p markdown-codec --example round_trip
```

## リポジトリの境界

- [commonmark](https://github.com/traq-markdown-parser/commonmark): 標準文法と汎用拡張
- [trap-extension](https://github.com/traq-markdown-parser/trap-extension): traP 固有の構文・描画・抽出部品
- [traq](https://github.com/traq-markdown-parser/traq): traQ の文法・処理の構成、Wasm 配布、TypeScript / Go bindings

core はこれらへ依存しません。テストにも独立した契約型を使います。リポジトリ内の crate は同じ版で管理し、外部からは確定した Git revision を指定して利用します。レジストリへの公開はまだ行っていません。
