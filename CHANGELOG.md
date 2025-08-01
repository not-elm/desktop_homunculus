## v0.1.0-alpha.3.2

### Bug Fixes

- Fixed the issue where the `.env` file was not being read correctly when launched from the app file.

## v0.1.0-alpha.3

Major design overhaul.

### Features

- Completely revamped the menu screen
    - Added FPS limit feature
- Added API support
- Improved animation transitions for smoother playback

### Bug Fixes

- Fixed flickering when dragging or while idle

## v0.1.0-alpha.2

### Features

- Supported Spring Bone.

### Improvements

- Split settings file into two files: `actions.json` and `mascot_locations.json`.
- Log file is now saved in the Logs directory.
- Accessibility permissions are no longer required.

### Bug Fixes

- Modified loading of mascot position at startup.
- Fixed a flickering problem when crossing between monitors.

## 0.1.0-alpha.1

First release

`./ui/settings`内にはデスクトップキャラクターの横に表示される設定のUIが宣言されています。
このUIは透明なWebview上に表示され、あたかもキャラクターの横でポップアップ表示されるようなUIになります。
このUI表示される際にアニメーションを追加してください。
