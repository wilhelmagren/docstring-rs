use std::fs;
use std::path::{Path, PathBuf};
use std::{io, io::Write};

use text_io::read;

use clap::Parser;

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
    println!("Directory to create docstring in: ");
    let d: String = read!();

    println!("Name of file to create docstring in: ");
    let f: String = read!();

    println!("Path to the LICENSE file to include in docstring: ");
    let l: String = read!();

    Args {
        directory: d,
        file_name: f,
        license: l,
    }
}

///
fn create_tmp_file_from_path(path: &Path) -> PathBuf {
    match path.parent() {
        Some(p) => p.join("tmpdocstring.tmp"),
        None => PathBuf::from("tmpdocstring.tmp"),
    }
}

///
fn prepend_to_file(data: &[u8], path: &Path) -> io::Result<()> {
    let tmp_path = create_tmp_file_from_path(path);
    fs::write(&tmp_path, data)?;

    let contents = fs::read(path)?;
    let mut tmp = fs::OpenOptions::new().append(true).open(&tmp_path)?;

    tmp.write_all(&contents[..])?;
    fs::copy(&tmp_path, &path)?;

    fs::remove_file(&tmp_path)?;

    Ok(())
}

///
// fn create_new_file(data: &[u8], path: &Path) -> io::Result<()> {
// }

fn main() {
    let args = match Args::try_parse() {
        Ok(a) => a,
        Err(e) => {
            println!(
                "Could not parse CLI args from std::env due to `{:?}`.",
                e.kind()
            );
            get_args_from_cli()
        }
    };

    let directory = Path::new(&args.directory);
    let file_name = Path::new(&args.file_name);
    let license = Path::new(&args.license);

    let path_builder: PathBuf = directory.join(file_name);
    let target_path = Path::new(&path_builder);
    println!("{:?}", target_path.exists());

    if !directory.exists() {
        println!(
            "Directory {} does not already exist, creating it...",
            &directory.display()
        );
        match fs::create_dir(directory) {
            Ok(()) => println!("Successfully created directory."),
            Err(e) => panic!(
                "Could not create directory `{}` due to `{:?}`",
                &directory.display(),
                e
            ),
        };
    }

    if target_path.exists() {
        println!("Target file already exists, will prepend to top of file...");
    }

    prepend_to_file("HAHA COPE XD\n".as_bytes(), &target_path);
}
