# Windows installer

`EncoderBridge-Setup-x64.exe` is not stored in git. GitHub Actions builds it from `installer/` and attaches it to a release when a `v*.*.*` tag is pushed, or when the Release workflow is run by hand.

The executable is a normal Rust program. The plugin files are compiled in with `include_bytes!`. It is not an NSIS or UPX self-extractor. Its version resource says product **EncoderBridge**, company **Devmaroli**. The manifest requests `asInvoker` (the current user, no UAC elevation).

Running it writes the same plugin directories to:

- `%LOCALAPPDATA%\EffectCraft\plugins\`
- `%APPDATA%\EffectCraft\Plug-ins\` — the folder EffectCraft v0.11.1 scans at startup
- `%LOCALAPPDATA%\Programs\Craft\effectcraft\plugins\` only when that EffectCraft folder already exists

v0.11.1 loads only `.wasm` effect plug-ins (API version 1) from `Plug-ins`. These directories are not `.wasm` files, and that release has no Composition command for EncodeCraft. The menu item needs a new EffectCraft release built with `patches/effectcraft-add-to-encodecraft-queue.patch`.

`--dest ABSOLUTE_PLUGINS_DIR` writes only there. `--uninstall` deletes the files this installer wrote when `plugin.json` still has the matching id, and leaves anything else in the folder.

That copy is the manifest and the screen library. The Composition menu item is compiled into EffectCraft by `patches/effectcraft-add-to-encodecraft-queue.patch`. Installing this program does not add the menu on its own.

Windows Defender can still flag an unsigned executable. The durable fix is code signing. Two options that do not need a private EV dongle: [Azure Trusted Signing](https://learn.microsoft.com/azure/trusted-signing/) and [SignPath](https://signpath.io/) (free for open source). Neither is set up in this repository.
