#[cfg(windows)]
use std::os::windows::process::CommandExt;

use anyhow::{Context, Result};
use colored::{ColoredString, Colorize};
use question::{Answer, Question};
use std::process::Command;
use toml::value::*;

const COMMON: &str = "common";
const PROG: &str = if cfg!(target_os = "windows") {
    "cmd"
} else {
    "sh"
};
const FLAG: &str = if cfg!(target_os = "windows") {
    "/C"
} else {
    "-c"
};

fn info() -> ColoredString {
    "INFO".to_owned().blue()
}

fn quote(s: &str) -> String {
    format!("\"{}\"", s)
}

fn replace(scr: &str, env_var: Vec<(String, String)>) -> String {
    let mut scr = String::from(scr);
    for (k, v) in env_var {
        scr = scr.replace(&k, &v);
    }
    scr
}

fn run(symbol: &str, scr: &str) -> Result<()> {
    #[cfg(windows)]
    let output = Command::new(PROG).arg(FLAG).raw_arg(scr).output()?;
    #[cfg(not(windows))]
    let output = Command::new(PROG).arg(FLAG).arg(scr).output()?;

    println!("{} {}", symbol.to_owned().red(), scr);
    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub fn run_core(
    symbol: &str,
    subcmd: &str,
    subsubcmd: &str,
    scrs_confirm: &Value,
    env_var: Vec<(String, String)>,
    args: &Vec<String>,
) -> Result<()> {
    let replace = |scr: &str| replace(scr, env_var.clone());
    let match_scrs = |scrs: &Value, confirm: bool| {
        match scrs {
            Value::String(scr) => {
                let scr = replace(scr.as_str());
                let mut scr_content = String::new();
                scr_content.push_str(&quote(&scr));

                println!("[{}] {}: {}", info(), PROG, scr_content);
                println!("[{}] args: {:?}", info(), args);
                anyhow::ensure!(
                    !confirm || Question::new("Continue?").confirm() == Answer::YES,
                    "Aborted"
                );

                run(symbol, &scr)?;
            }
            Value::Array(scrs_as_vec_value) => {
                let mut scrs = vec![];
                let mut scrs_content = "[".to_owned();
                for (i, scr) in scrs_as_vec_value.iter().enumerate() {
                    if i > 0 {
                        scrs_content.push_str(", ")
                    }
                    let scr = replace(scr.as_str().with_context(|| {
                        format!(
                            "[{}.{}.{}] Invalid scr at index {}",
                            subcmd, subsubcmd, PROG, i
                        )
                    })?);
                    scrs.push(scr.to_owned());
                    scrs_content.push_str(&quote(&scr));
                }
                scrs_content.push(']');

                println!("[{}] {}: {}", info(), PROG, scrs_content);
                println!("[{}] args: {:?}", info(), args);
                anyhow::ensure!(
                    !confirm || Question::new("Continue?").confirm() == Answer::YES,
                    "Aborted"
                );

                for scr in scrs {
                    run(symbol, &scr)?;
                }
            }
            _ => anyhow::bail!("[{}.{}.{}] is invalid: {:?}", subcmd, subsubcmd, PROG, scrs),
        };
        Ok(())
    };

    // Type of val is Table, String, or Vec<String>
    if let Some(scrs_confirm) = scrs_confirm.as_table() {
        let confirm = {
            if let Some(confirm) = scrs_confirm.get("confirm") {
                confirm.as_bool().unwrap_or(false)
            } else {
                false
            }
        };
        // scrs_confirm.get(PROG) or scrs_confirm.get(COMMON)
        let scrs = scrs_confirm
            .get(PROG)
            .or_else(|| scrs_confirm.get(COMMON))
            .with_context(|| format!("[{}.{}.{}] is not set", subcmd, subsubcmd, PROG))?;
        match_scrs(scrs, confirm)?;
    } else {
        let scrs = scrs_confirm;
        match_scrs(scrs, false)?;
    }
    Ok(())
}
