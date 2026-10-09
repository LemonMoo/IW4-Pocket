# IW4 Pocket 0.3.1 (test build, not a public release)

Built to find out **where the memory goes** and **why the app quits after loading `mp_rust`**. Not a fix.

- **Heap per asset kind.** The log gets a line `heap net growth per asset kind (top 10, ...)` with how much the heap grew while loading each kind of asset (animations, models, weapons, ...). It is a net, per-thread ranking, not exact sizes.
- **Graphics errors are logged.** If the renderer fails, the app still quits as before, but `iw4l-boot.log` now has a `RENDER ERROR <type>: <text>` line with the cause and the memory left. Before, such a quit looked like a silent `code=1`.
- **New app icon.**

Please send `iw4l-boot.log` and `iw4l-memory-settings.txt` after trying `mp_rust`. Look for `RENDER ERROR` and `heap net growth`.
