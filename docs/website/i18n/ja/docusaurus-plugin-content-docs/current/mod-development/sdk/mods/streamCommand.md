---
sidebar_position: 5
---

# streamCommand

コマンド実行中にイベントを yield する非同期ジェネレータを返します。リアルタイム出力が必要な長時間実行プロセスに使用してください。最後のイベントは常に `exit` イベントです。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `request` | `ExecuteCommandRequest` | コマンド実行パラメータ |
| `signal` | `AbortSignal`（オプション） | キャンセル用のシグナル |

## 戻り値

`AsyncGenerator<CommandEvent>`

## 使用例

```typescript
for await (const event of mods.streamCommand({ command: "build" })) {
  switch (event.type) {
    case "stdout":
      console.log(event.data);
      break;
    case "stderr":
      console.error(event.data);
      break;
    case "exit":
      console.log("完了、終了コード:", event.exitCode);
      break;
  }
}
```

`AbortSignal` によるキャンセル：

```typescript
const controller = new AbortController();
for await (const event of mods.streamCommand({ command: "slow" }, controller.signal)) {
  // ...
}
```
