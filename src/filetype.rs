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
use std::io;

use once_cell::sync::{Lazy, OnceCell};

use crate::*;

///
#[derive(Debug, Clone)]
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

    /// RIP Terry A. Davis
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
impl FileType {
    pub fn try_from_filename(fname: &str) -> Result<Self, io::Error> {
        let ft: &str = match fname.split('.').last() {
            Some(t) => t,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "could not find a file ending",
                ));
            }
        };

        match ft {
            "c" => Ok( FileType::C ),

            "cc" => Ok( FileType::CPP ),
            "cpp" => Ok( FileType::CPP ),
            "cxx" => Ok( FileType::CPP ),

            "cs" => Ok( FileType::CSharp ),

            "pyx" => Ok( FileType::Cython ),

            "ex" => Ok( FileType::Elixir ),
            "exs" => Ok( FileType::Elixir ),

            "erl" => Ok( FileType::Erlang ),
            "hrl" => Ok( FileType::Erlang ),

            "fs" => Ok( FileType::FSharp ),
            "fsi" => Ok( FileType::FSharp ),
            "fsx" => Ok( FileType::FSharp ),
            "fsscript" => Ok( FileType::FSharp ),

            "go" => Ok( FileType::Go ),

            "hs" => Ok( FileType::Haskell ),
            "lhs" => Ok( FileType::Haskell ),

            "HC" => Ok( FileType::HolyC ),

            "java" => Ok( FileType::Java ),

            "js" => Ok( FileType::JavaScript ),

            "jl" => Ok( FileType::Julia ),

            "kt" => Ok( FileType::Kotlin ),
            "kts" => Ok( FileType::Kotlin ),

            "lisp" => Ok( FileType::Lisp ),
            "lsp" => Ok( FileType::Lisp ),
            "l" => Ok( FileType::Lisp ),
            "cl" => Ok( FileType::Lisp ),
            "fasl" => Ok( FileType::Lisp ),

            "lua" => Ok( FileType::Lua ),

            "plx" => Ok( FileType::Perl ),
            "pm" => Ok( FileType::Perl ),
            "xs" => Ok( FileType::Perl ),
            "t" => Ok( FileType::Perl ),
            "pod" => Ok( FileType::Perl ),
            "cgi" => Ok( FileType::Perl ),

            "php" => Ok( FileType::PHP ),
            "phar" => Ok( FileType::PHP ),
            "phtml" => Ok( FileType::PHP ),
            "pht" => Ok( FileType::PHP ),
            "phps" => Ok( FileType::PHP ),

            "ps1" => Ok( FileType::PowerShell ),
            "psc1" => Ok( FileType::PowerShell ),
            "pssc" => Ok( FileType::PowerShell ),

            "pl" => Ok( FileType::Prolog ),
            "pro" => Ok( FileType::Prolog ),
            "P" => Ok( FileType::Prolog ),

            "py" => Ok( FileType::Python ),
            "pyi" => Ok( FileType::Python ),
            "pyc" => Ok( FileType::Python ),
            "pyd" => Ok( FileType::Python ),
            "pyw" => Ok( FileType::Python ),
            "pyz" => Ok( FileType::Python ),

            "qs" => Ok( FileType::QSharp ),

            "r" => Ok( FileType::R ),
            "rdata" => Ok( FileType::R ),
            "rds" => Ok( FileType::R ),

            "rb" => Ok( FileType::Ruby ),

            "rs" => Ok( FileType::Rust ),

            "scala" => Ok( FileType::Scala ),
            "sc" => Ok( FileType::Scala ),

            "swift" => Ok( FileType::Swift ),
            "SWIFT" => Ok( FileType::Swift ),

            "ts" => Ok( FileType::TypeScript ),
            "tsx" => Ok( FileType::TypeScript ),
            "mts" => Ok( FileType::TypeScript ),
            "cts" => Ok( FileType::TypeScript ),

            "vim" => Ok( FileType::Vim ),

            "zig" => Ok( FileType::Zig ),
            "zir" => Ok( FileType::Zig ),

            _ => Err(io::Error::new(
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
    use super::*;

    #[test]
    fn fail_try_from_filename() {
        let expected = Err(io::ErrorKind::NotFound);
        let result = FileType::try_from_filename("_lol.kebab");
        assert_eq!(expected, result);
    }

    #[test]
    fn success_try_from_filename() {
        let filenames: Vec<&str> = vec!["c", "", ""];
    }
}
