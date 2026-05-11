# パッチバージョンをインクリメント

## 目的
名前を修正したのでリリースを行う

## 要望
パッチバージョンをインクリメントしてください

## 完了サマリー
- **実行日時**: 2026-05-11T00:00:00+09:00
- **内容**: Cargo.toml の version を 1.1.1 → 1.1.2 に更新
- **確認**:
  - cargo build --release: 成功
  - ./target/release/case-converter --version: case-converter 1.1.2 表示
  - cargo test: 5 tests 全てパス
