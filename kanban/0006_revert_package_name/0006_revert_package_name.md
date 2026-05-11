# パッケージ名を luciferous-case-converter に戻す

## 目的
case-converter という名前だと既に使われていてpublishできなかったため

## 要望
Cargo.tomlにおいて名前を `luciferous-case-converter`に戻す

## プラン
1. Cargo.toml の `[package]` セクションで `name` フィールドを `luciferous-case-converter` に変更
2. バイナリ名（[[bin]] セクション）は `case-converter` のままにしておく
3. `cargo build` と `cargo test` でビルド・テスト成功確認

## 完了サマリー
**完了日時**: 2026-05-11T12:01:56+09:00

Cargo.toml のパッケージ名を `case-converter` から `luciferous-case-converter` に変更しました。

- ビルド成功 ✓
- テスト成功（5 passed） ✓
- バイナリ名は `case-converter` のままで正常 ✓

詳細は `log.md` を参照。
