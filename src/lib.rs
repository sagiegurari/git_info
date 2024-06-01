#![deny(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    unused
)]
#![warn(unknown_lints)]

//! # git_info
//!
//! Extracts git repository information.
//!
//! This library main goal is to provide development/build tools such as
//! [cargo-make](https://sagiegurari.github.io/cargo-make/)the needed information on the current git repository.
//!
//! # Examples
//!
//! ```
//! fn main() {
//!     let info = git_info::get();
//!
//!     println!("User Name: {}", info.user_name.unwrap_or("Unknown".to_string()));
//!     println!("User Email: {}", info.user_email.unwrap_or("Unknown".to_string()));
//!     println!("Dirty: {}", info.dirty.unwrap_or(false));
//!     println!("Current Branch: {}", info.current_branch.unwrap_or("Unknown".to_string()));
//!     println!("Last Commit Hash: {}", info.head.last_commit_hash.unwrap_or("Unknown".to_string()));
//!     println!("Last Commit Hash (short): {}", info.head.last_commit_hash_short.unwrap_or("Unknown".to_string()));
//!     println!("Config: {:#?}", info.config.unwrap());
//!     println!("Branches: {:#?}", info.branches.unwrap_or(vec![]));
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! git_info = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/git_info/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/git_info/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod gitinfo;
pub mod types;

use crate::types::GitInfo;

/// Returns the current git repository information.
///
/// # Example
///
/// ```
/// fn main() {
///     let info = git_info::get();
///
///     println!("User Name: {}", info.user_name.unwrap_or("Unknown".to_string()));
///     println!("User Email: {}", info.user_email.unwrap_or("Unknown".to_string()));
///     println!("Dirty: {}", info.dirty.unwrap_or(false));
///     println!("Current Branch: {}", info.current_branch.unwrap_or("Unknown".to_string()));
///     println!("Last Commit Hash: {}", info.head.last_commit_hash.unwrap_or("Unknown".to_string()));
///     println!("Last Commit Hash (short): {}", info.head.last_commit_hash_short.unwrap_or("Unknown".to_string()));
///     println!("Config: {:#?}", info.config.unwrap());
///     println!("Branches: {:#?}", info.branches.unwrap_or(vec![]));
/// }
/// ```
pub fn get() -> GitInfo {
    gitinfo::get()
}
