# IW4 Pocket 0.3.0 — device profiles (experimental)

The app now looks at the phone and picks its own memory settings. This is the biggest change since 0.2.0.
**No iPhone gameplay result is available for this exact build.** Not proven: that `mp_rust` now loads on a ~3 GB phone.

## What is new
- **Device profiles.** At launch the app reads how much memory iOS grants it and picks a tier:
  - **low** (under 4 GiB): no next-map cache, 1 shader worker
  - **mid** (4 to 5.5 GiB): 64 MiB next-map cache, 2 shader workers
  - **high** (5.5 GiB or more): 256 MiB next-map cache, 2 shader workers
  The resident map is off on iOS in every tier. Mid and high values are untested guesses.
  The chosen tier is the first thing in `iw4l-memory-settings.txt` and the log (`device tier: ...`). These thresholds are first guesses from two phones. Every `IW4L_*` setting in `iw4l-env.txt` still overrides the tier.
- **Resident map off on iOS.** The kept copy of the loaded map shared every decoded image, so images could not be moved and existed twice. Releasing it also frees the weapon GPU textures at teardown when the weapon cache is bounded.
- **Compressed BC textures on GPUs that support them** (Apple9+, e.g. A17 Pro and newer). Older chips such as A15/A16 keep expanding to RGBA8 as before. `IW4L_IOS_BC=0` forces the old path for comparison.

## Credits
The resident-map fix and BC-texture support are by **treuenten** ([issue #1](https://github.com/MarkusSela/IW4-Pocket/issues/1)), imported with their authorship kept in the history. `vendor/wgpu-hal` is wgpu-hal 29.0.4 with one 6-line change (backported from gfx-rs/wgpu#9656); licences included.
Not imported yet: the iOS 27 scene-lifecycle commits. This build still uses the iOS 18.5 SDK.

## Settings (restart the app after editing `iw4l-env.txt`)
```text
IW4L_FPV_RETAIN_MIB=0      # 0-1024, next-map weapon cache
IW4L_SHADER_WORKERS=1      # 1-64
IW4L_MOVE_IMAGES=1         # 0 or 1
IW4L_RESIDENT_MAP=0        # 0 or 1
IW4L_IOS_BC=1              # 0 or 1
```
Unknown names and invalid values are ignored. Please send `iw4l-boot.log` and `iw4l-memory-settings.txt` with every report; they say which tier your phone got.

## Install
Install `IW4.Pocket.ipa` with SideStore over the old version to keep your game files. No game data is included. Not affiliated with Activision, Infinity Ward or Apple.
