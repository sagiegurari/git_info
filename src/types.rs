//! # types
//!
//! Public library types.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
/// Holds git head info
pub struct Head {
    /// The last commit hash
    pub last_commit_hash: Option<String>,
    /// The last commit hash short prefix
    pub last_commit_hash_short: Option<String>,
}

impl Head {
    /// Returns new instance
    pub fn new() -> Head {
        Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// Holds git info for the given repo directory
pub struct GitInfo {
    /// User name
    pub user_name: Option<String>,
    /// User email
    pub user_email: Option<String>,
    /// True if there are non commited changes
    pub dirty: Option<bool>,
    /// Current branch name
    pub current_branch: Option<String>,
    /// All branch names
    pub branches: Option<Vec<String>>,
    /// Head information
    pub head: Head,
    /// Config key/value map
    pub config: Option<HashMap<String, String>>,
}

impl GitInfo {
    /// Returns new instance
    pub fn new() -> GitInfo {
        Default::default()
    }
}
