pub fn format_code(code: String) -> Result<String, Box<dyn std::error::Error>> {
    // There is no point in formatting if you dont have a code to format
    // we will just return the code if it is found to be empty.
    if code.is_empty() {
        return Ok(code);
    }

    let mut formatted = String::new();
    let mut indent_level = 0;
    let indent = "\t"; // tab
    let mut chars = code.chars().peekable();

    while let Some(c) = chars.next() {
        // Special-case #{ 
        if c == '#' {
            if let Some(&'{') = chars.peek() {
                formatted.push_str("#{" );
                chars.next();

                indent_level += 1;
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));
                continue; // skip the rest of match
            } else {
                formatted.push(c);
                continue;
            }
        }

        // Normal matching
        match c {
            '{' | '[' => {
                formatted.push(c);
                // formatted.push('\n');
                indent_level += 1;
                formatted.push_str(&indent.repeat(indent_level));
            }
            '}' | ']' => {
                formatted.push('\n');
                indent_level = indent_level.saturating_sub(1);
                formatted.push_str(&indent.repeat(indent_level));
                formatted.push(c);
            }
            ',' => {
                formatted.push(c);
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));
            }
            ';' => {
                formatted.push(c);
                formatted.push('\n');
                formatted.push_str(&indent.repeat(indent_level));
            }
            _ => {
                formatted.push(c);
            }
        }
    }


    let formatted = formatted.replace("}\nenter(", "}\n\nenter(");

    Ok(formatted)
}