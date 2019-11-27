//! # gitinfo
//!
//! Loads git information.
//!

#[cfg(test)]
#[path = "./gitinfo_test.rs"]
mod gitinfo_test;

use crate::types::GitInfo;
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

fn load_from_config(info: &mut GitInfo) {
    let result = Command::new("git").arg("config").arg("--list").output();

    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));

            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();
                for mut line in lines {
                    line = line.trim();

                    if line.starts_with("user.name=") {
                        let parts: Vec<&str> = line.split('=').collect();
                        let value = parts[1];
                        info.user_name = Some(value.to_string());
                    } else if line.starts_with("user.email=") {
                        let parts: Vec<&str> = line.split('=').collect();
                        let value = parts[1];
                        info.user_email = Some(value.to_string());
                    }
                }
            }
        }
        Err(_) => (),
    };
}

fn load_branch(info: &mut GitInfo) {
    let result = Command::new("git").arg("branch").output();

    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));

            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();
                for mut line in lines {
                    line = line.trim();

                    if line.starts_with("*") {
                        let parts: Vec<&str> = line.split(' ').collect();
                        let value = parts[1];
                        info.branch = Some(value.to_string());
                    }
                }
            }
        }
        Err(_) => (),
    };
}

pub(crate) fn get() -> GitInfo {
    let mut info = GitInfo::new();

    load_from_config(&mut info);
    load_branch(&mut info);

    info
}
