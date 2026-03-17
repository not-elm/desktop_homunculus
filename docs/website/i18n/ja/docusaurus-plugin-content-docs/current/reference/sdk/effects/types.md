---
sidebar_position: 100
---

# 型定義

## StampOptions

スタンプビジュアルエフェクトの設定オプションです。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `x` | `number` | スクリーン上の X 位置（ピクセル） |
| `y` | `number` | スクリーン上の Y 位置（ピクセル） |
| `width` | `number` | 幅（ピクセル） |
| `height` | `number` | 高さ（ピクセル） |
| `alpha` | `number` | 不透明度（0--1） |
| `duration` | `number` | スタンプが消えるまでの秒数 |

## StampRequestBody

スタンプエフェクトを作成するためのリクエストボディです。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `asset` | `string` | スタンプ画像のアセット ID |
| `x` | `number` | スクリーン上の X 位置（ピクセル） |
| `y` | `number` | スクリーン上の Y 位置（ピクセル） |
| `width` | `number` | 幅（ピクセル） |
| `height` | `number` | 高さ（ピクセル） |
| `alpha` | `number` | 不透明度（0--1） |
| `duration` | `number` | スタンプが消えるまでの秒数 |
