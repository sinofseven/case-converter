# CHANGELOGを追加

## 目的
`Keep a Changelog`形式のチェンジログが欲しい

## 要望
CHANGELOG.mdを書いてください

## プラン

Keep a Changelog フォーマット（v1.1.0）に従い、CHANGELOG.mdを作成。git履歴から抽出した各バージョンの変更内容を整理し、以下の構成で記載：

- Unreleased セクション（未リリース変更用）
- バージョン別セクション（新しい順：v1.1.2, v1.1.1, v1.1.0, v1.0.0）
- Added, Changed, Fixed などのカテゴリ分け
- GitHub のバージョン比較ページへのリンク定義

## 完了サマリー

**完了日時**: 2026-05-18T18:35:00+09:00

CHANGELOG.md を Keep a Changelog フォーマット（v1.0.0）に準拠して作成完了。

### 成果物
- **作成ファイル**: CHANGELOG.md
- **フォーマット**: Keep a Changelog v1.0.0 準拠
- **収録バージョン**: v1.1.2, v1.1.1, v1.1.0, v1.0.0

### 実施内容
1. git log からプロジェクトの全バージョン履歴を抽出
2. 各コミットメッセージから変更内容を分類（Added, Changed, Fixed）
3. Keep a Changelog フォーマットに従い整理
4. GitHub リポジトリへのバージョン比較リンクを定義
5. Unreleased セクションを用意し、今後の機能追加に対応

作業ログ: kanban/0010_add_changelog/log.md を参照。
