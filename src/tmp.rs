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

use std::path::{Path, PathBuf};

use rand::distributions::{Alphanumeric, DistString};

static FILENAMELEN: usize = 16;

/// Generate a pseudo-random string from alphanumerica characters of length
/// FILENAMELEN.
fn random_file_name() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), FILENAMELEN)
}

/// Create a relative path to a temporary file.
pub fn tmp_file_from_path(path: &Path) -> PathBuf {
    let tmp_file_name: String = random_file_name();
    match path.parent() {
        Some(p) => p.join(tmp_file_name),
        None => PathBuf::from(tmp_file_name),
    }
}

#[cfg(test)]
mod tests_tmp {
    use super::*;

    #[test]
    fn tmp_file_from() {
        let _tp: PathBuf = tmp_file_from_path(Path::new("src"));
    }
}
