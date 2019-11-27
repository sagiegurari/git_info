//! # gitinfo
//!
//! Loads git information.
//!

#[cfg(test)]
#[path = "./gitinfo_test.rs"]
mod gitinfo_test;

use crate::types::GitInfo;
use std::collections::HashMap;
use std::io::Error;
use std::process::{Command, ExitStatus};

/// Returns the exit code (-1 if no exit code found)
fn get_exit_code(exit_status: Result<ExitStatus, Error>) -> i32 {
    match exit_status {
        Ok(code) => {
            if !code.success() {
                match code.code() {
                    Some(value) => value,
                    None => -1,
                }
            } else {
                0
            }
        }
        _ => -1,
    }
}

fn load_config(info: &mut GitInfo) {
    let result = Command::new("git").arg("config").arg("--list").output();

    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));

            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();

                let mut config = HashMap::new();

                for mut line in lines {
                    line = line.trim();

                    let mut line_split = line.splitn(2, '=');

                    if let Some(key) = line_split.next() {
                        if let Some(value) = line_split.next() {
                            config.insert(key.to_string(), value.to_string());
                        }
                    }
                }

                info.config = Some(config);
            }
        }
        Err(_) => (),
    };
}

fn load_from_config(info: &mut GitInfo) {
    match info.config {
        Some(ref config) => {
            if let Some(value) = config.get("user.name") {
                info.user_name = Some(value.to_string());
            }
            if let Some(value) = config.get("user.email") {
                info.user_email = Some(value.to_string());
            }
        }
        None => (),
    };
}

fn load_branches(info: &mut GitInfo) {
    let result = Command::new("git").arg("branch").output();

    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));

            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();

                let mut branches = vec![];

                for mut line in lines {
                    line = line.trim();

                    let parts: Vec<&str> = line.split(' ').collect();

                    let name = if line.starts_with("*") {
                        let value = parts[1];
                        info.current_branch = Some(value.to_string());
                        value
                    } else {
                        parts[0]
                    };

                    if name.len() > 0 {
                        branches.push(name.to_string());
                    }
                }

                info.branches = Some(branches);
            }
        }
        Err(_) => (),
    };
}

pub(crate) fn get() -> GitInfo {
    let mut info = GitInfo::new();

    load_config(&mut info);
    load_from_config(&mut info);
    load_branches(&mut info);

    info
}
