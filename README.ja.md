# 🐈‍⬛ KuroCat · 2D カスタムブランチ（`feat/bellblack-2d`）

<p align="center">
  <a href="./README.md">🇨🇳 简体中文</a> &nbsp;|&nbsp;
  <a href="./README.en.md">🇺🇸 English</a> &nbsp;|&nbsp;
  <strong>🇯🇵 日本語</strong>
</p>

[qianyiai/kurocat](https://github.com/qianyiai/kurocat) の 2D カスタムブランチ、バージョン **5.2.0**。キーボード・マウス追従、透過最前面ウィンドウ、トレイ、パスワード覗き見防止、MCP、3D エンジンは上流のまま使えます。このブランチは 2D エンジンにもう 1 匹猫を追加したものです。

<p align="center">
  <img src="docs/2d/hero.webp" width="420" alt="ベル付き黒猫・立ち姿" />
</p>

> 🔗 **オンラインプレビュー**：[https://coocv.github.io/kurocat/](https://coocv.github.io/kurocat/?skin=bellblack&collar=none) —— ブラウザでそのまま猫を撫でたり、音楽グルーヴや閉眼を確認できます。ページ上でキーを叩いたりマウスを動かすと、前足と視線もそれに合わせて動きます。左上のパネルで首の傾きと横顔の向きを手動操作できます。

## キャラクター：鈴付き黒猫（`bellblack`）

丸い顔の黒猫。細い革紐に小さな金の鈴を付けています。

- **丸い頬のシルエット**：`BODY_SLICES_ROUND` の輪郭。顔は広く柔らかく、頬がふっくら
- **細い革紐＋金の玉鈴**：れんが色の革紐 `#6e2f21` は両端が細く先細り。金の鈴は放射状グラデーションの球体で、側面に偏平な楕円の音孔
- **一筆の前足**：両前脚を 1 本の連続した線で描き、黒い毛でも完全な輪郭と肉球の弧がある
- **キーキャップ配色のヘッドホン**：白いキーキャップ風のイヤーカップ、ブームマイクなし。再生中は手描きの音符が 1 つ浮かぶ（単音符と連桁の複音符）
- **独立した項目**：トレイメニューとアプリ内の「見た目」メニューの両方に「铃铛小黑」があり、選択すると `settings.json` の `catBreed` に保存され、再起動後も維持

<table>
  <tr>
    <td align="center" width="50%"><img src="docs/2d/paw.webp" width="330" alt="マウス押下時の姿勢" /><br/><sub>マウス押下時：片方の前足が地面を支え、肉球まで輪郭が描かれる</sub></td>
    <td align="center" width="50%"><img src="docs/2d/desktop.webp" width="300" alt="デスクトップ上の鈴付き黒猫" /><br/><sub>デスクトップ上：丸い頬、革紐と金鈴、一筆の前足、2D キーボード</sub></td>
  </tr>
</table>

## 音楽に合わせたグルーヴ

システムで音楽が流れると、猫全体がリズムに合わせて動きます：

- **基本の動き**：うなずき `beatSin*0.078 + beatHalf*0.026`、顔の揺れ `musicNodGain*beatHalf*3.6`、重心は `musicGroove*sin(phase*0.5)*0.055` で左右に移り、しっぽは半分の周期で振幅 6.4
- **ランダムな動作**：2200〜5200 ms ごとに `enjoy / enjoy / ear / slowBlink / look / tail / tilt` から 1 つ。BPM はセッションごとに 92〜126 でランダム
- **タイピング優先**：うなずきゲインに `(1 - min(1, typingHunch)*0.55)` を掛け、タイピングのリズムを邪魔しない

<table>
  <tr>
    <td align="center" width="56%"><img src="docs/2d/music.webp" width="360" alt="再生中：キーキャップ配色ヘッドホン＋音符" /><br/><sub>ヘッドホン＋音符 1 つ、2 本の交互レーンに配置</sub></td>
    <td align="center" width="44%"><img src="docs/2d/groove.webp" width="420" alt="6 フレームの連続写真" /><br/><sub>6 フレーム：首の傾き、重心の入れ替え、しっぽの追随</sub></td>
  </tr>
</table>

## 目を閉じてうっとり

- `IdleFidget.kind = 'enjoy'`：**0.38 秒で閉じ、0.45 秒で開く**、全体で 2400〜3300 ms
- 閉じ具合が 0.55 を超えると目がリラックスした弧になり（標準・カスタム両方の描画経路に対応）、0.042 の傾きと `targetGazeY -= 0.12*actEnv` が加わる
- 閉眼中はうなずきの振幅に `(1 - enjoyEnv*0.45)` を掛け、音楽に溶け込んだ姿になる

<p align="center">
  <img src="docs/2d/enjoy.webp" width="620" alt="開眼と閉眼" /><br/>
  <sub>左：通常の開眼。右：目を閉じてうっとり（弧状の目＋首の傾き）</sub>
</p>

## 動きのディテール

- **支える前足**：マウス押下時は片方の前足が地面を支え、黒い毛でも肉球まで輪郭が描かれる
- **ヘッドバンドの追従**：横を向くとバンドとイヤーカップが `faceTurnX*0.125` で一緒に回る
- **首輪が姿勢に沿う**：`collarAnchor()` の `leanDamp = 1 - min(1,|spineLean|)*0.66` により、大きく傾いた姿勢でも革紐はほぼ水平を保つ
- **メニューとバージョン**：`tray.rs` の `breed_bellblack`、`music_detect.rs` の `get_music_active` コマンドと再生ポーリング。バージョン 5.2.0 は `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`

<p align="center">
  <img src="docs/2d/menu.webp" width="330" alt="メニュー内の鈴付き黒猫" /><br/>
  <sub>「見た目」の独立項目。選択すると強調表示され、メニューヘッダーが `KuroCat 铃铛小黑` になる</sub>
</p>

## 定番の黒猫との違い

| | 定番の黒猫 | 鈴付き黒猫 |
|---|---|---|
| 顔 | 標準シルエット | `faceShape: 'round'`、最大 63.5 の丸い頬 |
| 首まわり | 飾りなし／カスタム spots | エンジン描画の革紐＋金の玉鈴 |
| 前脚 | 分割された線 | 1 本の連続した線＋実体の肉球の弧 |
| ヘッドホン | 暗いイヤーカップ＋発光リング | 白キーキャップのイヤーカップ＋琥珀リング、マイクなし |
| 音符 | — | 手描きの単音符／連桁音符、2 レーン |
| 音楽 | ヘッドホンのみ | うなずき、顔の揺れ、重心移動、半分周期のしっぽ＋ランダム動作 |
| 目 | 通常のまばたき | `enjoy` の閉眼、弧状の目＋わずかな傾き |

## 実装方法

すべて Canvas 2D の即時描画で、テクスチャ素材は使っていません。

- **スキンの組み込み**：`DEFAULT_BUILTIN_CATS` の `bellblack` 設定（`emoji: '🔔'`、`faceShape: 'round'`、琥珀色の目 `#fbcb45`、淡い脚線とひげ）、`src-tauri/src/cats.rs` にも同じ定義、トレイには `breed_bellblack`
- **丸い頬**：`BODY_SLICES_ROUND` は 14 個の `{t, hw}` サンプル（t 0.12→1.0、最大 63.5 は t=0.30、胸腹部は 73）。描画時に t で半幅を線形補間し、背骨を軸に左右反転して輪郭にする
- **革紐**：`spots` は使わず、`drawCollarBand()` が二次ベジェを中線に（`rx 49.4`、たるみ `3.2`、両端の跳ね上がり `2.6`）、`halfOf(t) = thick*0.5*(0.3+0.7*sin(πt)^0.8)` で 44 区間の帯を作る。中央 3.4 幅、両端は細く尖る。塗りは上から下への線形グラデーション（16% 明→素色→34% 暗）に 1.5px の接地影
- **首輪の位置**：`collarAnchor()` が `spinePoint(0.382 / 0.462 / 0.468)` の 3 点から `-atan2(Δx, Δy)` で背骨の法線角を求め、`leanDamp = 1-min(1,|spineLean|)*0.66` を掛け `headTilt*0.2` を加える。描画順は帯が前足より先、鈴は前足より後なので、前足を上げても鈴は隠れない
- **金の鈴**：半径 6.2 の球。楕円の接地影、`createRadialGradient`（ハイライト→`#edb938`→陰影）、0.8px の暗金の縁、正面に偏平な楕円の音孔。揺れは `sin(t*5.6)*(0.03 + typingEnergy*0.14 + |spineLean|*0.18) + strikeRock*0.1`
- **ヘッドホン**：2 パス描画。`drawHeadphonesBack()` はバンド（11.1px の淡い縁、9.4px のチャコール外殻、2.0px のサテン光、5.4px の内側パッド、チタンのスライダー）、`drawHeadphonesFront()` はイヤーカップ（`#9fa39c` のクッション、白キーキャップのグラデーション面、7.6×14.4 の凹んだドライバ面、5.4×11.4 の琥珀 LED リング）。着脱は `headsetSpring`、バンドとカップはそれぞれ `faceTurnX*0.125 / 0.25` で首の動きに追従
- **音符**：`noteSprite()` が 3 倍解像度でオフスクリーン canvas に焼き込んでキャッシュ（フェード中に 2 層の墨線が透けないため）、2 種類を交互に使う。出現位置は `spinePoint(0.18)` から左右 2 レーン（+20 / +45）、23×22 の範囲に同じ音符があれば 280 ms 待つ。寿命 2200 ms、上へ漂う
- **グルーヴと閉眼**：`musicBpm / musicBeatPhase / musicGroove / musicAct / nextMusicActAt` を状態として保持し、毎フレーム位相を進めて期限が来たら動作を抽選。`fidgetEyeClosure()` がまぶたの包絡を作り、`isBlissClosed()` が 0.55 を超えたら弧状の目に切り替える
- **音楽の信号**：`music_detect.rs` が 6 回連続で再生中と判定したときだけ、`kuro-music-active` イベントと `get_music_active` コマンドで前端に状態を渡す

## ビルドと実行

前提：macOS 11+ / Windows 10+、Node.js 18+、Rust ツールチェーン。

```bash
git clone https://github.com/coocv/kurocat.git -b feat/bellblack-2d
cd kurocat
npm install
npm run tauri:dev      # ローカル開発
npm run tauri:build    # リリースビルド
```

macOS で「アクセシビリティ / 入力監視」の許可をバージョン間で維持するには、固定の自己署名証明書で署名します。`tauri.conf.json` に `signingIdentity` を書かず、パッケージ後に手動で署名します

```bash
codesign --force --deep --sign "あなたの自己署名証明書" src-tauri/target/release/bundle/macos/KuroCat.app
```

自己署名証明書は Apple の公証の代わりにはならないため、初回起動時は「プライバシーとセキュリティ」で許可が必要です。

## 範囲

- **2D のみ**。3D エンジン（`catEngine3d.ts`）は本ブランチの対象外
- 鈴付き黒猫は内蔵 breed で、`cats.json` のカスタムスキンではない。純粋なカスタムスキンにするには首輪とヘッドホンの描画をパラメータ化する必要がある
- カスタムスキンは `spots` で模様を重ねられるが、首輪と鈴はエンジンが描画する
- `src-tauri/gen/schemas` などのビルド生成物はリポジトリに含めず、`.gitignore` に登録済み

## 上流とライセンス

上流プロジェクトとドキュメント：[qianyiai/kurocat](https://github.com/qianyiai/kurocat)、MIT License（[LICENSE](LICENSE) 参照）。本ブランチは 2D 描画とキャラクターのみを変更し、上流のクレジットとライセンス表記はすべて保持しています。
