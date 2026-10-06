<p align="center">
  <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4"><img src="media/demo.gif" width="720" alt="IW4 Pocket demo"></a>
  <br><sub>▶️ <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4">Watch the full demo</a> · 18 s, iPhone 13 Pro Max</sub>
</p>

<p align="center">
  <img src="media/icon.png" width="110" alt="IW4 Pocket icon">
</p>

<h1 align="center">IW4 Pocket</h1>

<p align="center">Modern Warfare 2 (2009) on iPhone, running on the open-source IW4L engine. Unofficial and experimental.</p>

<p align="center"><a href="https://ko-fi.com/marukoshi"><img src="https://img.shields.io/badge/Ko--fi-Support-FF5E5B?logo=ko-fi&logoColor=white" alt="Ko-fi"></a></p>

<p align="center">🌍 **English** · [Italiano](README.it.md) · [Español](README.es.md) · [Français](README.fr.md) · [Deutsch](README.de.md)</p>

## 📊 Status

- ✅ Runs natively on iPhone (Rust + Bevy + Metal), packaged as an IPA for SideStore
- ✅ Reaches the main menu: touch works as a click, a PS4 controller works
- ⚠️ Loading a match map (tested: `mp_rust`) still crashes: iOS closes the app for using too much memory. Not solved yet

## 📱 Compatibility

It should run on any iPhone or iPad with Metal, but it was **only tested on an iPhone 13 Pro Max (6 GB)**. Devices with less RAM will likely fail sooner.

## 🧰 You need

- An iPhone or iPad (iOS 15+) and **SideStore** or another sideloading tool
- **Your own copy of the Modern Warfare 2 (2009) PC game files**, the folder containing `zone/`. Nothing is included here
- A controller (PS4 verified; Xbox-style pads depend on the model)

## 🚀 Install

1. Get **IW4 Pocket.ipa** from the [latest release](../../releases/latest) (or the [`ipa`](ipa) folder)
2. Install it with SideStore, **over** any older version: uninstalling deletes your game files
3. Open the app once, then copy your MW2 folder to **Files > On My iPhone > IW4 Pocket > Games**
4. Launch it. The first load is slow and the screen may stay pink for a while

## 📂 Logs and settings

- `iw4l-boot.log` (same folder) records startup, memory use and crashes. Attach it to bug reports
- To force the texture size limit, add a text file `iw4l-texture-cap.txt` containing only a number such as `256`, then fully restart the app

## 🛠️ Build it yourself

Run the **ios-release** workflow from the Actions tab (macOS runner) and give it a tag to publish a release.

## ⚖️ Credits and legal

Port of [IW4L](https://github.com/vladtrc/iw4L) by vladtrc and contributors (Apache-2.0, see `LICENSE`, `NOTICE`; original README in `README.upstream.md`).

> Unofficial fan project, not affiliated with Activision, Infinity Ward, Apple or the IW4L authors. Call of Duty and Modern Warfare are trademarks of their owners. You must own a legitimate copy of the game.

---

<p align="center">☕ Like it? Support the project on [Ko-fi](https://ko-fi.com/marukoshi)</p>
