mod app;
mod drip;
mod run;

use anyhow::{Context, Result};
use std::process;

// Sort env_var by length of key in descending order
fn sort_env_var(env_var: &mut Vec<(String, String)>) {
    env_var.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
}

fn main() -> Result<()> {
    ctrlc::set_handler(move || {
        eprintln!("Keyboard interrupt");
        process::exit(0);
    })?;

    let drip::Drip {
        symbol,
        mut env_var,
        exec,
        misc,
    } = drip::load_drip("./drip.toml")?;
    let symbol = symbol.as_str();

    match app::command() {
        app::Commands::Exec { file_path, args } => {
            // FILE_PATH=tests\\data\\z-algorithm.cpp
            // FILE_PATH_WITHOUT_EXT=tests\\data\\z-algorithm
            // FILE_EXT=cpp
            let file_path_without_ext = file_path.split('.').next().with_context(|| {
                format!("failed to split file_path: {}", file_path)
            })?;
            let file_ext = file_path.split('.').last().with_context(|| {
                format!("failed to split file_path: {}", file_path)
            })?;
            // Add FILE_PATH, FILE_PATH_WITHOUT_EXT, FILE_EXT and args to env_var
            drip::push_to_env_var(&mut env_var, "FILE_PATH".to_owned(), file_path.to_owned());
            drip::push_to_env_var(
                &mut env_var,
                "FILE_PATH_WITHOUT_EXT".to_owned(),
                file_path_without_ext.to_owned(),
            );
            drip::push_to_env_var(&mut env_var, "FILE_EXT".to_owned(), file_ext.to_owned());
            for (i, arg) in args.iter().enumerate() {
                drip::push_to_env_var(&mut env_var, i.to_string(), arg.to_owned());
            }
            sort_env_var(&mut env_var);

            // Read scrs_confirm from misc
            let subcmd = "exec";
            let subsubcmd = file_ext;
            let exec = exec.with_context(|| format!("[{}] is not set", subcmd))?;
            let scrs_confirm = exec
                .get(subsubcmd)
                .with_context(|| format!("[{}.{}] is not set", subcmd, subsubcmd))?;

            run::run_core(&symbol, subcmd, subsubcmd, scrs_confirm, env_var, &args)?;
        }
        app::Commands::Misc { subsubcmd, args } => {
            // Add args to env_var
            for (i, arg) in args.iter().enumerate() {
                drip::push_to_env_var(&mut env_var, i.to_string(), arg.to_owned());
            }
            sort_env_var(&mut env_var);

            // Read scrs_confirm from misc
            let subcmd = "misc";
            let subsubcmd = subsubcmd.as_str();
            let misc = misc.with_context(|| format!("[{}] is not set", subcmd))?;
            let scrs_confirm = misc
                .get(subsubcmd)
                .with_context(|| format!("[{}.{}] is not set", subcmd, subsubcmd))?;

            run::run_core(&symbol, subcmd, subsubcmd, scrs_confirm, env_var, &args)?;
        }
    }
    Ok(())
}
