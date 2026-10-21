# EncoderBridge

EncoderBridge adds **Add to EncodeCraft Queue** to [EffectCraft](https://github.com/storytold/effectcraft) by patching EffectCraft itself. It is not EncodeCraft, and it is not a file EffectCraft v0.11.1 can load.

v0.11.1 has no Composition entry for EncodeCraft. Its plugin folder only loads `.wasm` effect modules (API version 1), and those show up in the Effect menu. Dropping `plugin.json` into `%APPDATA%\EffectCraft\Plug-ins` does not create **Add to EncodeCraft Queue**.

The menu item and the drag are in [`patches/effectcraft-add-to-encodecraft-queue.patch`](patches/effectcraft-add-to-encodecraft-queue.patch). Apply it to EffectCraft `v0.11.1` or to `main` at `9db7f3b`, then rebuild EffectCraft. See [patches/README.md](patches/README.md). No EffectCraft release contains the patch today.

Screen Suite and the Kuwait DOOH screen library (`screen-suite/screens`, crate `effectcraft-screens`) live in this repo too.

## Where to click

After rebuilding EffectCraft with the patch:

1. Open a saved composition.
2. **Composition ▸ Add to EncodeCraft Queue** — the item directly under **Add to Render Queue**.
3. The same item is under **File ▸ Export**, again directly under **Add to Render Queue**.
4. Shortcut: **Ctrl+Alt+M** (Cmd+Opt+M on macOS).

To drag a composition onto EncodeCraft: in the Project panel, drag that composition and **release outside the EffectCraft window**. EffectCraft's own drag does not cross into another process, so that release is the drop. EncodeCraft receives the job on `http://127.0.0.1:9878/v1/enqueue` when it is running, or from its inbox after this command launches it. The composition you dragged is the one that is queued. You can also right-click that composition in the Project panel and choose **Add to EncodeCraft Queue**.

The project has to be saved and unmodified. An unsaved project shows: “Save the project before adding it to the EncodeCraft Queue”.

## Why a current EffectCraft build does not show it

EffectCraft **v0.11.1** (the current release, 18 Oct 2026) and `main` at `9db7f3b` do the same thing. At startup the desktop app runs `effect.plugins.load` on `%APPDATA%\EffectCraft\Plug-ins` (or `<exe folder>\EffectCraftData\Plug-ins` when `portable.txt` is next to the exe). That scan keeps only `.wasm` files. Each module must be effect plug-in API version 1: a JSON manifest with `"api": 1` and the exports `ec_api_version`, `ec_manifest_ptr`, `ec_manifest_len`, `ec_alloc`, and `ec_render`. Loaded effects show up in the Effect menu. They cannot add a command.

`plugin.json` with `id`, `name`, `version`, and `minAppVersion` is not that format. v0.11.1 never reads it, and the Composition menu is compiled from `crates/engine/src/menus.rs`. v0.11.1 has no **Add to EncodeCraft Queue** entry. Dropping files into Plug-ins cannot create one.

The patch inserts the menu lines directly under `renderQueue.add` and registers `encodecraft.queue`, and it treats a composition released outside the window as that command.

```bash
git clone https://github.com/storytold/effectcraft
cd effectcraft
git apply /path/to/EncoderBridge/patches/effectcraft-add-to-encodecraft-queue.patch
cargo test -p effectcraft-engine encodecraft -- --test-threads=8
```

The patch applies to commit `9db7f3b21700fd821f215782ed1c82afe077880f` and to tag `v0.11.1`. On `9db7f3b` the EncodeCraft command tests passed inside `effectcraft-engine` (17), the menu registry tests passed, and `effectcraft-ui-egui` typechecked with the drag.

## Check that it loaded

After rebuilding with the patch, the menu item is the check: Composition, first entry under Add to Render Queue, labeled **Add to EncodeCraft Queue**. Click it on an unsaved project and the error toast must mention saving the project.

Help ▸ Enable Logging writes `%APPDATA%\EffectCraft\Logs\EffectCraft Log.txt`. A stock storytold build has nothing to log about this plugin, because it never scans the plugin folder. The patched command reports through the toast, not through that log.

The installer writes the same files to both of these folders:

- `%LOCALAPPDATA%\EffectCraft\plugins\org.effectcraft.encodecraft\plugin.json`
- `%APPDATA%\EffectCraft\Plug-ins\org.effectcraft.encodecraft\plugin.json` (the folder v0.11.1 scans; it ignores this non-`.wasm` tree)
- `%LOCALAPPDATA%\EffectCraft\plugins\org.effectcraft.screen-suite\` and the same path under Plug-ins (the screen library JSON)

**What has to ship in EffectCraft before the menu item exists:** a new EffectCraft release built with `patches/effectcraft-add-to-encodecraft-queue.patch` applied on top of `9db7f3b` (or any commit with the same menu and command registry, including the v0.11.1 tree where those files match). The patch adds the two menu lines directly under `renderQueue.add`, registers `encodecraft.queue`, and sends a composition dragged out of the Project panel. No EffectCraft release contains that patch today. v0.11.1 will not show the item no matter where the installer copies files.

## Windows installer

See [install/README.md](install/README.md). The setup program is built in CI from `installer/` and published as a release asset named `EncoderBridge-Setup-x64.exe`. It is not committed. It installs EncoderBridge and Screen Suite for the current user. It does not install EncodeCraft or EffectCraft.

## Layout

| Path | What it is |
|---|---|
| `encoderbridge/` | `encodecraft.queue`: menus, drag-out rule, HTTP and inbox handoff |
| `job/` | EncodeCraft job JSON schema (`encodecraft-job`) |
| `screen-suite/screens/` | Screen library used by Screen Suite |
| `screen-suite/commands.rs`, `panel.rs` | Screen Suite host UI, kept, not compiled here |
| `installer/` | User-level setup program |
| `patches/effectcraft-add-to-encodecraft-queue.patch` | Composition menu, command, and drag-out for storytold/effectcraft |

```bash
cargo test --workspace
```

## Signing

An unsigned installer can still be quarantined. The durable fix is code signing. [Azure Trusted Signing](https://learn.microsoft.com/azure/trusted-signing/) and [SignPath](https://signpath.io/) (free for open source) are the practical options. This repository does not configure either one.
