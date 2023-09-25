use std::fs;
use std::path::{Path, PathBuf};
use std::{io, io::Write};

use log::{error, info, warn};

use text_io::read;

use clap::Parser;

///
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Name of the directory in which to create a file,
    /// if it does not already exist, creates the directory.
    #[arg(short = 'd', long = "directory", required = true)]
    directory: String,

    /// Name of the new file to create with docstring as
    /// header. If it already exists, asks user whether
    /// to prepend the docstring to the file.
    #[arg(short = 'f', long = "file", required = true)]
    file_name: String,

    /// Relative path to the LICENSE file to use in header
    /// docstring. If not specified, expects a LICENSE file
    /// to exist in the current working directory.
    #[arg(
        short = 'l',
        long = "license",
        required = false,
        default_value = "LICENSE"
    )]
    license: String,
}

/// If the user does not provide the required CLI arguments
/// they will be prompted for them as the program is running.
/// Returns the provided arguments as the `Args` struct.
fn get_args_from_cli() -> Args {
    info!("Please input the DIRECTORY to create docstring in: ");
    let d: String = read!();

    info!("Please input the NAME OF FILE to create docstring in: ");
    let f: String = read!();

    info!("Please input the PATH TO LICENSE FILE to include in docstring: ");
    let l: String = read!();

    Args {
        directory: d,
        file_name: f,
        license: l,
    }
}

///
fn random_tmp_file_name<'a>() -> &'a str {
    "hahahatmp.tmp"
}

///
fn tmp_file_from_path(path: &Path) -> PathBuf {
    let tmp_file_name: &str = random_tmp_file_name();
    match path.parent() {
        Some(p) => p.join(tmp_file_name),
        None => PathBuf::from(tmp_file_name),
    }
}

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

///
fn read_docstring_from_file(path: &Path) -> Result<Vec<u8>, io::Error> {
    match fs::read(path) {
        Ok(docstring) => {
            info!("Read contents of `{}` successfully", &path.display());
            Ok(docstring)
        }
        Err(e) => Err(e),
    }
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
            get_args_from_cli()
        }
    };

    let directory = Path::new(&args.directory);
    let file_name = Path::new(&args.file_name);
    let license = Path::new(&args.license);

    let docstring: Vec<u8> = match read_docstring_from_file(license) {
        Ok(d) => d,
        Err(e) => {
            error!(
                "Could not read contents from license file `{}` due to `{}`",
                &license.display(),
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

    if target_path.exists() {
        warn!("Target file already exists, will prepend to top of file...");
        match prepend_to_file(&docstring[..], target_path) {
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
        match add_to_new_file(&docstring[..], target_path) {
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
