use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use toml::value::*;

#[derive(Debug, Serialize, Deserialize)]
struct RawDrip {
    symbol: Option<String>,
    env_var: Option<Map<String, Value>>,
    exec: Option<Table>,
    misc: Option<Table>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Drip {
    pub symbol: String,
    pub env_var: Vec<(String, String)>,
    pub exec: Option<Table>,
    pub misc: Option<Table>,
}

pub fn push_to_env_var(env_var: &mut Vec<(String, String)>, k: String, v: String) {
    env_var.push((format!("${}", k), v.clone()));
    env_var.push((format!("${{{}}}", k), v));
}

// Load config file
pub fn load_drip(file_path: &str) -> Result<Drip> {
    let lines = fs::read_to_string(file_path)?;
    let RawDrip {
        symbol,
        env_var,
        exec,
        misc,
    } = toml::from_str(&lines)?;
    let symbol = {
        if let Some(symbol) = symbol {
            symbol
        } else {
            ">".to_owned()
        }
    };
    let env_var = {
        let mut new_env_var = Vec::<(String, String)>::new();
        if let Some(env_var) = env_var {
            for (k, v) in env_var {
                let s = v
                    .as_str()
                    .expect("[env_var] only supports string")
                    .to_owned();
                push_to_env_var(&mut new_env_var, k, s);
            }
        }
        new_env_var
    };
    let res = Drip {
        symbol,
        env_var,
        exec,
        misc,
    };
    Ok(res)
}
