# Cargo.tomlのバイナリ名を統一する

## 目的
古い設定で名前を変えていなかったので修正したい

## 要望
Cargo.tomlで名前が'luciferous-case-converter'になっているので生成するバイナリと同じ名前にしたい

## プラン
Cargo.toml の package.name を "case-converter" に変更する。バイナリ名は既に `case-converter` で統一される。

## 完了サマリー
2026-05-11T14:32:00+09:00 に実装完了。

### 実施内容
- Cargo.toml の package.name を "luciferous-case-converter" から "case-converter" に変更
- ビルド成功、バイナリが `case-converter` として正しく生成されることを確認
- バイナリの実行確認も完了
