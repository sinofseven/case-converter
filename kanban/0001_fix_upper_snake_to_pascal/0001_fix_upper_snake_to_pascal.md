# Upper Snake CaseからPascal Caseへの変換バグ修正

## 目的
バグが判明した。困るので修正して欲しい。

## 要望
Upper Snake Caseの文字列をPascal Caseに変換しようとしたら意図しない値になった。修正して欲しい。

### バグ詳細
```bash
at 17:37:17 ❯ echo -n POLICY_DYNAMODB_GET_ITEM |case-converter -c pascal 
POLICYDYNAMODBGETITEM
```

期待値は `PolicyDynamodbGetItem` のような形式になるべき。

## 完了サマリー

- 完了日時: 2026-05-08T18:06:32+09:00
- 修正ファイル: `src/main.rs`

`capitalize` 関数の残り文字処理を `chars.as_str()` から `&chars.as_str().to_lowercase()` に変更した。
これにより `POLICY` → `Policy`、`DYNAMODB` → `Dynamodb` のように各単語の先頭のみ大文字、残りは小文字に正しく変換されるようになった。

