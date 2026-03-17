---
sidebar_position: 100
---

# 型定義

### AppInfo

```typescript
interface AppInfo {
  /** エンジンのバージョン文字列（例: "0.1.0-alpha.3.2"）。 */
  version: string;
  /** プラットフォーム情報。 */
  platform: PlatformInfo;
  /** このビルドで利用可能なエンジンレベルの機能。 */
  features: string[];
  /** メタデータ付きの全読み込み済み MOD。 */
  mods: InfoMod[];
}
```

### PlatformInfo

```typescript
interface PlatformInfo {
  /** オペレーティングシステム（例: "macos", "windows", "linux"）。 */
  os: string;
  /** CPU アーキテクチャ（例: "aarch64", "x86_64"）。 */
  arch: string;
}
```

### InfoMod

```typescript
interface InfoMod {
  /** MOD パッケージ名。 */
  name: string;
  /** MOD パッケージバージョン。 */
  version: string;
  /** 人間が読める説明。 */
  description: string | null;
  /** 作者。 */
  author: string | null;
  /** ライセンス識別子。 */
  license: string | null;
  /** MOD にメインプロセスが実行中かどうか。 */
  hasMain: boolean;
  /** 利用可能な bin コマンド名。 */
  binCommands: string[];
  /** 登録済みアセット ID。 */
  assetIds: string[];
}
```
