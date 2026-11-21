# Build status and what is left

This folder is the GJEB port for Horizon. The mod source is complete; the native build has not been
compiled or run on this machine, and this note records exactly why and what remains.

## Verified against this install

Read from the SDK bundles of the installed nightly (`nightly-2026-11-20-0a875da`,
Horizon commit `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, horizon-mixins `e59da909…`, content API
`0.2`), which the launcher keeps under `%APPDATA%\horizon-launcher\native\prebuilt\…`:

- `horizon_client::game::Game<'_>::renderer` is a game internal, and
  `horizon_client_render::gfx::Renderer::gamma` is the lightmap gamma the brightness slider feeds.
  Both are declared under `[[internals]]` in `horizon-mod.toml`.
- `horizon_client_render::gfx::Renderer::update_lightmap(…, gamma: f32, …)` takes that gamma as an
  argument, which is why writing `renderer.gamma` every frame is enough to change it.
- `horizon_client::game::native_hooks::render_frame_pre::Game<'_>::native_render_frame_pre` uses
  the `&mut Game<'_>` binding shape the mixin below declares (the same shape the installed Minimap
  mod's manifest uses).
- The hook, key and setting names in `src/lib.rs` (`Hooks::register`, `register_setting`,
  `register_key_mapping`, `ClientHost::setting_f64/setting_bool/is_down/on/set_setting`,
  `client_tick_post`) match the API the shipped Zoom and FPS Counter mods are written against.

## Not verified

- The crate has not been compiled. No `cargo check`, no mixin application, no `.hmod`, no in-game
  test, no screenshot.
- Two things are therefore unproven: that `Game.renderer` is spelled and typed exactly as the
  helper assumes (`Option<Renderer>` and `as_mut()`), and that writing `renderer.gamma` in
  `render_frame_pre` survives to the lightmap update later in the frame. If the build reports either,
  the fix is local to `src/mixins/frame.rs` and the `[[native.mixin]]` row.

## Why it stopped

`horizon-launcher native-build` needs the launcher's portable Rust toolchain
(`rust install` → `%APPDATA%\horizon-launcher\rust`). On this machine that download
(`static.rust-lang.org`, 161 MB rustc + 45 MB rust-std + 19 MB cargo + 9.5 MB rust-mingw + a 277 MB
WinLibs MinGW zip) stalls again and again: it either sits at 0 bytes or dies part-way, and the
launcher ignores archives placed by hand, so the toolchain never becomes "installed". The SDK assets
themselves were fetched by hand and staged (`native/prebuilt/…`), which is why the build gets as far
as "Checking the prebuilt game" and then asks for the toolchain.

The launcher's proxy setting, the toolchain directory and its build lock were put back the way they
were found; the game that was running was not touched.

## To finish

```powershell
# 1. the toolchain (retry until it completes; it is a one-time download)
horizon-launcher rust install
horizon-launcher rust status --json      # "installed" must stop being null

# 2. compile, install and select the build the instance plays
horizon-launcher native-build . --trust-native-code --profile optimized

# 3. launch and look: Options > Mods > GJEB (slider 100–1000%, toggle, key G)
```

If the build fails on the mixin, it names the row (`gjeb:frame`) and the reason:
- `E_MIXIN_TARGET_MISSING` → the frame function was renamed in this build; pick a neighbour from
  `native/build-data`/`functions.json` (`native_hooks::render_frame_pre` still exists here).
- `Game internals must be declared` → add the missing `[[internals]]` path the message names.
- A compiler error in `frame.rs` → the field shape differs; adapt the two lines there.
