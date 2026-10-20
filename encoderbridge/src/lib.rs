//! EncoderBridge, the EffectCraft → EncodeCraft queue bridge.
//!
//! The Composition menu item sits directly under **Add to Render Queue**. Dragging a
//! composition out of the EffectCraft window is the same command: egui drags cannot cross
//! into another process, so releasing outside the window sends that composition.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod queue;

pub use queue::queue;

use std::sync::OnceLock;

use serde_json::Value;

/// Failure shown to the user (toast and the command error string).
#[derive(Debug)]
pub struct QueueError(pub String);

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for QueueError {}

/// What EncoderBridge needs from EffectCraft. The host patch in
/// `patches/effectcraft-add-to-encodecraft-queue.patch` implements this with `Session`.
pub trait Host {
    /// Active composition, or the composition named by `params["comp"]`.
    fn composition_name(&self, params: &Value) -> Result<String, QueueError>;
    /// Saved project path. Empty or missing means the project has not been saved.
    fn project_path(&self) -> Option<&str>;
    fn is_dirty(&self) -> bool;
    fn toast(&mut self, message: String, error: bool);
}

/// Command id EffectCraft runs from the menu and from a drag released outside the window.
pub const COMMAND_ID: &str = "encodecraft.queue";
/// Menu label, directly under Add to Render Queue.
pub const COMMAND_LABEL: &str = "Add to EncodeCraft Queue";
/// `Cmd` is Ctrl on Windows. Shown as Ctrl+Alt+M there.
pub const COMMAND_SHORTCUT: &str = "Cmd+Alt+M";
/// Insert the item immediately after this existing command.
pub const AFTER_COMMAND: &str = "renderQueue.add";

/// One menu placement. `path` is the menu trail, not including the item label.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuPath {
    pub path: &'static [&'static str],
}

/// Composition, then File ▸ Export. Both sit immediately after [`AFTER_COMMAND`].
pub fn menu_paths() -> &'static [MenuPath] {
    &[MenuPath { path: &["Composition"] }, MenuPath { path: &["File", "Export"] }]
}

/// Project-panel gesture. True when a composition drag is released outside the EffectCraft
/// window. That release is the drop onto EncodeCraft.
pub fn drag_out_is_encodecraft_drop(is_composition: bool, pointer_inside_window: bool, released: bool) -> bool {
    is_composition && released && !pointer_inside_window
}

/// Called once at startup by a host that links this crate. Storytold EffectCraft does not
/// call it until the patch in this repository is applied.
pub fn register_loader() {
    static ONCE: OnceLock<&'static str> = OnceLock::new();
    let _ = ONCE.get_or_init(|| COMMAND_ID);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_is_directly_under_add_to_render_queue() {
        assert_eq!(COMMAND_ID, "encodecraft.queue");
        assert_eq!(COMMAND_LABEL, "Add to EncodeCraft Queue");
        assert_eq!(COMMAND_SHORTCUT, "Cmd+Alt+M");
        assert_eq!(AFTER_COMMAND, "renderQueue.add");
        let paths = menu_paths();
        assert_eq!(paths[0].path, ["Composition"]);
        assert_eq!(paths[1].path, ["File", "Export"]);
    }

    #[test]
    fn drag_out_only_a_composition_released_outside_the_window() {
        assert!(drag_out_is_encodecraft_drop(true, false, true));
        assert!(!drag_out_is_encodecraft_drop(true, true, true));
        assert!(!drag_out_is_encodecraft_drop(true, false, false));
        assert!(!drag_out_is_encodecraft_drop(false, false, true));
    }

    #[test]
    fn register_loader_is_idempotent() {
        register_loader();
        register_loader();
    }
}
