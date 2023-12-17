pub fn decode(input: &str) -> String {
    let mut output = String::new();
    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        if c == '%' {
            let hex = match (iter.next(), iter.next()) {
                (Some(first), Some(second)) => vec![first, second],
                _ => vec![],
            };

            if !hex.is_empty() {
                if let Ok(decoded) = u8::from_str_radix(&hex.iter().collect::<String>(), 16) {
                    output.push(decoded as char);
                } else {
                    output.push('%');
                    output.push_str(&hex.iter().collect::<String>());
                }
            } else {
                output.push('%');
            }
        } else {
            output.push(c);
        }
    }

    output
}

fn encode(input: &str) -> String {
    let mut output = String::new();

    for c in input.chars() {
        if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
            output.push(c);
        } else {
            output.push_str(&format!("%{:02X}", c as u8));
        }
    }

    output
}
