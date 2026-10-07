# IW4 Pocket 0.2.1 — memory candidate (experimental)

This candidate changes memory ownership and shader scheduling, not just diagnostics.
**No iPhone gameplay result is available for this version.** Previous versions reached the menu on an iPhone 13 Pro Max (6 GB), but `mp_rust` was terminated near the app's roughly 3 GiB memory allowance. This release does not claim that crash is solved.

## Changes

- iOS defaults to **zero optional FPV payload retention between maps**. `IW4L_FPV_RETAIN_MIB=0` disables this reuse cache; 1–1024 sets a payload-byte budget in MiB. Active weapon assets and required donor batches are untouched. Admission happens after image wrapping, when the actual retained size is known; rejected cache entries can be decoded again for a later map.
- **Move uniquely owned Bevy images** instead of unconditionally cloning their pixel buffers. Shared images still copy safely. `IW4L_MOVE_IMAGES=0` restores copying for A/B comparison; default is `1`.
- **One shader compilation worker by default on iOS**, configurable with `IW4L_SHADER_WORKERS=1` through `64` and capped by the existing load pool. The implementation limits runnable shader chunks, not merely their size. It does not reduce the number of shaders or their final storage, and may load more slowly.
- Alternate image wrappers preserve the already decoded image's texels, dimensions and mip count. They no longer feed RGBA back into the compressed-texture decoder. Payload accounting is rebalanced when the iOS conversion changes size, preventing mismatched subtraction on drop.
- Live footprint, malloc heap and optional FPV ownership gauge are separated from cumulative texture/transfer traffic. No inference of "GPU memory" from footprint minus heap. Phase deltas overlap and must not be summed.

## Restart-only settings and A/B

In Files > On My iPhone > IW4 Pocket, create `iw4l-env.txt`:

```text
IW4L_FPV_RETAIN_MIB=0
IW4L_SHADER_WORKERS=1
IW4L_MOVE_IMAGES=1
```

Fully quit and restart after editing. Change one setting at a time, using the same phone, map, game files and texture cap. Suggested comparison: FPV budget `64`, workers `4`, or image move `0`, separately. FPV admission counts payload bytes, not allocator overhead, and retained bytes overlap active assets.

The settings file now accepts only validated tuning controls: the three above; `IW4L_IMAGE_DECODE_BUDGET_MIB` / `IW4L_CACHE_BUDGET_MIB` (0–4096); `IW4L_SOUND` (`on`/`off`, `1`/`0`). Unknown names, path overrides and malformed values are ignored without echoing their contents. The texture-cap file remains supported separately.

`iw4l-memory-settings.txt` records the effective memory controls at startup, before a possible termination. `summary.json`, when generated, also records controls, effective shader concurrency and a memory snapshot. Send these with `iw4l-boot.log` and `iw4l-boot.prev.log`.

## Installation and remaining checks

Install over the existing app with SideStore to preserve game data. The bundle remains `com.markussela.iw4l`; the existing icon is unchanged. No game files are included.

The IPA verification report checks plist/version, arm64 Mach-O, retained Rust symbols, exact packaged icons and binary settings markers. It does **not** verify gameplay or signing. Device checks still required: menu/controller, first `mp_rust` load, weapon textures, return to menu and second map load, and the A/B memory logs.

No new lazy weapon loading, wholesale donor eviction, game-file deletion or promise of a specific MiB saving is included.
