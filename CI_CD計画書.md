# CI/CD計画書

## 概要
本計画書では、CD（継続的デリバリー）のみを対象とし、GitHub Actionsを使用してリリースプロセスを自動化します。

## トリガー条件
- `v*`形式のタグ（例: `v0.1.9`）がPushされた際にCDプロセスを開始します。

## 対応プラットフォーム
以下のプラットフォーム向けに実行ファイルをビルドします：
- x86_64 Linux (MUSL)
- aarch64 (ARM64) Linux (MUSL)
- Windows
- macOS

## ビルド環境
- Linux (x86_64, ARM): Ubuntu最新版
- Windows: Windows最新版
- macOS: macOS最新版

## ビルドプロセス
各ターゲットのビルドは、以下のジョブに分けて実行されます：

### 1. Linux (x86_64)ビルド
- Ubuntu最新版ランナーを使用
- MUSLツールチェーンをセットアップ
- x86_64-unknown-linux-muslターゲット向けにビルド
- 実行ファイルを圧縮し、アーティファクトとして保存

### 2. Linux (ARM)ビルド
- Ubuntu最新版ランナーを使用
- ARM向けクロスコンパイル環境をセットアップ
- aarch64-unknown-linux-muslターゲット向けにビルド
- 実行ファイルを圧縮し、アーティファクトとして保存

### 3. Windowsビルド
- Windows最新版ランナーを使用
- 標準ターゲット向けにビルド
- 実行ファイルを圧縮し、アーティファクトとして保存

### 4. macOSビルド
- macOS最新版ランナーを使用
- 標準ターゲット向けにビルド
- 実行ファイルを圧縮し、アーティファクトとして保存

### 5. リリース作成
- 前の4つのジョブが完了した後に実行
- 全アーティファクトをダウンロード
- GitHub Releaseを作成（ドラフト）
- 全プラットフォーム向けのZIPファイルをアセットとして添付
- リリースノートを自動生成

## 使用するGitHub Actionsコンポーネント
- actions/checkout@v4: ソースコード取得
- actions-rs/toolchain@v1: Rustツールチェーンセットアップ
- actions-rs/cargo@v1: Cargoコマンド実行
- actions/upload-artifact@v4: ビルド成果物の一時保存
- actions/download-artifact@v4: ビルド成果物のダウンロード
- softprops/action-gh-release@v1: GitHubリリース作成

## パーミッション設定
- contents: write - リポジトリの内容の読み書き
- packages: write - パッケージの読み書き

## 成果物
各プラットフォーム向けに以下の成果物が生成されます：
- case-converter-linux-x86_64.zip
- case-converter-linux-arm.zip
- case-converter-windows.zip
- case-converter-macos.zip

## 注意事項
- ビルドアーティファクトは1日間のみ保持されます（retention-days: 1）
- Linux向けビルドはMUSLを使用して静的リンクされます
- ARM Linuxビルドはクロスコンパイルで生成されます

## 今後の拡張
- CI（継続的インテグレーション）の導入
  - プルリクエスト時の自動ビルド・テスト
  - コードリント・フォーマットチェック
- テスト自動化の追加
- コードカバレッジの測定と報告
- 追加プラットフォームのサポート（例: FreeBSD, 32bit ARM）
