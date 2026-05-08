# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# ビルド（開発用）
cargo build

# ビルド（リリース用）
cargo build --release

# 直接実行
cargo run -- -c snake_case "ExampleText"

# テスト実行
cargo test

# 単一テスト実行
cargo test <test_name>

# リリースバイナリを直接実行
./target/release/case-converter -c snake_case "ExampleText"
```

## アーキテクチャ

コードベースは `src/main.rs` の単一ファイルで構成されており、以下の処理フローになっている：

1. **入力取得** (`main`): CLI引数またはstdinからテキストを受け取る
2. **単語分割** (`split_into_words`): 入力を単語トークンに分解する
   - camelCase / PascalCase → 大文字の直前で分割
   - snake_case / kebab-case → `_` `-` ` ` で分割
3. **ケース変換** (`convert_case`): トークン列を指定ケース形式に結合する

### `split_into_words` の重要な挙動

複数形式が混在した入力（例: `my-camelCase`）を扱えるよう、セパレータ文字と大文字検出の両方を組み合わせている。ただし、`ABCCase` のような連続大文字の途中では分割しない（直前の文字が小文字のときのみ分割）。

### バージョン表示

`-v / --version` は clap 標準機能ではなく独自実装。`CARGO_PKG_VERSION` を直接埋め込む形式にしているため、`Cargo.toml` のバージョン更新だけで反映される。

## 開発ワークフロー

このリポジトリでは `kanban-kit` プラグインを使ったタスクファイル経由の開発方式を採用している。

### タスクの作成

`/add-kanban` スキルで新規タスクファイルを作成する。ファイルはユーザーが以下の構造で記述する：

```markdown
# タイトル
## 目的
（なぜこの作業が必要か — 背景・動機・ゴール）

## 要望
（具体的に何をどうしてほしいか）
```

### タスクの実行

`/kanban` スキルでタスクを実行する。引数なしの場合は `## 完了サマリー` を含まない未完了タスクのうち、番号が最大のものが自動選択される。特定タスクを実行する場合は番号またはファイル名を引数に渡す。

### ファイル構成

```
kanban/
  {xxxx}_{title}/          # xxxx は4桁の連番
    {xxxx}_{title}.md      # タスクファイル（要望・プラン・完了サマリーを記載）
    log.md                 # 作業ログ（調査結果・会話・編集ファイル等を省略なく記録）
```

- **未完了タスク**: `## 完了サマリー` セクションを含まないもの
- タイムスタンプは JST（`TZ=Asia/Tokyo date +"%Y-%m-%dT%H:%M:%S+09:00"`）

## クレート

- `clap 4.4`（`derive` feature）: CLIパーサー

## 公開先

crates.io に `luciferous-case-converter` として公開済み。バイナリ名は `case-converter`。
