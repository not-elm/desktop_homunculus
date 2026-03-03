## Memo

- Webviewの奥行きがおかしい
  - カメラの位置が間違っている？

MODの構成を見直したいです。
具体的には現状MODにはアセットの配置のほかに、自動実行されるスクリプトの実行や、メニュー画面の項目の追加などができますが、これを適切に分離するようにしたいです。

"crates/homunculus_deno/src/runtime.rs"でdenoのランタイムを生成していますが、その際にユーザー側から設定値を設定できるようにしたいです。
設定値は./assets内にJSONファイルとして保存し、それを読み込むようにしたいと考えています。
今のところ `RuntimePermissionDescriptorParser`に関する

- play/stopはPOSTでいい
- webviewのcloseは削除


1. global/localは不要
2. サウンド設定は不要

I'd like to remove the following api:
- scripts/js
- mods/menus

modのディレクトリ構成を大幅に変更したいです。

## 1. ルートディレクトリの移動

MODディレクトリのルートディレクトリのパスを./assets/modsから$APP_DATAなどのパスに移動させたい。

## 2. MODのエコシステムの変更

mod.jsonという独自のファイル形式からpackage.jsonに変更する。
また、NPMパッケージをそのままMODとして扱えるようにしたい。例えば以下のような構成にする。
```
- mods
  - sample-mod
    - package.json
    - index.js
    - assets
      - sample.vrm
```
package.jsonに`main`で指定されているスクリプトはMODが読み込まれた際に自動的に実行される。
または、scripts.runに指定されたスクリプトを実行する。

## 3. MODのアセットの取得方法の変更

### 現状のアセットのロード方法

例えばmods/sample/hoge.vrmのアセットを取得したい場合、sample/hoge.vrmというパスを指定し、AssetServer::loadでパスを連結している。

### 修正案1

MODごとに独自のアセットソースを登録し、mods/sample/hoge.vrmというパスを指定した際に、sampleというアセットソースからhoge.vrmを取得するようにする。

```rust
use bevy::asset::io::{AssetSourceBuilder, AssetSourceId};
use bevy::asset::io::file::FileAssetReader;
use bevy::prelude::*;

fn main() {
    let appdata_dir = /* AppData パスを取得 */;

    App::new()
        .register_asset_source(
            AssetSourceId::from("appdata"),
            AssetSourceBuilder::new(move || Box::new(FileAssetReader::new(appdata_dir.clone()))),
        )
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(asset_server: Res<AssetServer>) {
    // 文字列のURLライク表現（embedded:// と同じ仕組み）
    let handle: Handle<Image> = asset_server.load("appdata://textures/icon.png");
}
```

### 修正案2

MODのアセットを取得するためのアセットソースを登録する。
modのディレクトリ内でassetsディレクトリを探し、その中にあるアセットを登録しておく。

Option Bの場合、mods/sample/assets/hoge.vrmのアセットを取得したい場合、"mods://sample/hoge.vrm"というパスになるのでしょうか？
それか"mods://hoge.vrm"になりますか？

このデスクトップマスコットアプリでは主にVRM,VRMA、Webview周りのAPIを提供していますが、これらを利用してMCPサーバを構築できるようにしたいと考えています。
エンジニア向け、または娯楽目的の一般ユーザ向けそれぞれにどのようなMCPツールが考えられるかを調査し、そして現状のAPIに不足している機能を洗い出したいです。

http serverのapiにWebviewのナビゲーションBackとForwardを追加したいです。
内部的にはRequestGoBackとRequestGoForwardイベントを送信するようにします。

現状のMODSのディレクトリ構成を見直したいです。具体的には以下のようにNPMパッケージの構成に近づけたいと考えています。
```
- mods
    - package.json
    - node_modules
      - mod1
        - package.json
        - index.js
        - assets
```
