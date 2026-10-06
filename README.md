<p align="center">
  <video src="https://github.com/MarkusSela/IW4-Pocket/raw/main/media/demo.mp4" controls muted loop playsinline width="100%"></video>
</p>

<p align="center"><sub>Demo (18 s, iPhone 13 Pro Max) · <a href="https://github.com/MarkusSela/IW4-Pocket/raw/main/media/demo.mp4">If the video does not play, open it directly</a></sub></p>

<p align="center">
  <img src="media/icon.png" width="128" alt="IW4 Pocket icon">
</p>

<h1 align="center">IW4 Pocket</h1>

<p align="center">Modern Warfare 2 (2009) on iPhone, powered by the open-source IW4L runtime. Unofficial and experimental.</p>

<p align="center"><a href="https://ko-fi.com/marukoshi"><img src="https://img.shields.io/badge/Ko--fi-Support-FF5E5B?logo=ko-fi&logoColor=white" alt="Ko-fi"></a></p>

<p align="center">**English** · [Italiano](README.it.md) · [Español](README.es.md) · [Français](README.fr.md) · [Deutsch](README.de.md)</p>

## Status

- **Builds and runs on iPhone** (Rust + Bevy + wgpu on Metal), packaged as an IPA for SideStore.
- **Reaches the main menu.** Touch works as a mouse click in the menus. A PS4 (DualShock 4) controller works through Apple's GameController framework.
- **Known issue:** loading a match map (tested: `mp_rust`) still crashes. iOS terminates the app for using too much memory. This is work in progress and not solved yet.
- Texture handling is adapted for Apple GPUs (no BC/DXT support, so textures are decoded to RGBA8 and capped in size).

## Compatibility

Technically it should run on any iPhone or iPad with Metal and enough free memory, but it has **only been tested on an iPhone 13 Pro Max (6 GB, iOS 27)**. Other devices are unverified. Memory is the limiting factor: iOS lets this app use about 3 GB on that phone, and models with less RAM are likely to fail sooner.

## What you need

- An iPhone or iPad with Metal (iOS 15 or later).
- SideStore (or another sideloading tool) to install the IPA.
- **Your own copy of the Modern Warfare 2 (2009) PC game data**, the folder that contains `zone/`. No game files are included here and none are provided.
- A controller is strongly recommended. A PS4 controller is verified; Xbox-style pads depend on the model.

## Install

1. Download **IW4 Pocket.ipa** from the [latest release](../../releases/latest).
2. Install it with SideStore. Install over an older version to keep your game files; uninstalling deletes them.
3. Open the app once, then copy your MW2 PC folder to **Files > On My iPhone > IW4 Pocket > Games** (any subfolder name works, as long as it contains `zone/`).
4. Launch the app. The first load is slow and the screen can stay pink for a while.

## Files and logs

The app writes `Documents/iw4l-boot.log` (visible in the Files app). It records startup steps, the texture cap that was chosen, memory use (`footprint`, and how much iOS still allows) and any panic or fatal signal. Attach it when reporting a problem.

### Texture size limit

The cap on texture size is chosen automatically from the memory iOS grants the app. To force a value, create a plain text file `iw4l-texture-cap.txt` in the same folder with only a number, for example `256`, then fully close and reopen the app.

## What was changed from upstream IW4L

- No BC texture feature on iOS: textures are decoded to RGBA8 with a size cap.
- Rendering stays on the main thread (the Metal surface must be created there).
- Sandbox paths in `Documents`, with an iOS `Info.plist` and app icon.
- GameController bridge for gamepads, and touch input mapped to the left mouse button.
- Boot log with memory tracing and crash signal capture.

## Build it yourself

Run the **ios-release** workflow from the Actions tab (macOS runner). Give it a tag to publish a release. Nothing runs automatically on push.

## Support

If you like this project, you can support it on [Ko-fi](https://ko-fi.com/marukoshi).

## Credits and license

This project is a port of [IW4L](https://github.com/vladtrc/iw4L) by vladtrc and contributors, licensed under Apache-2.0 (see `LICENSE` and `NOTICE`). The original README is kept in `README.upstream.md`.

> IW4 Pocket is an unofficial fan project. It is not affiliated with or endorsed by Activision, Infinity Ward, Apple or the IW4L authors. Call of Duty and Modern Warfare are trademarks of their owners. You need to own a legitimate copy of the game.
