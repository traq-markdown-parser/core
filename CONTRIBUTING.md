# 開発

## セットアップ

- Node.js 24 以降と npm
- Go 1.25 以降
- Rust（rustup が rust-toolchain.toml を参照）
- Bash

```sh
npm ci
npm run build
```

npm workspace が SDK と examples を接続します。別のアプリの checkout は不要です。
scripts/build.sh は Wasm をビルドし、Rust の定義から型・codec・catalog を生成します。
生成済み binding はソース管理に含めます。Wasm と artifact のハッシュを含む contract.json はビルド成果物です。

## 検証

```sh
npm test
npm run typecheck
npm run test:rust
npm run test:go
npm run examples
npm run check:package
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Go の並行処理を変更した場合は `go -C go test -race ./core` も実行します。
race 検査には対象環境の C コンパイラーが必要です。

Go のテストは生成済み Wasm を実行するため、test:go は `-count=1` で結果キャッシュを使わず実行します。

check:package は npm pack した2つのパッケージを新しい一時ディレクトリへオフライン導入し、
NodeNext の型検査と実行を確認します。先に npm ci と build を実行してください。
tarball は dist に残り、一時 consumer は検証後に削除します。レジストリへの公開は行いません。

Rust のテストは CommonMark の仕様 HTML と、traQ V1 の固定 AST を別々に検証します。
[fixture の説明](crates/markdown/tests/fixtures/README.md)に出典と更新方針を記載しています。
通常のテストは実メッセージや認証情報を必要としません。

## 変更箇所

文法を追加・変更する場合は crates/markdown/src/syntax と preset を編集し、
必要な payload を宣言したうえで build を実行します。
生成された TypeScript / Go のファイルは直接編集しません。
API と所有権の約束は [実装と利用](docs/implementation.md) を参照してください。

公開対象はライブラリ、テスト、生成済み binding、examples、現行の利用・開発ドキュメントです。
試作、アプリへの実験用パッチ、過去の設計資料、ローカルの検証ツールは .gitignore で除外しています。
これらを除いた checkout でビルド・テストできる状態を維持します。
