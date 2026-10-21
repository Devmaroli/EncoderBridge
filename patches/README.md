# EffectCraft patch

`effectcraft-add-to-encodecraft-queue.patch` is a source change to [storytold/effectcraft](https://github.com/storytold/effectcraft). It is not a plugin. EffectCraft v0.11.1 only loads `.wasm` effect modules from `%APPDATA%\EffectCraft\Plug-ins`, and those modules cannot add a Composition menu command.

The patch applies with `git apply` on both of these trees (`git apply --check` succeeded on each):

- `main` at `9db7f3b21700fd821f215782ed1c82afe077880f`
- the current release tag `v0.11.1` (`2a01226894c6bc58ae4fadcc24a10198d693acaa`)

`menus.rs` and the Project panel are the same in those two commits. The patch inserts:

- Composition ▸ **Add to EncodeCraft Queue**, directly under **Add to Render Queue**
- the same item under File ▸ Export, in the same position
- a Project-panel drag: drag a composition and release outside the EffectCraft window. That runs `encodecraft.queue` for the composition you dragged. Right-click the composition and choose the same command.

On `9db7f3b`, `cargo test -p effectcraft-engine --lib encodecraft` passed 17 tests (loopback enqueue, inbox, unsaved project, dirty project). `menus::tests` passed, including the check that every menu line names a registered command. `cargo check -p effectcraft-ui-egui` passed, so the Project-panel drag and the context-menu item typecheck.

```bash
git clone https://github.com/storytold/effectcraft
cd effectcraft
git checkout v0.11.1
git apply /path/to/EncoderBridge/patches/effectcraft-add-to-encodecraft-queue.patch
cargo test -p effectcraft-engine --lib encodecraft
cargo build --release -p effectcraft
```

Rust 1.95 or newer. The release binary is `target/release/effectcraft` (`effectcraft.exe` on Windows). The stock v0.11.1 installer does not contain this patch, so it will not show the menu item.

`.github/workflows/release.yml` builds that tree as version 0.11.2 on a Windows runner and publishes `EffectCraft-0.11.2-EncoderBridge-Setup-x64.exe` and the portable zip on the `v0.11.2` release.
