# IW4L for iOS (unofficial port)

A port of [IW4L](https://github.com/vladtrc/iw4L) (Apache-2.0), an open-source Rust/Bevy runtime for
Call of Duty: Modern Warfare 2 (2009), to iPhone/iPad. This repository contains **no game files**:
you need your own copy of the PC game data.

## What works / what does not
- Builds for `aarch64-apple-ios` (Metal through wgpu), packaged as an unsigned IPA for SideStore.
- Reaches the main menu on an iPhone 13 Pro Max (about 0.5-1.1 GB resident in the menu).
- iOS changes: no BC texture feature (textures are decoded to RGBA8), render on the main thread,
  sandbox paths in `Documents`, GameController bridge, touch-as-mouse, boot log with memory trace.
- Unverified: in-game performance and memory, controller mapping, long sessions.

## Install
1. Open the latest [Release](../../releases) and download `IW4L-unsigned.ipa`.
2. Install it with SideStore (or another sideloading tool).
3. Copy your MW2 PC folder (it must contain `zone/`) to `On My iPhone > IW4L > Games`.
4. Launch. Loading is slow; the first screen may be pink for a while.

## Build
Run the **ios-release** workflow from the Actions tab (macOS runner). Give it a tag to publish a release.

## Upstream
Everything under `crates/` is IW4L and follows its license (see `LICENSE`, `NOTICE`). The original README
is `README.md`.
