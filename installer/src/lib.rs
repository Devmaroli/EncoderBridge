//! Copies EncoderBridge and Screen Suite into EffectCraft's per-user plugin folders.
//!
//! The files are compiled into this binary with `include_bytes!`. Nothing is packed,
//! extracted to Temp, or run. The process asks for the user's own privileges only.

use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::Value;

struct PackedFile {
    rel: &'static str,
    bytes: &'static [u8],
}

struct Plugin {
    id: &'static str,
    files: &'static [PackedFile],
}

const ENCODERBRIDGE: Plugin = Plugin {
    id: "org.effectcraft.encodecraft",
    files: &[
        PackedFile {
            rel: "plugin.json",
            bytes: include_bytes!("../../encoderbridge/plugin.json"),
        },
        PackedFile {
            rel: "README.md",
            bytes: include_bytes!("../../encoderbridge/README.md"),
        },
    ],
};

const SCREEN_SUITE: Plugin = Plugin {
    id: "org.effectcraft.screen-suite",
    files: &[
        PackedFile {
            rel: "plugin.json",
            bytes: include_bytes!("../../screen-suite/plugin.json"),
        },
        PackedFile {
            rel: "README.md",
            bytes: include_bytes!("../../screen-suite/README.md"),
        },
        PackedFile {
            rel: "data/screens.json",
            bytes: include_bytes!("../../screen-suite/screens/data/screens.json"),
        },
        PackedFile {
            rel: "data/combiners.json",
            bytes: include_bytes!("../../screen-suite/screens/data/combiners.json"),
        },
        PackedFile {
            rel: "data/adapter-presets.json",
            bytes: include_bytes!("../../screen-suite/screens/data/adapter-presets.json"),
        },
        PackedFile {
            rel: "data/sizemaster-presets.json",
            bytes: include_bytes!("../../screen-suite/screens/data/sizemaster-presets.json"),
        },
        PackedFile {
            rel: "data/sm-presets.json",
            bytes: include_bytes!("../../screen-suite/screens/data/sm-presets.json"),
        },
        PackedFile {
            rel: "data/studio-overrides.json",
            bytes: include_bytes!("../../screen-suite/screens/data/studio-overrides.json"),
        },
    ],
};

const PLUGINS: &[Plugin] = &[ENCODERBRIDGE, SCREEN_SUITE];

/// Folders the installer writes on a normal Windows run.
///
/// * `%LOCALAPPDATA%\EffectCraft\plugins`
/// * `%APPDATA%\EffectCraft\Plug-ins` — the folder EffectCraft v0.11.1 scans at startup
/// * `%LOCALAPPDATA%\Programs\Craft\effectcraft\plugins` when that install folder already exists
///
/// EffectCraft v0.11.1 only loads `.wasm` files from the Plug-ins folder (effect API v1).
/// It does not read `plugin.json` and cannot add Composition menu commands from disk.
pub fn default_plugin_dirs(
    local_app_data: &Path,
    roaming_app_data: &Path,
    host_dir_exists: bool,
) -> Vec<PathBuf> {
    let mut dirs = vec![
        local_app_data.join("EffectCraft").join("plugins"),
        roaming_app_data.join("EffectCraft").join("Plug-ins"),
    ];
    if host_dir_exists {
        dirs.push(
            local_app_data
                .join("Programs")
                .join("Craft")
                .join("effectcraft")
                .join("plugins"),
        );
    }
    dirs
}

pub fn install_plugins(plugins_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let plugins_dir = check_dest(plugins_dir)?;
    let mut wrote = Vec::new();
    for plugin in PLUGINS {
        let dir = plugins_dir.join(plugin.id);
        for file in plugin.files {
            let path = dir.join(file.rel);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
            }
            fs::write(&path, file.bytes)
                .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
            wrote.push(path);
        }
    }
    Ok(wrote)
}

/// Deletes only the files this installer wrote, and only when `plugin.json` still has our id.
/// The directory is removed when nothing else is left in it.
pub fn uninstall_plugins(plugins_dir: &Path) -> Result<(), String> {
    let plugins_dir = check_dest(plugins_dir)?;
    for plugin in PLUGINS {
        let dir = plugins_dir.join(plugin.id);
        let manifest = dir.join("plugin.json");
        if !manifest.is_file() {
            continue;
        }
        let text = fs::read_to_string(&manifest)
            .map_err(|e| format!("cannot read {}: {e}", manifest.display()))?;
        let id = serde_json::from_str::<Value>(&text)
            .ok()
            .and_then(|v| v.get("id").and_then(Value::as_str).map(str::to_string));
        if id.as_deref() != Some(plugin.id) {
            continue;
        }
        for file in plugin.files {
            let path = dir.join(file.rel);
            if path.is_file() {
                fs::remove_file(&path)
                    .map_err(|e| format!("cannot remove {}: {e}", path.display()))?;
            }
        }
        remove_empty_dirs(&dir);
    }
    Ok(())
}

fn remove_empty_dirs(dir: &Path) {
    let mut stack = vec![dir.to_path_buf()];
    let mut seen = Vec::new();
    while let Some(current) = stack.pop() {
        let Ok(rd) = fs::read_dir(&current) else {
            continue;
        };
        for entry in rd.flatten() {
            if entry.path().is_dir() {
                stack.push(entry.path());
            }
        }
        seen.push(current);
    }
    seen.sort_by_key(|p| std::cmp::Reverse(p.components().count()));
    for path in seen {
        let Ok(rd) = fs::read_dir(&path) else {
            continue;
        };
        if rd.count() == 0 {
            let _ = fs::remove_dir(path);
        }
    }
}

fn check_dest(dir: &Path) -> Result<PathBuf, String> {
    let raw = dir.to_string_lossy();
    if !dir.is_absolute() {
        return Err("destination must be an absolute path".into());
    }
    if raw.contains('\0') || dir.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err("destination must not contain '..'".into());
    }
    Ok(dir.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Tmp(PathBuf);
    impl Drop for Tmp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn tmp() -> Tmp {
        let p = std::env::temp_dir().join(format!(
            "encoderbridge-setup-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        fs::create_dir_all(&p).unwrap();
        Tmp(p)
    }

    #[test]
    fn rejects_relative_and_parent_destinations() {
        assert!(
            install_plugins(Path::new("plugins"))
                .unwrap_err()
                .contains("absolute")
        );
        assert!(uninstall_plugins(Path::new("plugins")).is_err());
    }

    #[test]
    fn installs_both_plugins_and_uninstall_removes_only_those_files() {
        let t = tmp();
        let plugins = t.0.join("plugins");
        let wrote = install_plugins(&plugins).unwrap();
        assert!(wrote.len() >= 8);
        let queue = plugins
            .join("org.effectcraft.encodecraft")
            .join("plugin.json");
        let suite = plugins
            .join("org.effectcraft.screen-suite")
            .join("plugin.json");
        let screens = plugins
            .join("org.effectcraft.screen-suite")
            .join("data")
            .join("screens.json");
        assert!(queue.is_file());
        assert!(suite.is_file());
        assert!(screens.is_file());
        let id = |p: &Path| {
            serde_json::from_str::<Value>(&fs::read_to_string(p).unwrap()).unwrap()["id"]
                .as_str()
                .unwrap()
                .to_string()
        };
        assert_eq!(id(&queue), "org.effectcraft.encodecraft");
        assert_eq!(id(&suite), "org.effectcraft.screen-suite");
        let extra = plugins
            .join("org.effectcraft.encodecraft")
            .join("notes.txt");
        fs::write(&extra, b"keep").unwrap();
        uninstall_plugins(&plugins).unwrap();
        assert!(!queue.exists());
        assert!(!suite.exists());
        assert!(!screens.exists());
        assert!(extra.is_file(), "a file this installer did not write stays");
        assert!(plugins.join("org.effectcraft.encodecraft").is_dir());
    }

    #[test]
    fn uninstall_skips_a_different_plugin_id() {
        let t = tmp();
        let plugins = t.0.join("plugins");
        install_plugins(&plugins).unwrap();
        let manifest = plugins
            .join("org.effectcraft.encodecraft")
            .join("plugin.json");
        fs::write(
            &manifest,
            br#"{"id":"other.plugin","name":"Other","version":"0","minAppVersion":"0"}"#,
        )
        .unwrap();
        uninstall_plugins(&plugins).unwrap();
        assert!(manifest.is_file());
    }

    #[test]
    fn stock_plugins_folder_is_roaming_plug_ins() {
        let local = Path::new("/Users/me/AppData/Local");
        let roaming = Path::new("/Users/me/AppData/Roaming");
        let dirs = default_plugin_dirs(local, roaming, false);
        assert_eq!(
            dirs,
            vec![
                local.join("EffectCraft").join("plugins"),
                roaming.join("EffectCraft").join("Plug-ins"),
            ]
        );
        let with_host = default_plugin_dirs(local, roaming, true);
        assert_eq!(
            with_host[2],
            local
                .join("Programs")
                .join("Craft")
                .join("effectcraft")
                .join("plugins")
        );
    }

    #[test]
    fn parent_dir_is_rejected_on_an_absolute_path() {
        let sneaky: PathBuf = if cfg!(windows) {
            PathBuf::from(r"C:\temp\ec\..\etc")
        } else {
            PathBuf::from("/tmp/ec/../etc")
        };
        assert!(install_plugins(&sneaky).unwrap_err().contains(".."));
    }
}
