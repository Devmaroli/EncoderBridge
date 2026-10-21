# EncoderBridge

EncoderBridge is an EffectCraft plugin that sends compositions to the EncodeCraft queue. In EffectCraft the command is **Composition ▸ Add to EncodeCraft Queue**, directly under **Add to Render Queue**. Shortcut: **Ctrl+Alt+M**. The same item is under **File ▸ Export**.

The project has to be saved. Drag a composition in the Project panel and release outside the EffectCraft window to queue that composition, or right-click it and choose **Add to EncodeCraft Queue**.

## Patched EffectCraft v0.11.2

This command requires the patched EffectCraft **v0.11.2** build: EffectCraft v0.11.1 with [`patches/effectcraft-add-to-encodecraft-queue.patch`](patches/effectcraft-add-to-encodecraft-queue.patch) applied. Stock EffectCraft v0.11.1 does not include it.

## Windows install

Download the assets from the [v0.11.2 release](https://github.com/Devmaroli/EncoderBridge/releases/tag/v0.11.2).

1. Run **EffectCraft-0.11.2-EncoderBridge-Setup-x64.exe**, or unzip **EffectCraft-0.11.2-EncoderBridge-windows-x64-portable.zip** and run `effectcraft.exe`.
2. Run **EncoderBridge-Setup-x64.exe** to copy EncoderBridge and Screen Suite into the per-user plugin folders. The Composition menu item comes from the patched EffectCraft program in step 1.

Open a saved composition, then **Composition ▸ Add to EncodeCraft Queue**.

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
