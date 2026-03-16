---
sidebar_position: 4
---

# executeCommand

bin コマンドを実行し、プロセス終了後に収集された結果を返します。stdout と stderr は文字列に結合されます。全出力をバッファリングする `streamCommand` の便利なラッパーです。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `request` | [`ExecuteCommandRequest`](./types#executecommandrequest) | コマンド実行パラメータ |
| `signal` | `AbortSignal`（オプション） | キャンセル用のシグナル |

## 戻り値

`Promise<`[`CommandResult`](./types#commandresult)`>`

## 使用例

```typescript
const result = await mods.executeCommand({ command: "greet" });

if (result.exitCode === 0) {
  console.log("出力:", result.stdout);
} else {
  console.error("エラー:", result.stderr);
}
```

引数、stdin データ、カスタムタイムアウトを渡すこともできます：

```typescript
const result = await mods.executeCommand({
  command: "compile",
  args: ["--target", "es2020"],
  stdin: JSON.stringify({ input: "data" }),
  timeoutMs: 60000,
});
```

`AbortSignal` によるキャンセル：

```typescript
const controller = new AbortController();
const result = await mods.executeCommand({ command: "slow" }, controller.signal);
```
