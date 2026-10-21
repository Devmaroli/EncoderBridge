# EncoderBridge

EncoderBridge sends a composition from [EffectCraft](https://github.com/storytold/effectcraft) to the EncodeCraft queue. It is not EncodeCraft.

In the patched EffectCraft program the command is **Composition ▸ Add to EncodeCraft Queue**, directly under **Add to Render Queue**. Shortcut: **Ctrl+Alt+M**. The same item is under **File ▸ Export**. The project has to be saved. Drag a composition in the Project panel and release outside the EffectCraft window to queue that composition, or right-click it and choose **Add to EncodeCraft Queue**.

## Executable (latest EffectCraft)

The file to run is a portable build of the latest EffectCraft main, **0.13.0** (commit `9db7f3b`), with [`patches/effectcraft-add-to-encodecraft-queue.patch`](patches/effectcraft-add-to-encodecraft-queue.patch) compiled in. That is newer than the v0.11.1 release. storytold/effectcraft has not shipped this menu item.

Download [EffectCraft-0.13.0-encoderbridge-windows-x64.zip](https://github.com/Devmaroli/EncoderBridge/releases/download/effectcraft-0.13.0-encoderbridge/EffectCraft-0.13.0-encoderbridge-windows-x64.zip) from the [effectcraft-0.13.0-encoderbridge](https://github.com/Devmaroli/EncoderBridge/releases/tag/effectcraft-0.13.0-encoderbridge) release. [`.github/workflows/effectcraft-patched.yml`](.github/workflows/effectcraft-patched.yml) builds that zip on Windows and attaches it there. The same zip is an Actions artifact of the Patched EffectCraft workflow.

Unzip it and run `effectcraft.exe`. `portable.txt` is next to the exe, so this copy keeps its settings beside the exe and does not replace a v0.11.1 install. EncodeCraft is a separate install.

1. Open a saved composition.
2. **Composition ▸ Add to EncodeCraft Queue**, the item directly under **Add to Render Queue**.
3. The same item is under **File ▸ Export**.
4. Shortcut: **Ctrl+Alt+M**.

An unsaved project shows: “Save the project before adding it to the EncodeCraft Queue”. That toast is the load check. Help ▸ Enable Logging writes `%APPDATA%\EffectCraft\Logs\EffectCraft Log.txt`. A stock EffectCraft build has nothing to log about this command, because it never scans a plugin folder for it.

## Why stock EffectCraft does not show it

EffectCraft **v0.11.1** and `main` at `9db7f3b` load only `.wasm` effect modules (API version 1) from `%APPDATA%\EffectCraft\Plug-ins`. Those show up in the Effect menu. `plugin.json` cannot add a Composition command. The menu is compiled from `crates/engine/src/menus.rs`.

The patch inserts the two menu lines directly under `renderQueue.add`, registers `encodecraft.queue`, and queues a composition released outside the window. `git apply --check` succeeds on `9db7f3b` and on tag `v0.11.1`.

## v0.11.2

Tagging `v0.11.2` runs [`.github/workflows/release.yml`](.github/workflows/release.yml), which clones EffectCraft `v0.11.1`, applies the same patch, sets the version to 0.11.2, and publishes a per-machine setup exe plus a portable zip. That is the current release line, not the latest main. The 0.13.0 zip above is the latest source.

## Plugin files

`EncoderBridge-Setup-x64.exe` copies EncoderBridge and Screen Suite into the per-user plugin folders. It does not create the menu. See [install/README.md](install/README.md).

## Layout

| Path | What it is |
|---|---|
| `encoderbridge/` | `encodecraft.queue`: menus, drag-out rule, HTTP and inbox handoff |
| `job/` | EncodeCraft job JSON schema (`encodecraft-job`) |
| `screen-suite/screens/` | Screen library used by Screen Suite |
| `screen-suite/commands.rs`, `panel.rs` | Screen Suite host UI, kept, not compiled here |
| `installer/` | User-level setup program |
| `patches/effectcraft-add-to-encodecraft-queue.patch` | Composition menu, command, and drag-out |

```bash
cargo test --workspace
```

## Signing

These executables are unsigned. [Azure Trusted Signing](https://learn.microsoft.com/azure/trusted-signing/) and [SignPath](https://signpath.io/) (free for open source) are the practical options. Neither is set up in this repository.
