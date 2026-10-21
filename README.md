# EncoderBridge

EncoderBridge is an EffectCraft plugin that sends compositions to the EncodeCraft queue. It is not EncodeCraft, and it is not EffectCraft.

The command is **Composition ▸ Add to EncodeCraft Queue**, directly under **Add to Render Queue**. The same item is under **File ▸ Export**. Shortcut: **Ctrl+Alt+M**. The project has to be saved. Drag a composition in the Project panel and release outside the EffectCraft window to queue it, or right-click it and choose **Add to EncodeCraft Queue**.

Stock EffectCraft does not show that command. It only loads `.wasm` effect modules from `%APPDATA%\EffectCraft\Plug-ins`, and those appear in the Effect menu. Installing this plugin does not add a Composition item. EffectCraft itself has to include the host change in [`patches/effectcraft-add-to-encodecraft-queue.patch`](patches/effectcraft-add-to-encodecraft-queue.patch). Use an EffectCraft build that contains that change.

## Windows install

The installer is built from source and is not stored in git.

```bash
cargo build -p encoderbridge-setup --release
```

Run `target/release/encoderbridge-setup.exe`. It is a normal Rust program (no NSIS, no UPX). The version resource says product **EncoderBridge** and company **Devmaroli**. The manifest is `asInvoker` (the current user, no UAC elevation). It copies EncoderBridge and Screen Suite into:

- `%LOCALAPPDATA%\EffectCraft\plugins\`
- `%APPDATA%\EffectCraft\Plug-ins\`

Those copies do not create the menu. Install an EffectCraft version that includes the host change, then run this installer.

`--dest ABSOLUTE_PLUGINS_DIR` writes only there. `--uninstall` deletes the files this installer wrote when `plugin.json` still has the matching id.

## Layout

| Path | What it is |
|---|---|
| `encoderbridge/` | `encodecraft.queue`: menu placement, drag-out rule, HTTP and inbox handoff |
| `screen-suite/` | Screen Suite and the screen library (`effectcraft-screens`) |
| `job/` | EncodeCraft job JSON schema |
| `installer/` | User-level setup program |
| `patches/` | EffectCraft host patch for **Add to EncodeCraft Queue** |

```bash
cargo test --workspace
```

Windows CI on this repository runs that test, clippy, and the installer build.

## Signing

The installer is unsigned. Windows Defender can still quarantine it. [Azure Trusted Signing](https://learn.microsoft.com/azure/trusted-signing/) and [SignPath](https://signpath.io/) (free for open source) are the practical options. Neither is set up in this repository.
