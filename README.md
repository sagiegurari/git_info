# git_info

[![crates.io](https://img.shields.io/crates/v/git_info.svg)](https://crates.io/crates/git_info) [![Build Status](https://travis-ci.org/sagiegurari/git_info.svg?branch=master)](http://travis-ci.org/sagiegurari/git_info) [![Build status](https://ci.appveyor.com/api/projects/status/github/sagiegurari/git-info?branch=master&svg=true)](https://ci.appveyor.com/project/sagiegurari/git-info) [![codecov](https://codecov.io/gh/sagiegurari/git_info/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/git_info)<br>
[![license](https://img.shields.io/crates/l/git_info.svg)](https://github.com/sagiegurari/git_info/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/git_info.svg)](https://libraries.io/cargo/git_info) [![Documentation](https://docs.rs/git_info/badge.svg)](https://docs.rs/crate/git_info/) [![downloads](https://img.shields.io/crates/d/git_info.svg)](https://crates.io/crates/git_info)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Extracts git repository information.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/git_info/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](https://github.com/sagiegurari/git_info/blob/master/CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
This library main goal is to provide development/build tools such as [cargo-make](https://sagiegurari.github.io/cargo-make/) the needed information on the current git repository.

<a name="usage"></a>
## Usage
Simply include the library and invoke the get function to pull all info as follows:

```rust
extern crate git_info;

fn main() {
    let info = git_info::get();

    println!("User Name: {}", info.user_name.unwrap());
    println!("User Email: {}", info.user_email.unwrap());
    println!("Current Branch: {}", info.current_branch.unwrap());
    println!("Config: {:#?}", info.config.unwrap());
    println!("Branches: {:#?}", info.branches.unwrap());
}
```

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
git_info = "*"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/git_info/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](https://github.com/sagiegurari/git_info/blob/master/CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
