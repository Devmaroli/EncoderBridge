# EncoderBridge

EncoderBridge is an EffectCraft plugin that sends compositions to the EncodeCraft queue. In EffectCraft the command is **Composition ▸ Add to EncodeCraft Queue**, directly under **Add to Render Queue**. Shortcut: **Ctrl+Alt+M**. The same item is under **File ▸ Export**.

The project has to be saved. Drag a composition in the Project panel and release outside the EffectCraft window to queue that composition, or right-click it and choose **Add to EncodeCraft Queue**.

<<<<<<< HEAD
The menu item and the drag are in [`patches/effectcraft-add-to-encodecraft-queue.patch`](patches/effectcraft-add-to-encodecraft-queue.patch). storytold/effectcraft has not shipped that change. The Windows executable that already contains it is below.
=======
## Patched EffectCraft v0.11.2
>>>>>>> a575f45 (Build patched EffectCraft 0.11.2 for Windows and describe EncoderBridge in the README.)

This command requires the patched EffectCraft **v0.11.2** build: EffectCraft v0.11.1 with [`patches/effectcraft-add-to-encodecraft-queue.patch`](patches/effectcraft-add-to-encodecraft-queue.patch) applied. Stock EffectCraft v0.11.1 does not include it.

<<<<<<< HEAD
## Executable

The menu item is compiled into EffectCraft. The file you run is a portable build of the latest EffectCraft main (0.13.0, commit `9db7f3b`, newer than the v0.11.1 release) with this patch already applied.

Download [EffectCraft-0.13.0-encoderbridge-windows-x64.zip](https://github.com/Devmaroli/EncoderBridge/releases/download/effectcraft-0.13.0-encoderbridge/EffectCraft-0.13.0-encoderbridge-windows-x64.zip) from the [effectcraft-0.13.0-encoderbridge](https://github.com/Devmaroli/EncoderBridge/releases/tag/effectcraft-0.13.0-encoderbridge) release. `.github/workflows/effectcraft-patched.yml` builds that zip on Windows from `9db7f3b` plus `patches/effectcraft-add-to-encodecraft-queue.patch` and attaches it there. The same zip is also an Actions artifact of the Patched EffectCraft workflow.

Unzip it and run `effectcraft.exe`. `portable.txt` is next to the exe, so this copy keeps its own settings and does not replace a v0.11.1 install.

## Where to click

In that executable:
=======
## Windows install

Download the assets from the [v0.11.2 release](https://github.com/Devmaroli/EncoderBridge/releases/tag/v0.11.2).
>>>>>>> a575f45 (Build patched EffectCraft 0.11.2 for Windows and describe EncoderBridge in the README.)

1. Run **EffectCraft-0.11.2-EncoderBridge-Setup-x64.exe**, or unzip **EffectCraft-0.11.2-EncoderBridge-windows-x64-portable.zip** and run `effectcraft.exe`.
2. Run **EncoderBridge-Setup-x64.exe** to copy EncoderBridge and Screen Suite into the per-user plugin folders. The Composition menu item comes from the patched EffectCraft program in step 1.

<<<<<<< HEAD
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

**What has to ship in EffectCraft before the menu item exists:** a build with `patches/effectcraft-add-to-encodecraft-queue.patch` applied on top of `9db7f3b` (or any commit with the same menu and command registry, including the v0.11.1 tree where those files match). The patch adds the two menu lines directly under `renderQueue.add`, registers `encodecraft.queue`, and sends a composition dragged out of the Project panel. The zip in the Executable section is that build. The storytold v0.11.1 installer will not show the item no matter where plugin files are copied.

## Windows installer

See [install/README.md](install/README.md). The setup program is built in CI from `installer/` and published as a release asset named `EncoderBridge-Setup-x64.exe`. It is not committed. It installs EncoderBridge and Screen Suite for the current user. It does not install EncodeCraft or EffectCraft.
=======
Open a saved composition, then **Composition ▸ Add to EncodeCraft Queue**.
>>>>>>> a575f45 (Build patched EffectCraft 0.11.2 for Windows and describe EncoderBridge in the README.)

## Layout

| Path | What it is |
|---|---|
| `encoderbridge/` | Queue command: menu placement, drag-out rule, HTTP and inbox handoff |
| `screen-suite/` | Screen Suite and the screen library it uses |
| `job/` | EncodeCraft job schema (`encodecraft-job`) |
| `patches/` | Patch that adds the command to EffectCraft v0.11.1 |
| `installer/` | Source for `EncoderBridge-Setup-x64.exe` |

## Build from source

```bash
cargo test --workspace
cargo build -p encoderbridge-setup --release
```

The patched EffectCraft installer and portable zip are built on a Windows runner by [`.github/workflows/release.yml`](.github/workflows/release.yml) when `v0.11.2` is tagged: clone storytold/effectcraft at `v0.11.1`, apply the patch, set the version to 0.11.2, `cargo build --release`, then package.

These executables are unsigned. [Azure Trusted Signing](https://learn.microsoft.com/azure/trusted-signing/) and [SignPath](https://signpath.io/) (free for open source) are the practical signing options. Neither is set up in this repository.
