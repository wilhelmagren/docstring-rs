<div align="center">
<br/>
<div align="left">
<br/>
<p align="center">
</p>
</div>

[![Crates.io (latest)](https://img.shields.io/crates/v/docstring-rs)](https://crates.io/crates/docstring-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/wilhelmagren/docstring-rs/graph/badge.svg?token=DAOMDVU8QF)](https://codecov.io/gh/wilhelmagren/docstring-rs)
[![CI](https://github.com/wilhelmagren/docstring-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/wilhelmagren/docstring-rs/actions/workflows/ci.yml)
[![Tests](https://github.com/wilhelmagren/docstring-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/wilhelmagren/docstring-rs/actions/workflows/tests.yml)

</div>

## 🔎 Overview
Automatically generate new files or update existing codebases with header docstrings based on LICENSE information.

## 📦 Installation
```
cargo install docstring-rs
```

## 🚀 Usage
```
Usage: docstring-rs.exe [OPTIONS] --directory <DIRECTORY>

Options:
  -d, --directory <DIRECTORY>  Name of the directory in which to create new file or update existing files,
                               if it does not already exist, creates the directory
  -f, --file <FILE_NAME>       Name of a new file to create with docstring as header. Prepends to top of
                               file if the target file already exists [default: *.*]
  -l, --license <LICENSE>      Relative path to the LICENSE file to use as header docstring. If not specified,
                               expects a LICENSE file to exist in the current working directory [default: LICENSE]
  -u, --update                 Specify whether or not to try and update all available docstrings in a directory
                               recursively, requires <DIRECTORY> to have been set
  -h, --help                   Print help
  -V, --version                Print version
```

## 📋 License
All code is to be held under a general MIT license, please see [LICENSE](https://github.com/wilhelmagren/docstring-rs/blob/main/LICENSE) for specific information.
