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
* File created: 2023-10-01
* Last updated: 2023-10-01
*/

use std::collections::HashMap;
use std::fmt;
use std::io;

use once_cell::sync::Lazy;

use crate::*;

///
static FILE2TYPE: Lazy<HashMap<&'static str, FileType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("c", FileType::C);

    m.insert("cc", FileType::CPP);
    m.insert("cpp", FileType::CPP);
    m.insert("cxx", FileType::CPP);

    m.insert("cs", FileType::CSharp);

    m.insert("pyx", FileType::Cython);

    m.insert("ex", FileType::Elixir);
    m.insert("exs", FileType::Elixir);

    m.insert("erl", FileType::Erlang);
    m.insert("hrl", FileType::Erlang);

    m.insert("fs", FileType::FSharp);
    m.insert("fsi", FileType::FSharp);
    m.insert("fsx", FileType::FSharp);
    m.insert("fsscript", FileType::FSharp);

    m.insert("go", FileType::Go);

    m.insert("hs", FileType::Haskell);
    m.insert("lhs", FileType::Haskell);

    // RIP Terry A. Davis
    m.insert("HC", FileType::HolyC);

    m.insert("java", FileType::Java);

    m.insert("js", FileType::JavaScript);

    m.insert("jl", FileType::Julia);

    m.insert("kt", FileType::Kotlin);
    m.insert("kts", FileType::Kotlin);

    m.insert("lisp", FileType::Lisp);
    m.insert("lsp", FileType::Lisp);
    m.insert("l", FileType::Lisp);
    m.insert("cl", FileType::Lisp);
    m.insert("fasl", FileType::Lisp);

    m.insert("lua", FileType::Lua);

    m.insert("plx", FileType::Perl);
    m.insert("pm", FileType::Perl);
    m.insert("xs", FileType::Perl);
    m.insert("t", FileType::Perl);
    m.insert("pod", FileType::Perl);
    m.insert("cgi", FileType::Perl);

    m.insert("php", FileType::PHP);
    m.insert("phar", FileType::PHP);
    m.insert("phtml", FileType::PHP);
    m.insert("pht", FileType::PHP);
    m.insert("phps", FileType::PHP);

    m.insert("ps1", FileType::PowerShell);
    m.insert("psc1", FileType::PowerShell);
    m.insert("pssc", FileType::PowerShell);

    m.insert("pl", FileType::Prolog);
    m.insert("pro", FileType::Prolog);
    m.insert("P", FileType::Prolog);

    m.insert("py", FileType::Python);
    m.insert("pyi", FileType::Python);
    m.insert("pyc", FileType::Python);
    m.insert("pyd", FileType::Python);
    m.insert("pyw", FileType::Python);
    m.insert("pyz", FileType::Python);

    m.insert("qs", FileType::QSharp);

    m.insert("r", FileType::R);
    m.insert("rdata", FileType::R);
    m.insert("rds", FileType::R);

    m.insert("rb", FileType::Ruby);

    m.insert("rs", FileType::Rust);

    m.insert("scala", FileType::Scala);
    m.insert("sc", FileType::Scala);

    m.insert("swift", FileType::Swift);
    m.insert("SWIFT", FileType::Swift);

    m.insert("ts", FileType::TypeScript);
    m.insert("tsx", FileType::TypeScript);
    m.insert("mts", FileType::TypeScript);
    m.insert("cts", FileType::TypeScript);

    m.insert("vim", FileType::Vim);

    m.insert("zig", FileType::Zig);
    m.insert("zir", FileType::Zig);

    m
});

///
static TYPE2STYLE: Lazy<HashMap<FileType, CommentStyle>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(FileType::C, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::CPP, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::CSharp, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::Cython, CommentStyle::new("\"\"\"", "", "\"\"\""));
    m.insert(FileType::Elixir, CommentStyle::new("# ", "# ", "# "));
    m.insert(FileType::Erlang, CommentStyle::new("%", "% ", "%"));
    m.insert(FileType::FSharp, CommentStyle::new("(*", "* ", "*)"));
    m.insert(FileType::Go, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::Haskell, CommentStyle::new("{-", "- ", "-}"));
    m.insert(FileType::HolyC, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::Java, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::JavaScript, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::Julia, CommentStyle::new("#=", "= ", "=#"));
    m.insert(FileType::Kotlin, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::Lisp, CommentStyle::new(";;;;", ";;;; ", ";;;;"));
    m.insert(FileType::Lua, CommentStyle::new("--[[", "-- ", "--]]"));
    m.insert(FileType::Perl, CommentStyle::new("=", "", "=cut"));
    m.insert(FileType::PHP, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::Prolog, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::Python, CommentStyle::new("\"\"\"", "", "\"\"\""));
    m.insert(FileType::QSharp, CommentStyle::new("///", "///", "///"));
    m.insert(FileType::R, CommentStyle::new("#", "# ", "#"));
    m.insert(FileType::Ruby, CommentStyle::new("=begin", "", "=end"));
    m.insert(FileType::Rust, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::Scala, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::Swift, CommentStyle::new("/*", "* ", "*/"));
    m.insert(FileType::TypeScript, CommentStyle::new("/*", "* ", "*/"));
    m.insert(
        FileType::Vim,
        CommentStyle::new("\'\"\'", "\'\"\'", "\'\"\'"),
    );
    m.insert(FileType::Zig, CommentStyle::new("/*", "* ", "*/"));
    m
});

///
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FileType {
    C,
    CPP,
    CSharp,
    Cython,
    Elixir,
    Erlang,
    FSharp,
    Go,
    Haskell,
    HolyC,
    Java,
    JavaScript,
    Julia,
    Kotlin,
    Lisp,
    Lua,
    Perl,
    PHP,
    PowerShell,
    Prolog,
    Python,
    QSharp,
    R,
    Ruby,
    Rust,
    Scala,
    Swift,
    TypeScript,
    Vim,
    Zig,
}

///
/// fn main() {
///     let ft = FileType::HolyC;
///     println!("{}", ft);
/// }
///
/// >>> "HolyC"
///
impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use filetype::FileType::*;
        match self {
            C => write!(f, "C"),
            CPP => write!(f, "C++"),
            CSharp => write!(f, "C#"),
            Cython => write!(f, "Cython"),
            Elixir => write!(f, "Elixir"),
            Erlang => write!(f, "Erlang"),
            FSharp => write!(f, "FSharp"),
            Go => write!(f, "Go"),
            Haskell => write!(f, "Haskell"),
            HolyC => write!(f, "HolyC"),
            Java => write!(f, "Java"),
            JavaScript => write!(f, "JavaScript"),
            Julia => write!(f, "Julia"),
            Kotlin => write!(f, "Kotlin"),
            Lisp => write!(f, "Lisp"),
            Lua => write!(f, "Lua"),
            Perl => write!(f, "Perl"),
            PHP => write!(f, "PHP"),
            PowerShell => write!(f, "PowerShell"),
            Prolog => write!(f, "Prolog"),
            Python => write!(f, "Python"),
            QSharp => write!(f, "QSharp"),
            R => write!(f, "R"),
            Ruby => write!(f, "Ruby"),
            Rust => write!(f, "Rust"),
            Scala => write!(f, "Scala"),
            Swift => write!(f, "Swift"),
            TypeScript => write!(f, "TypeScript"),
            Vim => write!(f, "Vim"),
            Zig => write!(f, "Zig"),
        }
    }
}

///
impl FileType {
    pub fn try_from_filename(fname: &str) -> Result<FileType, io::Error> {
        let fe: &str = match fname.split('.').last() {
            Some(e) => e,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "could not find a file ending",
                ));
            }
        };

        match FILE2TYPE.get(fe) {
            Some(ft) => Ok(*ft),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no matching filetype",
            )),
        }
    }
}

///
impl FileType {
    pub fn get_comment_style(&self) -> CommentStyle {
        use filetype::FileType::*;
        let (start, normal, end) = match self {
            C => ("/*", "* ", "*/"),
            CPP => ("/*", "* ", "*/"),
            CSharp => ("/*", "* ", "*/"),
            Cython => ("\"\"\"", "", "\"\"\""),
            Elixir => ("# ", "# ", "# "),
            Erlang => ("%", "% ", "%"),
            FSharp => ("(*", "* ", "*)"),
            Go => ("/*", "* ", "*/"),
            Haskell => ("{-", "- ", "-}"),
            HolyC => ("/*", "* ", "*/"),
            Java => ("/*", "* ", "*/"),
            JavaScript => ("/*", "* ", "*/"),
            Julia => ("#=", "= ", "=#"),
            Kotlin => ("/*", "* ", "*/"),
            Lisp => (";;;;", ";;;; ", ";;;;"),
            Lua => ("--[[", "-- ", "--]]"),
            Perl => ("=", "", "=cut"),
            PHP => ("/*", "* ", "*/"),
            PowerShell => ("<#", "# ", "#>"),
            Prolog => ("/*", "* ", "*/"),
            Python => ("\"\"\"", "", "\"\"\""),
            QSharp => ("///", "///", "///"),
            R => ("#", "# ", "#"),
            Ruby => ("=begin", "", "=end"),
            Rust => ("/*", "* ", "*/"),
            Scala => ("/*", "* ", "*/"),
            Swift => ("/*", "* ", "*/"),
            TypeScript => ("/*", "* ", "*/"),
            Vim => ("\'\"\'", "\'\"\'", "\'\"\'"),
            Zig => ("/*", "* ", "*/"),
        };

        CommentStyle::new(start, normal, end)
    }
}

#[cfg(test)]
mod tests {
    use super::{io, FileType, FILE2TYPE, TYPE2STYLE};

    #[test]
    fn try_from_filename_error() {
        let expected = Err(io::ErrorKind::NotFound);
        let result = FileType::try_from_filename("_lol__haha.kebab").map_err(|e| e.kind());
        assert_eq!(expected, result);
    }

    #[test]
    fn try_from_filename_ok_all() {
        for file_ending in FILE2TYPE.keys() {
            let expected = FILE2TYPE.get(file_ending).unwrap();
            let result = FileType::try_from_filename(file_ending).unwrap();
            assert_eq!(expected, &result);
        }
    }

    #[test]
    fn get_comment_style_ok_all() {
        for filetype in TYPE2STYLE.keys() {
            let expected = TYPE2STYLE.get(filetype).unwrap();
            let result = filetype.get_comment_style();
            assert_eq!(expected, &result);
        }
    }
}
