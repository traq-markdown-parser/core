# traP 拡張と traQ preset

traP 固有の拡張と、それらを組み合わせた traQ preset を管理する系列です。
各行は独立した Cargo package で、共通のリリース単位として扱います。

| ディレクトリ | package | 責務 |
|---|---|---|
| contracts | markdown-trap-contracts | 参照・スタンプ・spoiler などのノード型 |
| syntax | markdown-trap-syntax | traP 固有の構文と traQ の互換処理 |
| [text](text/README.md) | markdown-trap-text | traP 拡張のテキスト描画 |
| [extraction](extraction/README.md) | markdown-trap-extraction | 参照情報の抽出 |
| [traq](traq/README.md) | markdown-traq | 文法 preset と SDK 向け catalog |
| [traq-processing](traq-processing/README.md) | markdown-traq-processing | 通知・参照抽出 preset |

core・CommonMark と同系列に依存できます。contracts は文法や描画の実装から独立し、
text / extraction / traq-processing は parser に依存しません。
将来の traQ 文法は presets::traq::v2 のように追加します。

配下の package は同じ版で管理します。core・CommonMark 系列とは独立して更新できます。
ルートの workspace で開発し、[共通の検証と更新手順](../../CONTRIBUTING.md#rust-の管理単位)に従います。
