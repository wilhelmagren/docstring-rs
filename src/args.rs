/*
* MIT License
*
* Copyright (c) 2023 Wilhelm Ågren
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*
* File created: 2023-10-02
* Last updated: 2023-10-02
*/

use std::path::PathBuf;

use clap::Parser;
use text_io::read;

///
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Name of the directory in which to create a file,
    /// if it does not already exist, creates the directory.
    #[arg(short = 'd', long = "directory", required = true)]
    pub directory: String,

    /// Name of the new file to create with docstring as
    /// header. If it already exists, asks user whether
    /// to prepend the docstring to the file.
    #[arg(short = 'f', long = "file", required = false, default_value = "*.*")]
    pub file_name: String,

    /// Relative path to the LICENSE file to use in header docstring.
    /// If not specified, expects a LICENSE file to exist in the
    /// current working directory.
    #[arg(
        short = 'l',
        long = "license",
        required = false,
        default_value = "LICENSE"
    )]
    pub license: String,

    /// Specify whether or not to try and upate all avilable docstrings
    /// in a directory recursively downwards.
    #[arg(short = 'u', long = "update", required = false, requires = "directory", default_value = "false")]
    pub update: bool,
}

///
impl Args {
    pub fn get_filetype_from_user(&mut self) {
        print!("Please input the file type to update: ");
        let f: String = read!();
        self.file_name = f;
    }
    ///
    pub fn try_from_user() -> Self {
        print!("Please input the DIRECTORY PATH to create create/update file at: ");
        let d: String = read!();

        print!("Please input FILE NAME to create/update docstring in: ");
        let f: String = read!();

        print!("Please input reative path to the wanted LICENSE: ");
        let l: String = read!();

        Self {
            directory: d,
            file_name: f,
            license: l,
            update: false,
        }
    }

    pub fn try_dir_and_filetype_from_user() -> Self {
        print!("Please input the DIRECTORY to start recursive update from: ");
        let d: String = read!();
        print!("Please input the FILETYPE (.rs, .py, ...) to update: ");
        let f: String = read!();
        print!("Please input reative path to the wanted LICENSE: ");
        let l: String = read!();

        Self {
            directory: d,
            file_name: f,
            license: l,
            update: true,
        }
    }

    ///
    pub fn paths(&self) -> (PathBuf, PathBuf, PathBuf) {
        (
            PathBuf::from(&self.directory),
            PathBuf::from(&self.file_name),
            PathBuf::from(&self.license),
        )
    }

    ///
    pub fn update(&self) -> bool {
        self.update
    }
}

#[cfg(test)]
mod tests_args {
    use super::*;

    #[test]
    #[should_panic]
    fn try_parse_panic() {
        Args::try_parse().unwrap();
    }

    #[test]
    fn args_into_paths() {
        let args = Args {
            directory: "src".into(),
            file_name: "nn.rs".into(),
            license: "LICENSE".into(),
            update: false,
        };
        let (d, f, l) = args.paths();
        assert_eq!(PathBuf::from("src"), d);
        assert_eq!(PathBuf::from("nn.rs"), f);
        assert_eq!(PathBuf::from("LICENSE"), l);
    }
}
