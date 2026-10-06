# IW4 Pocket 0.2.0 (experimental)

Unofficial iOS port of [IW4L](https://github.com/vladtrc/iw4L), the open-source Rust runtime for Modern Warfare 2 (2009).

**Tested only on an iPhone 13 Pro Max (6 GB).** It should technically run on other iPhones/iPads with Metal, but that is unverified.

## New in 0.2.0
- **New app icon.**
- **Diagnostics built to find where the memory goes** (the app still gets killed by iOS when a map loads):
  - memory is logged for every load stage (start and end): footprint, malloc heap, and what iOS still allows
  - allocation size classes and every single allocation of 16 MB or more
  - device model, iOS version and RAM at the top of the log
  - the previous run's log is kept as `iw4l-boot.prev.log`
  - the noisy "menu catalog" lines are filtered out
  - symbols are kept, so a crash backtrace is readable
- **`iw4l-env.txt`**: put a text file with that name in the app folder (Files > On My iPhone > IW4 Pocket) with one `IW4L_NAME=value` per line, to set the engine's own switches without a rebuild (for example `IW4L_CACHE_BUDGET_MIB=64`). Names must start with `IW4L_`. The applied lines are echoed in the log. Which switches actually reduce memory is not known yet.

## Works
- Starts, loads the game data and reaches the main menu.
- Touch acts as a mouse click in the menus.
- PS4 (DualShock 4) controller via Apple GameController.

## Known issue
- Loading a match map (tested: `mp_rust`) still crashes: iOS terminates the app for using too much memory (about 3 GB on the tested phone). Not solved yet.

## Install
1. Install `IW4.Pocket.ipa` with SideStore. Install over an older version to keep your game files.
2. Open the app once, then copy **your own** MW2 (2009) PC game folder (the one with `zone/`) to Files > On My iPhone > IW4 Pocket > Games.
3. Launch. The first load is slow.

No game files are included. Please attach `iw4l-boot.log` (and `iw4l-boot.prev.log`) when reporting a problem.

Support the project: https://ko-fi.com/marukoshi

Not affiliated with Activision, Infinity Ward, Apple or the IW4L authors.
