# traQ Markdown parser

Rust で文法を定義し、Go と TypeScript / JavaScript から同じ WebAssembly パーサーを使うライブラリです。
CommonMark 0.31.2、汎用の拡張、traQ 固有の拡張を分離し、preset の利用と文法の組み立てに対応します。

- [API と拡張の使い方](docs/implementation.md)
- [実行できる examples](examples/README.md)
- [開発・ビルド・テスト](CONTRIBUTING.md)

## 試す

Node.js 24 以降、Go 1.25 以降、Rust が必要です。
Rust のバージョンと Wasm target は [rust-toolchain.toml](rust-toolchain.toml) で固定しています。
Windows でも WSL や Bash を使わずにビルド・実行できます。

```sh
npm ci
npm run build
npm run examples
```

Wasm はビルド時に生成します。ソース管理には含めません。

## API

Rust:

```rust
use markdown_traq::presets;

let parser = presets::traq::v1::parser();
let document = parser.parse("**hello** :stamp:")?;
let inline = parser.parse_inline("**hello**")?;
```

TypeScript:

```ts
import { loadRuntime } from '@traptitech/markdown-parser'

const runtime = await loadRuntime(wasmBytes)
try {
  const parser = runtime.parser(runtime.presets.traq.v1)
  try {
    const document = parser.parse('**hello** :stamp:')
  } finally {
    parser.dispose()
  }
} finally {
  runtime.dispose()
}
```

Go:

```go
runtime, err := core.New(ctx, wasmBytes)
if err != nil { return err }
defer runtime.Close(ctx)

parser, err := runtime.Parser(ctx, runtime.Presets.TraQ.V1)
if err != nil { return err }
defer parser.Close()

result, err := parser.Parse(ctx, "**hello** :stamp:")
if err != nil { return err }
```

同じ Parser を繰り返し利用できます。Go の Parser は並行利用にも対応します。
組み込み preset は必要になるまで文法をコンパイルせず、作成済みの構成は共有します。
独自構成の builder.build / Build はその場で検証・構築します。

保存済みメッセージの文法版と Parser の対応は利用側で管理します。
SDK と AST に文法版や永続的な Profile ID はありません。
現在の配布契約は ABI 2 / AST 4。Wasm と生成済み binding を組にして利用します。

## 構成

| ディレクトリ | 内容 |
|---|---|
| [crates/core](crates/core/README.md) | 文法に依存しない AST、共有宣言、parser、renderer、extractor、codec |
| [crates/commonmark](crates/commonmark/README.md) | CommonMark と汎用拡張のノード契約・構文・テキスト描画 |
| [crates/trap](crates/trap/README.md) | traP 固有の契約・構文・描画・抽出と traQ preset |
| crates/wasm | Wasm の入出力とリソース管理 |
| go | Go SDK と型付き AST |
| packages/browser | TypeScript / JavaScript SDK |
| packages/renderer | AST から markdown-it token への描画アダプター |
| examples | 各言語の利用例 |
| scripts | ビルド、binding 生成、配布物の検証 |
| tests | JS と配布物のテスト |
| docs | 現行 API の説明 |

Rust の主要実装は core・CommonMark・traP の3系列で管理します。
各系列の内部は複数の Cargo package に分け、契約と実装、parser と renderer の境界を維持します。
同系列の package は同じ版で管理し、系列間では独立して版を上げられます。
workspace は全体で一つです。構成と更新手順は [開発](CONTRIBUTING.md#rust-の管理単位) を参照してください。

公開 TS / Go SDK は型付き AST と全ノード共通の通信形式へ移行しました。
[型付き parser core](crates/core/parser/README.md) と [文法 preset](crates/trap/traq/README.md) を利用します。
[通知と参照抽出の Rust 例](crates/trap/traq-processing/examples/notification.rs) は、同じ AST を両処理に渡します。
通知・抽出を含む統合 backend SDK の公開移植は引き続き後続作業です。
新しい codec は `#[derive(NodeType)]` を付けた型を `register::<T>()` で登録します。
AST の長期保存は想定せず、原文と利用側が管理する文法版から再生成します。

このプロジェクトは [MIT License](LICENSE) で公開します。
著作権表記: Copyright (c) 2026 東京工業大学デジタル創作同好会traP。

第三者コードの通知は [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)、
公開テストデータの出典・ライセンスは [fixtures/README.md](tests/fixtures/README.md) にあります。
