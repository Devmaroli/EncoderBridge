//! User-level installer for EncoderBridge and Screen Suite.
//!
//! Writes plugin folders only. It does not elevate, run a script, or touch startup locations.

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("encoderbridge-setup: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mut dest: Option<PathBuf> = None;
    let mut uninstall = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--uninstall" => uninstall = true,
            "--dest" => {
                let value = args
                    .next()
                    .ok_or("--dest needs an absolute plugins directory")?;
                dest = Some(PathBuf::from(value));
            }
            "--help" | "-h" => {
                println!("encoderbridge-setup [--dest ABSOLUTE_PLUGINS_DIR] [--uninstall]");
                println!(
                    "Installs EncoderBridge and Screen Suite for EffectCraft. This is not EncoderCraft."
                );
                return Ok(());
            }
            other => return Err(format!("unknown argument `{other}`")),
        }
    }
    let dirs = match dest {
        Some(path) => vec![path],
        None => default_dirs()?,
    };
    for dir in &dirs {
        if uninstall {
            encoderbridge_setup::uninstall_plugins(dir)?;
            println!(
                "removed EncoderBridge and Screen Suite from {}",
                dir.display()
            );
        } else {
            let wrote = encoderbridge_setup::install_plugins(dir)?;
            println!("installed {} files under {}", wrote.len(), dir.display());
        }
    }
    Ok(())
}

fn default_dirs() -> Result<Vec<PathBuf>, String> {
    let local =
        env::var_os("LOCALAPPDATA").ok_or("LOCALAPPDATA is not set. Re-run with --dest.")?;
    let roaming = env::var_os("APPDATA").ok_or("APPDATA is not set. Re-run with --dest.")?;
    let local = PathBuf::from(local);
    let roaming = PathBuf::from(roaming);
    let host = local.join("Programs").join("Craft").join("effectcraft");
    Ok(encoderbridge_setup::default_plugin_dirs(
        &local,
        &roaming,
        host.is_dir(),
    ))
}
