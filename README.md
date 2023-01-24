md-writer
=========
A collection of utilities to help make writing Markdown easier.

Usage
-----
```rust
// Create a Markdown code fence.

// With an info string:
let info_string = Some("rust");
let code_fence = md_writer::code_fence(info_string);

assert_eq!(code_fence, "```rust");

// Without an info string:
let code_fence = md_writer::code_fence(None);

assert_eq!(code_fence, "```");

// Create a Markdown code span.
let code = r#"println!("Hello world!");"#;
let code_span = md_writer::code_span(code);

assert_eq!(code_span, format!("`{code}`"));

// Create a Markdown fenced code block.

// With an info string:
let info_string = Some("rust");
let code = r#"println!("Hello world!");"#;
let fenced_code_block = md_writer::fenced_code_block(code, info_string);

assert_eq!(fenced_code_block, format!("```rust\n{code}\n```"));

// Without an info string:
let code = r#"println!("Hello world!");"#;
let fenced_code_block = md_writer::fenced_code_block(code, None);

assert_eq!(fenced_code_block, format!("```\n{code}\n```"));

// Create a Markdown fenced code block with a JavaScript info string.
let code = "console.log('Hello world!');";
let fenced_js_code_block = md_writer::fenced_js_code_block(code);

assert_eq!(fenced_js_code_block, format!("```javascript\n{code}\n```"));

// Create a Markdown fenced code block with a Rust info string.
let code = r#"println!("Hello world!");"#;
let fenced_rs_code_block = md_writer::fenced_rs_code_block(code);

assert_eq!(fenced_rs_code_block, format!("```rust\n{code}\n```"));

// Create a Markdown fenced code block with a shell script info string.
let code = r#"echo "Hello world!""#;
let fenced_sh_code_block = md_writer::fenced_sh_code_block(code);

assert_eq!(fenced_sh_code_block, format!("```shell\n{code}\n```"));

// Create a Markdown fenced code block with a TypeScript info string.
let code = "console.log('Hello world!');";
let fenced_ts_code_block = md_writer::fenced_ts_code_block(code);

assert_eq!(fenced_ts_code_block, format!("```typescript\n{code}\n```"));

// Create a level 1 Markdown setext header.
let text = "Hello world!";
let h1 = md_writer::h1(text);

assert_eq!(h1, format!("{text}\n============"));

// Create a level 2 Markdown setext header.
let text = "Hello world!";
let h2 = md_writer::h2(text);

assert_eq!(h2, format!("{text}\n------------"));

// Create a level 3 Markdown ATX header.
let text = "Hello world!";
let h3 = md_writer::h3(text);

assert_eq!(h3, format!("### {text}"));

// Create a level 4 Markdown ATX header.
let text = "Hello world!";
let h4 = md_writer::h4(text);

assert_eq!(h4, format!("#### {text}"));

// Create a level 5 Markdown ATX header.
let text = "Hello world!";
let h5 = md_writer::h5(text);

assert_eq!(h5, format!("##### {text}"));

// Create a level 6 Markdown ATX header.
let text = "Hello world!";
let h6 = md_writer::h6(text);

assert_eq!(h6, format!("###### {text}"));
```

License
-------
The MIT License. See the [license file](LICENSE) for details.
