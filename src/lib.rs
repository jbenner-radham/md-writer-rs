//! MD Writer
//! =========
//! A collection of utilities to help make writing Markdown easier.

/// The line feed control character.
pub const LF: char = '\n';

/// Create a Markdown code fence.
/// 
/// Examples
/// ========
/// With an info string:
/// ```
/// let info_string = Some("rust");
/// let code_fence = md_writer::code_fence(info_string);
/// 
/// assert_eq!(code_fence, "```rust");
/// ```
/// 
/// Without an info string:
/// ```
/// let code_fence = md_writer::code_fence(None);
/// 
/// assert_eq!(code_fence, "```");
/// ```
/// 
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#code-fence>
/// - <https://spec.commonmark.org/0.30/#info-string>
pub fn code_fence(info_string: Option<&str>) -> String {
    let mut code_fence = "`".repeat(3);
    let info_string = info_string.unwrap_or("");

    code_fence.push_str(info_string);

    code_fence
}

/// Create a Markdown code span.
/// 
/// Examples
/// ========
/// ```
/// let code = r#"println!("Hello world!");"#;
/// let code_span = md_writer::code_span(code);
/// 
/// assert_eq!(code_span, format!("`{code}`"));
/// ```
/// 
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#code-span>
pub fn code_span(code: &str) -> String {
    format!("`{code}`")
}

/// Create a Markdown fenced code block.
/// 
/// Examples
/// ========
/// With an info string:
/// ```
/// let info_string = Some("rust");
/// let code = r#"println!("Hello world!");"#;
/// let fenced_code_block = md_writer::fenced_code_block(code, info_string);
///
/// assert_eq!(fenced_code_block, format!("```rust\n{code}\n```"));
/// ```
/// 
/// Without an info string:
/// ```
/// let code = r#"println!("Hello world!");"#;
/// let fenced_code_block = md_writer::fenced_code_block(code, None);
///
/// assert_eq!(fenced_code_block, format!("```\n{code}\n```"));
/// ```
/// 
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#fenced-code-blocks>
/// - <https://spec.commonmark.org/0.30/#info-string>
pub fn fenced_code_block(code: &str, info_string: Option<&str>) -> String {
    [
        code_fence(info_string),
        code.to_owned(),
        code_fence(None)
    ].join(&LF.to_string())
}

/// Create a Markdown fenced code block with a JavaScript info string.
///
/// Examples
/// ========
/// ```
/// let code = "console.log('Hello world!');";
/// let fenced_js_code_block = md_writer::fenced_js_code_block(code);
///
/// assert_eq!(fenced_js_code_block, format!("```javascript\n{code}\n```"));
/// ```
///
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#fenced-code-blocks>
/// - <https://spec.commonmark.org/0.30/#info-string>
pub fn fenced_js_code_block(code: &str) -> String {
    fenced_code_block(code, Some("javascript"))
}

/// Create a Markdown fenced code block with a Rust info string.
///
/// Examples
/// ========
/// ```
/// let code = r#"println!("Hello world!");"#;
/// let fenced_rs_code_block = md_writer::fenced_rs_code_block(code);
///
/// assert_eq!(fenced_rs_code_block, format!("```rust\n{code}\n```"));
/// ```
///
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#fenced-code-blocks>
/// - <https://spec.commonmark.org/0.30/#info-string>
pub fn fenced_rs_code_block(code: &str) -> String {
    fenced_code_block(code, Some("rust"))
}

/// Create a Markdown fenced code block with a shell script info string.
///
/// Examples
/// ========
/// ```
/// let code = r#"echo "Hello world!""#;
/// let fenced_sh_code_block = md_writer::fenced_sh_code_block(code);
///
/// assert_eq!(fenced_sh_code_block, format!("```shell\n{code}\n```"));
/// ```
///
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#fenced-code-blocks>
/// - <https://spec.commonmark.org/0.30/#info-string>
pub fn fenced_sh_code_block(code: &str) -> String {
    fenced_code_block(code, Some("shell"))
}

/// Create a Markdown fenced code block with a TypeScript info string.
///
/// Examples
/// ========
/// ```
/// let code = "console.log('Hello world!');";
/// let fenced_ts_code_block = md_writer::fenced_ts_code_block(code);
///
/// assert_eq!(fenced_ts_code_block, format!("```typescript\n{code}\n```"));
/// ```
///
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#fenced-code-blocks>
/// - <https://spec.commonmark.org/0.30/#info-string>
pub fn fenced_ts_code_block(code: &str) -> String {
    fenced_code_block(code, Some("typescript"))
}

/// Create a level 1 Markdown setext header.
///
/// Examples
/// ========
/// ```
/// let text = "Hello world!";
/// let h1 = md_writer::h1(text);
/// 
/// assert_eq!(h1, format!("{text}\n============"));
/// ```
///
/// Panics
/// ======
/// This function utilizes the [`repeat`](https://doc.rust-lang.org/std/string/struct.String.html#method.repeat)
/// method on [`String`](https://doc.rust-lang.org/std/string/struct.String.html) and as such will
/// panic if provided a string with too many characters.
/// 
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#setext-headings>
pub fn h1(text: &str) -> String {
    let mut h1 = String::from(text);
    let char_count = text.chars().count();
    let underline = "=".repeat(char_count);

    h1.push(LF);
    h1.push_str(&underline);

    h1
}

/// Create a level 2 Markdown setext header.
///
/// Examples
/// ========
/// ```
/// let text = "Hello world!";
/// let h2 = md_writer::h2(text);
/// 
/// assert_eq!(h2, format!("{text}\n------------"));
/// ```
///
/// Panics
/// ======
/// This function utilizes the [`repeat`](https://doc.rust-lang.org/std/string/struct.String.html#method.repeat)
/// method on [`String`](https://doc.rust-lang.org/std/string/struct.String.html) and as such will
/// panic if provided a string with too many characters.
///
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#setext-headings>
pub fn h2(text: &str) -> String {
    let mut h2 = String::from(text);
    let char_count = text.chars().count();
    let underline = "-".repeat(char_count);

    h2.push(LF);
    h2.push_str(&underline);

    h2
}

/// Create a level 3 Markdown ATX header.
///
/// Examples
/// ========
/// ```
/// let text = "Hello world!";
/// let h3 = md_writer::h3(text);
/// 
/// assert_eq!(h3, format!("### {text}"));
/// ```
/// 
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#atx-heading>
pub fn h3(text: &str) -> String {
    format!("### {text}")
}

/// Create a level 4 Markdown ATX header.
///
/// Examples
/// ========
/// ```
/// let text = "Hello world!";
/// let h4 = md_writer::h4(text);
/// 
/// assert_eq!(h4, format!("#### {text}"));
/// ```
/// 
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#atx-heading>
pub fn h4(text: &str) -> String {
    format!("#### {text}")
}

/// Create a level 5 Markdown ATX header.
///
/// Examples
/// ========
/// ```
/// let text = "Hello world!";
/// let h5 = md_writer::h5(text);
///
/// assert_eq!(h5, format!("##### {text}"));
/// ```
///
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#atx-heading>
pub fn h5(text: &str) -> String {
    format!("##### {text}")
}

/// Create a level 6 Markdown ATX header.
///
/// Examples
/// ========
/// ```
/// let text = "Hello world!";
/// let h6 = md_writer::h6(text);
///
/// assert_eq!(h6, format!("###### {text}"));
/// ```
///
/// Reference
/// =========
/// - <https://spec.commonmark.org/0.30/#atx-heading>
pub fn h6(text: &str) -> String {
    format!("###### {text}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h1_returns_a_lvl1_header() {
        let text = "Hello!";
        let result = h1(text);

        assert_eq!(result, "Hello!\n======");
    }

    #[test]
    fn h2_returns_a_lvl2_header() {
        let text = "Hello!";
        let result = h2(text);

        assert_eq!(result, "Hello!\n------");
    }

    #[test]
    fn h3_returns_a_lvl3_header() {
        let text = "Hello!";
        let result = h3(text);

        assert_eq!(result, "### Hello!");
    }

    #[test]
    fn h4_returns_a_lvl4_header() {
        let text = "Hello!";
        let result = h4(text);

        assert_eq!(result, "#### Hello!");
    }

    #[test]
    fn h5_returns_a_lvl5_header() {
        let text = "Hello!";
        let result = h5(text);

        assert_eq!(result, "##### Hello!");
    }

    #[test]
    fn h6_returns_a_lvl6_header() {
        let text = "Hello!";
        let result = h6(text);

        assert_eq!(result, "###### Hello!");
    }
}
