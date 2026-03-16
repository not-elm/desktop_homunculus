---
sidebar_position: 1
---

# entities

名前による Bevy ECS エンティティの検索と操作を行います。エンティティは 3D シーンの構成要素です -- VRM キャラクター、カメラ、WebView、スポーンされたオブジェクトはすべて、数値 ID、オプションの名前、トランスフォーム（位置、回転、スケール）を持つエンティティです。

## インポート

```typescript
import { entities } from "@hmcs/sdk";
```

## 関数一覧

| 関数 | 説明 |
|------|------|
| [findByName](./findByName) | 人間が読める名前でエンティティを検索する |
| [name](./name) | エンティティ ID に紐づけられた名前を取得する |
| [transform](./transform) | エンティティの位置・回転・スケールを読み取る |
| [setTransform](./setTransform) | エンティティに（部分的な）トランスフォームを書き込む |
| [move](./move) | ワールドまたはビューポート座標でエンティティを移動する |
| [tweenPosition](./tweenPosition) | エンティティの位置をスムーズにアニメーションする |
| [tweenRotation](./tweenRotation) | エンティティの回転をスムーズにアニメーションする |
| [tweenScale](./tweenScale) | エンティティのスケールをスムーズにアニメーションする |
