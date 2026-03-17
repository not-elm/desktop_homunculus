---
sidebar_position: 100
---

# 型定義

## TimelineKeyframe

VRM モジュールで定義されています。各キーフレームは持続時間とオプションの表情ターゲットを指定します。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `duration` | `number` | 持続時間（秒） |
| `targets` | `Record<string, number>` | 表情名からウェイト（0--1）へのマッピング。無音の場合は省略します。 |
