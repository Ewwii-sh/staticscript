pub fn format_code(code: String) -> Result<String, Box<dyn std::error::Error>> {
    if code.is_empty() {
        return Ok(code);
    }

    #[derive(Debug, PartialEq)]
    enum Context {
        Curly,  // { or #{
        Square, // [
        Paren,  // (
    }

    let mut formatted = String::new();
    let mut indent_level = 0;
    let indent = "\t"; // tab
    let mut chars = code.chars().peekable();
    let mut ctx_stack: Vec<Context> = Vec::new();

    while let Some(c) = chars.next() {
        // Special-case #{
        if c == '#' {
            if let Some(&'{') = chars.peek() {
                formatted.push_str("#{");
                chars.next();

                indent_level += 1;
                ctx_stack.push(Context::Curly);
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));
                continue;
            } else {
                formatted.push(c);
                continue;
            }
        }

        match c {
            '{' => {
                formatted.push(c);
                indent_level += 1;
                ctx_stack.push(Context::Curly);
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));
            }
            '[' => {
                formatted.push(c);
                indent_level += 1;
                ctx_stack.push(Context::Square);
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));
            }
            '(' => {
                formatted.push(c);
                ctx_stack.push(Context::Paren);
            }
            '}' => {
                indent_level = indent_level.saturating_sub(1);
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));
                formatted.push(c);
                if ctx_stack.last() == Some(&Context::Curly) {
                    ctx_stack.pop();
                }
            }
            ']' => {
                indent_level = indent_level.saturating_sub(1);
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));
                formatted.push(c);
                if ctx_stack.last() == Some(&Context::Square) {
                    ctx_stack.pop();
                }
            }
            ')' => {
                formatted.push(c);
                if ctx_stack.last() == Some(&Context::Paren) {
                    ctx_stack.pop();
                }
            }
            ',' | ';' => {
                formatted.push(c);
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));
            }
            _ => {
                formatted.push(c);
            }
        }
    }

    // Extra spacing between top-level blocks
    let formatted = formatted.replace("}\nenter(", "}\n\nenter(");

    Ok(formatted)
}
