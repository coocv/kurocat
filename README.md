# 🐈‍⬛ KuroCat · 2D 定制分支（`feat/bellblack-2d`）

<p align="center">
  <strong>🇨🇳 简体中文</strong> &nbsp;|&nbsp;
  <a href="./README.en.md">🇺🇸 English</a> &nbsp;|&nbsp;
  <a href="./README.ja.md">🇯🇵 日本語</a>
</p>

[qianyiai/kurocat](https://github.com/qianyiai/kurocat) 的 2D 定制分支，版本 **5.2.0**。上游的键鼠跟随、透明置顶窗口、托盘、密码防偷窥、MCP、3D 引擎照常可用，这一支在 2D 引擎上多养了一只猫。

<p align="center">
  <img src="docs/2d/hero.webp" width="420" alt="铃铛小黑 静立姿态" />
</p>

## 角色：铃铛小黑（`bellblack`）

一只圆脸黑猫，脖子上一条细皮绳挂着小金铃。

- **圆颊剪影**：`BODY_SLICES_ROUND` 轮廓，脸宽而软，腮线饱满
- **细皮绳 + 金球铃**：砖红皮绳 `#6e2f21`，两端收成细尖；金铃为径向渐变球体，侧面扁椭圆出音孔
- **一笔实体爪**：两条前腿一条连续笔画画出，深色毛上同样有完整轮廓与爪球弧
- **键盘配色耳机**：白键帽配色耳罩，无耳麦；听歌时头顶飘一个手绘音符（单音符与双连音两种）
- **独立条目**：托盘菜单与内嵌「角色外观」中都有「铃铛小黑」，选择后写入 `settings.json` 的 `catBreed`，重启保持

<table>
  <tr>
    <td align="center" width="50%"><img src="docs/2d/paw.webp" width="330" alt="按住鼠标时的姿态" /><br/><sub>按住鼠标：一只爪撑地，前脚掌有实体轮廓与爪球弧</sub></td>
    <td align="center" width="50%"><img src="docs/2d/desktop.webp" width="300" alt="桌面上的铃铛小黑" /><br/><sub>桌面上：圆颊、皮绳金铃、一笔前爪、2D 键盘</sub></td>
  </tr>
</table>

## 听歌律动

系统播放音乐时，整只猫跟着节奏动起来：

- **基础律动**：点头 `beatSin*0.078 + beatHalf*0.026`，面部轻摆 `musicNodGain*beatHalf*3.6`，重心随 `musicGroove*sin(phase*0.5)*0.055` 左右换，尾巴半频摆尾（振幅 6.4）
- **随机动作**：每 2200–5200 ms 从 `enjoy / enjoy / ear / slowBlink / look / tail / tilt` 抽一个，BPM 每轮在 92–126 之间随机
- **打字优先**：点头增益乘 `(1 - min(1, typingHunch)*0.55)`，打字时让位给敲键盘的节奏

<table>
  <tr>
    <td align="center" width="56%"><img src="docs/2d/music.webp" width="360" alt="听歌状态：键盘配色耳机 + 单音符" /><br/><sub>耳机 + 单音符，两条交替轨道排布</sub></td>
    <td align="center" width="44%"><img src="docs/2d/groove.webp" width="420" alt="六帧律动连拍" /><br/><sub>六帧连拍：头左右倾、重心换边、尾巴跟随</sub></td>
  </tr>
</table>

## 闭眼享受

- `IdleFidget.kind = 'enjoy'`：**0.38 s 合眼、0.45 s 睁眼**，整段 2400–3300 ms
- 闭合度超过 0.55 时眼睛画成放松的弯弧（内置皮肤与自定义皮肤的绘制路径都支持），并伴 0.042 的侧歪、`targetGazeY -= 0.12*actEnv`
- 闭眼期间点头幅度乘 `(1 - enjoyEnv*0.45)`，看起来像瘫在音乐里

<p align="center">
  <img src="docs/2d/enjoy.webp" width="620" alt="睁眼与闭眼享受" /><br/>
  <sub>左：平常睁眼；右：闭眼享受（弯眼弧 + 头部侧歪）</sub>
</p>

## 动作细节

- **撑爪**：按住鼠标时一只前爪撑地，深色毛下爪球带完整描边
- **头箍跟随**：横向扭头时头箍与耳罩按 `faceTurnX*0.125` 同步转动
- **项圈随姿态**：`collarAnchor()` 用 `leanDamp = 1 - min(1,|spineLean|)*0.66` 让皮绳在强侧身姿态下也接近水平
- **菜单与版本**：`tray.rs` 的 `breed_bellblack`、`music_detect.rs` 的 `get_music_active` 命令与播放轮询；版本号在 `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` 三处为 5.2.0

<p align="center">
  <img src="docs/2d/menu.webp" width="330" alt="菜单中的铃铛小黑条目" /><br/>
  <sub>「角色外观」里的独立条目，选中后高亮，菜单头部显示 `KuroCat 铃铛小黑`</sub>
</p>

## 构建与运行

前置：macOS 11+ / Windows 10+、Node.js 18+、Rust 工具链。

```bash
git clone https://github.com/coocv/kurocat.git -b feat/bellblack-2d
cd kurocat
npm install
npm run tauri:dev      # 本地调试
npm run tauri:build    # 打包 Release
```

macOS 要让「辅助功能 / 输入监控」授权跨版本保留，用固定的自签名证书签名：`tauri.conf.json` 里不写 `signingIdentity`，打包后手动

```bash
codesign --force --deep --sign "你的自签证书名" src-tauri/target/release/bundle/macos/KuroCat.app
```

自签证书不能替代 Apple 公证，首次打开仍需在「隐私与安全性」里放行。

## 范围与说明

- 只做 **2D**，3D 引擎（`catEngine3d.ts`）不参与本分支改动
- 铃铛小黑是内置 breed，不属于 `cats.json` 自定义皮肤；想做成自定义皮肤需把项圈与耳机绘制参数化
- 自定义皮肤可用 `spots` 叠加图案，但项圈、铃铛由引擎绘制
- `src-tauri/gen/schemas` 等构建产物不入库，已列入 `.gitignore`

## 上游与许可

上游项目与文档：[qianyiai/kurocat](https://github.com/qianyiai/kurocat)，MIT License（见 [LICENSE](LICENSE)）。本分支只改 2D 渲染与角色，上游署名与许可声明全部保留。
