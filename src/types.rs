//! # types
//!
//! Public library types.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

#[derive(Debug, Clone)]
/// Holds git info for the given repo directory
pub struct GitInfo {
    /// user.name
    pub user_name: Option<String>,
    /// user.email
    pub user_email: Option<String>,
    /// branch name
    pub branch: Option<String>,
}

impl GitInfo {
    /// Returns new instance
    pub fn new() -> GitInfo {
        GitInfo {
            user_name: None,
            user_email: None,
            branch: None,
        }
    }
}
