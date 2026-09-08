# Examples

リポジトリのルートで一度 `npm ci` と `npm run build` を実行してください。
各例は traQ V1 で本文を解析し、数学拡張を取り除いた独自構成も作ります。

| 言語 | 実装 | ルートからの実行 |
|---|---|---|
| Rust | [main.rs](rust/src/main.rs) | `npm run example:rust` |
| Go | [main.go](go/main.go) | `npm run example:go` |
| TypeScript | [main.ts](typescript/main.ts) | `npm run example:ts` |

まとめて実行する場合は `npm run examples`、TypeScript の型検査は `npm run typecheck` です。
各例は AST を標準出力へ出します（Rust は型付きの Debug 表示）。TypeScript は markdown-it による HTML 化も示します。

Rust は workspace、Go は examples/go/go.mod の相対 replace、
TypeScript は npm workspace でこのリポジトリの SDK を参照します。
パッケージを先にレジストリへ公開する必要はありません。

新しい型付き AST の通知・参照抽出の例は
[notification.rs](../crates/trap/traq-processing/examples/notification.rs) です。
`cargo run -p markdown-traq-processing --example notification` で実行できます。
文法・通知・抽出をそれぞれの preset から構成し、同じ AST を共有します。

## 利用時の要点

Runtime と Parser はメッセージごとに作り直さず、アプリの適切な単位で保持してください。
Go / TS の Grammar を閉じても、そこから作成済みの Parser は引き続き有効です。
Parser と Runtime の明示解放も例に含めています。Rust では Drop が解放を担います。

Go の例は `go -C examples/go run . -wasm /path/to/parser.wasm` で artifact を指定できます。
Go SDK と Wasm は同じビルドの catalog を使ってください。

TypeScript の例は Node.js で実行します。
ブラウザーでは readFile の部分を `fetch(wasmUrl)` と `response.arrayBuffer()` に置き換え、
同じ loadRuntime を利用できます。

描画例は基本の renderer を使います。stamp・数式など、表示 handler を指定していない拡張は原文を表示します。
traQ 用の renderer 構成にはアプリ側の表示ルールを前提とする token もあるため、
実アプリでは必要な表示 handler とリンク方針を組み合わせてください。
