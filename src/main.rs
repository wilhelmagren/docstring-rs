use std::fs;
use std::path::{Path, PathBuf};
use std::{io, io::Write};

use log::{error, info, warn};

use text_io::read;

use clap::Parser;

use chrono;

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
fn read_docstring_from_file(path: &Path) -> Result<String, io::Error> {
    match fs::read_to_string(path) {
        Ok(docstring) => {
            info!("Read contents of `{}` successfully", &path.display());
            Ok(docstring)
        }
        Err(e) => Err(e),
    }
}

enum FileType {
    C,
    CPP,
    RS,
    PY,
    JS,
    TS,
    JAVA,
}

/*
struct Docstring {
    path: Path,
    contents: Option<String>,
    filetype: FileType,
}

impl Docstring {
    fn new(path: &Path, contents: String, filetype: FileType) -> Self {
        Docstring {
            path: path,
            contents: Some(contents),
            filetype: filetype,
        }
    }
}
*/

///
fn find_filetype(file: &String) -> Result<FileType, io::Error> {
    let file_type = match file.split(".").last() {
        Some(ft) => ft,
        None => {
            return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "could not find a file ending"
                    ));
        }
    };

    match file_type {
        "c" => Ok(FileType::C),
        "cc" => Ok(FileType::CPP),
        "cpp" => Ok(FileType::CPP),
        "cxx" => Ok(FileType::CPP),
        "rs" => Ok(FileType::RS),
        "PY" => Ok(FileType::PY),
        "JS" => Ok(FileType::JS),
        "TS" => Ok(FileType::TS),
        "JAVA" => Ok(FileType::JAVA),
        _ => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no matching filetype"
                )),
    }
}

/*
fn get_comment_by_filetype(ft: &FileType) -> &'static str {
    match ft {
        FileType::C => "//",
        FileType::CPP => "//",
        FileType::RS => "//",
        FileType::PY => "#",
        FileType::JS => "//",
        FileType::TS => "//",
        FileType::JAVA => "//",
    }
}
*/

///
fn get_multiline_comment_by_filetype(ft: &FileType) -> (&'static str, &'static str, &'static str) {
    match ft {
        FileType::C => ("/*", "* ", "*/"),
        FileType::CPP => ("/*", "* ", "*/"),
        FileType::RS => ("/*", "* ", "*/"),
        FileType::PY => (r#"""#, "", r#"""#),
        FileType::JS => ("/*", "* ", "*/"),
        FileType::TS => ("/*", "* ", "*/"),
        FileType::JAVA => ("/*", "* ", "*/"),
    }
}

///
fn format_docstring(contents: String, ft: FileType, created_date: &str) -> Vec<u8> {
    let (ml_start, ml_comment, ml_end) = get_multiline_comment_by_filetype(&ft);
    let mut docstring = String::new();
    docstring.push_str(ml_start);
    docstring.push('\n');

    for line in contents.split("\n") {
        docstring.push_str(ml_comment);
        docstring.push_str(line);
        docstring.push('\n');
    }

    docstring.push('\n');
    docstring.push_str(ml_comment);
    let local: String = chrono::Local::now().format("%Y-%m-%d").to_string();

    docstring.push_str("File created: ");
    docstring.push_str(created_date);
    docstring.push('\n');
    docstring.push_str(ml_comment);
    
    let mut last_updated = String::new();
    last_updated.push_str("Last updated: ");
    last_updated.push_str(local.as_str());

    docstring.push_str(last_updated.as_str());
    docstring.push('\n');
    docstring.push_str(ml_end);
    docstring.push('\n');

    docstring.as_bytes().to_vec()
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

    let contents: String = match read_docstring_from_file(license) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "Could not read contents from license file `{}` due to `{}`",
                &license.display(),
                e
            );
            return Err(e);
        }
    };

    let filetype: FileType = match find_filetype(&args.file_name) {
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

    let binding = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut created: &str = binding.as_str();
    if target_path.exists() {
        warn!("Target file already exists, will prepend to top of file...");
        let metadata = fs::metadata(target_path).unwrap();
        let t_created: chrono::DateTime<chrono::Local> = metadata.created().unwrap().clone().into();
        let b_created = t_created.format("%Y-%m-%d").to_string();
        created = b_created.as_str();
        let docstring: Vec<u8> = format_docstring(contents, filetype, created);
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
        let docstring: Vec<u8> = format_docstring(contents, filetype, created);
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
