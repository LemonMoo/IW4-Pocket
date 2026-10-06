# IW4 Pocket 0.1.8 (experimental)

Unofficial iOS port of [IW4L](https://github.com/vladtrc/iw4L), the open-source Rust runtime for Modern Warfare 2 (2009).

**Tested only on an iPhone 13 Pro Max (6 GB).** It should technically run on other iPhones/iPads with Metal, but that is unverified.

## Works
- Starts, loads the game data and reaches the main menu.
- Touch acts as a mouse click in the menus.
- PS4 (DualShock 4) controller via Apple GameController.

## Known issue
- Loading a match map (tested: `mp_rust`) still crashes: iOS terminates the app for using too much memory. Not solved yet.

## Install
1. Install `IW4 Pocket.ipa` with SideStore. Install over an older version to keep your game files.
2. Open the app once, then copy **your own** MW2 (2009) PC game folder (the one with `zone/`) to Files > On My iPhone > IW4 Pocket > Games.
3. Launch. The first load is slow.

No game files are included. A boot log is written to `Documents/iw4l-boot.log`; attach it when reporting a problem.

Support the project: https://ko-fi.com/marukoshi

Not affiliated with Activision, Infinity Ward, Apple or the IW4L authors.
