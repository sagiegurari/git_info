//! # types
//!
//! Public library types.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use std::collections::HashMap;

#[derive(Debug, Clone)]
/// Holds git info for the given repo directory
pub struct GitInfo {
    /// User name
    pub user_name: Option<String>,
    /// User email
    pub user_email: Option<String>,
    /// Config key/value map
    pub config: Option<HashMap<String, String>>,
    /// Current branch name
    pub current_branch: Option<String>,
    /// All branch names
    pub branches: Option<Vec<String>>,
}

impl GitInfo {
    /// Returns new instance
    pub fn new() -> GitInfo {
        GitInfo {
            user_name: None,
            user_email: None,
            config: None,
            current_branch: None,
            branches: None,
        }
    }
}
