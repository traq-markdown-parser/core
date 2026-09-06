# 実装と利用

## 文法から Parser を作る

```rust
use traq_markdown::{presets, Parser, Limits};

let parser = Parser::new(presets::traq::v1::grammar());
let document = parser.parse("**本文** :stamp:")?;
let inline = parser.parse_inline("**本文**")?;
let limited = presets::commonmark::parser().with_limits(Limits {
    input_bytes: 4096, ..Limits::default()
});
```

Grammar は確定した文法で、Parser はその共有データと制限値を保持する。
文法構築時に開始バイトごとの候補ルールと拡張の索引を作る。
本文、参照定義、スタック、処理量は一回の解析に属するため、同じ Parser を反復・並行利用できる。
Rust では元の Grammar を drop しても Parser は有効。

成功は完全な Document、失敗は ParseError。parse は block と inline、parse_inline は inline を解析する。

## Preset と組み立て

```rust
use traq_markdown::{presets, Parser, syntax::extensions::math};

let mut builder = presets::traq::v1::grammar().to_builder();
builder.remove(math::plugin())?;
let grammar = builder.build()?;
let parser = Parser::new(&grammar);
let document = parser.parse("$x$")?; // 数式拡張を除いた構成
```

Preset は通常の Grammar。専用の Profile ID、文法版、hash、別の合成機構はない。
独立した builder への変更は元の preset や作成済み Parser に影響しない。
CommonMark 0.31.2 の構成は presets::commonmark、traQ の構成は presets::traq::v1 にある。

```rust
use traq_markdown::{GrammarBuilder, syntax::{commonmark, extensions::math}};
let base = commonmark::Syntax::default();
let mut builder = GrammarBuilder::new();
builder.add(&base.plugin)?.add(math::plugin())?;
builder.before(math::inline_rule(), &base.inline.code)?;
builder.before(math::block_rule(), &base.block.fence)?;
let grammar = builder.build()?;
```

この組み合わせ例の base は HTML を含む完全な CommonMark preset とは別。
before の引数は同じフェーズのルール参照で、異なるフェーズの混在は Rust / TS の型検査で拒否する。
順序は文法の意味なので自動推測しない。preset は必要な順序を設定済み。

## 名前・階層・同一性

```rust
use traq_markdown::engine::Plugin;
let generic = Plugin::group().named("generic");
let github = generic.group().named("github");
let mut issue = github.new().named("issue");
let math = generic.new().named("math");
let anonymous = Plugin::new();
```

group は一度作って使い回す namespace symbol。文字列は任意の表示名であり、検索キーや identity には使わない。
同名でも new/group を再実行すれば別の symbol。登録・削除・順序変更はインスタンスの参照を渡す。
組み込み plugin / rule の関数は共有定義への静的参照を返すため、呼び出し側で clone する必要はない。

同じ plugin / rule の二重登録は add で拒否する。
build では、採用された同じ親の group / plugin の表示名と、同じ plugin・同じフェーズの rule 名の重複を拒否する。
無名同士は許可し、採用していない namespace は検査対象にしない。空文字を明示した場合は名前として扱う。

named は同じ symbol の表示を変えた値を返す。既に作られた子の表示階層を書き換える操作ではない。
定義を共有した後で plugin に rule を追加すると別定義になり、登録済みの snapshot は変化しない。
削除・順序変更には登録時の定義を使う。組み込み定義は読み取り専用で、named で得た値を編集できる。

grammar.describe() は表示階層と有効なルール順を返す。
Rust の plugins()、TS の plugins、Go の Plugins() から、確定した構成の plugin / rule 参照を調べられる。

## TypeScript / JavaScript

```ts
import { loadRuntime, Plugin } from '@traptitech/markdown-parser'
const runtime = await loadRuntime(wasmBytes)
const grammar = runtime.presets.traq.v1.toBuilder()
  .remove(runtime.plugins.generic.math)
  .build()
const parser = runtime.parser(grammar)
grammar.dispose()
const document = parser.parse('**本文**')
const inline = parser.parseInline('**本文**')
parser.dispose()
runtime.dispose()
```

```ts
const generic = Plugin.group().named('custom')
const math = generic.new().named('math')
math.add(runtime.plugins.generic.math.inlineRules[0])
const grammar = runtime.builder()
  .add(runtime.plugins.commonmark.core)
  .add(math)
  .build()
```

通常は plugin 全体を add する。上の例のように、配布済み rule を別の plugin にまとめることもできる。
inlineRules / blockRules / textRules の参照を before に渡せる。
bindings では Wasm に含まれる実装を組み合わせる。新しい解析コールバックの実装は Rust で追加して artifact を再生成する。

一つの Runtime が一つの Wasm instance を共有する。parser(grammar) は毎回独立した軽量な Parser を作る。
組み込み preset は取得・構成の説明・toBuilder だけではコンパイルせず、parser(grammar) で初めて構築する。
同じ Grammar から作る Parser はコンパイル済みデータを共有する。独自構成の builder.build() は即時構築する。
Grammar / Parser の dispose は冪等。Grammar を解放しても既存 Parser は最後まで使える。
解放済み Grammar から新規 Parser は作れないが、describe / plugins / toBuilder は保存済みの定義を参照できる。
Runtime.dispose は全ての Parser / Grammar の実行を終了する。GC による解放時期には依存しない。

既定の decoder は Rust catalog から生成する。必要な decoder の欠落は Parser 作成時に拒否する。
loadRuntime の extensions オプションで検証関数を差し替えられる。
表示用途で明示した allowUnknownExtensions は未対応 payload を許容し、描画側で原文を表示できる。
別 Runtime の Grammar / rule は混ぜられない。

## Go

```go
runtime, err := core.New(ctx, wasmBytes)
if err != nil { return err }
defer runtime.Close(ctx)

builder := runtime.Presets.TraQ.V1.ToBuilder()
if err := builder.Remove(runtime.Plugins.Generic.Math); err != nil { return err }
grammar, err := builder.Build(ctx)
if err != nil { return err }
defer grammar.Close()

parser, err := runtime.Parser(ctx, grammar)
if err != nil { return err }
defer parser.Close()
result, err := parser.Parse(ctx, source)
// result.Document: 型付き AST / result.JSON: wire JSON
```

core は go/core。group は core.NewPluginGroup().Named("generic")、
子は generic.Group().Named("github")、plugin は generic.New().Named("math")。
Plugin.Add(rule)、GrammarBuilder.Add(plugin) / Remove(plugin) / Before(rule, anchor) は error を返す。
名前は省略でき、builder は ID を受け取らない。

Runtime の4つの Wasm worker を全 Parser が共有する。builder は一つの goroutine 内で編集し、Grammar / Parser は並行利用できる。
組み込み preset は軽量な定義として保持し、取得・構成の説明・ToBuilder だけではコンパイルしない。
Parser(ctx, grammar) が最初の文法構築を行い、初期化エラーやキャンセルはそこで返す。失敗した初期化は再試行できる。
Build(ctx) は独自構成をその場で検証・構築する。どちらも一つの worker で構築し、他の worker は初回解析時に同じ定義を構築する。
最後の所有者が Close すると idle worker は即座に解放し、使用中の worker は戻った際に解放する。
処理中の呼び出しも文法を保持するため、並行する Close が途中で文法を消さない。
キャンセルで停止した Wasm instance は次回利用時に置換する。Go は未知拡張の無視を許可しない。

## 保存時の文法版

syntaxVersion は利用アプリの保存契約。parser SDK、Grammar、Document は持たない。
アプリが保存済み番号と preset / Parser の対応を定義する。

```ts
const parsers = new Map([[1, runtime.parser(runtime.presets.traq.v1)]])
const parser = parsers.get(message.syntaxVersion)
if (!parser) throw new Error('Unsupported syntaxVersion')
const document = parser.parse(message.content)
```

未知版を最新版へ読み替えない方針もアプリが決める。traq/v2 preset の追加と、
その preset をどの保存版へ割り当てるかは別の変更。SDK の ParserCache は今回導入していない。
正規表現などの rule 内部の遅延初期化は、その構文を初めて解析する際に発生する。
これらの再利用と、利用側が Parser を保持するだけで現在の用途を扱う。LRU / TTL による自動解放は導入していない。

## 拡張実装と AST

Plugin.add は InlineRule / BlockRule / TextRule の型から登録先を決める。
解析機構は engine、CommonMark 文法は syntax/commonmark、汎用拡張は syntax/extensions、
traP 拡張は syntax/trap、構成の定義は presets に分離している。

payload は ExtensionData と安定した wire 名を定義し、生成する rule に produces::<T>() を宣言する。
子に別の拡張ノードを含む場合はその型も宣言する。未宣言・不正な payload は解析結果検査で拒否する。
同じ payload 型は共有でき、異なる型による同じ wire 名の登録は拒否する。

BlockInput の leaf / blocks / matched / body は原文範囲を計算する。
非連続な本文などには DraftNode / SourceView を使える。参照定義は block 解析後に inline へ解決する。
例は [extensions.rs](../crates/markdown/tests/extensions.rs)。

文法ルール間で引き継ぐ状態は `SourceView::context::<T>()` / `set_context(value)` を使う。
拡張内の private な型で区別し、core は値の意味を解釈しない。CommonMark の引用継続行もこの方法で扱う。
範囲を結合した子の view やタブの展開・復元にも状態を引き継ぐ。
子で値を差し替えても親・兄弟は変わらない。値に内部可変性を持たせた場合はその内部の変更が共有されるため、
局所的な状態には不変な値を使う。状態は view とともに解放され、Parser の反復利用で次の本文に持ち越さない。
拡張が自身の状態を複製・加工する処理量は、そのルールの Budget に計上する。
この API は Rust の文法実装向けであり、TS / Go の parse 呼び出しや AST に context を追加しない。

Document は {source, children}。子は共通の children、span は保存原文の UTF-8 バイト範囲。
改行・NUL・タブの解析時処理でも source は変えず、Unicode 正規化もしない。
generic/math_inline@1 の @1 は payload 契約の版で、文法版や plugin の表示名とは独立している。

Rust の contracts feature で payload schema と型、catalog を出力し、JS / Go の codec と公開 catalog を生成する。
catalog の公開 member 名は SDK の API 名として定義する。任意の表示名から自動生成しない。
内部の数値 handle は artifact / Wasm instance 内だけで使い、保存や公開 API に使わない。

生成器は閉じた object、string、boolean、文字列 enum、nullable を扱う。
未対応の型・制約、schema 不一致、出力型名の衝突は生成時に拒否する。
任意の Rust 関数の意味を他言語へ変換する仕組みではない。

## 表示・制限・配布

renderer の createRenderer({overrides, extensions}) は指定された handler だけ差し替える。
単独描画は HTML をテキストとして扱う。S-UI の installParser は既存のリンク方針を引き継ぐ。
描画構成はアプリが選び、AST に Profile ID を付けて選別しない。

ABI 2 / AST 3。旧 Wasm / codec との混在は初期化時に拒否する。
入力64KiB、出力1MiB、Wasm memory32MiB、同時コンパイル済み文法256件（初期化済み preset を含む）。
未使用 preset は文法枠を消費しない。Go では全 worker に同じ文法が収まるよう Runtime 全体で枠を予約する。
解放により allocator の領域は再利用されるが、WebAssembly.Memory 自体のサイズは縮まらない。

scripts/build.sh が Wasm・型・codec・catalog・artifact 情報を更新する。
検証は cargo test / clippy / fmt、go test（並行処理は -race）、node --test tests/*.test.mjs。
npm ci と npm run build の後、npm test で renderer を含む JS の検証を行う。
配布確認は npm run check:package、実行例は [examples](../examples/README.md) を参照する。
