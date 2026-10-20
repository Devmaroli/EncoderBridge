# EncoderBridge

EncoderBridge is a plugin for [EffectCraft](https://github.com/storytold/effectcraft). It is not EffectCraft, and it is not EncodeCraft.

It adds **Add to EncodeCraft Queue** and the drag that sends one composition to EncodeCraft. Screen Suite and the Kuwait DOOH screen library (`screen-suite/screens`, crate `effectcraft-screens`) live here too.

## Where to click

Apply the patch below and rebuild EffectCraft. Then:

1. Open a saved composition.
2. **Composition ▸ Add to EncodeCraft Queue** — the item directly under **Add to Render Queue**.
3. The same item is under **File ▸ Export**, again directly under **Add to Render Queue**.
4. Shortcut: **Ctrl+Alt+M** (Cmd+Opt+M on macOS).

To drag a composition onto EncodeCraft: in the Project panel, drag that composition and **release outside the EffectCraft window**. EffectCraft's own drag does not cross into another process, so that release is the drop. EncodeCraft receives the job on `http://127.0.0.1:9878/v1/enqueue` when it is running, or from its inbox after this command launches it. The composition you dragged is the one that is queued. You can also right-click that composition in the Project panel and choose **Add to EncodeCraft Queue**.

The project has to be saved and unmodified. An unsaved project shows: “Save the project before adding it to the EncodeCraft Queue”.

## Why a current EffectCraft build does not show it

[storytold/effectcraft](https://github.com/storytold/effectcraft) `main` builds its menus from `crates/engine/src/menus.rs`. It loads WebAssembly effect plugins from `%APPDATA%\EffectCraft\Plug-ins`. It does not read `%LOCALAPPDATA%\EffectCraft\plugins` and it cannot add a menu command from `plugin.json`.

The patch inserts the menu lines directly under `renderQueue.add` and registers `encodecraft.queue`, and it treats a composition released outside the window as that command.

```bash
git clone https://github.com/storytold/effectcraft
cd effectcraft
git apply /path/to/EncoderBridge/patches/effectcraft-add-to-encodecraft-queue.patch
cargo test -p effectcraft-engine encodecraft -- --test-threads=8
```

The patch was made against commit `9db7f3b21700fd821f215782ed1c82afe077880f`.

## Check that it loaded

After rebuilding with the patch, the menu item is the check: Composition, first entry under Add to Render Queue, labeled **Add to EncodeCraft Queue**. Click it on an unsaved project and the error toast must mention saving the project.

Help ▸ Enable Logging writes `%APPDATA%\EffectCraft\Logs\EffectCraft Log.txt`. A stock storytold build has nothing to log about this plugin, because it never scans the plugin folder. The patched command reports through the toast, not through that log.

The installer still writes the manifests here, which you can confirm on disk:

- `%LOCALAPPDATA%\EffectCraft\plugins\org.effectcraft.encodecraft\plugin.json`
- `%LOCALAPPDATA%\EffectCraft\plugins\org.effectcraft.screen-suite\plugin.json`

`plugin.json` id `org.effectcraft.encodecraft` does not create the menu. The patch does.

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
