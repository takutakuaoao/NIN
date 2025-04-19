## Emitterの実装

### 概要
- Emitterのchange_modeを実装する

### 仕様
- パッケージはnin_coreに所属
- struct名はFrontEndEmitter
- 引数に受け取ったmodeをそのままtauriのemitを使用してFront側に通知する
- tauriのemitに送るイベント名はchanged_mode