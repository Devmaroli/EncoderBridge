# EncodeCraft Queue

EffectCraft command **Add to EncodeCraft Queue**. It is not EncodeCraft.

After the host patch is applied, the item is in two places, directly under **Add to Render Queue**:

- Composition ▸ Add to EncodeCraft Queue
- File ▸ Export ▸ Add to EncodeCraft Queue

Shortcut: Ctrl+Alt+M on Windows (Cmd+Opt+M on macOS).

Drag a composition in the Project panel and release outside the EffectCraft window. That sends the same command for the composition you dragged.

The project must be saved. EffectCraft then posts the job to `http://127.0.0.1:9878/v1/enqueue`, or writes EncodeCraft's inbox and launches EncodeCraft when it is not running.
