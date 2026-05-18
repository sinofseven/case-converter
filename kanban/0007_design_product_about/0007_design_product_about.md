# プロダクトのAboutを設計する

## 目的
Githubで表示するaboutを設定していない。HomebrewのFormulaとして公開する際にDescriptionとして使えると思うので考えて欲しい。

## 要望
このプロダクトのaboutを考えてください

## プラン

### 提案した説明文案

3つの案を検討した：

#### **Option 1（推奨・採用案）**
```
Convert text between case styles: snake_case, camelCase, PascalCase, kebab-case and more.
```
- 約80文字でシンプルで直感的
- 主要ケース形式を明示し機能が一目瞭然
- "and more" で拡張性を暗示
- GitHub / Homebrew 双方に最適

#### Option 2
```
Fast CLI tool to convert text between different case styles: snake_case, camelCase, PascalCase, kebab-case
```
- Rust 実装の高性能をアピール
- ただし約140文字で截断リスク

#### Option 3
```
A CLI tool for converting text between case styles like snake_case, camelCase, and PascalCase.
```
- 文法的に自然だが説明が簡潔化しすぎ

### 実装方針

1. **Cargo.toml の description フィールド更新**
   - 変更前: "A CLI tool to convert text between different cases"
   - 変更後: "Convert text between case styles: snake_case, camelCase, PascalCase, kebab-case and more."

2. **GitHub リポジトリ設定** (ユーザー手動)
   - Settings → About セクションに同じ説明文を設定

3. **Homebrew Formula** (ユーザー手動)
   - Formula の `desc` フィールドに同じ説明文を使用

## 完了サマリー

**実装完了**: 2026-05-18T19:45:12+09:00

### 実施内容

- Cargo.toml の `description` フィールドを新しい説明文に更新
- ログファイル (kanban/0007_design_product_about/log.md) を作成・記録

### 確認事項

- ✅ Cargo.toml の description が正しく更新された
- ✅ 新しい説明文は約80文字で GitHub / Homebrew 両対応
- ✅ ユーザーが別途 GitHub リポジトリと Homebrew Formula を更新可能な状態

### 次のステップ（ユーザー手動実施）

1. GitHub リポジトリの Settings → About セクションに説明文を設定
2. Homebrew Formula の `desc` フィールドを更新
3. (オプション) `cargo publish` で crates.io に反映確認
