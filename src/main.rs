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
* File created: 2023-09-30
* Last updated: 2023-10-02
*/

use std::fs;
use std::path::{Path, PathBuf};
use std::{io, io::Write};

use log::{error, info, warn};

use clap::Parser;

mod args;
mod comment;
mod docstring;
mod filetype;
mod tmp;

use args::Args;
use comment::CommentStyle;
use docstring::Docstring;
use filetype::FileType;
use tmp::tmp_file_from_path;

///
fn prepend_to_file(data: &[u8], path: &Path) -> Result<(), io::Error> {
    let tmp_path: PathBuf = tmp_file_from_path(path);
    match fs::write(&tmp_path, data) {
        Ok(_) => info!(
            "Wrote docstring contents to tmp file: `{}`",
            &tmp_path.display()
        ),
        Err(e) => return Err(e),
    };

    let contents: Vec<u8> = match fs::read(path) {
        Ok(c) => {
            info!("Read contents of `{}` successfully", &path.display());
            c
        }
        Err(e) => return Err(e),
    };

    let mut tmp = match fs::OpenOptions::new().append(true).open(&tmp_path) {
        Ok(f) => {
            info!("Opened `{}` with append mode", &tmp_path.display());
            f
        }
        Err(e) => return Err(e),
    };

    match tmp.write_all(&contents[..]) {
        Ok(_) => info!(
            "Wrote contents from `{}` to `{}`",
            &path.display(),
            &tmp_path.display()
        ),
        Err(e) => return Err(e),
    };

    match fs::copy(&tmp_path, path) {
        Ok(_) => info!(
            "Copied contents from `{}` to `{}`",
            &tmp_path.display(),
            &path.display()
        ),
        Err(e) => return Err(e),
    };

    match fs::remove_file(&tmp_path) {
        Ok(_) => info!("Removed the temporary file `{}`", &tmp_path.display()),
        Err(e) => return Err(e),
    };

    Ok(())
}

///
fn add_to_new_file(data: &[u8], path: &Path) -> Result<(), io::Error> {
    match fs::write(path, data) {
        Ok(_) => {
            info!("Wrote docstring contents to file: `{}`", &path.display());
            Ok(())
        }
        Err(e) => Err(e),
    }
}

///
fn create_directory(dir: &Path) -> Result<(), io::Error> {
    let mut pathbuf = PathBuf::new();
    for component in dir.components() {
        pathbuf.push(component);
        if pathbuf.as_path().exists() {
            info!("Path `{}` already exists", &pathbuf.display());
            continue;
        }
        match fs::create_dir(&pathbuf) {
            Ok(_) => info!("Creating path `{}`", &pathbuf.display()),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    env_logger::init();

    let args = match Args::try_parse() {
        Ok(a) => a,
        Err(e) => {
            warn!(
                "Could not parse CLI args from std::env due to `{:?}`.",
                e.kind()
            );
            Args::try_from_user()
        }
    };

    let directory = Path::new(&args.directory);
    let file_name = Path::new(&args.file_name);
    let license = Path::new(&args.license);

    let filetype: FileType = match FileType::try_from_filename(&args.file_name) {
        Ok(f) => f,
        Err(e) => {
            error!(
                "Could not find a filetype in the filename: `{}` due to `{}`",
                &file_name.display(),
                e
            );
            return Err(e);
        }
    };

    let path_builder: PathBuf = directory.join(file_name);
    let target_path = Path::new(&path_builder);

    if !directory.exists() {
        info!(
            "Directory `{}` does not already exist, creating it...",
            &directory.display()
        );

        match create_directory(directory) {
            Ok(()) => info!("Successfully created directory!"),
            Err(e) => {
                error!(
                    "Could not create directory `{}` due to `{:?}`",
                    &directory.display(),
                    e
                );
                return Err(e);
            }
        };
    }

    let mut docstring = Docstring::new(target_path.to_path_buf(), license.to_path_buf(), filetype);
    match docstring.try_read_license() {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    if docstring.target_exists() {
        warn!("Target file already exists, will prepend to top of file...");
        match docstring.try_find_created_date() {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match docstring.format_contents() {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let contents = docstring.get_formatted_contents().unwrap();
        match prepend_to_file(contents.as_bytes(), target_path) {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Could not prepend docstring to the file `{}` due to `{:?}`",
                    &target_path.display(),
                    e
                );
                return Err(e);
            }
        };
    } else {
        match docstring.format_contents() {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let contents = docstring.get_formatted_contents().unwrap();
        match add_to_new_file(contents.as_bytes(), target_path) {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Could not add docstring to the file `{}` due to `{:?}`",
                    &target_path.display(),
                    e
                );
                return Err(e);
            }
        }
    }

    info!(
        "⚡Successfully created docstring from `{}` at top of file `{}`!⚡",
        &license.display(),
        &target_path.display(),
    );

    Ok(())
}
