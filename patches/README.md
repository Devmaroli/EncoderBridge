# EffectCraft patch

`effectcraft-add-to-encodecraft-queue.patch` applies to [storytold/effectcraft](https://github.com/storytold/effectcraft) commit `9db7f3b21700fd821f215782ed1c82afe077880f`.

It does three things in that host:

- Composition ▸ **Add to EncodeCraft Queue**, directly under **Add to Render Queue**
- The same item under File ▸ Export, in the same position
- Drag a composition in the Project panel and release outside the EffectCraft window. That runs `encodecraft.queue` for the composition you dragged. Right-click the composition and choose the same command.

```bash
git clone https://github.com/storytold/effectcraft
cd effectcraft
git apply /path/to/EncoderBridge/patches/effectcraft-add-to-encodecraft-queue.patch
```

A stock EffectCraft build does not show the item. Its menus are a fixed list, and `plugin.json` cannot add a command.
