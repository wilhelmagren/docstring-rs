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
* Last updated: 2023-10-04
*/

use crate::FileType;

use log::info;

use regex::Regex;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

///
pub struct Docstring {
    target_path: PathBuf,
    license_path: PathBuf,
    file_type: FileType,
    contents: Option<String>,
    formatted_contents: Option<String>,
    file_created: Option<String>,
}

///
impl Docstring {
    ///
    pub fn new(target_path: PathBuf, license_path: PathBuf, file_type: FileType) -> Self {
        Self {
            target_path,
            license_path,
            file_type,
            contents: None,
            formatted_contents: None,
            file_created: None,
        }
    }

    ///
    fn try_get_contents(&self) -> Result<String, io::Error> {
        match &self.contents {
            Some(c) => Ok(c.clone()),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "you have not read the contents of the LICENSE file yet",
            )),
        }
    }

    ///
    pub fn target_exists(&self) -> bool {
        Path::new(&self.target_path).exists()
    }

    ///
    pub fn try_find_created_date(&mut self) -> Result<(), io::Error> {
        let fc = match fs::read_to_string(&self.target_path) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        let re = Regex::new(r"\d{4}\-(0?[1-9]|1[012])\-(0?[0-9]|[12][0-9]|3[01])*")
            .expect("could not compile regex");
        for line in fc.split('\n') {
            if line.contains("File created: ") {
                let date: String = match re.find(line) {
                    Some(d) => d.as_str().to_owned(),
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "could not parse date",
                        ))
                    }
                };
                self.file_created = Some(date);
                return Ok(());
            };
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "could not find a created date",
        ))
    }

    ///
    pub fn try_read_license(&mut self) -> Result<(), io::Error> {
        match fs::read_to_string(&self.license_path) {
            Ok(c) => {
                info!(
                    "Read contents of {} successfully",
                    &self.license_path.display()
                );
                self.contents = Some(c);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_formatted_contents(self) -> Option<String> {
        self.formatted_contents
    }

    ///
    pub fn format_contents(&mut self) -> Result<(), io::Error> {
        let contents = match self.try_get_contents() {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        let style = self.file_type.get_comment_style();
        let start = style.start();
        let comment = style.normal();
        let end = style.end();

        let mut formatted = String::new();

        // start the multiline comment
        formatted.push_str(start);
        formatted.push('\n');

        // add conents of the LICENSE file
        for line in contents.split('\n') {
            formatted.push_str(comment);
            formatted.push_str(line);
            formatted.push('\n');
        }

        // add created dates
        formatted.push_str(comment);
        formatted.push_str("File created: ");
        let local: String = chrono::Local::now().format("%Y-%m-%d").to_string().clone();
        let created = match &self.file_created {
            Some(c) => c.as_str(),
            None => local.as_str(),
        };
        formatted.push_str(created);
        formatted.push('\n');

        // add last updated date
        formatted.push_str(comment);
        formatted.push_str("Last updated: ");
        formatted.push_str(local.as_str());
        formatted.push('\n');

        // end the multiline comment
        formatted.push_str(end);
        formatted.push('\n');

        self.formatted_contents = Some(formatted);

        Ok(())
    }
}

#[cfg(test)]
mod tests_docstring {
    use super::*;

    #[test]
    fn read_license_ok() {
        let target = PathBuf::from("src/docstring.rs");
        let license = PathBuf::from("LICENSE");
        let mut ds = Docstring::new(target, license, FileType::Rust);
        ds.try_read_license().unwrap();
    }

    #[test]
    fn find_created_date_ok() {
        let target = PathBuf::from("src/docstring.rs");
        let license = PathBuf::from("LICENSE");
        let mut ds = Docstring::new(target, license, FileType::Rust);
        ds.try_find_created_date().unwrap();
    }

    #[test]
    fn format_contents_ok() {
        let target = PathBuf::from("src/docstring.rs");
        let license = PathBuf::from("LICENSE");
        let mut ds = Docstring::new(target, license, FileType::Rust);
        ds.try_read_license().unwrap();
        ds.try_find_created_date().unwrap();
        ds.format_contents().unwrap();
    }
}
