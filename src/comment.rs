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
* Last updated: 2023-10-04
*/

///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommentStyle<'a> {
    multi_line_start: &'a str,
    normal_comment: &'a str,
    multi_line_end: &'a str,
}

///
impl<'a> CommentStyle<'a> {
    pub fn new(start: &'a str, normal: &'a str, end: &'a str) -> Self {
        CommentStyle {
            multi_line_start: start,
            normal_comment: normal,
            multi_line_end: end,
        }
    }

    pub fn start(&self) -> &'a str {
        self.multi_line_start
    }

    pub fn normal(&self) -> &'a str {
        self.normal_comment
    }

    pub fn end(&self) -> &'a str {
        self.multi_line_end
    }
}

#[cfg(test)]
mod tests_comment {
    use super::*;

    #[test]
    fn pub_funcs() {
        let start: &str = "##/";
        let normal: &str = "# ";
        let end: &str = "/##";
        let cs = CommentStyle::new(start, normal, end);
        assert_eq!(cs.start(), start);
        assert_eq!(cs.normal(), normal);
        assert_eq!(cs.end(), end);
    }
}
