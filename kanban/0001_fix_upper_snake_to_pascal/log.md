# Upper Snake CaseからPascal Caseへの変換バグ修正 - 作業ログ

- 開始時刻: 2026-05-08T18:05:32+09:00
- 完了時刻: 2026-05-08T18:06:32+09:00

---

## タスク概要

Upper Snake Caseの文字列をPascal Caseに変換しようとしたら意図しない値になった。修正して欲しい。

**バグ詳細**:
- 入力: `POLICY_DYNAMODB_GET_ITEM`
- 現在の出力: `POLICYDYNAMODBGETITEM`
- 期待される出力: `PolicyDynamodbGetItem`

---

## 調査結果

### src/main.rs 全体構成

ケース変換ツールは単一ファイル `src/main.rs` に実装されている。主な関数：

- `main`（CLI引数処理・stdin入力処理）
- `convert_case`（87-113行目）：単語分割 → ケース変換
- `split_into_words`（115-164行目）：入力文字列を単語トークンに分解
- `capitalize`（166-172行目）：文字列の最初の文字を大文字化

### split_into_words の動作（Upper Snake Caseに対して）

`POLICY_DYNAMODB_GET_ITEM` の処理トレース：

```
P,O,L,I,C,Y → 大文字連続、前の文字が小文字でないため単語分割なし → current_word = "POLICY"
_ → セパレータ。add_word → words = ["POLICY"]
D,Y,N,A,M,O,D,B → 同様 → words = ["POLICY", "DYNAMODB"]
_ → words = ["POLICY", "DYNAMODB", "GET"]  (GET も同様)
I,T,E,M → words = ["POLICY", "DYNAMODB", "GET", "ITEM"]
```

**分割結果は正しい**: セパレータ（`_`）によって `["POLICY", "DYNAMODB", "GET", "ITEM"]` に正しく分割される。

### capitalize の問題点

```rust
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(), // ← 問題
    }
}
```

`chars.as_str()` は残りの文字列をそのまま返す。`capitalize("POLICY")` の場合：
1. 最初の文字 `P` → `to_uppercase()` → `P`
2. 残り `chars.as_str()` → `OLICY`（大文字のまま）
3. 結果: `P` + `OLICY` = `POLICY`（変わらず）

### convert_case の Pascal 処理

```rust
Case::Pascal => words
    .iter()
    .map(|word| capitalize(word))
    .collect::<Vec<_>>()
    .join(""),
```

各単語に `capitalize` を適用し結合する。上記バグにより `["POLICY", "DYNAMODB", "GET", "ITEM"]` → `POLICYDYNAMODBGETITEM` になってしまう。

### テスト状況

プロジェクト内に `#[test]` 属性を使ったテストは存在しない。

---

## 実装プラン

`capitalize` 関数の `chars.as_str()` を `&chars.as_str().to_lowercase()` に変更する1行修正。

```rust
// 変更前
Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),

// 変更後  
Some(c) => c.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
```

**影響範囲**:
- `capitalize` は `camelCase` 変換（2番目以降の単語）と `pascalCase` 変換の両方で使用される
- 既存の小文字入力（`example_text` → `Example`）には変化なし
- 大文字入力（`POLICY` → `Policy`）は正しく変換されるように修正される

---

## プランニング経緯

初回提案がそのまま承認された。

---

## 会話内容

1. ユーザーが `/kanban 0001` でタスクを実行
2. `split_into_words` と `capitalize` を調査するため Explore エージェントを起動
3. Explore エージェントが `capitalize` の `chars.as_str()` が残りを小文字化しないことをバグとして特定
4. プランとして1行修正（`.to_lowercase()` 追加）を提案
5. ユーザーが承認

---

## 編集したファイル

- `src/main.rs`（166-172行目）— `capitalize` 関数の1行修正

---

## 実行したコマンド

```bash
cargo test
# → テストなし (0 passed) でビルド成功

echo -n POLICY_DYNAMODB_GET_ITEM | cargo run -- -c pascal
# → PolicyDynamodbGetItem ✓

cargo run -- -c pascal "example_text"  # → ExampleText ✓
cargo run -- -c pascal "camelCase"     # → CamelCase ✓
cargo run -- -c camel "example_text"   # → exampleText ✓
cargo run -- -c camel "MY_CAMEL"       # → myCamel ✓
```

---

## 判断・意思決定

- `capitalize` 関数のみを修正する方針を採用。`split_into_words` の変更は不要（Upper Snake Case はセパレータ `_` で正しく分割されていた）。
- `chars.as_str()` を `&chars.as_str().to_lowercase()` に変更する最小限の修正とした。

---

## エラー・問題

特になし。
