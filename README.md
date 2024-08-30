# git_info

[![crates.io](https://img.shields.io/crates/v/git_info.svg)](https://crates.io/crates/git_info) [![CI](https://github.com/sagiegurari/git_info/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/git_info/actions) [![codecov](https://codecov.io/gh/sagiegurari/git_info/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/git_info)<br>
[![license](https://img.shields.io/crates/l/git_info.svg)](https://github.com/sagiegurari/git_info/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/git_info.svg)](https://libraries.io/cargo/git_info) [![Documentation](https://docs.rs/git_info/badge.svg)](https://docs.rs/crate/git_info/) [![downloads](https://img.shields.io/crates/d/git_info.svg)](https://crates.io/crates/git_info)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Extracts git repository information.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/git_info/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
This library main goal is to provide development/build tools such as [cargo-make](https://sagiegurari.github.io/cargo-make/) the needed information on the current git repository.

<a name="usage"></a>
## Usage
Simply include the library and invoke the get function to pull all info as follows:

<!--{ "examples/example.rs" | lines: 3 | code: rust }-->
```rust
fn main() {
    let info = git_info::get();

    println!(
        "User Name: {}",
        info.user_name.unwrap_or("Unknown".to_string())
    );
    println!(
        "User Email: {}",
        info.user_email.unwrap_or("Unknown".to_string())
    );
    println!("Dirty: {}", info.dirty.unwrap_or(false));
    println!(
        "Current Branch: {}",
        info.current_branch.unwrap_or("Unknown".to_string())
    );

    println!(
        "Last Commit Hash: {}",
        info.head.last_commit_hash.unwrap_or("Unknown".to_string())
    );
    println!(
        "Last Commit Hash (short): {}",
        info.head
            .last_commit_hash_short
            .unwrap_or("Unknown".to_string())
    );

    println!("Config: {:#?}", info.config.unwrap());
    println!("Branches: {:#?}", info.branches.unwrap_or(vec![]));
}
```
<!--{ end }-->

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
git_info = "^0.1.3"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/git_info/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
