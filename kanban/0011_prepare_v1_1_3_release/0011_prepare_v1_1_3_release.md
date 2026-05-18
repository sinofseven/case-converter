# v1.1.3 リリースノートの準備

## 目的
リリースに当たって何を書いて良いかわからない

## 要望
v1.1.3用にCHANGELOG.mdの更新とGithubのRelease用のリリースノートを考えてください

## プラン

1. CHANGELOG.md に v1.1.3 セクションを追加
   - v1.1.2 以降の 3 つのコミットから変更内容を抽出
   - Added/Changed/Fixed で分類して記載
   - リンク参照セクションを更新

2. Cargo.toml のバージョンを 1.1.3 に更新

3. GitHub Release テンプレートを作成（RELEASE_NOTES_v1.1.3.md）
   - Overview、What's New、Installation、Downloads で構成

## 完了サマリー

**実施日時**: 2026-05-18T16:35:00+09:00

✓ CHANGELOG.md を更新：[1.1.3] セクション追加（5/18日付、Added/Changed/Fixed記載）
✓ Cargo.toml をバージョン 1.1.3 に更新
✓ GitHub Release テンプレートを作成（RELEASE_NOTES_v1.1.3.md）
✓ ビルド確認：cargo build --release 成功

成果物：
- `/CHANGELOG.md` 更新（[1.1.3] セクション + リンク参照追加）
- `/Cargo.toml` 更新（version = "1.1.3"）
- `/RELEASE_NOTES_v1.1.3.md` 新規作成（GitHub Release 用テンプレート）
