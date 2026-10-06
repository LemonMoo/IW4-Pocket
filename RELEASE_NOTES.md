# IW4 Pocket 0.1.9 (experimental)

Unofficial iOS port of [IW4L](https://github.com/vladtrc/iw4L), the open-source Rust runtime for Modern Warfare 2 (2009).

**Tested only on an iPhone 13 Pro Max (6 GB).** It should technically run on other iPhones/iPads with Metal, but that is unverified.

## What changed in 0.1.9
- **Lightmaps no longer keep a second CPU copy on iOS.** A reviewer on Reddit pointed out that data may be copied several times; this removes one such copy (each lightmap page is six images, two of them clones of the others). Whether it is enough is unknown.
- **Much more detailed memory log.** `iw4l-boot.log` now records, several times per second: the footprint iOS enforces, what iOS still allows, malloc heap in use and reserved, memory outside the heap (GPU-backed and mapped memory), and per-category counters (textures on the CPU, bytes sent to the GPU, duplicate image copies, meshes, lightmaps). Engine log lines (map load stages, asset counts) are mirrored into the same file.

## Works
- Starts, loads the game data and reaches the main menu.
- Touch acts as a mouse click in the menus.
- PS4 (DualShock 4) controller via Apple GameController.

## Known issue
- Loading a match map (tested: `mp_rust`) still crashes: iOS terminates the app for using too much memory (about 3 GB on the tested phone). Not solved yet. The new log is meant to show where the memory goes.

## Install
1. Install `IW4.Pocket.ipa` with SideStore. Install over an older version to keep your game files.
2. Open the app once, then copy **your own** MW2 (2009) PC game folder (the one with `zone/`) to Files > On My iPhone > IW4 Pocket > Games.
3. Launch. The first load is slow.

No game files are included. Please attach `Documents/iw4l-boot.log` when reporting a problem.

Support the project: https://ko-fi.com/marukoshi

Not affiliated with Activision, Infinity Ward, Apple or the IW4L authors.
