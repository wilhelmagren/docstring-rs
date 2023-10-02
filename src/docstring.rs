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

use log::info;

use regex::Regex;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

///
pub struct Docstring {
    target_path: PathBuf,
    license_path: PathBuf,
    contents: Option<String>,
    file_created: Option<String>,
}

///
impl Docstring {

    ///
    pub fn new(target: PathBuf, license: PathBuf) -> Self {
        Self {
            target_path: target,
            license_path: license,
            contents: None,
            file_created: None,
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

        let re = Regex::new(r"/^(\d{1,4})-(\d{1,2})-(\d{1,2})$/").expect("could not compile regex");
        for line in fc.split('\n') {
            if line.contains("File created: ") {
                let date: String = match re.find(line) {
                    Some(d) => d.as_str().to_owned(),
                    None => return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "could not parse date"
                    )),
                };
                self.file_created = Some(date);
            };
        };

        Ok(())
    }

    ///
    pub fn try_read_license(&mut self) -> Result<(), io::Error> {
        match fs::read_to_string(&self.license_path) {
            Ok(c) => {
                info!("Read contents of {} successfully", &self.license_path.display());
                self.contents = Some(c);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests_docstring {
    use super::*;

    #[test]
    fn new_and_read() {

    }
}