use anyhow::{Context, Result};
use clap_complete::{generate_to, Shell};
use std::env;
use std::fs;

include!("src/app.rs");

fn main() -> Result<()> {
    let min_version = "1.56";
    // rustc version too small or can't figure it out
    anyhow::ensure!(
        version_check::is_min_version(min_version) == Some(true),
        "`drip-rs` requires rustc >= {}",
        min_version
    );

    let out_dir = match env::var_os("SHELL_COMPLETIONS_DIR").or_else(|| env::var_os("OUT_DIR")) {
        Some(out_dir) => out_dir,
        None => {
            anyhow::bail!(
                "OUT_DIR environment variable not defined. \
Please file a bug: \
https://github.com/nullputra/drip/issues/new"
            );
        }
    };
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("Failed to create directory: {}", out_dir.to_string_lossy()))?;

    let mut app = build_app();
    // let name = app.get_name().to_owned();
    for shell in [Shell::Bash, Shell::PowerShell, Shell::Fish, Shell::Elvish] {
        // generate_to(shell, &mut app, &name.clone(), &out_dir)?;
        generate_to(shell, &mut app, "drip-rs", &out_dir)?;
    }
    Ok(())
}
