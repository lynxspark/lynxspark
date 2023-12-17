pub fn is_match(pattern: &str, text: &str) -> bool {
    let mut p_chars = pattern.chars();
    let mut t_chars = text.chars();

    loop {
        match (p_chars.next(), t_chars.next()) {
            (Some(p), Some(t)) => {
                if p == '?' {
                    continue;
                } else if p == '*' {
                    if let Some(next_p) = p_chars.next() {
                        while let Some(next_t) = t_chars.next() {
                            if is_match(&format!("{}{}", next_p, p_chars.as_str()), t_chars.as_str()) {
                                return true;
                            }
                            if next_t == next_p {
                                break;
                            }
                        }
                    } else {
                        return true;
                    }
                } else if p == '[' {
                    let mut in_brackets = false;
                    let mut inverted = false;

                    while let Some(pc) = p_chars.next() {
                        if in_brackets {
                            if pc == ']' {
                                break;
                            } else if pc == '^' {
                                inverted = true;
                            } else if pc == t {
                                if !inverted {
                                    return true;
                                }
                            }
                        } else if pc == t {
                            if !inverted {
                                return true;
                            }
                        } else if pc == ']' {
                            in_brackets = true;
                        }
                    }
                } else if p == '\\' {
                    if let Some(escaped_char) = p_chars.next() {
                        if escaped_char != t {
                            return false;
                        }
                    }
                } else if p != t {
                    return false;
                }
            }
            (None, None) => return true,
            (_, None) => return false,
            (None, _) => return false,
        }
    }
}