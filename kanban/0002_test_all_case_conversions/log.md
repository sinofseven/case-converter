# 全ケース間の相互変換テスト - 作業ログ

- 開始日時: 2026-05-08T18:12:11+09:00
- 完了日時: 2026-05-08T18:12:57+09:00

## タスク概要

変換可能な全てのCaseに対して相互変換ができるかを確認するテストを追加して欲しい。またテストが失敗するとき修正もして欲しい。

## 調査結果

### src/main.rs の構造

- 172行のシングルファイル実装
- テストモジュールは完全に存在しない

### サポートされているケース形式（5つ）

1. `snake_case` (alias: `snake`)
2. `UPPER_SNAKE_CASE` (aliases: `upper_snake`, `upper-snake`)
3. `camelCase` (alias: `camel`)
4. `PascalCase` (alias: `pascal`)
5. `kebab-case` (alias: `kebab`)

### 変換ロジック（convert_case 関数, lines 87-113）

- `split_into_words` で入力を正規化した後、ケース形式に変換する
- Snake: `words.join("_").to_lowercase()`
- UpperSnake: `words.join("_").to_uppercase()`
- Camel: 先頭単語を小文字、残りを `capitalize()` で変換、セパレータなし
- Pascal: 全単語を `capitalize()` で変換、セパレータなし
- Kebab: `words.join("-").to_lowercase()`

### split_into_words の分割ロジック（lines 115-164）

- `_`, `-`, ` ` → 区切り文字として新しい単語を開始
- 大文字文字 → 前の文字が小文字の場合のみ新しい単語を開始（camelCase/PascalCase の検出）
- その他 → 現在の単語に追加

結果として各ケース形式の入力は以下のように正規化される：
- `POLICY_DYNAMODB_GET_ITEM` → `["POLICY", "DYNAMODB", "GET", "ITEM"]`
- `policyDynamodbGetItem` → `["policy", "Dynamodb", "Get", "Item"]`
- `PolicyDynamodbGetItem` → `["Policy", "Dynamodb", "Get", "Item"]`
- `policy_dynamodb_get_item` → `["policy", "dynamodb", "get", "item"]`
- `policy-dynamodb-get-item` → `["policy", "dynamodb", "get", "item"]`

### capitalize 関数（lines 166-172）

タスク 0001 で修正済み。`c.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()` により、先頭大文字+残り小文字に正規化される。

これにより、`POLICY` → `Policy`、`Dynamodb` → `Dynamodb` のように変換される。

### UPPER_SNAKE → Pascal 変換の確認

`POLICY_DYNAMODB_GET_ITEM`：
1. `split_into_words` → `["POLICY", "DYNAMODB", "GET", "ITEM"]`
2. `capitalize("POLICY")` = `P` + `olicy` = `Policy`
3. 結合 → `PolicyDynamodbGetItem` ✓（タスク 0001 の修正後は正しく動作）

## 実装プラン

### テストデータ定義

元の問題のケース `POLICY_DYNAMODB_GET_ITEM`（4語）を基準として：

| ケース | 文字列 |
|--------|--------|
| snake | `policy_dynamodb_get_item` |
| UPPER_SNAKE | `POLICY_DYNAMODB_GET_ITEM` |
| camel | `policyDynamodbGetItem` |
| Pascal | `PolicyDynamodbGetItem` |
| kebab | `policy-dynamodb-get-item` |

### テスト構造

`src/main.rs` 末尾に `#[cfg(test)] mod tests` を追加し、5グループのテスト関数を追加する：
- `test_snake_to_all`: snake から全5ケース変換
- `test_upper_snake_to_all`: UPPER_SNAKE から全5ケース変換
- `test_camel_to_all`: camel から全5ケース変換
- `test_pascal_to_all`: Pascal から全5ケース変換
- `test_kebab_to_all`: kebab から全5ケース変換

合計 25 アサーション。

## プランニング経緯

初回提案がそのまま承認された。

## 会話内容

- ユーザーが kanban 0002 タスクを作成し、/kanban を実行指示
- プランモードで探索を実施し、テスト追加計画を提示
- ユーザーが承認

## 編集したファイル

- `src/main.rs`: テストモジュール `#[cfg(test)] mod tests` を追加（lines 174-233）

## 実行したコマンド

```
cargo test
```

実行結果：
```
running 5 tests
test tests::test_camel_to_all ... ok
test tests::test_kebab_to_all ... ok
test tests::test_upper_snake_to_all ... ok
test tests::test_snake_to_all ... ok
test tests::test_pascal_to_all ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 判断・意思決定

- テスト定数名 `UPPER_SNAKE` を `UPPER_SNAKE_STR` に変更した。`UPPER_SNAKE` が `Case::UpperSnake` と混同しないよう。
- 全 25 パターンのテストが通ったため、バグ修正は不要だった。

## エラー・問題

なし。全テストが初回から通過した。
