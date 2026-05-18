# 作業ログ: HomebrewのTap経由でのインストールをREADMEに追加

開始時刻: 2026-05-18T19:59:14+09:00
完了時刻: 2026-05-18T19:59:33+09:00

## タスク概要

READMEを更新して、HomebrewのTap（`sinofseven/luciferous-tap`）経由でのインストールを書いてください

## 調査結果

### README.md

- `## Installation` セクションに `### From crates.io` と `### From Source` の2つのサブセクションが存在
- Homebrew経由のインストール方法は記載なし
- パッケージ名: `luciferous-case-converter`、バイナリ名: `case-converter`

### .github/workflows/publish_formula.yml

- リリース公開時にHomebrewフォーミュラを自動生成するワークフローが存在
- `sinofseven/homebrew-luciferous-tap` リポジトリにフォーミュラを作成
- `brew tap sinofseven/luciferous-tap` で追加可能なTapが既に整備済み

### Cargo.toml

- パッケージ名: `luciferous-case-converter`
- バイナリ名: `case-converter`
- バージョン: `1.1.2`

## 実装プラン

`README.md` の `## Installation` セクション内に `### Via Homebrew` セクションを追加する。

配置: 既存の `### From crates.io` より前（最も簡単なインストール方法として先頭に）

```markdown
### Via Homebrew

```bash
brew install sinofseven/luciferous-tap/case-converter
```
```

1行のワンライナー形式を採用（`brew tap` + `brew install` の2ステップより簡潔）。

## プランニング経緯

初回提案がそのまま承認された

## 会話内容

- ユーザーが `/add-kanban` でタスクを作成後、`/kanban 0009` で実行開始
- タスク開始前にTap名 `sinofseven/luciferous-tap` を追記
- プランモードでREADMEにHomebrewセクションを追加する計画を提示
- ユーザーが承認

## 編集したファイル

- `README.md`: `### Via Homebrew` セクションを追加

## 実行したコマンド

なし

## 判断・意思決定

- `brew tap` + `brew install` の2ステップではなく、`brew install sinofseven/luciferous-tap/case-converter` のワンライナー形式を採用（簡潔さ優先）

## エラー・問題

なし
