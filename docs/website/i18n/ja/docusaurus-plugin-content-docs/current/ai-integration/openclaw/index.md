---
title: "OpenClaw 連携"
sidebar_position: 1
---

# OpenClaw 連携

`@hmcs/openclaw-plugin` は [OpenClaw](https://docs.openclaw.ai) のエージェントを Desktop Homunculus のキャラクターと橋渡しするプラグインです。OpenClaw にインストールすると、DH の persona が OpenClaw のワークスペースとして扱われ、エージェントからの返答が対応するキャラクターに反映されます（フキダシ表示。VoiceVox MOD がインストールされていれば読み上げも）。

このページではセットアップ手順を一通り説明します。外部 MCP クライアント（Claude Desktop / Claude Code / Codex）との比較は [AI Integration 概要](/ai-integration) を参照してください。

## 前提

- **Desktop Homunculus** が起動していること。プラグインはデフォルトで `http://127.0.0.1:3100` に接続します。
- **OpenClaw** がインストール済みであること。インストール手順は [OpenClaw docs](https://docs.openclaw.ai) を参照。
- **Node.js ≥ 22**（OpenClaw の要件）。
- **[`@hmcs/persona` MOD](/mods/persona)** で persona を 1 件以上作成済みであること。プラグインは DH の persona を OpenClaw のワークスペースに同期します。persona が 1 件もなければ何も起こりません。
- *(任意)* **[`@hmcs/voicevox` MOD](/mods/voicevox)** — 返答を音声で読み上げたい場合。
- *(任意)* **[`@hmcs/stt` MOD](/mods/stt)** — 音声でエージェントに話しかけたい場合。

## プラグインをインストール

任意のターミナルで:

```bash
openclaw plugins install @hmcs/openclaw-plugin
```

OpenClaw がパッケージを取得し、拡張として登録します。OpenClaw が起動中だった場合は再起動してください。

## persona ごとに OpenClaw エージェントを登録する

プラグインは OpenClaw のエージェントを自動作成しません。DH 側で動かしたい persona ごとに、同じ ID を持つエージェントを登録してください:

```bash
openclaw agents add <persona-id>
```

DH の persona 一覧は persona MOD の UI、または HTTP API から確認できます:

```bash
curl http://127.0.0.1:3100/personas | jq '.[].id'
```

この手順を飛ばしても害はありません。プラグインは一致しない persona ごとに警告ログを出して待機します。

## 設定 (任意)

プラグインの設定は OpenClaw のプラグイン config から読み込まれます。通常の DH セットアップではデフォルト値のままで動作します。

| キー | デフォルト | 変更するとき |
|---|---|---|
| `hmcsBaseUrl` | `http://127.0.0.1:3100` | `~/.homunculus/config.toml` の `port` を変更している場合 |
| `soulMaxChars` | `10000` | 各エージェントのワークスペースに書き出す soul プロンプトの最大文字数 |

値を設定する場所は [OpenClaw のプラグイン設定ドキュメント](https://docs.openclaw.ai) を参照してください。

## 動作確認

1. Desktop Homunculus を起動してキャラクターを召喚する。
2. persona MOD の UI で persona を作成する（既に作成済みならスキップ）。
3. その persona ID に対して `openclaw agents add <persona-id>` を実行する。
4. OpenClaw を起動し、対応するエージェントのワークスペースを開く。
5. メッセージを送信する。DH の対応キャラクターが反応し、フキダシで返答が表示される（VoiceVox 導入時は音声でも読み上げ）。

## 次のステップ

- [Persona MOD](/mods/persona) — persona の作成と管理
- [VoiceVox MOD](/mods/voicevox) — 返答の音声読み上げ
- [STT MOD](/mods/stt) — 音声でエージェントに話しかける
- [AI Integration 概要](/ai-integration) — MCP ベースの連携との比較
