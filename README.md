# git_info

[![crates.io](https://img.shields.io/crates/v/git_info.svg)](https://crates.io/crates/git_info) [![Build Status](https://travis-ci.org/sagiegurari/git_info.svg)](http://travis-ci.org/sagiegurari/git_info) [![Build status](https://ci.appveyor.com/api/projects/status/yrb4y9cbaf6wtlk7?svg=true)](https://ci.appveyor.com/project/sagiegurari/git_info) [![codecov](https://codecov.io/gh/sagiegurari/git_info/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/git_info)<br>
[![license](https://img.shields.io/crates/l/git_info.svg)](https://github.com/sagiegurari/git_info/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/git_info.svg)](https://libraries.io/cargo/git_info) [![Documentation](https://docs.rs/git_info/badge.svg)](https://docs.rs/crate/git_info/) [![downloads](https://img.shields.io/crates/d/git_info.svg)](https://crates.io/crates/git_info)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Environment variables utility functions.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/git_info/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](https://github.com/sagiegurari/git_info/blob/master/CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
This library has many helper functions to access/modify/check environment variables.

<a name="usage"></a>
## Usage
Simply include the library and invoke the various utility functions.

**Get/Set/Remove environment variables**

```rust
extern crate git_info;

use git_info::{ExpandOptions, ExpansionType};

fn main() {
    if !git_info::exists("MY_ENV_VAR") {
        git_info::set("MY_ENV_VAR", "SOME VALUE");
    }

    let mut value = git_info::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
    println!("Env Value: {}", &value);

    value = git_info::get_or_panic("MY_ENV_VAR");
    println!("Env Value: {}", &value);

    let pre_value = git_info::get_set("MY_ENV_VAR", "SOME NEW VALUE");

    let value = git_info::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
    println!("New Env Value: {}", &value);
    println!("Previous Env Value: {:?}", &pre_value);

    let var_was_set = git_info::set_optional("MY_ENV_VAR", &Some("OPTIONAL VALUE"));
    println!("Env Was Modified: {}", var_was_set);

    let all_vars = git_info::vars(); // returned as Vec<(String, String)>

    for (key, value) in all_vars {
        println!("{}: {}", key, value);
    }

    git_info::set("MY_ENV_VAR2", "SOME VALUE2");

    let value = git_info::get_any(&vec!["MY_ENV_VAR1", "MY_ENV_VAR2"], "default");
    println!("MY_ENV_VAR1 exists: {}", git_info::exists("MY_ENV_VAR1"));
    println!("MY_ENV_VAR2 exists: {}", git_info::exists("MY_ENV_VAR2"));
    println!("Found value: {}", value);

    let mut options = ExpandOptions::new();
    options.expansion_type = Some(ExpansionType::Unix);
    let value = git_info::expand("Env: MY_ENV value is: ${MY_ENV}", Some(options));
    println!("Expanded: {}", &value);
}
```

**Get/Set boolean environment variables and other comparisons**

```rust
extern crate git_info;

fn main() {
    git_info::set_bool("FLAG_VAR", true);
    let mut flag_value = git_info::is_or("FLAG_VAR", false);
    println!("Bool Flag: {}", &flag_value);

    flag_value = git_info::is("FLAG_VAR");
    assert!(flag_value);

    git_info::set_bool("FLAG_VAR", true);
    assert!(git_info::is_equal("FLAG_VAR", "true"));

    git_info::set("MY_ENV_VAR", "SOME VALUE");
    let same = git_info::is_equal("MY_ENV_VAR", "SOME VALUE");
    println!("Value Is Same: {}", &same);
    let mut contains = git_info::contains("MY_ENV_VAR", "_ENV_");
    println!("Value Contained: {}", &contains);
    contains = git_info::contains_ignore_case("MY_ENV_VAR", "_env_");
    println!("Value Contained (case insensitive): {}", &contains);
}
```

**Get/Set list environment variables**

```rust
extern crate git_info;

fn main() {
    git_info::set_list(
        "LIST_TEST_ENV",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );

    let mut values = git_info::get_list("LIST_TEST_ENV").unwrap();
    println!("List Values: {:?}", values);

    let mut same = git_info::is_equal("LIST_TEST_ENV", "1;2;3");
    println!("Same: {}", same);

    let mut options = git_info::ListOptions::new();
    options.separator = Some(",".to_string());
    git_info::set_list_with_options(
        "LIST_TEST_ENV",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        &options,
    );

    values = git_info::get_list_with_options("LIST_TEST_ENV", &options).unwrap();
    println!("List Values: {:?}", values);

    same = git_info::is_equal("LIST_TEST_ENV", "1,2,3");
    println!("Same: {}", same);
}
```

**Bulk Operations**

```rust
extern crate git_info;
extern crate indexmap;

use indexmap::IndexMap;

fn main() {
    let mut env: IndexMap<String, String> = IndexMap::new();
    env.insert("ENV_VAR1".to_string(), "MY VALUE".to_string());
    env.insert("ENV_VAR2".to_string(), "MY VALUE2".to_string());

    git_info::set_all(&env);

    let value = git_info::get_or_panic("ENV_VAR1");
    println!("Value Is: {}", &value);

    let mut found = git_info::is_any_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);

    println!("Any Found: {}", &found);

    found = git_info::is_all_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);

    println!("All Found: {}", &found);

    git_info::remove_all(&vec!["ENV_VAR1", "ENV_VAR2"]);

    found = git_info::is_any_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);

    println!("Any Found: {}", &found);

    env = IndexMap::new();
    env.insert("ENV_VAR1".to_string(), "MY VALUE".to_string());
    env.insert("ENV_VAR2".to_string(), "MY VALUE2".to_string());

    let eval_env = |value: String| {
        let mut buffer = String::from("VALUE-");
        buffer.push_str(&value);
        buffer
    };

    git_info::evaluate_and_set_all(&env, eval_env);

    let value = git_info::get_or_panic("ENV_VAR1");
    println!("Value Is: {}", &value);
}
```

**File Operations**

```rust
extern crate git_info;

fn main() {
    let mut output = git_info::load_file("./src/test/var.env");
    assert!(output.is_ok());

    let eval_env = |value: String| {
        let mut buffer = String::from("PREFIX-");
        buffer.push_str(&value);
        buffer
    };

    output = git_info::evaluate_and_load_file("./src/test/var.env", eval_env);
    assert!(output.is_ok());
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
