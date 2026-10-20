# Screen Suite

Screen Suite for EffectCraft, and the Kuwait DOOH screen library it uses (`screens/`, crate `effectcraft-screens`).

The library is loaded by `Library::load()` from `screens/data/` (screens, Size Master presets, adapter presets, SizeMaster presets, combiners). `studio-overrides.json` is the studio size list those overrides were taken from. Piccadilly, Al Nassar, Khalijiya, Tawfeer, and the Palm Trees faces are also applied in code.

`commands.rs` and `panel.rs` are the EffectCraft panel and `screen.*` commands. They are kept here because they belong to Screen Suite. They are not part of this Cargo workspace: they call EffectCraft types (`Session`, `effectcraft_project`, egui) that [storytold/effectcraft](https://github.com/storytold/effectcraft) does not expose as a plugin SDK. `screen-suite-analysis.md` is the notes that go with that panel.

Screen Suite's "send to EncodeCraft" handoff id is `encodecraft`, which runs `encodecraft.queue` (EncoderBridge).
