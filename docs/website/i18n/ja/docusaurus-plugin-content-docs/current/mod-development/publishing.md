---
title: "公開と配布"
sidebar_position: 12
---

# 公開と配布

MOD は標準的な npm パッケージです。MOD を配布するには、[npm レジストリ](https://www.npmjs.com/)に公開します。ユーザーは公開された MOD を 1 つのコマンドでインストールできます：

```bash
hmcs mod install <package-name>
```

## パッケージの命名

:::warning
npm 上の `@hmcs/` スコープは公式 MOD 用に予約されています。このスコープでパッケージを公開しないでください。
:::

独自の MOD を公開する際は、以下の命名規則を使用してください：

- **スコープ付き**（推奨） — 自分の npm スコープを使用：`@yourname/hmcs-my-mod`
- **スコープなし** — `hmcs-` プレフィックスを使用：`hmcs-my-mod`

`package.json` のパッケージ `name` はユーザーが `hmcs mod install` に渡す値です。アセット ID、メニューエントリ、MOD コマンドの MOD 名の導出にも使用されます。詳細は[アセット ID](./project-setup/asset-ids.md)を参照してください。

## 公開前の準備

:::tip
公開前にローカルで MOD をテストしてください。ローカルパスからインストールし、アセットの読み込みとスクリプトの実行が正しく動作することを確認します：

```bash
hmcs mod install /path/to/your-mod
```
:::

## npm への公開

### 1. npm にログイン

npm アカウントをお持ちでない場合は、[npmjs.com/signup](https://www.npmjs.com/signup) で作成してください。ターミナルから認証します：

```bash
npm login
```

### 2. package.json の確認

`package.json` に必要なフィールドがすべて含まれていることを確認してください：

```json
{
  "name": "@yourname/hmcs-my-mod",
  "version": "1.0.0",
  "type": "module",
  "description": "A short description of your MOD",
  "homunculus": {
    "service": "index.ts",
    "assets": {}
  }
}
```

主要フィールド：

| フィールド | 必須 | 備考 |
| ------------- | -------- | ----------------------------- |
| `name` | はい | npm 上で一意であること |
| `version` | はい | [Semver](https://semver.org/) |
| `type` | はい | `"module"` であること |
| `description` | はい | `hmcs mod list` に表示される |
| `homunculus` | はい | MOD であることを示す |

`homunculus` フィールドと `bin` の詳細は[パッケージ設定](./project-setup/package-json.md)を参照してください。

### 3. 公開内容の制御

デフォルトでは、npm はプロジェクトディレクトリ内のすべてを公開します。`package.json` の `files` フィールドを使って必要なものだけを含めてください：

```json
{
  "files": [
    "index.ts",
    "commands/",
    "assets/"
  ]
}
```

または `.npmignore` ファイルを作成して特定のパスを除外することもできます。いずれの方法でも、`homunculus.assets` で宣言されたすべてのアセットファイルが公開パッケージに含まれていることを確認してください。

### 4. 公開

**スコープ付き**パッケージの場合（例：`@yourname/hmcs-my-mod`）：

```bash
npm publish --access public
```

**スコープなし**パッケージの場合：

```bash
npm publish
```

### 5. 確認

公開した MOD をインストールして、すべてが正しく動作することを確認します：

```bash
hmcs mod install @yourname/hmcs-my-mod
```

Desktop Homunculus を再起動し、MOD が正しく読み込まれることを確認してください。

## 公開済み MOD の更新

新しいバージョンを公開するには：

1. `package.json` の `version` フィールドを更新
2. `npm publish` を実行（スコープ付きパッケージの場合は `npm publish --access public`）

ユーザーは再インストールで更新します：

```bash
hmcs mod install @yourname/hmcs-my-mod
```
