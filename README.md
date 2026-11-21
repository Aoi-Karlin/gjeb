# GJEB (Gamma Just Extreme Bright) for Horizon

A port of the Java mod [GJEB](https://modrinth.com/mod/gjeb) to Horizon's native mod route.

GJEB's trick is simple: vanilla's brightness slider stops at 100% ("Bright"); GJEB lets it reach
1000%, so a player can see in the dark without torches, night vision or a resource pack. Horizon
has no such option, so this mod keeps its own brightness, pushes it into the renderer's lightmap
every frame, and adds a slider, a toggle and a key for it. The game's own brightness setting and
`options.txt` are never written, so switching the mod off restores the player's brightness at once.

## What it adds

- **Options > Mods > GJEB**, opened from its **Config** button:
  - **Full brightness** (toggle, on by default): use this mod's brightness instead of the video
    settings slider.
  - **Brightness** (100%–1000%, step 10, default 1000%): 100% is vanilla's Bright, 1000% is gamma
    10, well past full brightness.
- **Toggle full brightness** key (default **G**, in Options > Controls > Key Binds): flips the
  toggle without opening a menu.

## Building it

The mod is native code: it is compiled on the machine that plays it, against the exact game build
it pins, and it runs with the player's rights (no sandbox).

```powershell
# one time: the launcher's portable Rust toolchain
horizon-launcher rust install

# build, install and select the build the instance then plays
horizon-launcher native-build . --trust-native-code --profile optimized
```

The pins live in `horizon-mod.toml`:

```toml
[native]
game = "26.3"
horizon = "<the exact Horizon commit>"
mixins = "<the exact horizon-mixins commit>"
```

Moving to another nightly means changing `horizon` (and usually `mixins`) and rebuilding; a target
the new build renamed fails the build by name instead of misbehaving.

## How it works

- `src/lib.rs` registers the settings and the key, follows the key press on the client tick, and
  keeps the gamma the player picked in an atomic (`override_gamma()`).
- `src/mixins/frame.rs` is a `[[native.mixin]]` **HEAD** injection on the client's frame function:
  while the mod is on, it writes the chosen gamma into the renderer's lightmap gamma, the field the
  game itself fills from the brightness slider.
- Nothing else is touched: no options are rewritten, no packets are sent, and the mod is
  client-only.

## Limits

- The brightness is the lightmap gamma, exactly the quantity vanilla's brightness slider feeds, so
  it brightens the world the way vanilla does; it does not add light sources, so mob spawning and
  block light levels are unchanged.
- A native mod is tied to one game build. Horizon's own nightlies change their internals between
  releases; expect to rebuild after updating.
- 1000% is already far past "everything visible"; above it the lightmap saturates and the world
  turns flat white. The slider stops at 1000% for that reason.

## Credits

- The idea, and the behaviour being ported: **GJEB (GammaJustExtremeBright)**, Apache-2.0,
  <https://github.com/MC-U-Team/GJEB-GammaJustExtremeBright>. No code was copied; Horizon runs Rust.
- Prior art on Horizon that showed the lightmap route: **MoreGamma / fullbright** by tilen
  (`bezi5kt/gamma-rust`), MIT.
- This mod's code: MIT, see `LICENSE` and `NOTICE`.
