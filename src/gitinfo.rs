//! # gitinfo
//!
//! Loads git information.
//!

#[cfg(test)]
#[path = "./gitinfo_test.rs"]
mod gitinfo_test;

use crate::types::{GitInfo, Head};
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

fn get_command_output(command: &mut Command) -> Option<String> {
    let result = command.output();

    match result {
        Ok(output) => {
            let exit_code = get_exit_code(Ok(output.status));

            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let line = stdout.trim();

                Some(line.to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn load_state(info: &mut GitInfo) {
    let mut command = Command::new("git");
    command.arg("status").arg("--short");

    let result = get_command_output(&mut command);

    if let Some(output) = result {
        let dirty = output.len() > 0;

        info.dirty = Some(dirty);
    }
}

fn load_config(info: &mut GitInfo) {
    let mut command = Command::new("git");
    command.arg("config").arg("--list");

    let result = get_command_output(&mut command);

    if let Some(output) = result {
        let lines: Vec<&str> = output.split('\n').collect();

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
    let mut command = Command::new("git");
    command.arg("branch").arg("--list").arg("--no-color");

    let result = get_command_output(&mut command);

    if let Some(output) = result {
        let lines: Vec<&str> = output.split('\n').collect();

        let mut branches = vec![];

        for mut line in lines {
            line = line.trim();

            let mut line_split = line.splitn(2, ' ');

            let name = match line_split.next() {
                Some(marker_or_name) => {
                    if marker_or_name == "*" {
                        match line_split.next() {
                            Some(value) => {
                                info.current_branch = Some(value.to_string());
                                value
                            }
                            None => "",
                        }
                    } else {
                        marker_or_name
                    }
                }
                None => "",
            };

            if name.len() > 0 {
                branches.push(name.to_string());
            }
        }

        info.branches = Some(branches);
    }
}

fn load_head(head: &mut Head) {
    let mut command = Command::new("git");
    command.arg("rev-parse").arg("HEAD");

    let mut result = get_command_output(&mut command);

    if let Some(output) = result {
        if !output.is_empty() {
            head.last_commit_hash = Some(output);
        }
    }

    command = Command::new("git");
    command.arg("rev-parse").arg("--short").arg("HEAD");

    result = get_command_output(&mut command);

    if let Some(output) = result {
        if !output.is_empty() {
            head.last_commit_hash_short = Some(output);
        }
    }
}

pub(crate) fn get() -> GitInfo {
    let mut info = GitInfo::new();

    load_state(&mut info);
    load_config(&mut info);
    load_from_config(&mut info);
    load_branches(&mut info);
    load_head(&mut info.head);

    info
}
