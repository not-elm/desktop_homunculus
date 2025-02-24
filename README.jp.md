# bevy_baby

> [!CAUTION]
> This crate is in the early stages of development and is subject to breaking changes.

## What is this?

bevy_babyは[bevy](https://github.com/bevyengine/bevy)を使ってdesktop mascotを召喚するためのアプリです。
bevyはRustで書かれたゲームエンジンでECSというアーキテクチャを採用しており、高速で軽量なアプリケーションを作成することができます。
このアプリでは複数のモデルのアニメーションのリターゲッティングを行っているためこのアーキテクチャの恩恵がふんだんに受けられています。

モデルの表示には[VRM](https://vrm.dev/en/vrm1/)、アニメーションには[VRMA](https://vrm.dev/en/vrma/)を採用しています。
VRM0.Xのモデルの場合、一度BlenderなどのツールからVRM1.0に変換してください。

また、このプロジェクトは私の作成したライブラリの宣伝も兼ねています。
bevyを既に使っている方、またはこれから始めようとしている方はぜひお試しください。

| ライブラリ名           | 説明               | リンク                                                        |
|------------------|------------------|------------------------------------------------------------|
| bevy_flurx       | コルーチンの提供         | [github](https://github.com/not-elm/bevy_flurx)            | 
| bevy_webview_wry | Webviewが使えるようになる | [github](https://github.com/not-elm/bevy_webview_projects) |

## 目的

このプロジェクトはBlenderとBevyの学習を目的としてスタートしました。
またVRMの知識もゼロから始めているため、まだ実装が甘い箇所が多いかもしれません。
有識者の方々からのアドバイスをお待ちしています。

## Supported platforms

現状MacOSとWindowsにサポートさせる予定です。

| Platform | usable |
|----------|--------|
| Windows  | ✅      |
| MacOS    | ❌      |  

## Document

Coming soon...

## アクションの説明

詳細な仕様はこれからドキュメント化していきますが、ここでは基本的な仕様を説明します。
将来的に仕様は変更される可能性があることに注意してください。

現時点ではアクションはアニメーションの状態を表します。
アクションはグループごとに分類されています。
`assets/animaitons`配下を見てください。`idle/`や`drag/`などのディレクトリがあり、その中に幾つかのVRMAファイルが配置されています。
これらのディレクトリがアクションのグループで、その中のVRMAファイルがアクションになります。

![action_group](./docs/action_group.drawio.png)

ステートマシンのようにアクションから別のアクションに遷移させることができます。
現状３つの遷移状態が定義されています。

| 遷移状態   | 説明                           |
|--------|------------------------------|
| auto   | 一定時間経過後に同じグループ内の別のアクションに遷移する |
| manual | アニメーション再生後に指定したアクションに遷移する    |
| none   | 遷移なし                         |

アクションの遷移方法はメニュー画面から設定できます。
メニュー画面はマスコットを右クリックすることで開けます。

## TODO

- [ ] ドキュメントの作成
- [ ] Local HTTPサーバーの実装

### アクションの拡張

アニメーションだけでなくマスコットのスケールの変更などより多くの動作を行えるようにしたいと考えています。

### Http Server

ローカルHTTPサーバーを立ててリクエストから能動的にアクションを遷移できるようにしたいと考えています。
TwitchなどのAPIと連携して特定のコメントが来たらアクションを変更するようなことができると配信に彩りが出るかもしれません。

## カスタムVRMを使用するには？

サンプルで用意しているVRMとアニメーション（VRMA）はBlenderからエクスポートしています。
UnityからエクスポートされたVRMとはローカル軸やボーンの位置関係が異なる？ようなので、もしボーンが崩れる場合はBlenderからエクスポートしなおしてください。

Blenderからエクスポートする際は以下のアドオンをインストールしてください。

- [VRM Add-on for Blender(EN)](https://vrm-addon-for-blender.info/en/)
- [VRM Add-on for Blender(JP)](https://vrm-addon-for-blender.info/jp/)

## Credits

- [VRM Sample Model](https://vroid.pixiv.help/hc/ja/articles/4402394424089-AvatarSample-A-Z)
- Character animation credits to pixiv Inc.'s VRoid Project

## License

このプロジェクトはMITライセンスのもとで公開されています。

## 連絡先

- Discord: @not_not_elm

