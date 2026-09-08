# 開発

## セットアップ

- Node.js 24 以降と npm
- Go 1.25 以降
- Rust（rustup が rust-toolchain.toml を参照）

Windows は Rust の MSVC toolchain 用の C++ ビルドツールも必要です。
WSL や Bash は不要で、PowerShell から以下のコマンドをそのまま実行できます。

```sh
npm ci
npm run build
```

npm workspace が SDK と examples を接続します。別のアプリの checkout は不要です。
scripts/build.mjs は Wasm をビルドし、Rust の定義から型・codec・catalog を生成します。
生成済み binding はソース管理に含めます。Wasm と artifact のハッシュを含む contract.json はビルド成果物です。

## 検証

```sh
npm test
npm run typecheck
npm run test:rust
npm run test:go
npm run examples
npm run check:package
npm run check:architecture
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
[fixture の説明](tests/fixtures/README.md)に出典と更新方針を記載しています。
通常のテストは実メッセージや認証情報を必要としません。

## 変更箇所

CommonMark 文法は crates/commonmark/syntax、汎用拡張は crates/commonmark/extensions/syntax、
traP 拡張は crates/trap/syntax、traQ の文法構成は crates/trap/traq を変更します。
ノード型は各 contracts crate に宣言し、Wasm 配布層の node_types.rs に収録します。
この一つの一覧から codec 登録と全ノードの binding を生成します。
check:architecture は新しい core と文法 package の推移的な依存方向を検査します。
生成された TypeScript / Go のファイルは直接編集しません。
API と所有権の約束は [実装と利用](docs/implementation.md) を参照してください。

## Rust の管理単位

ルートの一つの workspace に、以下の3系列を収録します。各ディレクトリは Cargo package ではなく、
配下の package をまとめて開発・リリースする単位です。入れ子の workspace は作りません。

- [core](crates/core/README.md): 文法に依存しない共通機構。
- [CommonMark](crates/commonmark/README.md): CommonMark と汎用拡張。
- [traP](crates/trap/README.md): traP 固有の拡張と traQ preset。

通常の依存は core → core、CommonMark → core / CommonMark、traP → 全3系列を許可します。
系列内にも contracts / syntax / text などの crate 境界を設け、既存の依存方向を維持します。
テスト用の dev-dependencies は、上位系列の preset や fixture を利用できます。

各系列の version は配下の Cargo.toml に明記し、系列を更新するときに同じ版へ揃えます。
依存 API に互換性があれば、他系列の版を合わせて上げる必要はありません。
公開時は系列内でも複数の Cargo package になるため、
依存先の版指定を更新し、依存順に公開します。現在の publish = false は公開準備が完了するまで維持します。
Wasm 配布層と example の版は補助 package 用の workspace.package.version を使います。

`npm run check:architecture` は系列内の版の一致、系列間の依存方向、crate ごとの依存制限を検査します。
新しい crate を追加する場合は該当系列に置き、ルート workspace の members に登録してください。

公開対象はライブラリ、テスト、生成済み binding、examples、現行の利用・開発ドキュメントです。
試作、アプリへの実験用パッチ、過去の設計資料、ローカルの検証ツールは .gitignore で除外しています。
これらを除いた checkout でビルド・テストできる状態を維持します。
